use crate::cors::CorsConfiguration;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiConfiguration {
    pub port: u16,
    pub behind_proxy: bool,
    pub request_timeout_secs: u64,
    pub rate_limiting_burst_size: u32,
    pub rate_limiting_per_second: u64,
    pub rate_limiting_cleanup_duration: u64,
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
            rate_limiting_burst_size: env::var("RATE_LIMITING_BURST_SIZE")
                .unwrap_or("200".to_owned())
                .parse()
                .unwrap_or(200),
            rate_limiting_per_second: env::var("RATE_LIMITING_PER_SECOND")
                .unwrap_or("5".to_owned())
                .parse()
                .unwrap_or(200),
            rate_limiting_cleanup_duration: env::var("RATE_LIMITING_CLEANUP_DURATION")
                .unwrap_or("5".to_owned())
                .parse()
                .unwrap_or(200),
            cors: CorsConfiguration::load(),
        }
    }
}
