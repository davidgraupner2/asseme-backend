use crate::{
    actors::api::{routes::v1::responses::ApiResponse, state::ApiState},
    CONNECTION_STRING_ACTIVE_STATUS, CONNECTION_STRING_PENDING_STATUS,
};
use axum::{
    extract::{Json, State},
    response::IntoResponse,
};
use database_agent::models::connection_strings::{ConnectionStrings, NewConnectionString};
use database_agent::schema::connection_strings::dsl::connection_strings;
use database_agent::schema::connection_strings::status;
use diesel::{associations::HasTable, prelude::*};
use std::sync::Arc;

fn get_connection_strings_internal(
    state: Arc<ApiState>,
    status_filter: Option<&str>,
) -> impl IntoResponse {
    let mut db_conn = state.db_pool.get().unwrap();

    let mut query = connection_strings.into_boxed();
    if let Some(s) = status_filter {
        query = query.filter(status.eq(s));
    }

    match query
        .select(ConnectionStrings::as_select())
        .load(&mut db_conn)
    {
        Ok(list) => {
            if list.len() < 1 {
                return ApiResponse::ok_empty();
            }
            ApiResponse::ok(list)
        }
        Err(error) => ApiResponse::err(error.to_string()),
    }
}

pub async fn v1_get_connection_strings(State(state): State<Arc<ApiState>>) -> impl IntoResponse {
    get_connection_strings_internal(state, None);
}

pub async fn v1_get_connection_strings_active(
    State(state): State<Arc<ApiState>>,
) -> impl IntoResponse {
    get_connection_strings_internal(state, Some(CONNECTION_STRING_ACTIVE_STATUS))
}

pub async fn v1_get_connection_strings_pending(
    State(state): State<Arc<ApiState>>,
) -> impl IntoResponse {
    get_connection_strings_internal(state, Some(CONNECTION_STRING_PENDING_STATUS))
}

pub async fn v1_post_connection_strings(
    State(state): State<Arc<ApiState>>,
    Json(payload): Json<NewConnectionString>,
) -> impl IntoResponse {
    let mut db_conn = state.db_pool.get().unwrap();

    // Ensure the source is set to API
    let new_connection_string = payload;

    match diesel::insert_into(connection_strings::table())
        .values(&new_connection_string)
        .returning(ConnectionStrings::as_returning())
        .get_result(&mut db_conn)
    {
        Ok(connection_string) => ApiResponse::ok(connection_string),
        Err(error) => ApiResponse::err(error.to_string()),
    }
}
