pub(crate) mod v1;

use axum::{Extension, Router};

use std::sync::Arc;

// #[derive(Serialize)]
// pub struct PaginatedResponse<T> {
//     pub data: Vec<T>,
//     pub pagination: PaginationMeta,
// }

use crate::actors::api::{
    routes::v1::routes::{
        connection_strings::v1_connection_strings_router,
        function_hashes::v1_function_hashes_router, info::v1_info_router,
        properties::v1_properties_router,
    },
    state::{ApiState, V1ApiState},
};

pub fn api_router() -> Router<Arc<ApiState>> {
    Router::new()
        .nest("/api/v1", v1_router())
        .nest("/api", v1_router()) // transition to latest version
}

fn v1_router() -> Router<Arc<ApiState>> {
    let api_version = "v1".to_string();
    let v1_state = Arc::new(V1ApiState::new());
    let api_id = v1_state.id.clone();

    Router::new()
        .merge(v1_info_router(api_version, api_id))
        .merge(v1_connection_strings_router())
        .merge(v1_function_hashes_router())
        .merge(v1_properties_router())
        .layer(Extension(v1_state))
}
