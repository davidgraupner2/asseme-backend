use crate::actors::api::routes::v1::auth_routes::auth_router;
use crate::actors::api::routes::v1::misc_routes::misc_router;
use crate::actors::api::state::AxumApiState;
use axum::Router;
use std::sync::Arc;

pub fn api_router(state: AxumApiState) -> Router<Arc<AxumApiState>> {
    Router::new()
        .nest("/api/v1", v1_router())
        .nest("/api", v1_router()) // <- This will be transitioned to the latest version for simplicity
}

fn v1_router() -> Router<Arc<AxumApiState>> {
    Router::new()
        .nest("/auth", auth_router())
        .merge(misc_router())
}
