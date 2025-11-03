use semver::Version;
use serde::Serialize;
use surrealdb::{engine::remote::ws::Client, Surreal};
use tracing_appender::non_blocking::WorkerGuard;

pub const APP_NAME: &str = "asseme";
pub const DB_TIMEOUT: u64 = 2000;

use crate::{
    folders::{ensure_required_folders_exist, logs_folder},
    logging::setup_tracing,
    properties::{log_file_name, runtime_version},
};

#[derive(Serialize, Clone)]
pub struct AppVersion {
    api_version: String,
    db_version: Version,
}

impl AppVersion {
    pub async fn new(db: Surreal<Client>) -> Self {
        let db_version = db.version().await.unwrap();
        let api_version = runtime_version();
        Self {
            api_version,
            db_version,
        }
    }
}

pub async fn bootstrap_runtime(log_level: &str) -> Vec<WorkerGuard> {
    // Ensure that the required folders exist
    ensure_required_folders_exist();

    // Setup the tracing / logging system
    let tracing_guards = setup_tracing(log_level, &logs_folder(), &log_file_name());

    tracing_guards
}

// Simple email format validation
pub fn is_valid_email_format(email: &str) -> bool {
    email.contains('@') && email.contains('.') && !email.is_empty() && email.len() > 3
}
