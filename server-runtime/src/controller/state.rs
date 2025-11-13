use crate::actors::api::messages::APIMessage;
use ractor::ActorRef;
use server_config::Config;
use tracing_appender::non_blocking::WorkerGuard;

#[derive(Debug)]
pub struct ControllerState {
    pub config: Config,
    pub tracing_worker_guards: Vec<WorkerGuard>,
    // pub api_server: Option<(ActorRef<APIMessage>, JoinHandle<()>)>,
    pub api_server: Option<ActorRef<APIMessage>>,
}

impl ControllerState {
    pub fn new() -> Self {
        let config = Config::from_env();

        Self {
            config,
            tracing_worker_guards: vec![],
            api_server: None,
        }
    }
}
