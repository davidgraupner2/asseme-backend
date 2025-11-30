use config_agent::{
    api::ApiConfiguration, logging::LoggingConfiguration, DEFAULT_LOG_FORMAT, DEFAULT_LOG_LEVEL,
};

#[derive(Debug)]
pub struct AgentControllerArguments {
    pub api_config: ApiConfiguration,
    pub log_config: LoggingConfiguration,
}

impl AgentControllerArguments {
    pub fn new(api_config: ApiConfiguration, mut log_config: LoggingConfiguration) -> Self {
        //Validate the log format passed in
        let valid_log_formats = ["json", "pretty", "full", "compact"];
        if !valid_log_formats.contains(&log_config.format.as_str()) {
            log_config.format = DEFAULT_LOG_FORMAT.to_string();
        }

        Self {
            api_config,
            log_config,
        }
    }
}
