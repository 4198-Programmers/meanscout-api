use std::path::Path;
use std::fs::File;
use std::io::Write;
use serde::{Serialize, Deserialize};
use std::borrow::Cow;
use std::fs;

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
    pub defenceplayti: Cow<'r, f64>,
    pub defensiverati: Cow<'r, i64>,
    pub teamattemptsc: Cow<'r, bool>,
    pub chargestation: Cow<'r, str>,
    pub links: Cow<'r, i64>,
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

    pub endgameabilitys: Cow<'r, str>,
    pub endgametraction: Cow<'r, i64>,
    pub wherearepneumat: Cow<'r, str>,
    pub whereare3dprint: Cow<'r, str>,

    pub programmedautoc: Cow<'r, str>,
    pub limelightcapabi: Cow<'r, str>,
    pub apriltagsused: Cow<'r, bool>,
    pub reflectivetapeu: Cow<'r, bool>,
    pub extracamerasuse: Cow<'r, bool>,
    pub automationviase: Cow<'r, bool>,

    pub whatisyourfavor: Cow<'r, str>,
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
pub fn init_files() {
    if !file_exists("data.csv") {
        let _userfile = File::create("data.csv");
    }
    if !file_exists("pits.csv") {
        let _userfile = File::create("pits.csv");
    }
    if !file_exists("logs/scouting.log") {
        let _userfile = File::create("logs/scouting.log");
    }
    if !file_exists("test.csv") {
        let _userfile = File::create("test.csv");
    }
    return
}

/// Checks if file exists
pub fn file_exists(file: &str) -> bool {
    return Path::new(file).exists()
}

/// Adds to data.csv
pub fn append_csv(content: &str) {
    init_files();
    let mut file = fs::OpenOptions::new()
      .append(true)
      .open("data.csv")
      .unwrap();
    
    let _ = writeln!(file, "{}", format!("{}", content));
    return;
}

/// Instead adds to the garbage data csv
pub fn test_csv(content: &str) {
    init_files();
    let mut file = fs::OpenOptions::new()
      .append(true)
      .open("test.csv")
      .unwrap();
    
    let _ = writeln!(file, "{}", format!("{}", content));
    return;
}

/// Adds to data.csv
pub fn append_pits(content: &str) {
    init_files();
    let mut file = fs::OpenOptions::new()
      .append(true)
      .open("pits.csv")
      .unwrap();
    
    let _ = writeln!(file, "{}", format!("{}", content));
    return
}

/// Wipes data.csv
pub fn wipe_data() {
    let _ = fs::write("data.csv", "");
    return
}
