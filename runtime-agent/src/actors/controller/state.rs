use crate::actors::api::messages::ApiMessage;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;
use ractor::ActorRef;
use tracing_appender::non_blocking::WorkerGuard;

#[derive(Debug)]
pub struct Actors {
    pub api_server: Option<ActorRef<ApiMessage>>,
}

#[derive(Debug)]
pub struct AgentControllerState {
    pub tracing_worker_guards: Vec<WorkerGuard>,
    pub spawned_actors: Actors,
    // pub api_config: ApiConfiguration,
    // pub log_config: LoggingConfiguration,
    pub db_pool: Option<Pool<ConnectionManager<SqliteConnection>>>,
}

impl AgentControllerState {
    pub fn new() -> Self {
        Self {
            tracing_worker_guards: vec![],
            spawned_actors: Actors { api_server: None },
            db_pool: None,
        }
    }
}
