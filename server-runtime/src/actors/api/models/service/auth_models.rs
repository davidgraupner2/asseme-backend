use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::opt::auth::Jwt;

#[derive(Debug, Serialize, Deserialize)]
pub struct SigninRequest {
    pub email: String,
    pub password: String,
    pub namespace: String,
    pub database: String,
    pub access_method: String,
}

#[derive(Debug, Serialize)]
pub struct SigninResponse {
    pub jwt: Jwt,
}
