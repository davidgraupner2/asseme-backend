// signup/handlers.rs
use crate::actors::api::{
    api_handlers::auth::signup::types::{SignupRequest, SignupResponse},
    state::AxumApiState,
    types::{ApiResponse, ApiResult},
};
use axum::{extract::State, response::IntoResponse, Json};
use std::sync::Arc;
use tracing::{error, info};

pub async fn handle_signup(
    State(app): State<Arc<AxumApiState>>,
    Json(signup_request): Json<SignupRequest>,
) -> impl IntoResponse {
    info!(
        "Processing signup request for user: {}",
        signup_request.email
    );

    match process_signup(app, signup_request).await {
        Ok(success_response) => {
            let response: ApiResponse<SignupResponse> = success_response;
            response.into_response()
        }
        Err(error_response) => error_response.into_response(),
    }
}

async fn process_signup(
    app: Arc<AxumApiState>,
    request: SignupRequest,
) -> ApiResult<SignupResponse> {
    // Validate request
    if let Err(validation_error) = request.validate() {
        return Err(ApiResponse::validation_error(validation_error));
    }

    let signup_response = match signup_customer_tenant(app, &request).await {
        Ok(response) => response,
        Err(e) => {
            error!("Failed to signup customer tenant: {}", e);
            return Err(ApiResponse::internal_error(format!(
                "Failed to create user account - {}",
                e.to_string()
            )));
        }
    };

    info!("Signup completed successfully for user: {}", request.email);
    Ok(ApiResponse::ok(signup_response))
}

async fn signup_customer_tenant(
    app: Arc<AxumApiState>,
    request: &SignupRequest,
) -> Result<SignupResponse, Box<dyn std::error::Error + Send + Sync>> {
    let response = app
        .db_client
        .run("fn::signup_customer_tenant")
        .args((
            &request.first_name,
            &request.last_name,
            &request.tenant_name,
            &request.email,
            &request.password,
            &request.settings,
        ))
        .await?;

    Ok(response)
}
