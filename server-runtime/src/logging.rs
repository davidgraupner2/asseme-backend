use std::path::PathBuf;
use tracing::Level;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt;

pub(crate) fn setup_tracing(
    log_level: &str,
    log_file_folder: &PathBuf,
    log_file_name: &str,
) -> Vec<WorkerGuard> {
    let mut writer_worker_guards = Vec::new();

    let log_file_appender = tracing_appender::rolling::daily(log_file_folder, log_file_name);
    let (non_blocking, worker_guard) = tracing_appender::non_blocking(log_file_appender);

    let subscriber = fmt::Subscriber::builder()
        .with_max_level(get_log_level_from_str(&log_level))
        .with_writer(non_blocking.clone())
        .json()
        // .with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default tracing subscriber failed");

    writer_worker_guards.push(worker_guard);
    writer_worker_guards
}

fn get_log_level_from_str(log_level: &str) -> tracing::Level {
    match log_level.to_lowercase().as_str() {
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        "trace" => {
            if cfg!(debug_assertions) {
                Level::TRACE
            } else {
                Level::INFO // Default to INFO in release builds
            }
        }
        "debug" => {
            if cfg!(debug_assertions) {
                Level::DEBUG
            } else {
                Level::INFO // Default to INFO in release builds
            }
        }
        _ => Level::INFO,
    }
}