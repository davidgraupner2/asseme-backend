use server_config::{ApiConfiguration, CorsConfiguration};

#[derive(Debug)]
pub struct ControllerArguments {
    pub log_format: String,
    pub log_output: String,
    pub api_configuration: ApiConfiguration,
    pub cors_configuration: CorsConfiguration,
}
