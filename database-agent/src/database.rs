use super::traits::Table;
use crate::table_connection_string::ConnectionStringTable;
use anyhow::{anyhow, Result};
use deadpool_sqlite::{Config, InteractError, Runtime};
use tracing::{error, info};

#[derive(Debug, Clone)]
pub struct Database {
    pub pool: deadpool_sqlite::Pool,
    pub tables: Vec<Box<dyn Table>>,
}

// Ensure Database implements Send and Sync
unsafe impl Send for Database {}
unsafe impl Sync for Database {}

impl Database {
    pub fn new(database_file_name: String) -> Self {
        let cfg = Config::new(database_file_name);
        let pool = cfg.create_pool(Runtime::Tokio1).unwrap();

        let mut tables: Vec<Box<dyn Table>> = vec![];
        tables.push(Box::new(ConnectionStringTable::new()));
        Self { pool, tables }
    }

    pub async fn initialise(&self) -> Result<bool> {
        let conn = self.pool.get().await?;

        // Execute the create table SQL for each table
        for table in &self.tables {
            let create_sql = table.create_table_sql();

            match conn
                .interact(move |conn| conn.execute_batch(&create_sql))
                .await
            {
                Ok(table_initialise_result) => match table_initialise_result {
                    Ok(_) => info!("Table: {:#?} initialised successfully!", table),
                    Err(error) => error!(
                        "Table {:#?} NOT initialised successfully - {:#?}",
                        table, error
                    ),
                },
                Err(error) => {
                    error!("Error creating table: {:?}", error);
                    return Err(anyhow!(error.to_string()));
                }
            }
        }

        Ok(true)
    }

    pub async fn drop_tables(&self) -> Result<()> {
        let conn = self.pool.get().await?;

        Ok(for table in &self.tables {
            let drop_sql = table.drop_table_sql();
            let _ = match conn
                .interact(move |conn| conn.execute_batch(&drop_sql))
                .await
            {
                Ok(_) => Ok::<_, InteractError>(()),
                Err(error) => {
                    error!("Error dropping table: {:?}", error);
                    return Err(anyhow!(error.to_string()));
                }
            };
        })
    }
}
