use serde::{Deserialize, Serialize};

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
