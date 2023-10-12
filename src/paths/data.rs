
use axum::{
    http::{self, HeaderValue, header::HeaderMap, Method, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
    extract,
};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use serde_json::Value;
use std::io::Write;
use crate::{debug_log, success, error, csvstuff, settings};

pub fn authentication(password: String) -> Result<String, StatusCode> {
    let settings = crate::settings::Settings::new().unwrap();
    if settings.passwords.contains(&password) {
        return Ok("It worked!".into())
    } else {
        return Err(StatusCode::UNAUTHORIZED)
    }
}

pub async fn scouting_post(headers: HeaderMap, extract::Json(data): Json<csvstuff::Data>) -> Result<String, StatusCode> {
    debug_log!("- Scouting data was posted to the server");
    let settings = crate::settings::Settings::new().unwrap();

    println!("{:?}", headers["x-pass-key"]);
    let password = headers["x-pass-key"].to_str().unwrap().to_string();
    debug_log!("- Password accepted")
    if authentication(password).is_err() {return Err(StatusCode::UNAUTHORIZED)}
    
    let mut owned_string: String = "".to_owned(); // String for later to append to
    let mut thing: String; // Placeholder string

    let mut hash_vec: Vec<(&String, &Value)> = data.data.iter().collect();

    // Sorts the vector by category
    hash_vec.sort_by(|a, b| {
        a.1.as_object().expect("Failed to turn into object").get_key_value("category").expect("Failed to get category").1.as_str().expect("Failed to get content").cmp(
            b.1.as_object().expect("Failed to turn into object").get_key_value("category").expect("Failed to get category").1.as_str().expect("Failed to get content")
        )
    });

    // Makes the headers if the file is empty
    if csvstuff::file_empty(settings.stands_data).unwrap() {
        success!("File was empty, made headers");
        let mut header: String = "".to_owned();
        let mapped: Vec<String> = hash_vec.iter().map(|point| point.0.to_string()).collect();
        for val in mapped {header.push_str(format!("{},", val).as_str())}
        let _ = csvstuff::append_csv(&header);
    }

    for i in hash_vec {
        // Iterates through the list and appends the data to a string
        thing = format!("{}, ", i.1.as_object().unwrap().get_key_value("content").expect("Failed to get content").1.to_string().replace(",", ""));
        owned_string.push_str(&thing)
    }

    // Adds the information to data.csv
    match csvstuff::append_csv(&owned_string) {
        Ok(_e) => {}
        Err(error) => {
            error!(format!("Uh oh, {}", error));
            return Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    } 
    return Ok("It worked!".into()); // Returns accepted status when done
}