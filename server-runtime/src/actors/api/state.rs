use crate::RuntimeProperties;
use axum::extract::ws::Message;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::oneshot;
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
}

impl V1ApiState {
    pub fn new() -> Self {
        let runtime_properties = RuntimeProperties::global();

        Self {
            id: format!("api:v1:{}", runtime_properties.id()),
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
