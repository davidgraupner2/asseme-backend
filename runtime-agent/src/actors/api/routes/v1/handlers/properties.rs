use crate::actors::api::pagination::{PaginationMeta, PaginationQuery};
use crate::actors::api::{
    routes::v1::responses::{ApiResponse, PaginatedApiResponse},
    state::ApiState,
};
use axum::extract::{Path, Query};
use axum::{
    extract::{Json, State},
    response::IntoResponse,
};
use database_agent::models::properties::{
    get_properties, get_property, get_property_count, NewProperty, Property, PropertyValue,
    TypedProperty,
};
use database_agent::schema::properties;
use diesel::prelude::*;
use serde::Deserialize;
use std::sync::Arc;

// POST /properties - accepts flat JSON structure
#[derive(Deserialize)]
pub struct NewPropertyRequest {
    key: String,
    #[serde(rename = "type")]
    type_: String,
    description: Option<String>,
    value: serde_json::Value, // Accept raw JSON value
}

impl NewPropertyRequest {
    // Convert to PropertyValue based on type
    fn to_property_value(&self) -> Result<PropertyValue, String> {
        match self.type_.as_str() {
            "int" => self
                .value
                .as_i64()
                .and_then(|v| i32::try_from(v).ok())
                .map(PropertyValue::Int)
                .ok_or_else(|| "Invalid integer value".to_string()),
            "string" => self
                .value
                .as_str()
                .map(|s| PropertyValue::String(s.to_string()))
                .ok_or_else(|| "Invalid string value".to_string()),
            "bool" => self
                .value
                .as_bool()
                .map(PropertyValue::Bool)
                .ok_or_else(|| "Invalid boolean value".to_string()),
            "json" => Ok(PropertyValue::Json(self.value.clone())),
            _ => Err(format!("Invalid property type: {}", self.type_)),
        }
    }
}

pub async fn v1_get_property(
    State(state): State<Arc<ApiState>>,
    Path(key): Path<String>,
) -> impl IntoResponse {
    let db_conn = state.db_pool.get().unwrap();

    match get_property(key, db_conn) {
        Some(property) => ApiResponse::ok(property),
        None => ApiResponse::ok_empty(),
    }
}

pub async fn v1_get_properties(
    State(state): State<Arc<ApiState>>,
    Query(pagination_query): Query<PaginationQuery>,
) -> impl IntoResponse {
    let mut db_conn = state.db_pool.get().unwrap();

    let pagination = pagination_query.pagination();

    // Get total count for pagination metadata
    let total = match get_property_count(&mut db_conn) {
        Ok(count) => count,
        Err(e) => return PaginatedApiResponse::<TypedProperty>::err(e.to_string()).into_response(),
    };

    match get_properties(&mut db_conn, pagination.per_page, pagination.offset) {
        Ok(properties) => {
            let pagination_meta = PaginationMeta::new(&pagination, total);
            let pagination_json = serde_json::to_value(pagination_meta).unwrap();

            PaginatedApiResponse::ok(properties, pagination_json).into_response()
        }
        Err(error) => PaginatedApiResponse::<TypedProperty>::err(error.to_string()).into_response(),
    }
}

pub async fn v1_post_properties(
    State(state): State<Arc<ApiState>>,
    Json(payload): Json<NewPropertyRequest>,
) -> impl IntoResponse {
    let mut db_conn = state.db_pool.get().unwrap();

    // Validate the value matches the declared type
    let new_prop = match validate_and_create_property(payload) {
        Ok(prop) => prop,
        Err(e) => return ApiResponse::err(e),
    };

    match diesel::insert_into(properties::table)
        .values(&new_prop)
        .returning(Property::as_returning())
        .get_result(&mut db_conn)
    {
        Ok(prop) => ApiResponse::ok(prop.to_typed()),
        Err(e) => ApiResponse::err(e.to_string()),
    }
}

fn validate_and_create_property(req: NewPropertyRequest) -> Result<NewProperty, String> {
    // Convert the raw JSON value to PropertyValue based on declared type
    let property_value = req.to_property_value()?;

    match property_value {
        PropertyValue::Int(val) => Ok(NewProperty {
            key: req.key,
            type_: req.type_,
            description: req.description,
            value_int: Some(val),
            value_string: None,
            value_bool: None,
            value_json: None,
        }),
        PropertyValue::String(val) => Ok(NewProperty {
            key: req.key,
            type_: req.type_,
            description: req.description,
            value_int: None,
            value_string: Some(val),
            value_bool: None,
            value_json: None,
        }),
        PropertyValue::Bool(val) => Ok(NewProperty {
            key: req.key,
            type_: req.type_,
            description: req.description,
            value_int: None,
            value_string: None,
            value_bool: Some(if val { 1 } else { 0 }),
            value_json: None,
        }),
        PropertyValue::Json(val) => {
            let json_str =
                serde_json::to_string(&val).map_err(|e| format!("Invalid JSON: {}", e))?;

            Ok(NewProperty {
                key: req.key,
                type_: req.type_,
                description: req.description,
                value_int: None,
                value_string: None,
                value_bool: None,
                value_json: Some(json_str),
            })
        }
    }
}
