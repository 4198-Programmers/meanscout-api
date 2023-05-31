#[macro_use]
extern crate rocket;
#[macro_use]
extern crate tera;
#[macro_use]
extern crate lazy_static;
use chrono;

use rocket::request::{FromRequest, Outcome};
use rocket::serde::json::Json;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::fs::NamedFile;
use rocket::http::Header;
use rocket::http::Status;
use rocket::{Request, Response};
use serde_json::Value;

use std::io::Write;
use std::fs::File;
use std::io::prelude::*;

mod csvstuff;
mod settings;
mod catchers;
mod graphs;
mod paths;

// Just a silly little easter egg
#[get("/teapot")]
async fn teapot() -> Status {
    Status::ImATeapot
}

// A basic implementation of Catching Headers
struct ApiKey<'r>(&'r str);

#[derive(Debug)]
enum ApiKeyError {
    Missing,
    Invalid,
}

struct PassKey<'r>(&'r str);

// Request header checks
#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey<'r> {
    type Error = ApiKeyError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        /// Returns true if `key` is a valid API key string.
        fn is_valid(key: &str) -> bool {
            key == "valid_api_key"
        }

        match req.headers().get_one("x-api-key") {
            None => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
            Some(key) if is_valid(key) => Outcome::Success(ApiKey(key)),
            Some(_) => Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid)),
        }
    }
}

// Password header checks
#[rocket::async_trait]
impl<'r> FromRequest<'r> for PassKey<'r> {
    type Error = ApiKeyError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        
        let passwords = ["ChangeMe!".to_string()];
        /// Returns true if `key` is a valid API key string.
        fn is_valid(key: &str, passwords: Vec<String>) -> bool {
            passwords.contains(&format!("{}", key))
        }

        match req.headers().get_one("x-pass-key") {
            None => Outcome::Failure((Status::Unauthorized, ApiKeyError::Missing)),
            Some(key) if is_valid(key, passwords.into()) => Outcome::Success(PassKey(key)),
            Some(_) => Outcome::Failure((Status::Unauthorized, ApiKeyError::Invalid)),
        }
    }
}

#[get("/sensitive")]
fn sensitive(_key: ApiKey<'_>) -> &'static str {
    "Sensitive data."
}

pub struct CORS;

// Needed implementation of CORS headers
#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[get("/")]
async fn index() -> &'static str {
    "Hello, world!"
}

/// Catches all OPTION requests in order to get the CORS related Fairing triggered.
#[options("/<_..>")]
fn all_options() {
    /* Intentionally left empty */
}

// Mainly so it doesn't keep returning 404 on any webpage that's pulled up on here
#[get("/favicon.ico")]
async fn favicon() -> Option<NamedFile> {
    NamedFile::open("favicon.ico").await.ok()
}

// Accepting POST requests from Meanscout
#[post("/scouting", format = "json", data = "<json>")]
async fn scouting_post(_key: PassKey<'_>, json: String) -> Status {

    let mut file = std::fs::OpenOptions::new()
      .append(true)
      .open("test.json")
      .unwrap();

    let _ = writeln!(file, "{}", format!("{}", json));

    let data: csvstuff::Data = serde_json::from_str(&json).unwrap();

    let mut owned_string: String = "".to_owned(); // String for later to append to
    let mut thing: String; // Placeholder string

    let mut hash_vec: Vec<(&String, &Value)> = data.data.iter().collect();

    hash_vec.sort_by(|a, b| {
        a.1.as_object().expect("Failed to turn into object").get_key_value("category").expect("Failed to get category").1.as_str().expect("Failed to get content").cmp(
            b.1.as_object().expect("Failed to turn into object").get_key_value("category").expect("Failed to get category").1.as_str().expect("Failed to get content")
        )
    });

    if csvstuff::file_empty("data.csv".into()).unwrap() {
        let mut header: String = "".to_owned();
        println!("yeah");
        let mapped: Vec<String> = hash_vec.iter().map(|point| point.0.to_string()).collect();
        for val in mapped {header.push_str(format!("{},", val).as_str())}
        let _ = csvstuff::append_csv(&header);
    }

    for i in hash_vec {
        // Iterates through the list and appends the data to a string
        thing = format!("{}, ", i.1.as_object().unwrap().get_key_value("content").expect("Failed to get content").1.to_string().replace(",", ""));
        owned_string.push_str(&thing)
    }
    match csvstuff::append_csv(&owned_string) {
        Ok(_e) => {}
        Err(error) => {
            error!(format!("Uh oh, {}", error));
            return Status::InternalServerError
        }
    } // Adds the information to data.csv
    return Status::Accepted; // Returns accepted status when done
}

