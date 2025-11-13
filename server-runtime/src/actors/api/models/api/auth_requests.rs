use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiSigninRequest {
    pub email: String,
    pub password: String,
}
