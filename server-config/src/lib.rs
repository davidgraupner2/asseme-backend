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
    pub fn from_env() -> Config {
        let api = ApiConfiguration::default();
        let database = DatabaseConfiguration::default();
        let logging = Logging::default();

        Config {
            api,
            database,
            logging,
        }
    }
}
