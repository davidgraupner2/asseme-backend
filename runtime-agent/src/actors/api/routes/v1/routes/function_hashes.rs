use crate::actors::api::routes::v1::handlers::function_hashes::*;
use crate::actors::api::state::ApiState;
use axum::routing::{get, post};
use axum::Router;
use std::sync::Arc;

pub fn v1_function_hashes_router() -> Router<Arc<ApiState>> {
    Router::new()
        .route("/function_hashes", get(v1_get_function_hashes))
        .route("/function_hashes", post(v1_post_function_hashes))
}
