use serde::{Deserialize, Serialize};
use std::{env, str::FromStr};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Logging {
    pub log_level: Level,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Level {
    TRACE,
    DEBUG,
    INFO,
    WARN,
    ERROR,
}

impl FromStr for Level {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "TRACE" => Ok(Level::INFO),
            "DEBUG" => Ok(Level::DEBUG),
            "INFO" => Ok(Level::INFO),
            "WARN" => Ok(Level::WARN),
            "ERROR" => Ok(Level::ERROR),
            _ => Ok(Level::INFO),
        }
    }
}

impl Default for Logging {
    fn default() -> Self {
        Self {
            log_level: Level::from_str(
                &env::var("LOG_LEVEL")
                    .unwrap_or("INFO".to_string())
                    .to_uppercase(),
            )
            .unwrap_or(Level::INFO),
        }
    }
}
