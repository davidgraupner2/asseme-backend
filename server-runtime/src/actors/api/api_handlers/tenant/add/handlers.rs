use super::types::{AddNewCustomerTenantToMSPRequest, AddNewCustomerTenantToMSPRequestResponse};
use crate::actors::api::{
    state::AxumApiState,
    types::{ApiResponse, ApiResult},
};
use axum::{extract::State, response::IntoResponse, Json};
use std::sync::Arc;
use tracing::{error, info};

pub async fn handle_add_customer_tenant_to_msp(
    State(app): State<Arc<AxumApiState>>,
    Json(signup_request): Json<AddNewCustomerTenantToMSPRequest>,
) -> impl IntoResponse {
    info!(
        "Processing add new customer tenant to MSP: {}",
        signup_request.email
    );

    match process_add_new_custom_tenant_to_msp(app, signup_request).await {
        Ok(success_response) => {
            let response: ApiResponse<AddNewCustomerTenantToMSPRequestResponse> = success_response;
            response.into_response()
        }
        Err(error_response) => error_response.into_response(),
    }
}

pub async fn process_add_new_custom_tenant_to_msp(
    app: Arc<AxumApiState>,
    signup_request: AddNewCustomerTenantToMSPRequest,
) -> ApiResult<AddNewCustomerTenantToMSPRequestResponse> {
    info!(
        "Processing MSP signup request for user: {}",
        signup_request.email
    );

    match add_new_customer_tenant_to_msp(app, &signup_request).await {
        Ok(response) => Ok(ApiResponse::ok(response)),
        Err(error_response) => {
            error!(
                "Unable to add new customer tenant {} to msp - {}",
                signup_request.tenant_name,
                error_response.to_string()
            );
            return Err(ApiResponse::internal_error(format!(
                "Failed to add customer tenant to MSP - {}",
                error_response.to_string()
            )));
        }
    }
}

async fn add_new_customer_tenant_to_msp(
    app: Arc<AxumApiState>,
    request: &AddNewCustomerTenantToMSPRequest,
) -> Result<AddNewCustomerTenantToMSPRequestResponse, Box<dyn std::error::Error + Send + Sync>> {
    let response = app
        .db_client
        .run("fn::add_new_msp_customer_tenant")
        .args((
            &request.first_name,
            &request.last_name,
            &request.tenant_name,
            &request.email,
            &request.password,
            &request.settings,
            &request.msp_tenant,
            &request.is_msp_billing,
        ))
        .await?;

    Ok(response)
}
