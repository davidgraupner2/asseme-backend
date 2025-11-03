pub(self) mod signin;
pub(self) mod signup;

// Visibility Explained
/// pub ... - Visible everywhere (External crates can use)
/// pub(self) - same as private - internal helper functions withoin same module
/// pub(super) - visible to parent module (e.g. Only Auth module)
/// pub(crate) - visible to this crate only
use crate::actors::api::{
    api_handlers::auth::signin::handlers::handle_signin, state::AxumApiState,
};
use axum::{routing::post, Router};
use std::sync::Arc;

pub fn auth_router() -> Router<Arc<AxumApiState>> {
    Router::new()
        .route("/signin", post(handle_signin))
        .route("/signup", post(signup::handlers::handle_signup)) // Add this
}
