use dashmap::DashMap;
use serde::{Deserialize, Deserializer, Serialize};
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot, Mutex};

#[derive(Debug)]
pub struct AgentInfo {
    pub id: String,
    pub groups: Vec<String>,
    // last_seen stored for dashboard; using Mutex for demo
    pub last_seen: Mutex<chrono::DateTime<chrono::Utc>>,
    // pending pong oneshot: server waits on this after sending ping
    pub pending_pong: Mutex<Option<oneshot::Sender<()>>>,
    pub token: String,
}

#[derive(Clone, Debug)]
pub struct AgentEntry {
    pub info: Arc<AgentInfo>,
    pub tx: mpsc::UnboundedSender<String>, // outbound JSON strings to writer task
}

pub type AgentRegistry = Arc<DashMap<String, AgentEntry>>;

#[derive(Debug, Deserialize, Serialize)]
pub struct WSConnect {
    pub id: String,
    pub token: String,
    #[serde(deserialize_with = "deserialize_groups")]
    pub groups: Vec<String>,
}

// helper to accept either repeated `groups=` params or a single CSV string
#[derive(Deserialize)]
#[serde(untagged)]
enum GroupsHelper {
    Vec(Vec<String>),
    Str(String),
}

fn deserialize_groups<'de, D>(des: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let helper = GroupsHelper::deserialize(des)?;
    Ok(match helper {
        GroupsHelper::Vec(v) => v
            .into_iter()
            .flat_map(|s| {
                s.split(',')
                    .map(|p| p.trim().to_string())
                    .collect::<Vec<_>>()
            })
            .filter(|p| !p.is_empty())
            .collect(),
        GroupsHelper::Str(s) => s
            .split(',')
            .map(|p| p.trim().to_string())
            .filter(|p| !p.is_empty())
            .collect(),
    })
}
