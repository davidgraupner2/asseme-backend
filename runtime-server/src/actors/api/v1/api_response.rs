use axum::http::StatusCode;
use axum::{response::IntoResponse, Json};
use serde::Serialize;
use serde_json::Value;
use thiserror::Error;

/// Generic API envelope for successful or error responses.
#[derive(Debug, Serialize)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    /// Success with a payload
    pub fn ok(data: T) -> Self {
        ApiResponse {
            ok: true,
            data: Some(data),
            error: None,
        }
    }

    /// Success with no payload
    pub fn ok_empty() -> Self {
        ApiResponse {
            ok: true,
            data: None,
            error: None,
        }
    }

    pub fn err(msg: impl Into<String>) -> Self {
        ApiResponse {
            ok: false,
            data: None,
            error: Some(msg.into()),
        }
    }
}

/// Convert ApiResponse<T> into an HTTP response (JSON) with a status code.
/// By default we return 200 for ok responses, 400 for error envelopes.
/// You can customize this mapping as you need.
impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        if self.ok {
            // 200 OK with JSON body
            (StatusCode::OK, Json(self)).into_response()
        } else {
            // error envelope -> 400 Bad Request; change mapping as desired
            (StatusCode::BAD_REQUEST, Json(self)).into_response()
        }
    }
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("internal error: {0}")]
    Internal(String),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("bad request: {0}")]
    BadRequest(String),
}

impl ApiError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            ApiError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        // Use the same ApiResponse error envelope so clients always get the same shape
        let body = ApiResponse::<Value>::err(self.to_string());
        (self.status_code(), Json(body)).into_response()
    }
}
