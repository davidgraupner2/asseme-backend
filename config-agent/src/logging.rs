use crate::{DEFAULT_LOG_FORMAT, DEFAULT_LOG_LEVEL};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct LoggingConfiguration {
    pub format: String,
    pub level: String,
}

impl Default for LoggingConfiguration {
    fn default() -> Self {
        LoggingConfiguration {
            format: DEFAULT_LOG_FORMAT.to_string(),
            level: DEFAULT_LOG_LEVEL.to_string(),
        }
    }
}

impl LoggingConfiguration {}
