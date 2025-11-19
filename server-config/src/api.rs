use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiConfiguration {
    pub port: u16,
    pub behind_proxy: bool,
    pub request_timeout_secs: u64,
}

impl ApiConfiguration {
    pub fn default() -> Self {
        ApiConfiguration {
            port: 8000,
            behind_proxy: false,
            request_timeout_secs: 30,
        }
    }
}

pub trait LoadApiConfiguration {
    fn load_config(&self) -> ApiConfiguration;
}
