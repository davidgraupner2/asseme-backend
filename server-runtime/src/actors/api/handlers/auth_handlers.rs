use crate::actors::api::{
    models::{
        api::{auth_requests::ApiSigninRequest, responses::ApiResponse},
        errors::AuthError,
        service::auth_models::{SigninRequest, SigninResponse},
    },
    state::AxumApiState,
};
use axum::{extract::State, http::StatusCode, Json};
use std::sync::Arc;

pub async fn signin_handler(
    State(state): State<Arc<AxumApiState>>,
    Json(request): Json<ApiSigninRequest>,
) -> Result<Json<ApiResponse<SigninResponse>>, Json<ApiResponse<()>>> {
    // Populate a new signin request with the database details
    let signin_request = SigninRequest {
        email: request.email,
        password: request.password,
        namespace: state.db_config.namespace.clone(),
        database: state.db_config.database.clone(),
        access_method: state.db_config.access_method.clone(),
    };

    match state.auth_service.signin(signin_request).await {
        Ok(response) => Ok(Json(ApiResponse::success(response, StatusCode::OK))),
        Err(auth_error) => {
            // Map domain errors to ApiResponse errors
            let api_response = match auth_error {
                AuthError::InvalidEmail => {
                    ApiResponse::bad_request("Invalid email format".to_string())
                }
                AuthError::InvalidCredentials => {
                    ApiResponse::unauthorized("Invalid credentials".to_string())
                }
                _ => ApiResponse::server_error(),
            };
            Err(Json(api_response))
        }
    }
}
