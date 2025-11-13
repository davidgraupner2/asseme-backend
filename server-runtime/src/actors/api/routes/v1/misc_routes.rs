use crate::actors::api::handlers::misc_handlers;
use crate::actors::api::state::AxumApiState;
use axum::routing::get;
use axum::Router;
use std::sync::Arc;

pub fn misc_router() -> Router<Arc<AxumApiState>> {
    Router::new().route("/info", get(misc_handlers::api_get_info))
}
