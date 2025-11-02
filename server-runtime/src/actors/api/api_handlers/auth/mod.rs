use std::sync::Arc;

use anyhow::Result;
use axum::{
    extract::State, http::StatusCode, response::Json as ResponseJson, routing::post, Json, Router,
};
use chrono::{DateTime, Utc};
use database::model::user::User;
use database::repository::user::repository::UserRepository;
use serde::{Deserialize, Serialize};
use surrealdb::opt::auth::{Jwt, Record};
use tracing::{info, warn};

use crate::actors::api::state::AxumApiState;

#[derive(Debug, Serialize, Deserialize)]
pub struct RecordAuthentication {
    pub tb: String,
    pub id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SigninRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct Tenant {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct SigninResponse {
    pub success: bool,
    pub message: String,
    pub jwt: Option<Jwt>,
    pub timestamp: DateTime<Utc>,
    pub user: User,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub timestamp: DateTime<Utc>,
}

pub fn auth_router() -> Router<Arc<AxumApiState>> {
    Router::new().route("/signin", post(api_post_signin))
}

pub async fn api_post_signin(
    State(app): State<Arc<AxumApiState>>,
    Json(signin_request): Json<SigninRequest>,
) -> Result<ResponseJson<SigninResponse>, (StatusCode, ResponseJson<ErrorResponse>)> {
    info!(
        "Processing signin request for user: {}",
        signin_request.email
    );

    let timestamp = Utc::now();

    // Attempt to sign in with database-level authentication
    let jwt = app
        .db_client
        .signin(Record {
            namespace: &app.db_config.namespace,
            database: &app.db_config.database,
            access: &app.db_config.access_method,
            params: SigninRequest {
                email: signin_request.email.clone(),
                password: signin_request.password.clone(),
            },
        })
        .await;

    match jwt {
        Ok(jwt) => match app.db_client.authenticate(jwt.clone()).await {
            Ok(_) => {
                info!("Signin succeeded for {}", signin_request.email);

                let user_repository = UserRepository::new(app.db_client.clone());
                let user = user_repository
                    .get_by_email(signin_request.email.clone())
                    .await
                    .unwrap();

                // Update last login directly in the database
                let user = user_repository
                    .update_last_login(user.id.clone().unwrap().id.to_string())
                    .await
                    .unwrap();

                // Clear the password hash before returning
                // user.password_hash = "<Redacted>".to_string();

                Ok(Json(SigninResponse {
                    success: true,
                    message: "Signed in successfully".to_string(),
                    jwt: Some(jwt),
                    timestamp,
                    user,
                }))
            }
            Err(auth_error) => {
                warn!(
                    "Authentication failed for {} - {}",
                    signin_request.email, auth_error
                );

                let error_response = ErrorResponse {
                    success: false,
                    error: format!("Authentication failed - Access denied"),
                    timestamp,
                };
                Err((StatusCode::UNAUTHORIZED, ResponseJson(error_response)))
            }
        },
        Err(signin_error) => {
            warn!(
                "Signin failed for {} - {}",
                signin_request.email, signin_error
            );

            let error_response = ErrorResponse {
                success: false,
                error: format!("Signin failed - Invalid Username and/or password!"),
                timestamp,
            };
            Err((StatusCode::UNAUTHORIZED, ResponseJson(error_response)))
        }
    }
}
