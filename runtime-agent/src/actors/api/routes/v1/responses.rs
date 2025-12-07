use axum::http::StatusCode;
use axum::{response::IntoResponse, Json};
use serde::Serialize;

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
        if self.ok && self.data.is_some() {
            // 200 OK with JSON body
            (StatusCode::OK, Json(self)).into_response()
        } else if self.ok && self.data.is_none() {
            // 200 OK with JSON body
            (StatusCode::NOT_FOUND, Json(self)).into_response()
        } else {
            // error envelope -> 400 Bad Request; change mapping as desired
            (StatusCode::BAD_REQUEST, Json(self)).into_response()
        }
    }
}
