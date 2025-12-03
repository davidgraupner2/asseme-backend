use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

const VALUETYPEMISMATCH: &str = "Value type mismatch";

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub enum ValueType {
    String,
    Integer,
    Boolean,
    Json,
}

pub fn validate_value(value_type: ValueType, value: serde_json::Value) -> Result<()> {
    match value_type {
        ValueType::String => {
            if !value.is_string() {
                return Err(anyhow!(
                    "{}: expected String, got {:?}",
                    VALUETYPEMISMATCH,
                    value
                ));
            }
        }
        ValueType::Integer => {
            if !value.is_i64() {
                return Err(anyhow!(
                    "{}: expected Integer, got {:?}",
                    VALUETYPEMISMATCH,
                    value
                ));
            }
        }
        ValueType::Boolean => {
            if !value.is_boolean() {
                return Err(anyhow!(
                    "{}: expected Boolean, got {:?}",
                    VALUETYPEMISMATCH,
                    value
                ));
            }
        }
        ValueType::Json => {
            if !value.is_object() && !value.is_array() {
                return Err(anyhow!(
                    "{}: expected Json (object or array), got {:?}",
                    VALUETYPEMISMATCH,
                    value
                ));
            }
        }
    }
    Ok(())
}
