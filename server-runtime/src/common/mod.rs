pub(super) mod bootstrap;

use crate::properties::runtime_version;
use semver::Version;
use serde::Serialize;
use surrealdb::{engine::remote::ws::Client, Surreal};

pub const APP_NAME: &str = "asseme";
pub const DB_TIMEOUT: u64 = 2000;

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

// Simple email format validation
pub fn is_valid_email_format(email: &str) -> bool {
    email.contains('@') && email.contains('.') && !email.is_empty() && email.len() > 3
}
