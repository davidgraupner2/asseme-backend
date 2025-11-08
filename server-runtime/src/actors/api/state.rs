use crate::{actors::ActorStatus, properties::runtime_id};
use axum::extract::ws::Message;
use database::{self, context::get_database};
use server_config::api::ApiConfiguration;
use server_config::cors::CorsConfiguration;
use server_config::database::DatabaseConfiguration;
use std::sync::Arc;
use surrealdb::{engine::remote::ws::Client, Surreal};
use tokio::sync::broadcast;
use tokio::sync::{broadcast::Sender, Mutex};

/// Axum State
///
/// This is the AXUM shared state that is initialised at start and propogated to all API
/// handlers if needed.
///
/// Define shared data in this AxumState for all API handlers to consume
#[derive(Clone)]
pub struct AxumApiState {
    pub id: String,
    pub db_client: Surreal<Client>,
    pub db_config: DatabaseConfiguration,
    pub broadcast_tx: Arc<Mutex<Sender<Message>>>,
    pub behind_proxy: bool,
    pub request_timeout_secs: u64,
    pub rate_limiting_burst_size: u32,
    pub rate_limiting_per_second: u64,
    pub rate_limiting_cleanup_duration: u64,
}

impl AxumApiState {
    pub async fn new(
        database_config: DatabaseConfiguration,
        api_configuration: ApiConfiguration,
    ) -> Self {
        let (tx, _) = broadcast::channel(32);

        let db_client = get_database(
            database_config.connection_type.clone(),
            database_config.url.clone(),
            database_config.user_name.clone(),
            database_config.password.clone(),
            database_config.namespace.clone(),
            database_config.database.clone(),
        )
        .await
        .unwrap();

        Self {
            id: runtime_id(),
            db_client,
            db_config: database_config.clone(),
            broadcast_tx: Arc::new(Mutex::new(tx)),
            behind_proxy: api_configuration.behind_proxy,
            request_timeout_secs: api_configuration.request_timeout_secs,
            rate_limiting_burst_size: api_configuration.rate_limiting_burst_size,
            rate_limiting_cleanup_duration: api_configuration.rate_limiting_cleanup_duration,
            rate_limiting_per_second: api_configuration.rate_limiting_per_second,
        }
    }
}

pub struct ApiActorState {
    pub status: ActorStatus,
}

impl ApiActorState {
    pub fn new() -> Self {
        Self {
            status: ActorStatus::INITIALISING,
        }
    }
}
