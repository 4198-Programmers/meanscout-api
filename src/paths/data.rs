#[allow(unused_imports)]
use axum::{
    http::{self, HeaderValue, header::HeaderMap, Method, StatusCode},
    response::{Html, IntoResponse},
    Json,
    extract,
};
use serde_json::Value;
use std::io::{Write, Read};
use std::fs::File;
use crate::{log_debug, log_success, log_error, csvstuff, settings};

/// Function for authentication (duh)
pub fn authentication(password: String) -> Result<String, StatusCode> {
    let settings = crate::settings::Settings::new().unwrap();
    if settings.passwords.contains(&password) {
        return Ok("It worked!".into())
    } else {
        return Err(StatusCode::UNAUTHORIZED)
    }
}

pub async fn scouting_post(headers: HeaderMap, extract::Json(data): Json<csvstuff::Data>) -> Result<String, StatusCode> {
    log_debug!("- Scouting data was posted to the server");
    let settings = crate::settings::Settings::new().unwrap();

    let test_confirmation = headers.contains_key("x-test");
    let data_directory = if test_confirmation { settings.test_data_dir } else { settings.stands_data_dir };
    let password = headers["x-pass-key"].to_str().unwrap_or("NotAtAllACorrectPassword").to_string();

    if authentication(password).is_err() {
        log_debug!("- Password was incorrect");
        return Err(StatusCode::UNAUTHORIZED)
    }
    
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
    if csvstuff::file_empty(&data_directory).unwrap() {
        log_success!("File was empty, made headers");
        let mut header: String = "".to_owned();
        let mapped: Vec<String> = hash_vec.iter().map(|point| point.0.to_string()).collect();
        for val in mapped {header.push_str(format!("{},", val).as_str())}
        let _ = csvstuff::append(&header, &data_directory);
    }

    for i in hash_vec {
        // Iterates through the list and appends the data to a string
        thing = format!("{}, ", i.1.as_object().unwrap().get_key_value("content").expect("Failed to get content").1.to_string().replace(",", ""));
        owned_string.push_str(&thing)
    }

    // Adds the information to data.csv
    match csvstuff::append(&owned_string, &data_directory) {
        Ok(_e) => {}
        Err(error) => {
            log_error!(format!("Uh oh, {}", error));
            return Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    } 
    return Ok("It worked!".into()); // Returns accepted status when done
}

/// Silly http path for pits
pub async fn pits_post(headers: HeaderMap, extract::Json(data): Json<csvstuff::Data>) -> Result<String, StatusCode> {
    log_debug!("- Pits data was posted to the server");
    let settings = crate::settings::Settings::new().unwrap();

    let test_confirmation = headers.contains_key("x-test");
    let data_directory = if test_confirmation { settings.test_data_dir } else { settings.pits_data_dir };
    let password = headers["x-pass-key"].to_str().unwrap_or("NotAtAllACorrectPassword").to_string();

    if authentication(password).is_err() {
        log_debug!("- Password was incorrect");
        return Err(StatusCode::UNAUTHORIZED)
    }
    
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
    if csvstuff::file_empty(&data_directory).unwrap() {
        log_success!("File was empty, made headers");
        let mut header: String = "".to_owned();
        let mapped: Vec<String> = hash_vec.iter().map(|point| point.0.to_string()).collect();
        for val in mapped {header.push_str(format!("{},", val).as_str())}
        let _ = csvstuff::append(&header, &data_directory);
    }

    for i in hash_vec {
        // Iterates through the list and appends the data to a string
        thing = format!("{}, ", i.1.as_object().unwrap().get_key_value("content").expect("Failed to get content").1.to_string().replace(",", ""));
        owned_string.push_str(&thing)
    }

    // Adds the information to data.csv
    match csvstuff::append(&owned_string, &data_directory) {
        Ok(_e) => {}
        Err(error) => {
            log_error!(format!("Uh oh, {}", error));
            return Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    } 
    return Ok("It worked!".into()); // Returns accepted status when done
}

/// Simply getting the contents of the data.csv file
pub async fn scouting_get() -> Result<impl IntoResponse, StatusCode> {
    let data_list = csvstuff::get_data().unwrap();
    Ok(data_list)
}