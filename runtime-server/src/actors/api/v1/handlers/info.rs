use axum::{response::IntoResponse, Json};
use runtime_shared::RuntimeProperties;
use serde::Serialize;

use crate::actors::api::v1::api_response::ApiResponse;

#[derive(Serialize)]
struct V1Info {
    api_version: String,
    binary_version: String,
    id: String,
    status: String,
}

pub async fn get_info(api_version: String, id: String) -> impl IntoResponse {
    let properties = RuntimeProperties::global();

    let version = properties.version().to_string();

    let info = V1Info {
        api_version,
        binary_version: version,
        id,
        status: "ok".to_string(),
    };

    ApiResponse::ok(info) // ApiResponse<User> -> IntoResponse

    // Json(serde_json::json!({
    //     "api_version": api_version,
    //     "binary_version": version,
    //     "id": id,
    //     "status": "ok",
    // }))
}
