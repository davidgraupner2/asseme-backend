use surrealdb::{
    engine::remote::ws::{Client, Ws, Wss},
    opt::auth::Root,
    Result, Surreal,
};
use tracing::{error, event, info, warn, Level};

/// Database connection and schema initialization module
///
/// This module provides functions to connect to SurrealDB and initialize
/// the complete MSP multi-tenant schema including all tables, relationships,
/// roles, and sample data.
///
/// Example usage:
/// ```rust
/// use database::context::{get_initialized_database, check_database_health};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // Get database with automatic schema initialization
///     let db = get_initialized_database(
///         "ws".to_string(),
///         "localhost:8000".to_string(),
///         "root".to_string(),
///         "root".to_string(),
///         "asseme".to_string(),
///         "msp".to_string(),
///         false, // Don't force reinit if schema exists
///     ).await?;
///     
///     // Check database health
///     let health = check_database_health(&db).await?;
///     println!("{}", health.summary());
///     
///     // Database is ready to use with all models!
///     Ok(())
/// }
/// ```

pub async fn get_database(
    connection_type: String,
    url: String,
    user_name: String,
    password: String,
    namespace: String,
    database: String,
) -> Result<Surreal<Client>> {
    event!(
        Level::DEBUG,
        "Attempting Connection to database on: {}://{}",
        &connection_type,
        &url
    );

    let db = match connection_type.as_str() {
        "wss" => Surreal::new::<Wss>(&url).await,
        _ => Surreal::new::<Ws>(&url).await,
    };

    let db_client =
        db.expect("DATABASE_CONNECTION_ERROR: API Server Failed to connect to database");

    if let Err(e) = db_client
        .signin(Root {
            username: &user_name,
            password: &password,
        })
        .await
    {
        panic!("DATABASE_AUTH_ERROR: API Server Failed to authenticate with database. Check username/password in config: {}", e);
    } else {
        db_client.use_ns(&namespace).await.expect(&format!(
            "DATABASE_CONNECTION_ERROR: Namespace {} is not valid",
            namespace
        ));
        db_client.use_db(&database).await.expect(&format!(
            "DATABASE_CONNECTION_ERROR: Database {} is not valid",
            database
        ));

        Ok(db_client)
    }
}

/// Initialize the database schema
///
/// This function creates all tables, fields, indexes, roles, and sample data
/// based on the MSP multi-tenant schema. It can be run multiple times safely.
pub async fn initialize_schema(db: &Surreal<Client>) -> Result<()> {
    info!("Starting database schema initialization...");

    // Execute the complete schema setup
    let schema_sql = include_str!("../../db_scripts/create_msp_tables.sql");

    match db.query(schema_sql).await {
        Ok(_response) => {
            info!("Schema initialization completed successfully");

            // Verify schema was created by checking for super admin user
            match verify_schema_initialization(db).await {
                Ok(true) => {
                    info!("Schema verification successful - super admin user found");
                    Ok(())
                }
                Ok(false) => {
                    warn!("Schema verification failed - super admin user not found");
                    Err(surrealdb::Error::Db(surrealdb::error::Db::QueryNotExecuted))
                }
                Err(e) => {
                    error!("Schema verification error: {}", e);
                    Err(e)
                }
            }
        }
        Err(e) => {
            error!("Failed to initialize schema: {}", e);
            Err(e)
        }
    }
}

/// Verify that the schema was properly initialized
async fn verify_schema_initialization(db: &Surreal<Client>) -> Result<bool> {
    // Check if super admin user exists
    let query = "SELECT * FROM user WHERE id = 'user:super_admin'";

    match db.query(query).await {
        Ok(mut response) => match response.take::<Vec<crate::model::user::User>>(0) {
            Ok(users) => Ok(!users.is_empty()),
            Err(_) => Ok(false),
        },
        Err(e) => Err(e),
    }
}

