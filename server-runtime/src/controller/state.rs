use crate::actors::api::messages::APIMessage;
use crate::properties::config_file_name;
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
        let config = Config::create(config_file_name());

        Self {
            config,
            tracing_worker_guards: vec![],
            api_server: None,
        }
    }
}
