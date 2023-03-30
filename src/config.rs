use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::collections::HashMap;
type JsonMap = HashMap<String, serde_json::Value>;
use std::{env, fs};


// Function for trying out new things
pub fn test() {
    let path = "config.json";
    let data = fs::read_to_string(path).expect("Unable to read file");
    let res: JsonMap = serde_json::from_str(&data).expect("Unable to parse");
    let (_, data) = res.get_key_value("datasets").unwrap();
    let datasets: JsonMap = serde_json::from_str(&data.to_string()).expect("Unable to parse");
    for (key, value) in datasets.iter() {
        println!("{}: {}", key, value);
        if value.is_object() {
            // println!("it sure is an object");
            println!("{:?}", value.as_object().unwrap());
        }
    }
    // println!("{:?}", res);
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub debug: bool,
    // pub thing: Thing,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Thing {
    pub thing: String,
    pub thin: Thin,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Thin {
    pub urmom: String,
}


impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let s = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name("config.json"))
            // Add in the current environment file
            // Default to 'development' env
            // Note that this file is _optional_
            // .add_source(
            //     File::with_name(&format!("examples/hierarchical-env/config/{}", run_mode))
            //         .required(false),
            // )
            // Add in a local configuration file
            // This file shouldn't be checked in to git
            // .add_source(File::with_name("examples/hierarchical-env/config/local").required(false))
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            // .add_source(Environment::with_prefix("app"))
            // You may also programmatically change settings
            // .set_override("database.url", "postgres://")?
            .build()?;

        // Now that we're done, let's access our configuration
        // println!("debug: {:?}", s.get_bool("debug"));
        // println!("database: {:?}", s.get::<String>("database.url"));

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_deserialize()
    }
}