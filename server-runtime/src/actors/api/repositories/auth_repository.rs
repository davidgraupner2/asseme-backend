use crate::actors::api::models::errors::AuthError;
use std::sync::Arc;

use async_trait::async_trait;
use serde::Serialize;
use serde_json::to_string;
use surrealdb::{
    engine::remote::ws::Client,
    opt::auth::{Jwt, Record},
    Surreal,
};
use tracing::{info, instrument, warn};

#[derive(Serialize, Debug)]
struct Credentials<'a> {
    email: &'a str,
    password: &'a str,
}

// Interface
#[async_trait]
pub trait AuthRepository {
    async fn signin_record_user(
        &self,
        email: &str,
        password: &str,
        namespace: &str,
        database: &str,
        access_method: &str,
    ) -> Result<Jwt, AuthError>;
}

// Implementation of the interface
pub struct SurrealAuthRepository {
    db: Arc<Surreal<Client>>,
}

impl SurrealAuthRepository {
    pub fn new(db: Surreal<Client>) -> Self {
        Self { db: db.into() }
    }
}

#[async_trait]
impl AuthRepository for SurrealAuthRepository {
    #[instrument(skip(self, password), fields(email = %email, namespace = %namespace))]
    async fn signin_record_user(
        &self,
        email: &str,
        password: &str,
        namespace: &str,
        database: &str,
        access_method: &str,
    ) -> Result<Jwt, AuthError> {
        let result = self
            .db
            .signin(Record {
                namespace: namespace,
                database: database,
                access: access_method,
                params: Credentials {
                    email: email,
                    password: password,
                },
            })
            .await;

        match result {
            Ok(jwt) => {
                info!("Signin Successful");
                Ok(jwt)
            }
            Err(error) => {
                warn!("Signin Failed = {}", error.to_string());
                Err(AuthError::InvalidCredentials)
            }
        }
    }
}
