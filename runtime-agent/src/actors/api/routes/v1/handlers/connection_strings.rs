use crate::actors::api::{routes::v1::responses::ApiResponse, state::ApiState};
use crate::actors::{API_SOURCE_NAME, CONNECTION_STRING_PENDING_STATUS};
use axum::debug_handler;
use axum::{
    extract::{Json, State},
    response::IntoResponse,
};
use database_agent::models::connection_strings::{ConnectionStrings, NewConnectionString};
use database_agent::schema::connection_strings::dsl::connection_strings;
use diesel::{associations::HasTable, prelude::*};
use std::sync::Arc;

pub async fn v1_get_connection_strings(State(state): State<Arc<ApiState>>) -> impl IntoResponse {
    let mut db_conn = state.db_pool.get().unwrap();

    match connection_strings
        .select(ConnectionStrings::as_select())
        .load::<ConnectionStrings>(&mut db_conn)
    {
        Ok(connection_strings_vec) => ApiResponse::ok(connection_strings_vec),
        Err(error) => ApiResponse::err(error.to_string()),
    }
}

pub async fn v1_post_connection_strings(
    State(state): State<Arc<ApiState>>,
    Json(payload): Json<NewConnectionString>,
) -> impl IntoResponse {
    let mut db_conn = state.db_pool.get().unwrap();

    // Ensure the source is set to API
    let mut new_connection_string = payload;
    new_connection_string.source = Some(String::from(API_SOURCE_NAME));
    new_connection_string.status = Some(String::from(CONNECTION_STRING_PENDING_STATUS));

    match diesel::insert_into(connection_strings::table())
        .values(&new_connection_string)
        .returning(ConnectionStrings::as_returning())
        .get_result(&mut db_conn)
    {
        Ok(connection_string) => ApiResponse::ok(connection_string),
        Err(error) => ApiResponse::err(error.to_string()),
    }
}
