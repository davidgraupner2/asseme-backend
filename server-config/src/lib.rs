pub mod api;
pub mod cors;
pub mod database;
pub mod logging;

use serde::{Deserialize, Serialize};
// use std::{fs, path::Path};

use crate::api::ApiConfiguration;
use crate::database::DatabaseConfiguration;
use crate::logging::Logging;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub api: api::ApiConfiguration,
    pub database: DatabaseConfiguration,
    pub logging: logging::Logging,
}

impl Config {
    pub fn load() -> Config {
        let api = ApiConfiguration::default();
        let database = DatabaseConfiguration::default();
        let logging = Logging::default();

        Config {
            api,
            database,
            logging,
        }
    }

    // pub fn load<P: AsRef<Path>>(path: P) -> Self {
    //     let content = fs::read_to_string(path).unwrap_or_else(|_| String::from(""));

    //     toml::from_str(&content).unwrap_or_else(|_| Config {
    //         api: api::ApiConfiguration::default(),
    //         database: database::DatabaseConfiguration::default(),
    //         logging: logging::Logging::default(),
    //     })
    // }

    // pub fn create<P: AsRef<Path>>(path: P) -> Self {
    //     // First ensure the configuration folder exists
    //     if let Some(parent) = path.as_ref().parent() {
    //         let _ = fs::create_dir_all(parent);
    //     }

    //     let config = Config::load(&path);
    //     if fs::exists(path.as_ref()).unwrap() {
    //         config
    //     } else {
    //         Config::save(&config, path);
    //         config
    //     }
    // }

    // pub fn save<P: AsRef<Path>>(config: &Config, path: P) {
    //     let content = toml::to_string(config).unwrap();
    //     fs::write(path, content).unwrap();
    // }

    // pub fn update(&mut self, other: Config) {
    //     self.api = other.api;
    //     self.logging = other.logging;
    // }
}
