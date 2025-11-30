use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiConfiguration {
    pub port: u16,
    pub behind_proxy: bool,
    pub request_timeout_secs: u64,
    pub agent_ping_interval: u64,
    pub agent_ping_timeout: u64,
}

impl ApiConfiguration {
    pub fn default() -> Self {
        ApiConfiguration {
            port: 8000,
            behind_proxy: false,
            request_timeout_secs: 30,
            agent_ping_interval: 10,
            agent_ping_timeout: 5,
        }
    }
}

pub trait LoadApiConfiguration {
    fn load_config(&self) -> ApiConfiguration;
}
