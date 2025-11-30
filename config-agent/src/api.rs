use serde::Deserialize;

use crate::DEFAULT_API_PORT;

#[derive(Deserialize, Debug, Clone)]
pub struct ApiConfiguration {
    pub port: u16,
    pub workers: usize,
}

impl Default for ApiConfiguration {
    fn default() -> Self {
        ApiConfiguration {
            port: DEFAULT_API_PORT,
            workers: 1,
        }
    }
}

impl ApiConfiguration {}
