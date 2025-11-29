use axum::Json;
use runtime_shared::RuntimeProperties;

pub async fn get_info(api_version: &str, id: &str) -> Json<serde_json::Value> {
    let properties = RuntimeProperties::global();

    let id = &id;
    let version = properties.version();

    Json(serde_json::json!({
        "api_version": api_version,
        "binary_version": version,
        "id": id,
        "status": "ok",
    }))
}
