use std::path::Path;
use std::fs::File;
use std::io::Write;
use serde::{Serialize, Deserialize};
use std::borrow::Cow;
use std::fs;
use crate::settings;
use std::error::Error;
use std::io::prelude::*;
// use config::{ConfigError, Config};


#[derive(Serialize, Deserialize)]
pub struct Data {
    pub data: serde_json::Map<String, serde_json::Value>,
}



/// Struct for the form data
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct FormData<'r> {
    pub team: Cow<'r, str>,
    pub matchnum: Cow<'r, str>,
    pub absent: Cow<'r, bool>,
    pub name: Cow<'r, str>,
    pub location: Cow<'r, str>, //
    pub teamleftcommu: Cow<'r, bool>,
    pub teamcollected: Cow<'r, bool>,
    pub autochargesta: Cow<'r, str>,
    pub toggletesting: Cow<'r, [GridData<'r>; 27]>,
    // pub topcubes: Cow<'r, i64>,
    // pub middlecubes: Cow<'r, i64>,
    // pub bottomcubes: Cow<'r, i64>,
    // pub missedcubes: Cow<'r, i64>,
    // pub topcones: Cow<'r, i64>,
    // pub middlecones: Cow<'r, i64>,
    // pub bottomcones: Cow<'r, i64>,
    // pub missedcones: Cow<'r, i64>,
    // pub topcube: Cow<'r, i64>,
    // pub middlecube: Cow<'r, i64>,
    // pub bottomcube: Cow<'r, i64>,
    // pub missedcube: Cow<'r, i64>,
    // pub topcone: Cow<'r, i64>,
    // pub middlecone: Cow<'r, i64>,
    // pub bottomcone: Cow<'r, i64>,
    // pub missedcone: Cow<'r, i64>,
    pub defenseplayti: Cow<'r, f64>,
    pub defensiverati: Cow<'r, i64>,
    pub teamattemptsc: Cow<'r, bool>,
    pub chargestation: Cow<'r, str>,
    // pub links: Cow<'r, i64>,
    pub anyrobotprobl: Cow<'r, str>,
    pub fouls: Cow<'r, str>,
    pub extranotes: Cow<'r, str>,
    pub driveteamrati: Cow<'r, str>,
    pub playstylesumm: Cow<'r, str>,
    pub password: Cow<'r, str>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct PitData<'r> {
    pub team: Cow<'r, str>,
    pub absent: Cow<'r, bool>,
    pub name: Cow<'r, str>,
    pub location: Cow<'r, str>, //
    pub fullteamname: Cow<'r, str>,
    pub teamlocation: Cow<'r, str>,
    pub robotname: Cow<'r, str>,
    pub drivetraintype: Cow<'r, str>,
    pub motortype: Cow<'r, str>,
    pub abilitytomoveco: Cow<'r, f64>,
    pub abilitytomovecu: Cow<'r, f64>,
    pub averageconecycl: Cow<'r, f64>,
    pub averagecubecycl: Cow<'r, f64>,
    pub successfullgrab: Cow<'r, f64>,
    pub robotweightlbs: Cow<'r, f64>,
    pub maxheightcapabi: Cow<'r, str>,
    pub totalwheelsused: Cow<'r, i64>,

    
    // pub endgametraction: Cow<'r, i64>,
    pub wherearepneumat: Cow<'r, str>,
    pub whereare3dprint: Cow<'r, str>,

    pub programmedautoc: Cow<'r, str>,
    // pub limelightcapabi: Cow<'r, str>,
    pub apriltagsused: Cow<'r, bool>,
    pub reflectivetapeu: Cow<'r, bool>,
    pub extracamerasuse: Cow<'r, bool>,
    pub automationviase: Cow<'r, bool>,

    pub endgameabilitys: Cow<'r, str>,
    pub whatisyourfavor: Cow<'r, str>,
    pub drivestationsum: Cow<'r, str>,
    pub arethereanyothe: Cow<'r, str>,
    pub password: Cow<'r, str>,
}

/// the thingies in the thingy
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct GridData<'r> {
    pub name: Cow<'r, str>,
    pub value: Cow<'r, i32>,
    // element: Option<Cow<'r, str>>,
    // toggle: Option<Cow<'r, str>>,
}

/// Just a test struct
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct FormDataTest<'r> {
    test: Cow<'r, str>
}

/// Initializing the data file
pub fn init_files() -> Result<(), Box<dyn Error>> {
    let settings_stuff = settings::Settings::new()?;
    if !file_exists(&settings_stuff.stands_data) {
        let _userfile = File::create(&settings_stuff.stands_data);
    }
    if !file_exists(&settings_stuff.pits_data) {
        let _userfile = File::create(&settings_stuff.pits_data);
    }
    if !file_exists(&settings_stuff.logs_dir) {
        let _userfile = File::create(&settings_stuff.logs_dir);
    }
    if !file_exists(&settings_stuff.test_data) {
        let _userfile = File::create(&settings_stuff.test_data);
    }
    if !file_exists(&settings_stuff.logs_dir) {
        let _userfile = File::create(&settings_stuff.logs_dir);
    }
    Ok(())
}

/// Checks if file exists
pub fn file_exists(file: &str) -> bool {
    return Path::new(file).exists()
}

/// Adds to data.csv
pub fn append_csv(content: &str) -> Result<(), Box<dyn Error>> {
    init_files()?;
    let settings_stuff = settings::Settings::new()?;
    let mut file = fs::OpenOptions::new()
      .append(true)
      .open(&settings_stuff.stands_data)
      .unwrap();
    
    let _ = writeln!(file, "{}", format!("{}", content));
    Ok(())
}

/// Instead adds to the garbage data csv
pub fn append_test(content: &str) -> Result<(), Box<dyn Error>> {
    let settings_stuff = settings::Settings::new()?;
    init_files()?;
    let mut file = fs::OpenOptions::new()
      .append(true)
      .open(&settings_stuff.test_data)
      .unwrap();
    
    let _ = writeln!(file, "{}", format!("{}", content));
    Ok(())
}

/// Adds to data.csv
pub fn append_pits(content: &str) -> Result<(), Box<dyn Error>> {
    let settings_stuff = settings::Settings::new()?;
    init_files()?;
    let mut file = fs::OpenOptions::new()
      .append(true)
      .open(&settings_stuff.pits_data)
      .unwrap();
    
    let _ = writeln!(file, "{}", format!("{}", content));
    Ok(())
}

/// Checks if file is empty
pub fn file_empty(file: String) -> Result<bool, Box<dyn Error>> {
    let mut thing = fs::File::open(file).expect("Failed to open file");
    let mut string = String::new();
    thing.read_to_string(&mut string).unwrap();
    if string == "".to_string() {
        return Ok(true)
    }
    else {
        return Ok(false)
    }
}

/// Wipes data.csv
pub fn wipe_data() {
    let _ = fs::write("data.csv", "");
    return
}
