use crate::schema::connection_strings;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::connection_strings)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ConnectionStrings {
    pub id: i32,
    pub value: String,
    pub description: Option<String>,
    pub source: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = connection_strings)]
pub struct NewConnectionString {
    pub value: String,
    pub source: Option<String>,
    pub status: Option<String>,
    pub description: Option<String>,
}
