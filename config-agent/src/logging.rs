use crate::DEFAULT_LOG_FORMAT;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct LoggingConfiguration {
    pub format: String,
}

impl Default for LoggingConfiguration {
    fn default() -> Self {
        LoggingConfiguration {
            format: DEFAULT_LOG_FORMAT.to_string(),
        }
    }
}

impl LoggingConfiguration {}
