use config_server::{ApiConfiguration, CorsConfiguration, RateLimitingConfiguration};

#[derive(Debug)]
pub struct ControllerArguments {
    pub log_format: String,
    pub log_output: String,
    pub api_configuration: ApiConfiguration,
    pub cors_configuration: CorsConfiguration,
    pub rate_limiting_configuration: RateLimitingConfiguration,
}
