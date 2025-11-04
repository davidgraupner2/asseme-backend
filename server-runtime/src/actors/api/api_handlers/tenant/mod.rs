pub(self) mod add;

use crate::actors::api::middleware::authentication::authenticated;
use crate::actors::api::middleware::permissions::permissions_check;
use crate::actors::api::{
    api_handlers::tenant::add::handlers::handle_add_customer_tenant_to_msp, state::AxumApiState,
};
use axum::middleware::{self};
use axum::{routing::post, Router};
use std::sync::Arc;

pub fn tenant_router(state: AxumApiState) -> Router<Arc<AxumApiState>> {
    let msp_routes = Router::new()
        .route("/add_new_customer", post(handle_add_customer_tenant_to_msp))
        .layer(middleware::from_fn_with_state(state.clone(), authenticated))
        .layer(middleware::from_fn(permissions_check));

    Router::new().nest("/msp", msp_routes)
}
