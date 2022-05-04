use std::path::Path;
use std::fs::File;
use std::io::{Write, prelude::*, BufReader};
use serde::{Serialize, Deserialize};
use std::borrow::Cow;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct FormData<'r> {
    pub team: Cow<'r, i64>,
    pub matchnum: Cow<'r, str>,
    pub absent: Cow<'r, bool>,
    pub teamlefttarm: Cow<'r, bool>,
    pub teamcollecte: Cow<'r, bool>,
    pub toppre: Cow<'r, i64>,
    pub bottompre: Cow<'r, i64>,
    pub missedpre: Cow<'r, i64>,
    pub top: Cow<'r, i64>,
    pub bottom: Cow<'r, i64>,
    pub missed: Cow<'r, i64>,
    pub safeareausag: Cow<'r, str>,
    pub defenceplaye: Cow<'r, str>,
    pub barnumberrea: Cow<'r, str>,
    pub teamattempts: Cow<'r, bool>,
    pub anyrobotprob: Cow<'r, str>,
    pub extranotes: Cow<'r, str>,
    pub driveteamrat: Cow<'r, str>,
    pub password: Cow<'r, str>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct FormDataTest<'r> {
    test: Cow<'r, str>
}

/// Initializing the users and loginpage files
fn init_files() {
    if !file_exists("data.csv") {
        let _userfile = File::create("data.csv");
    }
    return
}

/// Checks if file exists
pub fn file_exists(file: &str) -> bool {
    return Path::new(file).exists()
  }

pub fn append_csv(content: &str) {
    init_files();
    let mut file = fs::OpenOptions::new()
      .append(true)
      .open("data.csv")
      .unwrap();
    
    let _ = writeln!(file, "{}", format!("{}", content));
    return
}

/// Wipes data.csv
pub fn wipe_data() {
    let _ = fs::write("data.csv", "");
    return
  }

pub fn read_file(path: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(&path).expect("Cannot open file");

    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}