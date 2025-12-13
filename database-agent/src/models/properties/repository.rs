use super::types::{Property, PropertyValue, TypedProperty};
use crate::schema::properties;
use anyhow::{anyhow, Error};
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
};
use tracing::error;

/// Get a single property by key
pub fn get_property(
    key: String,
    mut connection: PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Option<TypedProperty> {
    match properties::table
        .filter(properties::key.eq(&key))
        .select(Property::as_select())
        .first(&mut connection)
        .optional()
    {
        Ok(Some(prop)) => match prop.to_typed() {
            Some(typed) => Some(typed),
            None => {
                error!(property=%key,"Invalid property value");
                None
            }
        },
        Ok(None) => None,
        Err(error) => {
            error!(errorMsg=%error,property=%key,"Error getting property");
            None
        }
    }
}

/// Get total count of properties
pub fn get_property_count(
    connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Result<i64, Error> {
    match properties::table.count().get_result(connection) {
        Ok(count) => Ok(count),
        Err(e) => Err(e.into()),
    }
}

/// Get paginated properties
pub fn get_properties(
    connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    per_page: i64,
    offset: i64,
) -> Result<Vec<TypedProperty>, Error> {
    match properties::table
        .limit(per_page)
        .offset(offset)
        .select(Property::as_select())
        .load(connection)
    {
        Ok(properties) => {
            // Convert each Property to TypedProperty
            let typed_props: Vec<TypedProperty> = properties
                .into_iter()
                .filter_map(|p| p.to_typed())
                .collect();

            Ok(typed_props)
        }
        Err(e) => Err(anyhow!(e.to_string())),
    }
}

/// Get a property value or return a default value
///
/// # Examples
/// ```
/// let port = get_property_value_or(conn, "api_port", PropertyValue::Int(8080));
/// let name = get_property_value_or(conn, "app_name", PropertyValue::String("default".to_string()));
/// ```
pub fn get_property_value_or(
    mut connection: PooledConnection<ConnectionManager<SqliteConnection>>,
    key: &str,
    default: PropertyValue,
) -> PropertyValue {
    match properties::table
        .filter(properties::key.eq(key))
        .select(Property::as_select())
        .first(&mut connection)
        .optional()
    {
        Ok(Some(prop)) => prop.value().unwrap_or(default),
        Ok(None) => default,
        Err(error) => {
            error!(errorMsg=%error,property=%key,"Error getting property, using default");
            default
        }
    }
}

/// Convenience methods for getting specific typed values with defaults
impl PropertyValue {
    /// Get an integer property or return a default
    pub fn get_int_or(
        connection: PooledConnection<ConnectionManager<SqliteConnection>>,
        key: &str,
        default: i32,
    ) -> i32 {
        match get_property_value_or(connection, key, PropertyValue::Int(default)) {
            PropertyValue::Int(v) => v,
            _ => default,
        }
    }

    /// Get a string property or return a default
    pub fn get_string_or(
        connection: PooledConnection<ConnectionManager<SqliteConnection>>,
        key: &str,
        default: String,
    ) -> String {
        match get_property_value_or(connection, key, PropertyValue::String(default.clone())) {
            PropertyValue::String(v) => v,
            _ => default,
        }
    }

    /// Get a boolean property or return a default
    pub fn get_bool_or(
        connection: PooledConnection<ConnectionManager<SqliteConnection>>,
        key: &str,
        default: bool,
    ) -> bool {
        match get_property_value_or(connection, key, PropertyValue::Bool(default)) {
            PropertyValue::Bool(v) => v,
            _ => default,
        }
    }

    /// Get a JSON property or return a default
    pub fn get_json_or(
        connection: PooledConnection<ConnectionManager<SqliteConnection>>,
        key: &str,
        default: serde_json::Value,
    ) -> serde_json::Value {
        match get_property_value_or(connection, key, PropertyValue::Json(default.clone())) {
            PropertyValue::Json(v) => v,
            _ => default,
        }
    }
}
