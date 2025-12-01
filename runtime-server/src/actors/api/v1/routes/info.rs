use crate::actors::api::state::ApiState;
use crate::actors::api::v1::handlers::get_info;
use axum::routing::get;
use axum::Router;
use std::sync::Arc;

pub fn info_router(api_version: String, api_id: String) -> Router<Arc<ApiState>> {
    let version = api_version;
    let id = api_id;

    Router::new().route(
        "/info",
        get(move || {
            let version = version.clone();
            let id = id.clone();
            async move { get_info(version, id).await }
        }),
    )
}
