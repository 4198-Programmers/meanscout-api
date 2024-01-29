use crate::{data_utils, settings};
#[allow(unused_imports)]
use axum::{
    extract,
    http::{self, header::HeaderMap, HeaderValue, Method, StatusCode},
    response::{Html, IntoResponse},
    Json,
};
use serde_json::Value;
use csv::Writer;
use std::fs::OpenOptions;
use json_objects_to_csv::flatten_json_object::ArrayFormatting;
use json_objects_to_csv::flatten_json_object::Flattener;
use json_objects_to_csv::Json2Csv;
use std::{io::BufRead, string};

/// Function for authentication (duh)
pub fn authentication(password: String) -> Result<String, StatusCode> {
    let config = crate::settings::Settings::new().unwrap();
    if config.passwords.contains(&password) {
        return Ok("It worked!".into());
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    }
}

pub async fn scouting_post(
    headers: HeaderMap,
    data: string::String,
) -> Result<String, StatusCode> {
    tracing::debug!("- Scouting data was posted to the server");
    let config = crate::settings::Settings::new().unwrap();

    let test_confirmation = headers.contains_key("x-test");
    let data_directory = if test_confirmation {
        config.test_data_dir
    } else {
        config.stands_data_dir
    };
    let mut file = OpenOptions::new()
    .write(true)
    .append(true)
    .create(true)
    .open(&data_directory)
    .unwrap();

    let password = headers["x-pass-key"]
        .to_str()
        .unwrap_or("NotAtAllACorrectPassword") // Uses the password from the header, if it doesn't exist, it uses a default password that will be wrong
        .to_string();

    if authentication(password).is_err() {
        tracing::debug!("- Password was incorrect");
        return Err(StatusCode::UNAUTHORIZED);
    }

    // let mut og_csv_writer = Writer::from_path(&data_directory).unwrap();
    let mut header_written = false;
    // for item in data.data {
    //     let json_value: Value = serde_json::from_str(&item.1.as_str().unwrap()).unwrap();

    //     if !header_written {
    //         let header: Vec<String> = json_value
    //             .as_object()
    //             .unwrap()
    //             .keys()
    //             .map(|key| key.to_string())
    //             .collect();
    //         csv_writer.write_record(&header).unwrap();
    //         header_written = true;
    //     }

    //     let row: Vec<String> = json_value
    //         .as_object()
    //         .unwrap()
    //         .values()
    //         .map(|value| value.to_string())
    //         .collect();
    //     csv_writer.write_record(&row).unwrap();\
    // }

        let flattener = Flattener::new()
            .set_key_separator(".")
            .set_array_formatting(ArrayFormatting::Surrounded{
                start: "[".to_string(),
                end: "]".to_string()
            })
            .set_preserve_empty_arrays(true)
            .set_preserve_empty_objects(true);
        let input = data.as_bytes();
        let mut output = Vec::<u8>::new();

        let csv_writer = csv::WriterBuilder::new()
            .delimiter(b',')
            .has_headers(false)
            .from_writer(&mut output);

        Json2Csv::new(flattener)
            .convert_from_reader(input, csv_writer)
            .expect("Couldn't convert csv");

        if data_utils::file_empty(&data_directory).unwrap() {
            tracing::info!("File was empty, made headers");
            data_utils::append(&output.lines()
                .map(|x| x.unwrap())
                .collect::<Vec<String>>().join("\n"), 
            &data_directory).expect("Couldn't output data to file");
        }
        else {
            data_utils::append(&output.lines()
                .skip(1)
                .map(|x| x.unwrap())
                .collect::<Vec<String>>().join(""),
                &data_directory
            ).expect("Couldn't output data to file");
        }
        return Ok("It worked!".into()); // Returns accepted status when done
}

/// Silly http path for pits
pub async fn pits_post(
    headers: HeaderMap,
    extract::Json(data): Json<data_utils::Data>,
) -> Result<String, StatusCode> {
    tracing::debug!("- Pits data was posted to the server");
    let config = crate::settings::Settings::new().unwrap();

    let test_confirmation = headers.contains_key("x-test");
    let data_directory = if test_confirmation {
        config.test_data_dir
    } else {
        config.pits_data_dir
    };
    let password = headers["x-pass-key"]
        .to_str()
        .unwrap_or("NotAtAllACorrectPassword")
        .to_string();

    if authentication(password).is_err() {
        tracing::debug!("- Password was incorrect");
        return Err(StatusCode::UNAUTHORIZED);
    }

    let mut owned_string: String = "".to_owned(); // String for later to append to
    let mut thing: String; // Placeholder string

    let mut hash_vec: Vec<(&String, &Value)> = data.data.iter().collect();

    // Sorts the vector by category
    hash_vec.sort_by(|a, b| {
        a.1.as_object()
            .expect("Failed to turn into object")
            .get_key_value("category")
            .expect("Failed to get category")
            .1
            .as_str()
            .expect("Failed to get content")
            .cmp(
                b.1.as_object()
                    .expect("Failed to turn into object")
                    .get_key_value("category")
                    .expect("Failed to get category")
                    .1
                    .as_str()
                    .expect("Failed to get content"),
            )
    });

    // Makes the headers if the file is empty
    if data_utils::file_empty(&data_directory).unwrap() {
        tracing::info!("File was empty, made headers");
        let mut header: String = "".to_owned();
        let mapped: Vec<String> = hash_vec.iter().map(|point| point.0.to_string()).collect();
        for val in mapped {
            header.push_str(format!("{},", val).as_str())
        }
        let _ = data_utils::append(&header, &data_directory);
    }

    for i in hash_vec {
        // Iterates through the list and appends the data to a string
        thing = format!(
            "{}, ",
            i.1.as_object()
                .unwrap()
                .get_key_value("content")
                .expect("Failed to get content")
                .1
                .to_string()
                .replace(",", "")
        );
        owned_string.push_str(&thing)
    }

    // Adds the information to data.csv
    match data_utils::append(&owned_string, &data_directory) {
        Ok(_e) => {}
        Err(error) => {
            tracing::error!("Uh oh, {}", error);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
    return Ok("It worked!".into()); // Returns accepted status when done
}

/// Simply getting the contents of the data.csv file
pub async fn scouting_get() -> Result<impl IntoResponse, StatusCode> {
    let config = settings::Settings::new().unwrap();
    let data_list = data_utils::get_data(&config.stands_data_dir).unwrap();
    Ok(data_list)
}

/// Simply getting the contents of the pits.csv file
pub async fn pits_get() -> Result<impl IntoResponse, StatusCode> {
    let config = settings::Settings::new().unwrap();
    let data_list = data_utils::get_data(&config.pits_data_dir).unwrap();
    Ok(data_list)
}
