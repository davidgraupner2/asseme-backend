use axum::routing::get;
use axum::Router;
use std::sync::Arc;

use crate::actors::api::state::{ApiState, V1ApiState};
use crate::actors::api::v1::handlers::get_info;

pub fn info_router(api_version: &str, state: Arc<V1ApiState>) -> Router<Arc<ApiState>> {
    let api_version = api_version.to_owned();
    let id = state.id.clone();
    Router::new().route(
        "/info",
        get(move || async move { get_info(&api_version, &id).await }),
    )
}
