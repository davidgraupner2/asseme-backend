pub(self) mod add;

use crate::actors::api::{
    api_handlers::tenant::add::handlers::handle_add_customer_tenant_to_msp, state::AxumApiState,
};
use axum::{routing::post, Router};
use std::sync::Arc;

pub fn tenant_router() -> Router<Arc<AxumApiState>> {
    let msp_routes =
        Router::new().route("/add_new_customer", post(handle_add_customer_tenant_to_msp));

    Router::new().nest("/msp", msp_routes)
}
