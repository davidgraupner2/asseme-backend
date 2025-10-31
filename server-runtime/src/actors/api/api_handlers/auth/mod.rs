use std::sync::Arc;

use axum::{
    extract::State, http::StatusCode, response::Json as ResponseJson, routing::post, Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::{Database, Jwt, Record},
    Surreal,
};
use tracing::{error, info, warn};

use crate::actors::api::state::AxumApiState;

#[derive(Debug, Serialize, Deserialize)]
pub struct SigninRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct SigninResponse {
    pub success: bool,
    pub message: String,
    pub jwt: Option<Jwt>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct UserLoginRecord {
    pub user_id: String,
    pub username: String,
    pub login_time: DateTime<Utc>,
    pub access_method: String,
    pub namespace: String,
    pub database: String,
    pub success: bool,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
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
            Ok(_) => Ok(Json(SigninResponse {
                success: true,
                message: "Signed in successfully".to_string(),
                jwt: Some(jwt),
                timestamp,
            })),
            Err(auth_error) => {
                let error_response = ErrorResponse {
                    success: false,
                    error: format!("Authentication failed - {:#?}", auth_error),
                    timestamp,
                };
                Err((StatusCode::UNAUTHORIZED, ResponseJson(error_response)))
            }
        },
        Err(error) => {
            let error_response = ErrorResponse {
                success: false,
                error: format!("Signin failed - {:#?}", error),
                timestamp,
            };
            Err((StatusCode::UNAUTHORIZED, ResponseJson(error_response)))
        }
    }

    // Create user login record
    // let login_record = UserLoginRecord {
    //     user_id: if login_successful {
    //         format!("{}::{}", signin_request.namespace, signin_request.email)
    //     } else {
    //         "unknown".to_string()
    //     },
    //     username: signin_request.email.clone(),
    //     login_time: timestamp,
    //     access_method: signin_request.access_method.clone(),
    //     namespace: signin_request.namespace.clone(),
    //     database: signin_request.database.clone(),
    //     success: login_successful,
    //     ip_address: None, // TODO: Extract from request headers
    //     user_agent: None, // TODO: Extract from request headers
    // };

    // Record the login attempt
    // if let Err(e) = record_user_login(&db, login_record.clone()).await {
    //     warn!("Failed to record login attempt: {}", e);
    //     // Don't fail the request if logging fails, just warn
    // }

    // match jwt {
    //     Ok(_jwt) => {
    //         info!("User {} successfully signed in", signin_request.email);
    //         // For now, we'll generate a simple session token
    //         // In production, you'd want to use the actual JWT or create your own session token
    //         let session_token = format!(
    //             "session_{}_{}_{}",
    //             signin_request.email,
    //             signin_request.namespace,
    //             timestamp.timestamp()
    //         );

    //         let response = SigninResponse {
    //             success: true,
    //             message: "Successfully signed in".to_string(),
    //             session_token: Some(session_token),
    //             user_id: Some(login_record.user_id),
    //             timestamp,
    //         };
    //         Ok(ResponseJson(response))
    //     }
    //     Err(e) => {
    //         warn!("Signin failed for user {}: {}", signin_request.email, e);
    //         let error_response = ErrorResponse {
    //             success: false,
    //             error: format!("Authentication failed: {}", e),
    //             timestamp,
    //         };
    //         Err((StatusCode::UNAUTHORIZED, ResponseJson(error_response)))
    //     }
    // }
}

async fn record_user_login(
    db: &Surreal<Client>,
    login_record: UserLoginRecord,
) -> Result<(), surrealdb::Error> {
    // Create the login record using the create method
    let _result: Option<UserLoginRecord> = db
        .create("user_logins")
        .content(login_record.clone())
        .await?;

    // Update user statistics
    let username = login_record.username.clone();
    let login_time = login_record.login_time;
    let success = login_record.success;

    let _stats_result = db
        .query(
            "
            LET $existing = (SELECT * FROM user_stats WHERE username = $username LIMIT 1)[0];
            
            IF $existing {
                UPDATE user_stats SET 
                    total_logins = total_logins + 1,
                    last_login = $login_time,
                    last_success = IF($success, $login_time, last_success)
                WHERE username = $username;
            } ELSE {
                CREATE user_stats CONTENT {
                    username: $username,
                    total_logins: 1,
                    first_login: $login_time,
                    last_login: $login_time,
                    last_success: IF($success, $login_time, NONE)
                };
            }
            ",
        )
        .bind(("username", username))
        .bind(("login_time", login_time))
        .bind(("success", success))
        .await?;

    Ok(())
}
