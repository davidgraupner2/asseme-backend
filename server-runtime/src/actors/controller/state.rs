use crate::actors::api::ApiMessage;
use ractor::ActorRef;
use server_config::{ApiConfiguration, CorsConfiguration};
use tracing_appender::non_blocking::WorkerGuard;

#[derive(Debug)]
pub struct Actors {
    pub api_server: Option<ActorRef<ApiMessage>>,
}

#[derive(Debug)]
pub struct ControllerState {
    pub tracing_worker_guards: Vec<WorkerGuard>,
    pub api_configuration: ApiConfiguration,
    pub cors_configuration: CorsConfiguration,
    pub spawned_actors: Actors,
}

impl ControllerState {
    pub fn new(api_configuration: ApiConfiguration, cors_configuration: CorsConfiguration) -> Self {
        Self {
            tracing_worker_guards: vec![],
            api_configuration,
            cors_configuration,
            spawned_actors: Actors { api_server: None },
        }
    }
}
