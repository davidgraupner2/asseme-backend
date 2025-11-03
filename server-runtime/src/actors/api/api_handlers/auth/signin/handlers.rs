use crate::{
    actors::api::{
        api_handlers::auth::signin::types::{SigninRequest, SigninResponse},
        state::AxumApiState,
        types::{ApiResponse, ApiResult},
    },
    common::is_valid_email_format,
};
use axum::{extract::State, response::IntoResponse, Json};
use database::repository::user::repository::UserRepository;
use std::sync::Arc;
use surrealdb::opt::auth::Record;
use tracing::{info, warn};

pub async fn handle_signin(
    State(app): State<Arc<AxumApiState>>,
    Json(signin_request): Json<SigninRequest>,
) -> impl IntoResponse {
    info!(
        "Processing signin request for user: {}",
        signin_request.email
    );

    match process_signin(app, signin_request).await {
        Ok(success_response) => {
            let response: ApiResponse<SigninResponse> = success_response;
            response.into_response()
        }
        Err(error_response) => {
            let response: ApiResponse<()> = error_response;
            response.into_response()
        }
    }
}

async fn process_signin(
    app: Arc<AxumApiState>,
    request: SigninRequest,
) -> ApiResult<SigninResponse> {
    // Basic email validation
    if !is_valid_email_format(&request.email) {
        return Err(ApiResponse::validation_error(
            "Invalid email format".to_string(),
        ));
    }

    // Attempt authentication
    match authenticate_user(app, &request).await {
        Ok(signin_response) => Ok(ApiResponse::ok(signin_response)),
        Err(auth_error) => {
            warn!(
                "Authentication failed for {}: {}",
                request.email, auth_error
            );
            Err(ApiResponse::auth_failed())
        }
    }
}

async fn authenticate_user(
    app: Arc<AxumApiState>,
    request: &SigninRequest,
) -> Result<SigninResponse, Box<dyn std::error::Error + Send + Sync>> {
    // Attempt to sign in with database-level authentication
    let jwt = app
        .db_client
        .signin(Record {
            namespace: &app.db_config.namespace,
            database: &app.db_config.database,
            access: &app.db_config.access_method,
            params: SigninRequest {
                email: request.email.clone(),
                password: request.password.clone(),
            },
        })
        .await?;

    // Authenticate with the JWT
    app.db_client.authenticate(jwt.clone()).await?;

    info!("Signin succeeded for {}", request.email);

    // Get and update user
    let user_repository = UserRepository::new(app.db_client.clone());
    let user = user_repository.get_by_email(request.email.clone()).await?;

    // Update last login using our new method
    let updated_user = user_repository
        .update_last_login(user.id.clone().unwrap().id.to_string())
        .await?;

    Ok(SigninResponse {
        jwt: Some(jwt),
        user: updated_user,
    })
}
