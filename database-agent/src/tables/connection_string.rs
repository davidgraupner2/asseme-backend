use crate::traits::Table;

#[derive(Debug)]
pub struct ConnectionStringTable;

impl ConnectionStringTable {
    pub fn new() -> Self {
        Self {}
    }
}

impl Table for ConnectionStringTable {
    fn create_table_sql(&self) -> String {
        r#"
        CREATE TABLE IF NOT EXISTS "connection_strings" (
            "id"	integer NOT NULL,
            "value"	varchar NOT NULL,
            "description"	varchar,
            "source" varchar NOT NULL,
            "created_at"	timestamp_with_timezone_text NOT NULL DEFAULT CURRENT_TIMESTAMP,
	        "updated_at"	timestamp_with_timezone_text NOT NULL DEFAULT CURRENT_TIMESTAMP,
            PRIMARY KEY("id" AUTOINCREMENT)
        );

        CREATE UNIQUE INDEX IF NOT EXISTS  "unique_connection_strings" ON "connection_strings" ("value");

        -- Create a trigger to update the updated_at column when the row is updated
        CREATE TRIGGER IF NOT EXISTS "update_connection_strings_updated_at"
        AFTER UPDATE ON connection_strings
        FOR EACH ROW
        BEGIN
	        UPDATE connection_strings SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.id;
        END;
        "#
        .to_string()
    }

    fn drop_table_sql(&self) -> String {
        "DROP TABLE IF EXISTS connection_strings".to_string()
    }

    fn clone_box(&self) -> Box<dyn Table> {
        Box::new((*self).clone())
    }
}

impl Clone for ConnectionStringTable {
    fn clone(&self) -> Self {
        ConnectionStringTable::new()
    }
}
