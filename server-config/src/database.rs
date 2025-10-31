use serde::{Deserialize, Serialize};

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
            connection_type: "".to_string(),
            url: "".to_string(),
            user_name: "".to_string(),
            password: "".to_string(),
            namespace: "".to_string(),
            database: "".to_string(),
            access_method: "user".to_string(),
        }
    }
}
