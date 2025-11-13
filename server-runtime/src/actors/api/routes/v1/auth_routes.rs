use crate::actors::api::handlers::auth_handlers;
use crate::actors::api::state::AxumApiState;
use axum::{routing::post, Router};
use std::sync::Arc;

pub fn auth_router() -> Router<Arc<AxumApiState>> {
    Router::new().route("/signin", post(auth_handlers::signin_handler))
}
