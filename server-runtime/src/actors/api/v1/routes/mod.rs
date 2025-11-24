pub(crate) mod info;

use axum::Router;
use std::sync::Arc;

use crate::actors::api::{
    state::{ApiState, V1ApiState},
    v1::routes::info::info_router,
};

pub fn api_router(state: ApiState, v1_state: Arc<V1ApiState>) -> Router<Arc<ApiState>> {
    Router::new()
        .nest("/api/v1", v1_router(v1_state.clone()))
        .nest("/api", v1_router(v1_state.clone())) // <- This will be transitioned to the latest version for simplicity
}

fn v1_router(v1_state: Arc<V1ApiState>) -> Router<Arc<ApiState>> {
    Router::new().merge(info_router("v1", v1_state))
}
