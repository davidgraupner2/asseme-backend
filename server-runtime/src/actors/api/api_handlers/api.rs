use crate::actors::api::api_handlers;
use crate::actors::api::state::AxumApiState;
use axum::Router;
use std::sync::Arc;

pub fn api_router(state: AxumApiState) -> Router<Arc<AxumApiState>> {
    // Create the API Routes
    let api = Router::new()
        .nest("/auth", api_handlers::auth::routes::auth_router())
        .nest(
            "/tenant",
            api_handlers::tenant::tenant_router(state.clone()),
        )
        .merge(api_handlers::misc::misc_router()); // Move misc router inside /api

    // Nest the API Routes under /api
    Router::new().nest("/api", api)
}
