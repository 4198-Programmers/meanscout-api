use config::{Config, ConfigError, File};
use serde::Deserialize;
use std::collections::HashMap;
type JsonMap = HashMap<String, serde_json::Value>;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

// #[allow(unused)]
// pub fn settings() -> std::io::Result<Config> {
//     let settings = Config::builder()
//         // Add in `./Settings.toml`
//         .add_source(config::File::with_name("settings.toml"))
//         // Add in settings from the environment (with a prefix of APP)
//         // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
//         // .add_source(config::Environment::with_prefix("APP"))
//         .build().unwrap();
//     Ok(settings)
// }

// Function for trying out new things
#[allow(unused)]
pub fn test() -> std::io::Result<()> {
    let path = "config.json";
    let data = fs::read_to_string(path).expect("Unable to read file");
    let res: JsonMap = serde_json::from_str(&data).expect("Unable to parse");
    let (_, data) = res.get_key_value("datasets").unwrap();
    let datasets: JsonMap = serde_json::from_str(&data.to_string()).expect("Unable to parse");
    let f = fs::File::create("test.txt").unwrap();
    let mut file = OpenOptions::new().write(true).append(true).open("test.txt").unwrap();
    let mut datapoint: Option<&str>;

    let mut data_list: Vec<&str> = vec![];

    // Returning each dataset
    for (key, value) in datasets.iter() {
        // println!("{}: {}", key, value);
        writeln!(file, "{}: {}", key, value);

        // Actually iterates over each dataset
        for dataset in value.as_object().iter() {
            // Iterates from each individual item from each dataset
            for item in dataset.iter() {
                // writeln!(file, "{}: {}", item.0, item.1);
                if item.1.is_object() {
                    // println!("{:?}", item.1.as_object());
                    for data in item.1.as_object().iter() {
                        println!("{:?}", data);
                    }
                }
                else {
                    datapoint = item.1.as_str();
                    if datapoint.is_some() {data_list.push(datapoint.unwrap())}
                }
            }
        }
    }
    println!("{:?}", data_list);
    Ok(())
    // println!("{:?}", res);
}

/// Converting config.json to json usable by meanscout
#[allow(unused)]
pub fn convert_to_mean() -> std::io::Result<()> {
    let path = "config.json";
    let data = fs::read_to_string(path).expect("Unable to read file");
    let res: JsonMap = serde_json::from_str(&data).expect("Unable to parse");
    let (_, data) = res.get_key_value("datasets").unwrap();

    let datasets: JsonMap = serde_json::from_str(&data.to_string()).expect("Unable to parse");
    for (key, value) in datasets.iter() {
        // println!("{}: {}", key, value);
            // Going over each item in the config file
        for dataset in value.as_object().iter() {
            let f = fs::File::create(format!("meanscout/{}.json", key))?;
            let mut file = OpenOptions::new().write(true).append(true).open(format!("meanscout/{}.json", key))?;
            writeln!(file, "{{\n\"metrics\": [\n")?;
            // println!("{:?}\n\n", dataset);
            for item in dataset.iter() {
                // println!("{:?}", item);
                if !item.1.is_object() {
                    println!("{:?}", item);
                    let metric = format!("  {{\"name\": \"{}\", \"type\": {}}},", item.0, item.1);
                    writeln!(file, "{}", metric)?;
                }
                else {
                    for category_items in item.1.as_object() {
                        // println!("{:?}", category_items);
                        for asdf in category_items.iter() {
                            let metric = format!("  {{\"name\": \"{}\", \"type\": {}{}}},", asdf.0, asdf.1, if(Some(asdf) == category_items.iter().next()) {format!(", \"group\": \"{}\"", item.0)} else {"".into()});
                            writeln!(file, "{}", metric)?;
                        }
                    }
                }
            }
            writeln!(file, "\n]\n}}")?;
        }
    }
    Ok(())
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub debug: bool,
    pub logs_dir: String,
    pub pits_data: String,
    pub stands_data: String,
    pub test_data: String,
    pub passwords: Vec<String>,
}

impl Settings {
    #[allow(unused)]
    pub fn new() -> Result<Self, ConfigError> {
        // let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let s = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name("settings.toml"))
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