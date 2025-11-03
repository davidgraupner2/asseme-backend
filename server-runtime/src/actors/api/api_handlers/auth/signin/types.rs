use chrono::{DateTime, Utc};
use database::model::user::User;
use serde::{Deserialize, Serialize};
use surrealdb::opt::auth::Jwt;

#[derive(Debug, Serialize, Deserialize)]
pub struct SigninRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct Tenant {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct SigninResponse {
    // pub success: bool,
    // pub message: String,
    pub jwt: Option<Jwt>,
    // pub timestamp: DateTime<Utc>,
    pub user: User,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub timestamp: DateTime<Utc>,
}
