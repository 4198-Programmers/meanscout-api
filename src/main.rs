use axum::{
    http::{HeaderValue, Request, HeaderMap},
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
    extract::MatchedPath,
};
use axum_server::tls_rustls::RustlsConfig;
use tower_http::cors::{Any, CorsLayer};
use tower_http::{
    trace::TraceLayer,
    services::ServeDir,
};
use tracing::{info_span, Span};
use tracing_subscriber::{fmt::time::OffsetTime, layer::SubscriberExt, util::SubscriberInitExt, Layer};
use std::{
    time::Duration, 
    fs::OpenOptions, 
    sync::Arc,
    net::SocketAddr,
};
use time::macros::format_description;
use time::UtcOffset;

mod data_utils;
mod settings;
mod paths;

#[tokio::main]
async fn main() {
    let config = settings::Settings::new().unwrap();

    match data_utils::init_files() {
        Ok(_e) => {}
        Err(error) => {
            tracing::error!("Uh oh, {}", error);
            println!("- Failed to initialize files");
        }
    }

    let file = OpenOptions::new()
    .append(true)
    .open(config.logs_dir)
    .unwrap();

    // Checks for debug mode, and sets the subscriber accordingly
    let subscriber = match cfg!(debug_assertions) {
        true => "meanapi=debug,tower_http=debug,axum::rejection=trace",
        false => "meanapi=info,tower_http=info,axum::rejection=info",
    };

    let offset = UtcOffset::from_hms(-5, 0, 0).expect("should get local offset!");
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                subscriber.into()
            }),
        )
        .with(tracing_subscriber::fmt::layer().pretty()
        .and_then(tracing_subscriber::fmt::layer()
            .with_writer(Arc::new(file))
            .with_timer(OffsetTime::new(offset, format_description!( // Adds the formatted time to the logs
                "[year]-[month]-[day] [hour repr:24]:[minute]:[second] -"
            )))
            .with_thread_ids(false).with_thread_names(false).with_ansi(false)
        ))
        .init();

    
    serve(app(), config.port).await;

}

/// Having a function that produces our app makes it easy to call it from tests
/// without having to create an HTTP server.
fn app() -> Router {
    Router::new()
            // frontend stuff
            .route("/", get(index))
            .route("/api/scouting", get(paths::data::scouting_get))
            .route("/api/pits", get(paths::data::pits_get))
            .route("/api/logs", get(paths::logs::logs_get))
            .nest_service("/favicon.ico", ServeDir::new("favicon.ico"))

            // backend stuff
            .route("/api/scouting", post(paths::data::scouting_post)).layer(
                // see https://docs.rs/tower-http/latest/tower_http/cors/index.html
                // for more details
                //
                // pay attention that for some request types like posting content-type: application/json
                // it is required to add ".allow_headers([http::header::CONTENT_TYPE])" OR ".allow_headers(Any)"
                // or see this issue https://github.com/tokio-rs/axum/issues/849
                CorsLayer::new()
                    .allow_origin("*".parse::<HeaderValue>().unwrap())
                    .allow_methods(Any)
                    .allow_headers(Any),
            )
            .route("/api/pits", post(paths::data::pits_post)).layer(
                CorsLayer::new()
                    .allow_origin("*".parse::<HeaderValue>().unwrap())
                    .allow_methods(Any)
                    .allow_headers(Any),
            )
            .layer(
                TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    // Log the matched route's path (with placeholders not filled in).
                    // Use request.uri() or OriginalUri if you want the real path.
                    let matched_path = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(MatchedPath::as_str);

                    info_span!(
                        "http_request",
                        method = ?request.method(),
                        matched_path,
                        some_other_field = tracing::field::Empty,
                    )
                })
                .on_request(|_request: &Request<_>, _span: &Span| {
                    // You can use `_span.record("some_other_field", value)` in one of these
                    // closures to attach a value to the initially empty field in the info_span
                    // created above.
                })
                .on_response(|_response: &Response, _latency: Duration, _span: &Span| {
                    // ...
                })
                .on_eos(
                    |_trailers: Option<&HeaderMap>, _stream_duration: Duration, _span: &Span| {
                        // ...
                    },
                )
            ).fallback(handler_404)
}

async fn serve(app: Router, port: u16) {
    let config = settings::Settings::new().unwrap();

    let addr = SocketAddr::from((config.ip_address, port));

    // If built in debug mode, doesn't worry about https stuff
    if cfg!(debug_assertions) {
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        tracing::info!("Successfully started on port: {}", &port);
        axum::serve(listener, app.clone()).await.unwrap();
    }

    let tls_config = RustlsConfig::from_pem_file(
        config.tls_cert_dir,
        config.tls_key_dir,
    )
    .await
    .unwrap();

    tracing::info!("Successfully started on port: {}", &port);
    
    axum_server::bind_rustls(addr, tls_config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> impl IntoResponse {
    "Hello, World!"
}

async fn handler_404() -> impl IntoResponse {
    (axum::http::StatusCode::NOT_FOUND, "nothing to see here")
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use serde_json::json;
    use tower::ServiceExt; // for `oneshot` and `ready`

    #[tokio::test]
    async fn root() {
        let app = app();

        // `Router` implements `tower::Service<Request<Body>>` so we can
        // call it like any tower service, no need to run an HTTP server.
        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert_eq!(&body[..], b"Hello, World!");
    }

    #[tokio::test]
    async fn not_found() {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/does-not-exist")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn scouting_post() {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/scouting")
                    .header("x-pass-key", "ChangeMe!")
                    .header("Content-Type", "application/json")
                    .header("x-test", "True")
                    .body(Body::from(
                        json!({
                            "data": {
                                "team": {"content": "1234", "category": "team"},
                                "match": {"content": "1", "category": "match"},
                                "category": {"content": "test", "category": "category"},
                                "content": {"content": "test", "category": "content"},
                            }
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn scouting_post_wrong_password() {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/scouting")
                    .header("x-pass-key", "ThisWillNeverBeThePassword")
                    .header("Content-Type", "application/json")
                    .header("x-test", "True")
                    .body(Body::from(
                        json!({
                            "data": {
                                "team": {"content": "1234", "category": "team"},
                                "match": {"content": "1", "category": "match"},
                                "category": {"content": "test", "category": "category"},
                                "content": {"content": "test", "category": "content"},
                            }
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn pits_post_wrong_password() {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/pits")
                    .header("x-pass-key", "ThisWillNeverBeThePassword")
                    .header("Content-Type", "application/json")
                    .header("x-test", "True")
                    .body(Body::from(
                        json!({
                            "data": {
                                "team": {"content": "1234", "category": "team"},
                                "match": {"content": "1", "category": "match"},
                                "category": {"content": "test", "category": "category"},
                                "content": {"content": "test", "category": "content"},
                            }
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
    
    #[tokio::test]
    async fn pits_post() {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/pits")
                    .header("x-pass-key", "ChangeMe!")
                    .header("Content-Type", "application/json")
                    .header("x-test", "True")
                    .body(Body::from(
                        json!({
                            "data": {
                                "team": {"content": "1234", "category": "team"},
                                "name": {"content": "Clearly, just a name", "category": "name"},
                                "category": {"content": "test", "category": "category"},
                                "content": {"content": "test", "category": "content"},
                            }
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn scouting_get() {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/scouting")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn pits_get() {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/pits")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn get_logs() {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/api/logs")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
