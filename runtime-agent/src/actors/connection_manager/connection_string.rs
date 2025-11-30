use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::sync::RwLock;

// Initialise a global variable that stores the Agent ConnectionString Structs
static CONNECTION_STRINGS: Lazy<RwLock<AgentConnectionStrings>> =
    Lazy::new(|| RwLock::new(AgentConnectionStrings::default()));

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentConnectionStrings {
    pub current: Option<String>,
    pub new: Option<String>,
}

impl Default for AgentConnectionStrings {
    fn default() -> Self {
        AgentConnectionStrings {
            current: None,
            new: None,
        }
    }
}

impl AgentConnectionStrings {
    pub fn is_empty(&self) -> bool {
        self.current.is_none() && self.new.is_none()
    }
}
