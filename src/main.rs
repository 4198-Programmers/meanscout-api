#[macro_use] extern crate rocket;
use rocket::serde::json::Json;
mod csvstuff;
use rocket::fs::NamedFile;
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
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

#[post("/scouting", data="<csv>")]       // The thing for post requests
async fn scouting_post(csv: Json<csvstuff::FormData<'_>>) -> String {
    let passwords = ["password".to_string()];
    
    if passwords.contains(&csv.password.to_string()) == false {return "bad you didn't use the password".to_string()}    // If the json interpreted doesn't have the right password, it's bad
    let mut owned_string: String = "".to_owned();       // Original String
    let mut thing: String;      // Placeholder string
    // Puts all of the data into a vector
    let yes = [csv.team.to_string(), csv.matchnum.to_string(), csv.absent.to_string().to_uppercase(), csv.name.to_string(), csv.location.to_string(), csv.teamlefttarm.to_string().to_uppercase(), csv.teamcollecte.to_string().to_uppercase(), csv.toppre.to_string(), csv.bottompre.to_string(), csv.missedpre.to_string(), csv.top.to_string(), csv.bottom.to_string(), csv.missed.to_string(), csv.safeareausag.to_string(), csv.defenceplaye.to_string(), csv.barnumberrea.to_string(), csv.teamattempts.to_string().to_uppercase(), csv.roughestimat.to_string().replace(" seconds", ""), csv.anyrobotprob.to_string(), csv.extranotes.to_string().replace(",", ""), csv.driveteamrat.to_string().replace(",", "")];
    for i in yes.iter() {   // Iterates through the list and appends the data to a string
        thing = format!("{}, ", i);
        if String::from(i) == csv.driveteamrat.to_string() {
            thing = format!("{}", i)
        }
        owned_string.push_str(&thing)
    }
    csvstuff::append_csv(&owned_string);    // Adds the stuff to data.csv
    String::from("Added Data!")     // Returns Added Data!
}

#[get("/scouting")]     // The thing for get requests
async fn scouting_get() -> Option<NamedFile>{
    NamedFile::open("~/4198/scouting_data/current/data.csv").await.ok()    // Returns the filename
}

#[delete("/scouting")]      // The thing for delete requests
async fn scouting_delete() -> String {
    csvstuff::wipe_data();
    String::from("Wiped data.csv")
}

#[rocket::main]
async fn main() {
    let config = rocket::Config::figment()
    .merge(("address", "0.0.0.0"))
    .merge(("port", 8000));
    // .merge(("tls.certs", "cert.pem"))
    // .merge(("tls.key", "key.pem"));
    // .finalize();

    let _ = rocket::custom(config)
        .mount("/", routes![index, scouting_post, scouting_get, scouting_delete, all_options])  // Just put all of the routes in here
        .attach(CORS)
        .launch()
        .await;
}

#[macro_export]
macro_rules! error {
    ( $x:expr ) => {{    
        let mut file = fs::OpenOptions::new()
        .append(true)
        .open("logs/yes.log.csv")
        .unwrap();
      
        let _ = writeln!(file, "[ERROR] [time] - {}", format!("{}", $x));   
    }};
}

#[macro_export]
macro_rules! success {
    ( $x:expr ) => {{    
        let mut file = fs::OpenOptions::new()
        .append(true)
        .open("logs/yes.log.csv")
        .unwrap();
      
         let _ = writeln!(file, "[SUCCESS] [time] - {}", format!("{}", $x));    
    }};
}

#[macro_export]
macro_rules! warning {
    ( $x:expr ) => {{    
        let mut file = fs::OpenOptions::new()
        .append(true)
        .open("logs/yes.log.csv")
        .unwrap();
      
        let _ = writeln!(file, "[WARNING] [time] - {}", format!("{}", $x));   
    }};
}