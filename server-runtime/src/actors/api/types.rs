use axum::http::StatusCode;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub(crate) type ApiResult<T> = Result<ApiResponse<T>, ApiResponse<()>>;

use server_config::{api::ApiConfiguration, database::DatabaseConfiguration};

pub struct APIStartupArguments {
    pub api_config: ApiConfiguration,
    pub database_config: DatabaseConfiguration,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub status_code: u16,
    pub timestamp: DateTime<Utc>,
}

impl<T> ApiResponse<T> {
    /// Create a successful response
    pub fn success(data: T, status_code: StatusCode) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            status_code: status_code.as_u16(),
            timestamp: Utc::now(),
        }
    }

    /// Create a successful response with default 200 OK
    pub fn ok(data: T) -> Self {
        Self::success(data, StatusCode::OK)
    }

    /// Create an error response
    pub fn error(error_message: String, status_code: StatusCode) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error_message),
            status_code: status_code.as_u16(),
            timestamp: Utc::now(),
        }
    }

    /// Create a bad request error
    pub fn bad_request(error_message: String) -> Self {
        Self::error(error_message, StatusCode::BAD_REQUEST)
    }

    /// Create an unauthorized error
    pub fn unauthorized(error_message: String) -> Self {
        Self::error(error_message, StatusCode::UNAUTHORIZED)
    }

    /// Create an internal server error
    pub fn internal_error(error_message: String) -> Self {
        Self::error(error_message, StatusCode::INTERNAL_SERVER_ERROR)
    }

    /// Create a not found error
    pub fn not_found(error_message: String) -> Self {
        Self::error(error_message, StatusCode::NOT_FOUND)
    }

    pub fn validation_error(message: String) -> Self {
        Self::bad_request(format!("Validation error: {}", message))
    }

    pub fn auth_failed() -> Self {
        Self::unauthorized("Authentication failed".to_string())
    }

    pub fn server_error() -> Self {
        Self::internal_error("Internal server error".to_string())
    }
}

// Conversion from ApiResponse to HTTP response
impl<T> From<ApiResponse<T>> for (StatusCode, axum::Json<ApiResponse<T>>)
where
    T: Serialize,
{
    fn from(response: ApiResponse<T>) -> Self {
        let status =
            StatusCode::from_u16(response.status_code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status, axum::Json(response))
    }
}

// IntoResponse implementation for ApiResponse
impl<T> axum::response::IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        let (status, json) = self.into();
        (status, json).into_response()
    }
}
