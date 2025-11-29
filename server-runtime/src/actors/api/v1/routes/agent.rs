use crate::actors::api::{
    state::{ApiState, V1ApiState},
    v1::handlers::agent_handler,
};
use axum::{routing::get, Extension, Router};
use std::sync::Arc;

pub fn agent_router() -> Router<Arc<V1ApiState>> {
    // Build a router for the /agent endpoint and attach the V1 state as an Extension
    Router::new().route("/agent", get(agent_handler))
}
