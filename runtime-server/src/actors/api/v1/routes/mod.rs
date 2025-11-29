pub(crate) mod agent;
pub(crate) mod info;

use axum::Router;
use std::sync::Arc;

use crate::actors::api::{
    state::{ApiState, V1ApiState},
    v1::routes::{agent::agent_router, info::info_router},
};

/// Build the API router. `api_state` is the global application state (shared
/// across API versions). `v1_state` is the version-specific state which will
/// be attached to the v1 routes as an `Extension` so handlers can extract it.
pub fn api_router(api_state: ApiState, v1_state: V1ApiState) -> Router<Arc<ApiState>> {
    Router::new()
        .nest("/api/v1", v1_router(api_state.clone(), v1_state.clone()))
        .nest("/api", v1_router(api_state.clone(), v1_state.clone())) // transition to latest version
}

fn v1_router(api_state: ApiState, v1_state: V1ApiState) -> Router<Arc<ApiState>> {
    // build the v1 router. The `info_router` and `agent_router` functions
    // already accept `v1_state` and capture it where needed, so simply
    // merge them here and set the application state to `api_state`.
    Router::new()
        .merge(info_router("v1", v1_state.clone().into()))
        .merge(agent_router().with_state(v1_state.clone().into()))
        .with_state(api_state.into())
}
