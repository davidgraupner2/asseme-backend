use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use surrealdb::sql::Thing;

/// Token model for API access and agent authentication
///
/// This table manages tokens for various purposes including agent authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    /// The unique identifier for the token
    pub id: Option<Thing>,

    /// The tenant this token belongs to
    pub tenant: Thing,

    /// Type of token (currently supports 'agent')
    #[serde(rename = "type")]
    pub token_type: HashSet<TokenType>,

    /// The JWT token string
    pub jwt: String,

    /// The user who owns this token
    pub owner: Thing,

    /// When the token was created
    pub created_at: Option<DateTime<Utc>>,

    /// When the token was last updated
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TokenType {
    #[serde(rename = "agent")]
    Agent,
}

impl Token {
    /// Create a new token
    pub fn new(tenant: Thing, token_type: TokenType, jwt: String, owner: Thing) -> Self {
        let mut types = HashSet::new();
        types.insert(token_type);

        Self {
            id: None,
            tenant,
            token_type: types,
            jwt,
            owner,
            created_at: None, // Will be set by SurrealDB
            updated_at: None, // Will be set by SurrealDB
        }
    }

    /// Create a new agent token
    pub fn new_agent_token(tenant: Thing, jwt: String, owner: Thing) -> Self {
        Self::new(tenant, TokenType::Agent, jwt, owner)
    }

    /// Check if this is an agent token
    pub fn is_agent_token(&self) -> bool {
        self.token_type.contains(&TokenType::Agent)
    }

    /// Add a token type
    pub fn add_token_type(&mut self, token_type: TokenType) {
        self.token_type.insert(token_type);
    }

    /// Remove a token type
    pub fn remove_token_type(&mut self, token_type: &TokenType) {
        self.token_type.remove(token_type);
    }

    /// Check if token has specific type
    pub fn has_token_type(&self, token_type: &TokenType) -> bool {
        self.token_type.contains(token_type)
    }

    /// Get all token types as a vector
    pub fn get_token_types(&self) -> Vec<TokenType> {
        self.token_type.iter().cloned().collect()
    }

    /// Update the JWT token
    pub fn update_jwt(&mut self, new_jwt: String) {
        self.jwt = new_jwt;
    }

    /// Get token types as display string
    pub fn token_types_display(&self) -> String {
        self.token_type
            .iter()
            .map(|t| match t {
                TokenType::Agent => "Agent",
            })
            .collect::<Vec<_>>()
            .join(", ")
    }

    /// Check if token is valid (has at least one type and JWT)
    pub fn is_valid(&self) -> bool {
        !self.token_type.is_empty() && !self.jwt.is_empty()
    }
}
