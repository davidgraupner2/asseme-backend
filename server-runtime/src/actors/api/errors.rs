use axum::{http::StatusCode, response::IntoResponse};
use surrealdb::error::Api;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("User not found")]
    UserNotFound,
    #[error("Database error: {0}")]
    Database(#[from] surrealdb::Error),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
}

pub fn handle_api_error(error: ApiError) -> (StatusCode, String) {
    match error {
        ApiError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()),
        ApiError::InvalidCredentials => (StatusCode::UNAUTHORIZED, error.to_string()),
        ApiError::UserNotFound => (StatusCode::BAD_REQUEST, error.to_string()),
        ApiError::Unauthorized(_) => (StatusCode::UNAUTHORIZED, error.to_string()),
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            ApiError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            ApiError::InvalidCredentials => (StatusCode::UNAUTHORIZED, self.to_string()),
            ApiError::UserNotFound => (StatusCode::BAD_REQUEST, self.to_string()),
            ApiError::Unauthorized(_) => (StatusCode::UNAUTHORIZED, self.to_string()),
        };
        (status, error_message).into_response()
    }
}
