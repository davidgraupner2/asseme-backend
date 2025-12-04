use std::fmt;
use thiserror::Error;

type Result<T> = std::result::Result<T, ApiServerError>;

#[derive(Debug, Clone, Error)]
pub enum ApiServerError {
    #[error("certificate error: {0}")]
    CertError(String),
    #[error("server error: {0}")]
    ServerError(String),
}
