#[macro_use] extern crate rocket;
use rocket::serde::json::Json;
mod csvstuff;
use rocket::fs::NamedFile;

#[get("/")]
async fn index() -> &'static str {
    "Hello, world!"
}

#[post("/scouting", format="json", data="<csv>")]
async fn scouting_post(csv: Json<csvstuff::FormData<'_>>) -> String {
    let mut owned_string: String = "".to_owned();
    let mut thing: String;
    let yes = [csv.team.to_string(), csv.matchnum.to_string(), csv.absent.to_string(), csv.teamlefttarm.to_string(), csv.teamcollecte.to_string(), csv.toppre.to_string(), csv.bottompre.to_string(), csv.missedpre.to_string(), csv.top.to_string(), csv.bottom.to_string(), csv.missed.to_string(), csv.safeareausag.to_string(), csv.defenceplaye.to_string(), csv.barnumberrea.to_string(), csv.teamattempts.to_string(), csv.anyrobotprob.to_string(), csv.extranotes.to_string(), csv.driveteamrat.to_string()];
    for i in yes.iter() {
        thing = format!("{}, ", i);
        if String::from(i) == csv.driveteamrat.to_string() {
            thing = format!("{}", i)
        }
        owned_string.push_str(&thing)
    }
    csvstuff::append_csv(&owned_string); 
    String::from("Added Data!")
}

#[get("/scouting")]
async fn scouting_get() -> Option<NamedFile>{
    NamedFile::open("./data.csv").await.ok()
}

#[delete("/scouting")]
async fn scouting_delete() -> String {
    csvstuff::wipe_data();
    String::from("Wiped data.csv")
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![index, scouting_post, scouting_get, scouting_delete])  // Just put all of the routes in here
        .launch()
        .await;
}
