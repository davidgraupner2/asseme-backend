use std::sync::Arc;

// This is your application layer - orchestrates everything
use crate::actors::api::models::errors::AuthError;
use crate::actors::api::models::service::auth_models::{SigninRequest, SigninResponse};
use crate::actors::api::repositories::auth_repository::AuthRepository;
use crate::actors::api::services::email_service::EmailService;
use async_trait::async_trait;

#[async_trait]
pub trait AuthServiceTrait: Send + Sync {
    async fn signin(
        &self,
        request: crate::actors::api::models::service::auth_models::SigninRequest,
    ) -> Result<SigninResponse, AuthError>;
}

pub struct AuthService<R: AuthRepository + Send + Sync, E: EmailService + Send + Sync> {
    repository: Arc<R>,
    email_service: Arc<E>,
}

impl<R: AuthRepository + Send + Sync, E: EmailService + Send + Sync> AuthService<R, E> {
    pub fn new(repository: R, email_service: E) -> Self {
        Self {
            repository: Arc::new(repository),
            email_service: Arc::new(email_service),
        }
    }
}

#[async_trait]
impl<R, E> AuthServiceTrait for AuthService<R, E>
where
    R: AuthRepository + Send + Sync + 'static, // Add 'static bound
    E: EmailService + Send + Sync + 'static,   // Add 'static bound
{
    async fn signin(&self, request: SigninRequest) -> Result<SigninResponse, AuthError> {
        // 1. Validate email format
        if !self.email_service.is_valid_email(&request.email).await {
            return Err(AuthError::InvalidEmail);
        }

        // 2. Signin User
        let jwt = self
            .repository
            .signin_record_user(
                &request.email,
                &request.password,
                &request.namespace,
                &request.database,
                &request.access_method,
            )
            .await?;

        // 2. Return response
        Ok(SigninResponse { jwt: jwt })
    }
}
