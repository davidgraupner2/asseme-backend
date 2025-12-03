use axum::response::IntoResponse;
use runtime_shared::RuntimeProperties;
use serde::Serialize;

use crate::actors::api::routes::v1::responses::ApiResponse;

#[derive(Serialize)]
struct V1Info {
    api_version: String,
    binary_version: String,
    id: String,
    status: String,
}

pub async fn v1_get_info(api_version: String, id: String) -> impl IntoResponse {
    let properties = RuntimeProperties::global();

    let version = properties.version().to_string();

    let info = V1Info {
        api_version,
        binary_version: version,
        id,
        status: "ok".to_string(),
    };

    ApiResponse::ok(info) // ApiResponse<User> -> IntoResponse
}
