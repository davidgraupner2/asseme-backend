use crate::traits::Table;

#[derive(Debug)]
pub struct PropertyTable;

impl PropertyTable {
    pub fn new() -> Self {
        Self {}
    }
}

impl Table for PropertyTable {
    fn create_table_sql(&self) -> String {
        r#"
        CREATE TABLE IF NOT EXISTS "properties" (
            "id"	integer NOT NULL,
            "name" varchar NOT NULL,
            "description" varchar,
            "value_string"	varchar,
	        "value_int"	integer,
	        "value_boolean"	boolean,
	        "value_json"	json_text,
	        "value_type"	varchar NOT NULL,
            "created_at"	timestamp_with_timezone_text NOT NULL DEFAULT CURRENT_TIMESTAMP,
	        "updated_at"	timestamp_with_timezone_text NOT NULL DEFAULT CURRENT_TIMESTAMP,
            "source" varchar NOT NULL,
            PRIMARY KEY("id" AUTOINCREMENT)
        );

        CREATE UNIQUE INDEX IF NOT EXISTS  "unique_properties" ON "properties" ("name");

        -- Create a trigger to update the updated_at column when the row is updated
        CREATE TRIGGER IF NOT EXISTS "update_properties_updated_at"
        AFTER UPDATE ON properties
        FOR EACH ROW
        BEGIN
	        UPDATE properties SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.id;
        END;
        "#
        .to_string()
    }

    fn drop_table_sql(&self) -> String {
        "DROP TABLE IF EXISTS properties".to_string()
    }

    fn clone_box(&self) -> Box<dyn Table> {
        Box::new((*self).clone())
    }
}

impl Clone for PropertyTable {
    fn clone(&self) -> Self {
        PropertyTable::new()
    }
}
