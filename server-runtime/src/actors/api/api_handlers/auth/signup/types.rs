// signup/types.rs
use database::model::{tenant::Tenant, user::User};
use serde::{Deserialize, Serialize};
use surrealdb::Object;

use crate::common::is_valid_email_format;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct SignupRequestSettings {
    pub plan: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct SignupRequest {
    pub first_name: String,
    pub last_name: String,
    pub tenant_name: String,
    pub email: String,
    pub password: String,
    pub settings: SignupRequestSettings,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignupResponse {
    pub user: User,
    pub tenant: Tenant,
}

// #[derive(Debug, Serialize)]
// pub struct UserInfo {
//     pub id: String,
//     pub email: String,
//     pub name: String,
// }

// #[derive(Debug, Serialize)]
// pub struct TenantInfo {
//     pub id: String,
//     pub name: String,
//     pub slug: String,
//     pub tenant_type: String,
// }

impl SignupRequest {
    pub fn validate(&self) -> Result<(), String> {
        if !is_valid_email_format(&self.email) {
            return Err("Invalid email format".to_string());
        }

        if self.password.len() < 8 {
            return Err("Password must be at least 8 characters long".to_string());
        }

        if self.first_name.trim().is_empty() {
            return Err("First name is required".to_string());
        }

        if self.last_name.trim().is_empty() {
            return Err("Last name is required".to_string());
        }

        if self.tenant_name.trim().is_empty() {
            return Err("Tenant name is required".to_string());
        }

        Ok(())
    }
}
