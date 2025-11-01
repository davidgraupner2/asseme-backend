use std::sync::Arc;

use anyhow::Result;
use axum::{
    extract::State, http::StatusCode, response::Json as ResponseJson, routing::post, Json, Router,
};
use chrono::{DateTime, Utc};
use database::model::user::User;
use database::repository::user::repository::UserRepository;
use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::remote::ws::Client,
    opt::auth::{Jwt, Record},
    sql::Thing,
    Surreal,
};
use tracing::{error, info, warn};

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
                let mut user = user_repository
                    .get_by_email(signin_request.email.clone())
                    .await
                    .unwrap();

                user.set_last_login();

                user = user_repository
                    .update_user(user.id.clone().unwrap().id.to_string(), user.clone())
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

async fn get_user_id(db: Surreal<Client>, email: String) -> Option<String> {
    // Query to get the user ID as a Thing object
    match db
        .query(format!(
            "select VALUE id from only user where email = '{}'",
            email
        ))
        .await
    {
        Ok(mut result) => {
            println!("Result: {:#?}", result);

            // Try to get the Thing directly
            match result.take::<Option<Thing>>(0) {
                Ok(Some(thing)) => {
                    // Convert Thing to string format: "table:id"
                    let user_id = format!("{}:{}", thing.tb, thing.id);
                    Some(user_id)
                }
                Ok(None) => {
                    println!("No user found with email: {}", email);
                    None
                }
                Err(e) => {
                    error!("Failed to parse Thing from result: {}", e);
                    None
                }
            }
        }
        Err(e) => {
            error!("Failed to query user: {}", e);
            None
        }
    }
}
