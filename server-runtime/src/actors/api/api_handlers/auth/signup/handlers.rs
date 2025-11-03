use crate::actors::api::{
    api_handlers::auth::signup::types::{SignupRequest, SignupResponse},
    state::AxumApiState,
    types::{ApiResponse, ApiResult},
};
use axum::http::StatusCode;
use axum::{extract::State, response::IntoResponse, Json};
use std::sync::Arc;
use tracing::{error, info};

pub async fn handle_customer_signup(
    State(app): State<Arc<AxumApiState>>,
    Json(signup_request): Json<SignupRequest>,
) -> impl IntoResponse {
    info!(
        "Processing signup request for user: {}",
        signup_request.email
    );

    match process_signup(app, signup_request, false).await {
        Ok(success_response) => {
            let response: ApiResponse<SignupResponse> = success_response;
            response.into_response()
        }
        Err(error_response) => error_response.into_response(),
    }
}

pub async fn handle_msp_signup(
    State(app): State<Arc<AxumApiState>>,
    Json(signup_request): Json<SignupRequest>,
) -> impl IntoResponse {
    info!(
        "Processing MSP signup request for user: {}",
        signup_request.email
    );

    match process_signup(app, signup_request, true).await {
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
    is_msp: bool,
) -> ApiResult<SignupResponse> {
    // Validate request
    if let Err(validation_error) = request.validate() {
        return Err(ApiResponse::validation_error(validation_error));
    }

    let signup_response: SignupResponse = {
        if !is_msp {
            match signup_customer_tenant(app, &request).await {
                Ok(response) => response,
                Err(e) => {
                    error!("Failed to signup customer tenant: {}", e);
                    return Err(ApiResponse::internal_error(format!(
                        "Failed to create user account - {}",
                        e.to_string()
                    )));
                }
            }
        } else {
            match signup_msp_tenant(app, &request).await {
                Ok(response) => response,
                Err(e) => {
                    error!("Failed to signup msp tenant: {}", e);
                    return Err(ApiResponse::internal_error(format!(
                        "Failed to create msp user account - {}",
                        e.to_string()
                    )));
                }
            }
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

async fn signup_msp_tenant(
    app: Arc<AxumApiState>,
    request: &SignupRequest,
) -> Result<SignupResponse, Box<dyn std::error::Error + Send + Sync>> {
    let response = app
        .db_client
        .run("fn::signup_msp_tenant")
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
