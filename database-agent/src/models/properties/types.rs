use crate::schema::properties;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Debug)]
#[diesel(table_name = properties)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Property {
    pub id: i32,
    pub key: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub description: Option<String>,
    pub value_int: Option<i32>,
    pub value_string: Option<String>,
    pub value_bool: Option<i32>, // 0 or 1
    pub value_json: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Insertable)]
#[diesel(table_name = properties)]
pub struct NewProperty {
    pub key: String,
    pub type_: String,
    pub description: Option<String>,
    pub value_int: Option<i32>,
    pub value_string: Option<String>,
    pub value_bool: Option<i32>,
    pub value_json: Option<String>,
}

/// Typed enum for API usage
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "value")]
pub enum PropertyValue {
    #[serde(rename = "int")]
    Int(i32),
    #[serde(rename = "string")]
    String(String),
    #[serde(rename = "bool")]
    Bool(bool),
    #[serde(rename = "json")]
    Json(serde_json::Value),
}

/// Clean API response struct
#[derive(Serialize, Debug)]
pub struct TypedProperty {
    pub id: i32,
    pub key: String,
    pub description: Option<String>,
    #[serde(flatten)]
    pub value: PropertyValue,
}

// Helper to convert DB row to typed value
impl Property {
    pub fn value(&self) -> Option<PropertyValue> {
        match self.type_.as_str() {
            "int" => self.value_int.map(PropertyValue::Int),
            "string" => self.value_string.clone().map(PropertyValue::String),
            "bool" => self.value_bool.map(|v| PropertyValue::Bool(v != 0)),
            "json" => self
                .value_json
                .as_ref()
                .and_then(|s| serde_json::from_str(s).ok().map(PropertyValue::Json)),
            _ => None,
        }
    }

    /// Convert to API-friendly format
    pub fn to_typed(&self) -> Option<TypedProperty> {
        self.value().map(|v| TypedProperty {
            id: self.id,
            key: self.key.clone(),
            description: self.description.clone(),
            value: v,
        })
    }
}

impl PropertyValue {
    /// Create NewProperty from typed value
    pub fn to_new_property(self, key: String, description: Option<String>) -> NewProperty {
        match self {
            PropertyValue::Int(v) => NewProperty {
                key,
                type_: "int".to_string(),
                description,
                value_int: Some(v),
                value_string: None,
                value_bool: None,
                value_json: None,
            },
            PropertyValue::String(v) => NewProperty {
                key,
                type_: "string".to_string(),
                description,
                value_int: None,
                value_string: Some(v),
                value_bool: None,
                value_json: None,
            },
            PropertyValue::Bool(v) => NewProperty {
                key,
                type_: "bool".to_string(),
                description,
                value_int: None,
                value_string: None,
                value_bool: Some(if v { 1 } else { 0 }),
                value_json: None,
            },
            PropertyValue::Json(v) => NewProperty {
                key,
                type_: "json".to_string(),
                description,
                value_int: None,
                value_string: None,
                value_bool: None,
                value_json: Some(serde_json::to_string(&v).unwrap()),
            },
        }
    }
}
