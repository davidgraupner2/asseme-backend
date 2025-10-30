use serde::{Deserialize, Serialize};

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

impl Default for Logging {
    fn default() -> Self {
        Self {
            log_level: Level::INFO,
        }
    }
}