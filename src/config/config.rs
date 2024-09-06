use super::database::Database;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::from_reader;
use std::fs::File;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub port: String,
    pub database: Database,
    pub log_level: String,
}

lazy_static! {
    static ref c: Config = {
        // Load the configuration file and parse it
        let file = File::open("config.json").expect("Failed to open config file");
        from_reader(file).expect("Failed to parse config file")
    };
}

// Function to get the singleton instance
pub fn get_config() -> &'static Config {
    &*c
}
