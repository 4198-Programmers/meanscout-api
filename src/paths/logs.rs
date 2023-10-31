use axum::response::{Html, IntoResponse};
use serde_json::Value;
use std::io::{Write, Read};
use std::fs::File;
use crate::{log_debug, log_success, log_error, csvstuff, settings};

pub async fn logs_get() -> impl IntoResponse {
    let settings = crate::settings::Settings::new().unwrap();
    let mut file = File::open(settings.logs_dir).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}