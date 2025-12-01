use crate::actors::api::v1::handlers::agent::get_agent_token_handler;
use crate::actors::api::{state::ApiState, v1::handlers::agent_connection_handler};
use axum::{routing::get, Router};
use std::sync::Arc;

pub fn agent_router() -> Router<Arc<ApiState>> {
    Router::new()
        .route("/agent", get(agent_connection_handler))
        .route("/agent/token", get(get_agent_token_handler))
}
