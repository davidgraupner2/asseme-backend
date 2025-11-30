use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RateLimitingConfiguration {
    pub burst_size: u32,
    pub per_second: u64,
    pub cleanup_duration: u64,
}

impl RateLimitingConfiguration {
    pub fn default() -> Self {
        Self {
            burst_size: 200,
            per_second: 5,
            cleanup_duration: 60,
        }
    }
}

pub trait LoadRateLimitingConfiguration {
    fn load_config(&self) -> RateLimitingConfiguration;
}
