use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DatabaseConfiguration {
    pub connection_type: String,
    pub url: String,
    pub user_name: String,
    pub password: String,
    pub namespace: String,
    pub database: String,
    pub access_method: String,
}

impl Default for DatabaseConfiguration {
    fn default() -> Self {
        DatabaseConfiguration {
            connection_type: env::var("DB_CONNECTION_TYPE").unwrap_or("ws".to_string()),
            url: env::var("DB_URL").unwrap_or("".to_string()),
            user_name: env::var("DB_USER_NAME").unwrap_or("".to_string()),
            password: env::var("DB_PASSWORD").unwrap_or("".to_string()),
            namespace: env::var("DB_NAMESPACE").unwrap_or("".to_string()),
            database: env::var("DB_DATABASE").unwrap_or("".to_string()),
            access_method: env::var("DB_ACCESS_METHOD").unwrap_or("".to_string()),
        }
    }
}
