// Visibility Explained
/// pub ... - Visible everywhere (External crates can use)
/// pub(self) - same as private - internal helper functions withoin same module
/// pub(super) - visible to parent module (e.g. Only Auth module)
/// pub(crate) - visible to this crate only
use crate::actors::api::{
    api_handlers::auth::{
        signin::handlers::handle_signin,
        signup::handlers::{handle_customer_signup, handle_msp_signup},
    },
    state::AxumApiState,
};
use axum::{routing::post, Router};
use std::sync::Arc;
use utoipa::OpenApi;

pub fn auth_router() -> Router<Arc<AxumApiState>> {
    Router::new()
        .route("/signin", post(handle_signin))
        .route("/signup", post(handle_customer_signup))
        .route("/signup_msp", post(handle_msp_signup))
}
