use crate::actors::api::state::AxumApiState;
use crate::common::AppVersion;
use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use std::sync::Arc;

pub fn misc_router() -> Router<Arc<AxumApiState>> {
    Router::new().route("/info", get(api_get_info))
}

pub async fn api_get_info(State(state): State<Arc<AxumApiState>>) -> Json<serde_json::Value> {
    let id = &state.id;
    let version = AppVersion::new(state.db_client.clone()).await;

    Json(serde_json::json!({
        "status": "ok",
        "id": id,
        "version": version
    }))
}
