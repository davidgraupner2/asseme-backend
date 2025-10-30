use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiConfiguration {
    pub workers: Option<usize>,
    pub port: u16,
}

impl Default for ApiConfiguration {
    fn default() -> Self {
        ApiConfiguration {
            workers: Some(1),
            port: 8000,
        }
    }
}
