use chrono::{DateTime, Utc};
use serde::Serialize;

pub enum AuthError {
    InvalidCredentials,
    InvalidEmail,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub timestamp: DateTime<Utc>,
}