/// Get or create database with schema initialization
///
/// This is a convenience function that combines database connection and schema setup
pub async fn get_initialized_database(
    connection_type: String,
    url: String,
    user_name: String,
    password: String,
    namespace: String,
    database: String,
    force_reinit: bool,
) -> Result<Surreal<Client>> {
    // Get database connection
    let db = get_database(
        connection_type,
        url,
        user_name,
        password,
        namespace,
        database,
    )
    .await?;

    // Check if schema needs initialization
    let needs_init = force_reinit || !verify_schema_initialization(&db).await.unwrap_or(false);

    if needs_init {
        info!("Database schema needs initialization...");
        initialize_schema(&db).await?;
    } else {
        info!("Database schema already initialized");
    }

    Ok(db)
}

/// Reset database (remove and recreate)
///
/// WARNING: This will delete all data! Only use for development/testing
pub async fn reset_database(
    connection_type: String,
    url: String,
    user_name: String,
    password: String,
    namespace: String,
    database: String,
) -> Result<Surreal<Client>> {
    warn!("RESETTING DATABASE - ALL DATA WILL BE LOST!");

    // Connect with root access to manage databases
    let db = match connection_type.as_str() {
        "wss" => Surreal::new::<Wss>(&url).await,
        _ => Surreal::new::<Ws>(&url).await,
    }?;

    db.signin(Root {
        username: &user_name,
        password: &password,
    })
    .await?;

    db.use_ns(&namespace).await?;

    // Remove and recreate database
    let reset_sql = format!(
        "REMOVE DATABASE IF EXISTS {}; DEFINE DATABASE {} COMMENT 'Asset Me - MSP Primary Database';",
        database, database
    );

    db.query(reset_sql).await?;
    db.use_db(&database).await?;

    // Initialize schema
    initialize_schema(&db).await?;

    Ok(db)
}

/// Check database health and schema status
pub async fn check_database_health(db: &Surreal<Client>) -> Result<DatabaseHealth> {
    let mut health = DatabaseHealth::default();

    // Check if we can query the database
    match db.query("SELECT time::now() as current_time").await {
        Ok(_) => health.connection_ok = true,
        Err(e) => {
            health.connection_ok = false;
            health.errors.push(format!("Connection test failed: {}", e));
        }
    }

    // Check if super admin exists
    match verify_schema_initialization(db).await {
        Ok(true) => health.schema_initialized = true,
        Ok(false) => {
            health.schema_initialized = false;
            health
                .warnings
                .push("Super admin user not found - schema may not be initialized".to_string());
        }
        Err(e) => {
            health.schema_initialized = false;
            health.errors.push(format!("Schema check failed: {}", e));
        }
    }

    // Check table counts
    let tables = vec![
        "user",
        "tenant",
        "role",
        "user_role",
        "msp_customer_relationship",
        "billing",
        "token",
    ];
    for table in tables {
        match db.query(format!("SELECT count() FROM {}", table)).await {
            Ok(mut response) => {
                if let Ok(result) = response.take::<Option<surrealdb::sql::Value>>(0) {
                    if let Some(surrealdb::sql::Value::Number(count)) = result {
                        health
                            .table_counts
                            .insert(table.to_string(), count.as_int() as usize);
                    }
                }
            }
            Err(e) => {
                health
                    .warnings
                    .push(format!("Could not count {} table: {}", table, e));
            }
        }
    }

    Ok(health)
}

#[derive(Debug, Default)]
pub struct DatabaseHealth {
    pub connection_ok: bool,
    pub schema_initialized: bool,
    pub table_counts: std::collections::HashMap<String, usize>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

impl DatabaseHealth {
    pub fn is_healthy(&self) -> bool {
        self.connection_ok && self.schema_initialized && self.errors.is_empty()
    }

    pub fn summary(&self) -> String {
        format!(
            "Database Health: {} | Connection: {} | Schema: {} | Errors: {} | Warnings: {}",
            if self.is_healthy() {
                "HEALTHY"
            } else {
                "UNHEALTHY"
            },
            if self.connection_ok { "OK" } else { "FAILED" },
            if self.schema_initialized {
                "OK"
            } else {
                "MISSING"
            },
            self.errors.len(),
            self.warnings.len()
        )
    }
}
