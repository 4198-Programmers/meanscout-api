#![allow(unused_imports)]
use axum::{
    http::{self, HeaderValue, Method},
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use axum_server::tls_rustls::RustlsConfig;
use std::{net::SocketAddr, path::PathBuf};
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::{ServeDir, ServeFile};
mod csvstuff;
mod settings;
mod paths;
use std::io::Write;

use tracing::{info_span, Span, instrument::WithSubscriber};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer};
use std::{fs::File, sync::Arc};
use std::fs::OpenOptions;

#[tokio::main]
#[tracing::instrument(ret)]
async fn main() {
    let config = settings::Settings::new().unwrap();

    let file = OpenOptions::new()
    .append(true)
    .open("logs/scouting.log")
    .unwrap();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "meanapi=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer().pretty().and_then(tracing_subscriber::fmt::layer().with_writer(Arc::new(file)).with_thread_ids(false).with_thread_names(false).with_ansi(false)))
        .init();

    match csvstuff::init_files() {
        Ok(_e) => {}
        Err(error) => {
            log_error!(format!("Uh oh, {}", error));
            println!("- Failed to initialize files");
        }
    }
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
                    .allow_methods([Method::POST])
                    .allow_headers(Any),
            )
            .route("/api/pits", post(paths::data::pits_post)).layer(
                CorsLayer::new()
                    .allow_origin("*".parse::<HeaderValue>().unwrap())
                    .allow_methods([Method::POST])
                    .allow_headers(Any),
            )
}

async fn serve(app: Router, port: u16) {
    let config = settings::Settings::new().unwrap();

    let addr = SocketAddr::from((config.ip_address, port));

    // If built in debug mode, doesn't worry about https stuff
    if cfg!(debug_assertions) {
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        tracing::debug!("Successfully started on port: {}", &port);
        axum::serve(listener, app.clone()).await.unwrap();
    }

    let tls_config = RustlsConfig::from_pem_file(
        config.tls_cert_dir,
        config.tls_key_dir,
    )
    .await
    .unwrap();

    tracing::debug!("Successfully started on port: {}", &port);
    
    axum_server::bind_rustls(addr, tls_config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> impl IntoResponse {
    "Hello, World!"
}

/// Logs a success into the configured log file
#[macro_export]
macro_rules! log_success {
    ( $x:expr ) => {{
        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .open(format!("{}", settings::Settings::new().expect("Failed to open settings for log").logs_dir))
            .unwrap();
        let _ = writeln!(
            file,
            "[ SUCCESS ] {} - {}",
            chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
            format!("{}", $x)
        );
    }};
}

/// Logs a warning into the configured log file
#[macro_export]
macro_rules! log_warning {
    ( $x:expr ) => {{
        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .open(format!("{}", settings::Settings::new().expect("Failed to open settings for log").logs_dir))
            .unwrap();
        let _ = writeln!(
            file,
            "[ WARNING ] {} - {}",
            chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
            format!("{}", $x)
        );
    }};
}

/// Logs an error into the configured log file
#[macro_export]
macro_rules! log_error {
    ( $x:expr ) => {{
        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .open(format!("{}", settings::Settings::new().expect("Failed to open settings for log").logs_dir))
            .unwrap();
        let _ = writeln!(
            file,
            "[ ERROR ] {} - {}",
            chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
            format!("{}", $x)
        );
    }};
}

/// Logs to the console if in debug mode
#[macro_export]
macro_rules! log_debug {
    ( $x:expr ) => {{
        if cfg!(debug_assertions) {
            println!("{}", $x);
        }
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        extract::connect_info::MockConnectInfo,
        http::{self, Request, StatusCode},
    };
    use serde_json::{json, Value};
    use std::net::SocketAddr;
    use tokio::net::TcpListener;
    use tower::Service; // for `call`
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
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert!(body.is_empty());
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
