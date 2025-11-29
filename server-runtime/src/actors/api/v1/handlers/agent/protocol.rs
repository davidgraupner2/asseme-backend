use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Inbound {
    // Register {
    //     id: String,
    //     token: String,
    //     groups: Vec<String>,
    // },
    Pong { nonce: String },
    Ack { command_id: String },
    Disconnect { reason: Option<String> },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Outbound {
    Ping {
        nonce: String,
    },
    Command {
        command_id: String,
        verb: String,
        payload: serde_json::Value,
    },
    Disconnect {
        reason: Option<String>,
    },
}
