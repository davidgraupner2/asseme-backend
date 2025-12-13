// use crate::actors::api::routes::v1::responses::ApiResponse;
// use axum::http::StatusCode;
// use axum::{response::IntoResponse, Json};
// use serde_json::Value;
// use thiserror::Error;

// #[derive(Debug, Error)]
// pub enum ApiError {
//     #[error("internal error: {0}")]
//     Internal(String),

//     #[error("not found: {0}")]
//     NotFound(String),

//     #[error("bad request: {0}")]
//     BadRequest(String),
// }

// impl ApiError {
//     pub fn status_code(&self) -> StatusCode {
//         match self {
//             ApiError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
//             ApiError::NotFound(_) => StatusCode::NOT_FOUND,
//             ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
//         }
//     }
// }

// impl IntoResponse for ApiError {
//     fn into_response(self) -> axum::response::Response {
//         // Use the same ApiResponse error envelope so clients always get the same shape
//         let body = ApiResponse::<Value>::err(self.to_string());
//         (self.status_code(), Json(body)).into_response()
//     }
// }
