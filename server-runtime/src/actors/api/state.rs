use crate::actors::api::v1::handlers::agent::types::AgentRegistry;
use crate::RuntimeProperties;
use axum::extract::ws::Message;
use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::{broadcast::Sender, Mutex};

#[derive(Clone, Debug)]
pub(crate) struct ApiState {
    pub id: String,
    pub broadcast_tx: Arc<Mutex<Sender<Message>>>,
}

impl ApiState {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(32);
        let runtime_properties = RuntimeProperties::global();

        Self {
            id: format!("api:{}", runtime_properties.id()),
            broadcast_tx: Arc::new(Mutex::new(tx)),
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct V1ApiState {
    pub id: String,
    pub agent_registry: AgentRegistry,
    pub agent_ping_interval: u64,
    pub agent_ping_timeout: u64,
}

impl V1ApiState {
    pub fn new(agent_ping_interval: u64, agent_ping_timeout: u64) -> Self {
        let runtime_properties = RuntimeProperties::global();
        let agent_registry: AgentRegistry = Arc::new(DashMap::new());

        Self {
            id: format!("api:v1:{}", runtime_properties.id()),
            agent_registry,
            agent_ping_interval,
            agent_ping_timeout,
        }
    }
}

#[derive(Debug)]
pub struct ApiActorState {
    pub shutdown_tx: Option<tokio::sync::oneshot::Sender<()>>,
}

impl ApiActorState {
    pub fn new() -> Self {
        Self { shutdown_tx: None }
    }
}
