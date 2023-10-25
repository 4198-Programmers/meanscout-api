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

#[tokio::main]
async fn main() {
    let config = settings::Settings::new().unwrap();

    let frontend = async {
       let app = Router::new().route("/", get(index))
       .route("/scouting", get(paths::data::scouting_get))
       .nest_service("/favicon.ico", ServeDir::new("favicon.ico"));
       serve(app, config.frontend_port).await;
    };

    let backend = async {
        let app = Router::new()
            .route("/scouting", post(paths::data::scouting_post)).layer(
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
            .route("/pits", post(paths::data::pits_post)).layer(
                CorsLayer::new()
                    .allow_origin("*".parse::<HeaderValue>().unwrap())
                    .allow_methods([Method::POST])
                    .allow_headers(Any),
            );
        serve(app, config.backend_port).await;
    };

    match csvstuff::init_files() {
        Ok(_e) => {}
        Err(error) => {
            log_error!(format!("Uh oh, {}", error));
            println!("- Failed to initialize files");
        }
    }
    
    // tokio::join!(backend);
    tokio::join!(frontend, backend);
}

async fn serve(app: Router, port: u16) {
    let config = settings::Settings::new().unwrap();

    let addr = SocketAddr::from((config.ip_address, port));

    // If built in debug mode, doesn't worry about https stuff
    if cfg!(debug_assertions) {
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        println!("Running on port: {}", &port);
        axum::serve(listener, app.clone()).await.unwrap();
    }

    let tls_config = RustlsConfig::from_pem_file(
        config.tls_cert_dir,
        config.tls_key_dir,
    )
    .await
    .unwrap();

    println!("Running on port: {}", &port);
    axum_server::bind_rustls(addr, tls_config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> impl IntoResponse {
    Html(
        r#"
        <head>
            <title>Scouting Root</title>
            <link rel="icon" type="image/x-icon" href="/favicon.ico">
        </head>
        Hello World!
        "#,
    )
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
