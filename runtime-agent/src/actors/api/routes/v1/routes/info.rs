use crate::actors::api::routes::v1::handlers::info::v1_get_info;
use crate::actors::api::state::ApiState;
use axum::routing::get;
use axum::Router;
use std::sync::Arc;

pub fn v1_info_router(api_version: String, api_id: String) -> Router<Arc<ApiState>> {
    let version = api_version;
    let id = api_id;

    Router::new().route(
        "/info",
        get(move || {
            let version = version.clone();
            let id = id.clone();
            async move { v1_get_info(version, id).await }
        }),
    )
}
