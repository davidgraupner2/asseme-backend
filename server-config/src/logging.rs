use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoggingConfiguration {
    pub log_format: String,
    pub log_output: String,
}

impl LoggingConfiguration {
    pub fn default() -> Self {
        Self {
            log_format: "json".to_string(),
            log_output: "console".to_string(),
        }
    }
}

pub trait LoadLoggingConfiguration {
    fn load_config(&self) -> LoggingConfiguration;
}
