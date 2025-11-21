use tracing_appender::non_blocking::WorkerGuard;

#[derive(Debug)]
pub struct ControllerState {
    pub tracing_worker_guards: Vec<WorkerGuard>,
}

impl ControllerState {
    pub fn new() -> Self {
        Self {
            tracing_worker_guards: vec![],
        }
    }
}
