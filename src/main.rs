#[macro_use] extern crate rocket;
use rocket::tokio::time::{sleep, Duration};
mod csvstuff;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/", routes![index, delay])  // Just put all of the routes in here
        .launch()
        .await;
}
