pub mod schema;

use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use tracing::{error, info, warn};

// Embed migrations from the default "migrations" directory
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn establish_db_connection(db_name: String) -> Result<SqliteConnection, ConnectionError> {
    // Connect to our agent database and execute any pending migrations
    match SqliteConnection::establish(&db_name) {
        Ok(mut connection) => {
            info!(database=%db_name, "Connected to database successfully");

            match connection.run_pending_migrations(MIGRATIONS) {
                Ok(migrated) => {
                    info!(database_migrations=%migrated.len(), "Database migrations executed successfully");
                    Ok(connection)
                }
                Err(error) => {
                    warn!(errorMsg=%error,"Database migrations did NOT execute successfully!");
                    Ok(connection)
                }
            }
        }
        Err(error) => {
            error!(errorMsg=%error, database=%db_name, "Unable to connect to database");
            Err(error)
        }
    }
}
