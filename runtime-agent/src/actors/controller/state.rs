use crate::actors::api::messages::ApiMessage;
use config_agent::{api::ApiConfiguration, logging::LoggingConfiguration};
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
    pub api_config: ApiConfiguration,
    pub log_config: LoggingConfiguration,
}

impl AgentControllerState {
    pub fn new(api_config: ApiConfiguration, log_config: LoggingConfiguration) -> Self {
        Self {
            tracing_worker_guards: vec![],
            spawned_actors: Actors { api_server: None },
            api_config,
            log_config,
        }
    }
}
