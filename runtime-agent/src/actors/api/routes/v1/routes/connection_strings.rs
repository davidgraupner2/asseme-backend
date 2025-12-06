use crate::actors::api::routes::v1::handlers::connection_strings::*;
use crate::actors::api::state::ApiState;
use axum::routing::{get, post};
use axum::Router;
use std::sync::Arc;

pub fn v1_connection_strings_router() -> Router<Arc<ApiState>> {
    Router::new()
        .route("/connection_strings", get(v1_get_connection_strings))
        .route(
            "/connection_strings/active",
            get(v1_get_connection_strings_active),
        )
        .route(
            "/connection_strings/pending",
            get(v1_get_connection_strings_pending),
        )
        .route("/connection_strings", post(v1_post_connection_strings))
}