// Accepting POST requests from Meanscout
#[post("/pits", data = "<json>", format = "json")]
async fn pits_post(_key: PassKey<'_>, json: String) -> Status {
    let mut file = std::fs::OpenOptions::new()
      .append(true)
      .open("test.json")
      .unwrap();

    let _ = writeln!(file, "{}", format!("{}", json));

    let data: csvstuff::Data = serde_json::from_str(&json).unwrap();

    let mut owned_string: String = "".to_owned(); // String for later to append to
    let mut thing: String; // Placeholder string

    let mut hash_vec: Vec<(&String, &Value)> = data.data.iter().collect();

    hash_vec.sort_by(|a, b| {
        a.1.as_object().expect("Failed to turn into object").get_key_value("category").expect("Failed to get category").1.as_str().expect("Failed to get content").cmp(
            b.1.as_object().expect("Failed to turn into object").get_key_value("category").expect("Failed to get category").1.as_str().expect("Failed to get content")
        )
    });

    if csvstuff::file_empty("pits.csv".into()).unwrap() {
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
    match csvstuff::append_pits(&owned_string) {
        Ok(_e) => {}
        Err(error) => {
            error!(format!("Uh oh, {}", error));
            return Status::InternalServerError
        }
    } // Adds the information to data.csv
    return Status::Accepted; // Returns accepted status when done
}

// When you send a GET request or open it in a web browser it will send the file for data.csv
#[get("/scouting")]
async fn scouting_get() -> Option<NamedFile> {
    NamedFile::open("data.csv").await.ok() // Returns the filename
}

// Function for accepting DELETE requests to delete data.csv
#[delete("/scouting")]
async fn scouting_delete() -> String {
    csvstuff::wipe_data();
    String::from("Wiped data.csv")
}

// Accessing Logs
#[get("/logs")]
async fn logs() -> String {
    let mut file = File::open("logs/scouting.log").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read the file");
    contents
}

#[rocket::main]
async fn main() {
    if cfg!(debug_assertions) {
        // let settings = config::Settings::new().unwrap();
        // println!("{:?}", settings.thing.thin);
        // config::convert_to_mean();
    }


    let config = rocket::Config::figment()
    // The address is set to 0.0.0.0 so it sets the ip to whatever the public network ip is
    .merge(("address", "0.0.0.0"))
    .merge(("port", 8000))
    // Replace the file paths below with wherever your needed pem files are for the right certifications
    // Or comment it out if you want to live the dangerous life
    .merge(("tls.certs", "/etc/letsencrypt/live/data.team4198.org/fullchain.pem"))
    .merge(("tls.key", "/etc/letsencrypt/live/data.team4198.org/privkey.pem"));
    // .finalize();
    match csvstuff::init_files() {
        Ok(_e) => {}
        Err(error) => {
            error!(format!("Uh oh, {}", error))
        }
    }
    success!("Started API");
    let _ = rocket::custom(config)
        .mount(
            "/",
            routes![
                favicon,
                index,
                scouting_post,
                test_post,
                scouting_get,
                logs,
                scouting_delete,
                pits_post,
                all_options,
                sensitive,
                teapot,
                paths::graphs::linegraph,
                paths::graphs::piegraph
            ],
        ) // Just put all of the routes in here
        .register(
            "/",
            catchers![
                catchers::default_catcher, // All of the status code catchers put in catchers.rs goes here
                catchers::not_found,
                catchers::im_a_teapot,
                catchers::bad_request,
                catchers::content_too_large,
                catchers::forbidden,
            ],
        )
        .attach(CORS)
        .launch()
        .await;
}

#[macro_export]
macro_rules! success {
    ( $x:expr ) => {{
        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .open(format!("{}", settings::Settings::new().unwrap().logs_dir))
            .unwrap();
        let _ = writeln!(
            file,
            "[ SUCCESS ] {} - {}",
            chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
            format!("{}", $x)
        );
    }};
}

#[macro_export]
macro_rules! warning {
    ( $x:expr ) => {{
        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .open(format!("{}", settings::Settings::new().unwrap().logs_dir))
            .unwrap();
        let _ = writeln!(
            file,
            "[ WARNING ] {} - {}",
            chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
            format!("{}", $x)
        );
    }};
}

#[macro_export]
macro_rules! error {
    ( $x:expr ) => {{
        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .open(format!("{}", settings::Settings::new().unwrap().logs_dir))
            .unwrap();
        let _ = writeln!(
            file,
            "[ ERROR ] {} - {}",
            chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
            format!("{}", $x)
        );
    }};
}
