use crate::actors::api::{routes::v1::responses::ApiResponse, state::ApiState};
use axum::extract::Query;
use axum::{
    extract::{Json, State},
    response::IntoResponse,
};
use database_agent::models::function_hashes::{FunctionHashes, NewFunctionHash};
use database_agent::schema::function_hashes::dsl::function_hashes;
use diesel::associations::HasTable;
use diesel::prelude::*;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct FunctionHashPagination {
    function_hash: Option<String>,
    page: Option<isize>,
    per_page: Option<usize>,
}

pub async fn v1_get_function_hashes(
    State(state): State<Arc<ApiState>>,
    Query(params): Query<FunctionHashPagination>,
) -> impl IntoResponse {
    let mut db_conn = state.db_pool.get().unwrap();

    let query = match &params.function_hash {
        Some(hash) => function_hashes
            .filter(database_agent::schema::function_hashes::function_hash.eq(hash))
            .into_boxed(),
        None => function_hashes.into_boxed(),
    };

    let results = query.select(FunctionHashes::as_select()).load(&mut db_conn);

    match results {
        Ok(list) => ApiResponse::ok(list),
        Err(error) => ApiResponse::err(error.to_string()),
    }
}

pub async fn v1_post_function_hashes(
    State(state): State<Arc<ApiState>>,
    Json(payload): Json<NewFunctionHash>,
) -> impl IntoResponse {
    let mut db_conn = state.db_pool.get().unwrap();

    // Ensure the source is set to API
    let new_function_hash = payload;

    match diesel::insert_into(function_hashes::table())
        .values(&new_function_hash)
        .returning(FunctionHashes::as_returning())
        .get_result(&mut db_conn)
    {
        Ok(connection_string) => ApiResponse::ok(connection_string),
        Err(error) => ApiResponse::err(error.to_string()),
    }
}
