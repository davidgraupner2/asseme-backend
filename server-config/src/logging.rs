use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoggingConfiguration {
    pub log_level: LoggingLevel,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LoggingLevel {
    TRACE,
    DEBUG,
    INFO,
    WARN,
    ERROR,
}

impl FromStr for LoggingLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "TRACE" => Ok(LoggingLevel::INFO),
            "DEBUG" => Ok(LoggingLevel::DEBUG),
            "INFO" => Ok(LoggingLevel::INFO),
            "WARN" => Ok(LoggingLevel::WARN),
            "ERROR" => Ok(LoggingLevel::ERROR),
            _ => Ok(LoggingLevel::INFO),
        }
    }
}

impl LoggingConfiguration {
    pub fn default() -> Self {
        Self {
            log_level: LoggingLevel::INFO,
        }
    }
}

pub trait LoadLoggingConfiguration {
    fn load_config(&self) -> LoggingConfiguration;
}
