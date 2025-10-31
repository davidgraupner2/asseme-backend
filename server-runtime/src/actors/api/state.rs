use crate::{actors::ActorStatus, properties::runtime_id};
use axum::extract::ws::Message;
use server_config::database::DatabaseConfiguration;
use std::sync::Arc;
use surrealdb::{
    engine::remote::ws::{Client, Ws, Wss},
    opt::auth::Root,
    Surreal,
};
use tokio::sync::broadcast;
use tokio::sync::{broadcast::Sender, Mutex};
use tracing::{event, Level};

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
}

impl AxumApiState {
    pub async fn new(database_config: DatabaseConfiguration) -> Self {
        let (tx, _) = broadcast::channel(32);

        // Let the API connect to the SurrealDB Database
        event!(
            Level::DEBUG,
            "API Server Connecting to database on: {}://{}",
            &database_config.connection_type,
            &database_config.url
        );

        let db = match database_config.connection_type.as_str() {
            "wss" => Surreal::new::<Wss>(&database_config.url).await,
            _ => Surreal::new::<Ws>(&database_config.url).await,
        };

        let db_client =
            db.expect("DATABASE_CONNECTION_ERROR: API Server Failed to connect to database");

        if let Err(e) = db_client
            .signin(Root {
                username: &database_config.user_name,
                password: &database_config.password,
            })
            .await
        {
            panic!("DATABASE_AUTH_ERROR: API Server Failed to authenticate with database. Check username/password in config: {}", e);
        }

        Self {
            id: runtime_id(),
            db_client,
            db_config: database_config.clone(),
            broadcast_tx: Arc::new(Mutex::new(tx)),
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
