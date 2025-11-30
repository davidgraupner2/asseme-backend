use config::{Config, ConfigError, File};
use serde::Deserialize;

use crate::{api::ApiConfiguration, logging::LoggingConfiguration};

#[derive(Deserialize, Debug, Clone)]
#[allow(unused)]
pub struct AgentSettings {
    pub api: ApiConfiguration,
    pub logging: LoggingConfiguration,
}

impl AgentSettings {
    pub fn new(config_file_name: String) -> Result<Self, ConfigError> {
        let settings = Config::builder()
            .add_source(File::with_name(&config_file_name))
            .build()?;

        settings.try_deserialize()
    }
}
