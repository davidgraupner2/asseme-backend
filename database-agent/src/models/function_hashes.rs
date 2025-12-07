use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Debug)]
#[diesel(table_name = crate::schema::function_hashes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct FunctionHashes {
    pub id: i32,
    pub function_hash: String,
    pub description: Option<String>,
    pub source: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::function_hashes)]
pub struct NewFunctionHash {
    pub function_hash: String,
    pub description: Option<String>,
    pub source: String,
}
