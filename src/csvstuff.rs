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

/// Struct for form data
#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub data: serde_json::Map<String, serde_json::Value>,
}

/// the thingies in the thingy (bad)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GridData<'r> {
    pub name: Cow<'r, str>,
    pub value: Cow<'r, i32>,
    // element: Option<Cow<'r, str>>,
    // toggle: Option<Cow<'r, str>>,
}

/// Just a test struct
#[derive(Serialize, Deserialize, Debug)]
pub struct FormDataTest<'r> {
    test: Cow<'r, str>
}

/// Initializing the data file
pub fn init_files() -> Result<(), Box<dyn Error>> {
    let settings_stuff = settings::Settings::new()?;
    if !file_exists(&settings_stuff.stands_data_dir) {
        std::fs::create_dir_all(std::path::Path::new(&settings_stuff.stands_data_dir).parent().unwrap()).unwrap();
        File::create(&settings_stuff.stands_data_dir)?;
    }
    if !file_exists(&settings_stuff.pits_data_dir) {
        std::fs::create_dir_all(std::path::Path::new(&settings_stuff.pits_data_dir).parent().unwrap()).unwrap();
        File::create(&settings_stuff.pits_data_dir)?;
    }
    if !file_exists(&settings_stuff.logs_dir) {
        std::fs::create_dir_all(std::path::Path::new(&settings_stuff.logs_dir).parent().unwrap()).unwrap();
        File::create(&settings_stuff.logs_dir)?;
    }
    if !file_exists(&settings_stuff.test_data_dir) {
        std::fs::create_dir_all(std::path::Path::new(&settings_stuff.test_data_dir).parent().unwrap()).unwrap();
        File::create(&settings_stuff.test_data_dir)?;
    }
    Ok(())
}

/// Checks if file exists
pub fn file_exists(file: &str) -> bool {
    Path::new(file).exists()
}

/// Adds to data.csv
pub fn append_csv(content: &str) -> Result<(), Box<dyn Error>> {
    init_files()?;
    let settings_stuff = settings::Settings::new()?;
    let mut file = fs::OpenOptions::new()
      .append(true)
      .open(&settings_stuff.stands_data_dir)
      .unwrap();
    
    let _ = writeln!(file, "{}", format!("{}", content));
    Ok(())
}

/// Instead adds to the garbage data csv
#[allow(unused)]
pub fn append_test(content: &str) -> Result<(), Box<dyn Error>> {
    let settings_stuff = settings::Settings::new()?;
    init_files()?;
    let mut file = fs::OpenOptions::new()
      .append(true)
      .open(&settings_stuff.test_data_dir)
      .unwrap();
    
    let _ = writeln!(file, "{}", format!("{}", content));
    Ok(())
}

/// Adds to data.csv
#[allow(unused)]
pub fn append_pits(content: &str) -> Result<(), Box<dyn Error>> {
    let settings_stuff = settings::Settings::new()?;
    init_files()?;
    let mut file = fs::OpenOptions::new()
      .append(true)
      .open(&settings_stuff.pits_data_dir)
      .unwrap();
    
    let _ = writeln!(file, "{}", format!("{}", content));
    Ok(())
}

/// Gets data.csv's content
pub fn get_data() -> Result<String, Box<dyn Error>> {
    let config = settings::Settings::new()?;
    let mut thing = fs::File::open(&config.stands_data_dir).expect("Failed to open file");
    let mut string = String::new();
    thing.read_to_string(&mut string).unwrap();
    Ok(string)
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
#[allow(unused)]
pub fn wipe_data() {
    let _ = fs::write("data.csv", "");
    return
}
