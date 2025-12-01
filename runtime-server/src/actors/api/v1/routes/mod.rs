pub(crate) mod agent;
pub(crate) mod info;

use axum::{Extension, Router};
use std::sync::Arc;

use crate::actors::api::{
    state::{ApiState, V1ApiState},
    v1::routes::{agent::agent_router, info::info_router},
};

pub fn api_router() -> Router<Arc<ApiState>> {
    Router::new()
        .nest("/api/v1", v1_router())
        .nest("/api", v1_router()) // transition to latest version
}

fn v1_router() -> Router<Arc<ApiState>> {
    let api_version = "v1".to_string();
    let v1_state = Arc::new(V1ApiState::new());
    let api_id = v1_state.id.clone();

    Router::new()
        .merge(info_router(api_version, api_id))
        .merge(agent_router())
        .layer(Extension(v1_state))
}
