use crate::{
    folders::{ensure_required_folders_exist, logs_folder},
    logging::setup_tracing,
    properties::log_file_name,
};
use tracing_appender::non_blocking::WorkerGuard;

pub async fn bootstrap_runtime(log_level: &str) -> Vec<WorkerGuard> {
    // Ensure that the required folders exist
    ensure_required_folders_exist();

    // Setup the tracing / logging system
    let tracing_guards = setup_tracing(log_level, &logs_folder(), &log_file_name());

    tracing_guards
}
