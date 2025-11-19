use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CorsConfiguration {
    pub mode: CorsMode,
    pub allowed_origins: Vec<String>,
    pub allow_credentials: bool,
    pub max_age_seconds: u64,
    pub allowed_headers: Vec<String>,
    pub allowed_methods: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum CorsMode {
    /// Allow all origins - development/testing only
    Permissive,
    /// Allow specific origins only
    Restrictive,
    /// MSP mode - allow tenant subdomains + admin domain
    MultiTenant {
        admin_domain: String,
        tenant_pattern: String, // e.g., "*.yourmsp.com"
        allow_localhost: bool,
    },
    /// Single frontend mode
    SingleFrontend {
        frontend_url: String,
        allow_localhost: bool,
    },
}

impl Default for CorsConfiguration {
    fn default() -> Self {
        Self {
            mode: CorsMode::Permissive,
            allowed_origins: vec!["http://localhost:8000".to_string()],
            allow_credentials: true,
            max_age_seconds: 3600,
            allowed_headers: vec![
                "authorization".to_string(),
                "content-type".to_string(),
                "accept".to_string(),
                "x-tenant-id".to_string(), // For MSP tenant identification
            ],
            allowed_methods: vec![
                "GET".to_string(),
                "POST".to_string(),
                "PUT".to_string(),
                "DELETE".to_string(),
                "OPTIONS".to_string(),
            ],
        }
    }
}

pub trait LoadCorsConfiguration {
    fn load_config(&self) -> CorsConfiguration;
}
