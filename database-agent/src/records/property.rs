use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::{
    common::{validate_value, ValueType},
    database::Database,
    traits::TableRecord,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PropertyRecord {
    pub id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
    pub value_type: ValueType,
    pub value: serde_json::Value,
    pub created_at: String,
    pub updated_at: String,
}

impl PropertyRecord {}

// impl TableRecord for Database {
//     fn create(&self) -> Result<Option<&Self>> {
//         // Check the value and value type of the record
//         validate_value(self.value_type.clone(), self.value.clone())?;

//         let connection = self.pool
//     }
// }
