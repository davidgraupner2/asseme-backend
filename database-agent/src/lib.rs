pub mod models;
pub mod schema;

use std::path::PathBuf;

use anyhow::{anyhow, Error};
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel_migrations::embed_migrations;
use diesel_migrations::EmbeddedMigrations;
use diesel_migrations::MigrationHarness;
use tracing::{error, info, warn};

pub type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;

// Embed migrations from the default "migrations" directory
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn get_db_connection_pool(folder_name: &PathBuf, db_name: &str) -> Result<SqlitePool, Error> {
    let db_file_name = folder_name.join(db_name).to_string_lossy().to_string();

    let manager = ConnectionManager::<SqliteConnection>::new(db_file_name.clone());

    Ok(Pool::builder()
        .max_size(10)
        .test_on_check_out(true)
        .build(manager)?)
}

pub fn ensure_database_schema(db_name: String) -> Result<(), Error> {
    // Connect to our agent database and execute any pending migrations
    match SqliteConnection::establish(&db_name) {
        Ok(mut connection) => match connection.run_pending_migrations(MIGRATIONS) {
            Ok(migrated) => {
                info!(database_migrations=%migrated.len(), "Database migrations executed successfully");
                Ok(())
            }
            Err(error) => {
                warn!(errorMsg=%error,"Database migrations did NOT execute successfully!");
                Err(anyhow!(error.to_string()))
            }
        },
        Err(error) => {
            error!(errorMsg=%error, database=%db_name, "Unable to connect to database");
            Err(anyhow!(error.to_string()))
        }
    }
}

// Public re-exports
pub use models::connection_strings::ConnectionStrings;
pub use models::tags::Tags;
