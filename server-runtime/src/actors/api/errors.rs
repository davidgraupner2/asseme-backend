use axum::http::StatusCode;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("User not found")]
    UserNotFound,
    #[error("Database error: {0}")]
    Database(#[from] surrealdb::Error),
}

pub fn handle_api_error(error: ApiError) -> (StatusCode, String) {
    match error {
        ApiError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()),
        ApiError::InvalidCredentials => (StatusCode::UNAUTHORIZED, error.to_string()),
        ApiError::UserNotFound => (StatusCode::BAD_REQUEST, error.to_string()),
    }
}
