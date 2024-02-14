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

/// Initializing the data file
pub fn init_files() -> Result<(), Box<dyn Error>> {
    let config = settings::Settings::new()?;
    if !file_exists(&config.stands_data_dir) {
        std::fs::create_dir_all(std::path::Path::new(&config.stands_data_dir).parent().unwrap()).unwrap();
        File::create(&config.stands_data_dir)?;
    }
    if !file_exists(&config.pits_data_dir) {
        std::fs::create_dir_all(std::path::Path::new(&config.pits_data_dir).parent().unwrap()).unwrap();
        File::create(&config.pits_data_dir)?;
    }
    if !file_exists(&config.logs_dir) {
        std::fs::create_dir_all(std::path::Path::new(&config.logs_dir).parent().unwrap()).unwrap();
        File::create(&config.logs_dir)?;
    }
    if !file_exists(&config.test_data_dir) {
        std::fs::create_dir_all(std::path::Path::new(&config.test_data_dir).parent().unwrap()).unwrap();
        File::create(&config.test_data_dir)?;
    }
    if !file_exists(&config.stands_data_json) {
        std::fs::create_dir_all(std::path::Path::new(&config.stands_data_json)).unwrap();
        // File::create(&config.stands_data_json)?;
    }
    Ok(())
}

/// Checks if file exists
pub fn file_exists(file: &str) -> bool {
    Path::new(file).exists()
}

/// Adds to data.csv
#[allow(unused)]
pub fn append_csv(content: &str) -> Result<(), Box<dyn Error>> {
    init_files()?;
    let config = settings::Settings::new()?;
    let mut file = fs::OpenOptions::new()
      .append(true)
      .open(&config.stands_data_dir)
      .unwrap();
    
    let _ = writeln!(file, "{}", format!("{}", content));
    Ok(())
}

/// Appends to dedicated file
pub fn append(content: &str, file: &str) -> Result<(), Box<dyn Error>> {
    init_files()?;
    let mut file = fs::OpenOptions::new()
      .append(true)
      .open(file)
      .unwrap();
    
    let _ = writeln!(file, "{}", format!("{}", content));
    Ok(())
}

/// Instead adds to the garbage data csv
#[allow(unused)]
pub fn append_test(content: &str) -> Result<(), Box<dyn Error>> {
    let config = settings::Settings::new()?;
    init_files()?;
    let mut file = fs::OpenOptions::new()
      .append(true)
      .open(&config.test_data_dir)
      .unwrap();
    
    let _ = writeln!(file, "{}", format!("{}", content));
    Ok(())
}

/// Adds to pits.csv
#[allow(unused)]
pub fn append_pits(content: &str) -> Result<(), Box<dyn Error>> {
    let config = settings::Settings::new()?;
    init_files()?;
    let mut file = fs::OpenOptions::new()
      .append(true)
      .open(&config.pits_data_dir)
      .unwrap();
    
    let _ = writeln!(file, "{}", format!("{}", content));
    Ok(())
}

/// Gets data.csv's content
pub fn get_data(file: &str) -> Result<String, Box<dyn Error>> {
    let mut thing = fs::File::open(file).expect("Failed to open file");
    let mut string = String::new();
    thing.read_to_string(&mut string).unwrap();
    Ok(string)
}

/// Checks if file is empty
pub fn file_empty(file: &str) -> Result<bool, Box<dyn Error>> {
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

/// Backs up json to a file in data/json
/// Filename is named by the person who marked down the data and a timestamp
pub fn backup_json(json: &str, file: &str) -> Result<(), Box<dyn Error>> {
    let config = settings::Settings::new()?;
    let mut file = fs::OpenOptions::new()
      .write(true)
      .create(true)
      .open(format!("{}/{}", &config.stands_data_json, file))
      .unwrap();
    
    let _ = writeln!(file, "{}", format!("{}", json));
    Ok(())
}

/// Wipes data.csv
#[allow(unused)]
pub fn wipe_data() {
    let _ = fs::write("data.csv", "");
    return
}