use tracing_appender::non_blocking::WorkerGuard;

#[derive(Debug)]
pub struct Actors {}

#[derive(Debug)]
pub struct AgentControllerState {
    pub tracing_worker_guards: Vec<WorkerGuard>,
}

impl AgentControllerState {
    pub fn new() -> Self {
        Self {
            tracing_worker_guards: vec![],
        }
    }
}
