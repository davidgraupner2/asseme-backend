use jsonwebtoken::{
    decode, encode, errors::Error, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use runtime_shared::RuntimeProperties;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

pub enum JwtType {
    Agent,
}

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
struct AgentClaims {
    sub: String,
    iat: usize,
    aud: String,
    exp: usize,
    iss: String,
    nbf: usize,
}

fn get_claims(tenant: &str, jwt_type: JwtType) -> AgentClaims {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;

    match jwt_type {
        JwtType::Agent => {
            AgentClaims {
                sub: "Agent".to_string(),
                iat: now,
                aud: tenant.to_string(),
                exp: usize::MAX, // "never" expires
                iss: RuntimeProperties::global().app_name().to_string(),
                nbf: now,
            }
        }
    }
}

pub fn generate_jwt(tenant: &str, secret: &str, jwt_type: JwtType) -> Result<String, Error> {
    match jwt_type {
        JwtType::Agent => {
            let claims = get_claims(tenant, jwt_type);
            let header = Header::new(Algorithm::HS512);
            let encoding_key = EncodingKey::from_secret(secret.as_bytes());

            match encode(&header, &claims, &encoding_key) {
                Ok(token) => Ok(token),
                Err(error) => Err(error),
            }
        }
    }
}
