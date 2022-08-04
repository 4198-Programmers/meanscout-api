#[macro_use] extern crate rocket;
use rocket::serde::json::Json;
mod csvstuff;
use rocket::fs::NamedFile;


#[get("/")]
async fn index() -> &'static str {
    "Hello, world!"
}

#[post("/scouting", format="json", data="<csv>")]       // The thing for post requests
async fn scouting_post(csv: Json<csvstuff::FormData<'_>>) -> String {
    let mut owned_string: String = "".to_owned();       // Original String
    let mut thing: String;      // Placeholder string
    // Puts all of the data into a vector
    let yes = [csv.team.to_string(), csv.matchnum.to_string(), csv.absent.to_string(), csv.teamlefttarm.to_string(), csv.teamcollecte.to_string(), csv.toppre.to_string(), csv.bottompre.to_string(), csv.missedpre.to_string(), csv.top.to_string(), csv.bottom.to_string(), csv.missed.to_string(), csv.safeareausag.to_string(), csv.defenceplaye.to_string(), csv.barnumberrea.to_string(), csv.teamattempts.to_string(), csv.anyrobotprob.to_string(), csv.extranotes.to_string(), csv.driveteamrat.to_string()];
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
    NamedFile::open("./data.csv").await.ok()    // Returns the filename
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
    .merge(("port", 80));
    // .finalize();

    let _ = rocket::custom(config)
        .mount("/", routes![index, scouting_post, scouting_get, scouting_delete])  // Just put all of the routes in here
        .launch()
        .await;
}
