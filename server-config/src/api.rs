use crate::cors::CorsConfiguration;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiConfiguration {
    pub port: u16,
    pub behind_proxy: bool,
    pub request_timeout_secs: u64,
    pub cors: CorsConfiguration,
}

impl Default for ApiConfiguration {
    fn default() -> Self {
        ApiConfiguration {
            port: env::var("API_PORT")
                .unwrap_or("8000".to_owned())
                .parse()
                .unwrap_or(8000),
            behind_proxy: env::var("API_BEHIND_PROXY")
                .unwrap_or("false".to_string())
                .parse()
                .unwrap_or(false),
            request_timeout_secs: env::var("API_REQUEST_TIMEOUT_SECS")
                .unwrap_or("30".to_owned())
                .parse()
                .unwrap_or(30),
            cors: CorsConfiguration::load(),
        }
    }
}
