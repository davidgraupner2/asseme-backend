use crate::actors::api::routes::v1::handlers::properties::*;
use crate::actors::api::state::ApiState;
use axum::routing::{get, post};
use axum::Router;
use std::sync::Arc;

pub fn v1_properties_router() -> Router<Arc<ApiState>> {
    Router::new()
        .route("/property", get(v1_get_properties))
        .route("/property/{key}", get(v1_get_property))
        .route("/property", post(v1_post_properties))
}
