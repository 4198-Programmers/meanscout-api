use crate::{data_utils, settings};
#[allow(unused_imports)]
use axum::{
    extract,
    http::{self, header::HeaderMap, HeaderValue, Method, StatusCode},
    response::{Html, IntoResponse},
    Json,
};
use serde_json::Value;
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

/// Silly https path for posting scouting data
pub async fn scouting_post(
    headers: HeaderMap,
    data: string::String,
) -> Result<String, StatusCode> {
    tracing::debug!("- Scouting data was posted to the server");
    let config = crate::settings::Settings::new().expect("Couldn't get settings");

    let test_confirmation = headers.contains_key("x-test");
    let data_directory = if test_confirmation {
        config.test_data_dir
    } else {
        config.stands_data_dir
    };

    let password = headers["x-pass-key"]
        .to_str()
        .unwrap_or("NotAtAllACorrectPassword!!!!! (probably don't edit this)") // Uses the password from the header, if it doesn't exist, it uses a default password that will be wrong
        .to_string();

    if authentication(password).is_err() {
        tracing::debug!("- Password was incorrect");
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Adds a backup of the data to a backup file in config.stands_data_json
    let filename = format!("backup-{}.json", chrono::Local::now().format("%Y-%m-%d-%H-%M-%S"));
    data_utils::backup_json(&data, &filename).expect("Couldn't backup data");

    let mut json_data: Value = serde_json::from_str(&data).unwrap();

    let teleop_datapoints = ["teleop-scored-in-amp", "teleop-missed-in-amp", "teleop-scored-in-non-amplified", "teleop-scored-in-amplified", "teleop-missed-in-speaker", "teleop-scored-in-trap", "teleop-missed-in-trap"];
    for point in teleop_datapoints {json_data["data"][point] = 0.into()}
    if let Some(teleop_scoring) = json_data.clone()["data"]["teleop-scoring-2024"].as_array() {
        for point in teleop_scoring {
            match point.as_str() {  // Adds the total of each point to the corresponding data object
                Some("as") => json_data["data"]["teleop-scored-in-amp"] = (json_data["data"]["teleop-scored-in-amp"].as_i64().expect("Something went wrong as")+1).into(),
                Some("am") => json_data["data"]["teleop-missed-in-amp"] = (json_data["data"]["teleop-missed-in-amp"].as_i64().expect("Something went wrong am")+1).into(),
                Some("ss") => json_data["data"]["teleop-scored-in-non-amplified"] = (json_data["data"]["teleop-scored-in-non-amplified"].as_i64().expect("Something went wrong ss")+1).into(),
                Some("sa") => json_data["data"]["teleop-scored-in-amplified"] = (json_data["data"]["teleop-scored-in-amplified"].as_i64().expect("Something went wrong sa")+1).into(),
                Some("sm") => json_data["data"]["teleop-missed-in-speaker"] = (json_data["data"]["teleop-missed-in-speaker"].as_i64().expect("Something went wrong sm")+1).into(),
                Some("ts") => json_data["data"]["teleop-scored-in-trap"] = (json_data["data"]["teleop-scored-in-trap"].as_i64().expect("Something went wrong ts")+1).into(),
                Some("tm ") => json_data["data"]["teleop-missed-in-trap"] = (json_data["data"]["teleop-missed-in-trap"].as_i64().expect("Something went wrong tm")+1).into(),
                _ => {tracing::info!("Another datapoint was detected in teleop-scoring-2024")} // Ignore other values, but log it anyway
            }
        }
    }

    let auto_datapoints = ["auto-scored-in-amp", "auto-missed-in-amp", "auto-scored-in-speaker", "auto-missed-in-speaker"];
    for point in auto_datapoints {json_data["data"][point] = 0.into()}
    if let Some(teleop_scoring) = json_data.clone()["data"]["auto-scoring-2024"].as_array() {
        for point in teleop_scoring {
            match point.as_str() {  // Adds the total of each point to the corresponding data object
                Some("as") => json_data["data"]["auto-scored-in-amp"] = (json_data["data"]["auto-scored-in-amp"].as_i64().expect("Something went wrong as")+1).into(),
                Some("am") => json_data["data"]["auto-missed-in-amp"] = (json_data["data"]["auto-missed-in-amp"].as_i64().expect("Something went wrong am")+1).into(),
                Some("ss") => json_data["data"]["auto-scored-in-speaker"] = (json_data["data"]["auto-scored-in-speaker"].as_i64().expect("Something went wrong ss")+1).into(),
                Some("sm") => json_data["data"]["auto-missed-in-speaker"] = (json_data["data"]["auto-missed-in-speaker"].as_i64().expect("Something went wrong sm")+1).into(),
                _ => {tracing::info!("Another datapoint was detected in teleop-scoring-2024")} // Ignore other values, but log it anyway
            }
        }
    }
    json_data["data"].as_object_mut().unwrap().remove("teleop-scoring-2024");   // Removes the list of points scored, as it's no longer needed
    json_data["data"].as_object_mut().unwrap().remove("auto-scoring-2024");     // Removes the list of points scored, as it's no longer needed
    println!("{}", json_data.to_string());

    let new_data = json_data.clone().to_string();

    let flattener = Flattener::new()
        .set_key_separator(".")
        .set_array_formatting(ArrayFormatting::Surrounded{
            start: "[".to_string(),
            end: "]".to_string()
        })
        .set_preserve_empty_arrays(true)
        .set_preserve_empty_objects(true);
    let input = new_data.as_bytes();
    let mut output = Vec::<u8>::new();

    let csv_writer = csv::WriterBuilder::new()
        .delimiter(b',')
        .has_headers(false)
        .from_writer(&mut output);
    Json2Csv::new(flattener)
        .convert_from_reader(input, csv_writer)
        .expect("Couldn't convert csv");

    // If there are no headers, it makes headers
    if data_utils::file_empty(&data_directory).expect("Couldn't check if file was empty") {
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
