use axum::http::{HeaderName, HeaderValue, Method};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;

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

// impl Default for CorsConfiguration {
//     fn default() -> Self {
//         Self {
//             mode: CorsMode::Permissive,
//             allowed_origins: vec!["http://localhost:3000".to_string()],
//             allow_credentials: true,
//             max_age_seconds: 3600,
//             allowed_headers: vec![
//                 "authorization".to_string(),
//                 "content-type".to_string(),
//                 "x-requested-with".to_string(),
//                 "accept".to_string(),
//                 "x-tenant-id".to_string(), // For MSP tenant identification
//             ],
//             allowed_methods: vec![
//                 "GET".to_string(),
//                 "POST".to_string(),
//                 "PUT".to_string(),
//                 "DELETE".to_string(),
//                 "OPTIONS".to_string(),
//             ],
//         }
//     }
// }

impl CorsConfiguration {
    pub fn load() -> Self {
        let mode = match std::env::var("CORS_MODE").as_deref() {
            Ok("permissive") => CorsMode::Permissive,
            Ok("restrictive") => CorsMode::Restrictive,
            Ok("multi_tenant") => CorsMode::MultiTenant {
                admin_domain: std::env::var("MSP_ADMIN_DOMAIN")
                    .unwrap_or_else(|_| "admin.localhost".to_string()),
                tenant_pattern: std::env::var("MSP_TENANT_PATTERN")
                    .unwrap_or_else(|_| "*.localhost".to_string()),
                allow_localhost: std::env::var("MSP_ALLOW_LOCALHOST")
                    .map(|v| v.parse().unwrap_or(true))
                    .unwrap_or(true),
            },
            Ok("single_frontend") => CorsMode::SingleFrontend {
                frontend_url: std::env::var("FRONTEND_URL")
                    .unwrap_or_else(|_| "http://localhost:3000".to_string()),
                allow_localhost: std::env::var("SINGLE_FRONTEND_ALLOW_LOCALHOST")
                    .map(|v| v.parse().unwrap_or(false))
                    .unwrap_or(false),
            },
            _ => CorsMode::Permissive, // Default for development
        };

        Self {
            mode,
            allowed_origins: Self::parse_origins_from_env(),
            allow_credentials: std::env::var("CORS_ALLOW_CREDENTIALS")
                .map(|v| v.parse().unwrap_or(true))
                .unwrap_or(true),
            max_age_seconds: std::env::var("CORS_MAX_AGE")
                .map(|v| v.parse().unwrap_or(3600))
                .unwrap_or(3600),
            allowed_headers: Self::parse_list_from_env("CORS_ALLOWED_HEADERS"),
            allowed_methods: Self::parse_list_from_env("CORS_ALLOWED_METHODS"),
        }
    }

    fn resolve_allowed_origins(&self) -> Vec<String> {
        match &self.mode {
            CorsMode::Permissive => vec!["*".to_string()],
            CorsMode::Restrictive => self.allowed_origins.clone(),
            CorsMode::MultiTenant {
                admin_domain,
                tenant_pattern: _,
                allow_localhost,
            } => {
                let mut origins = vec![format!("https://{}", admin_domain)];

                // Add tenant subdomains (you'd implement subdomain resolution logic)
                // This could query your database for active tenants
                origins.extend(self.get_active_tenant_domains());

                if *allow_localhost {
                    origins.extend([
                        "http://localhost:3000".to_string(),
                        "http://localhost:5173".to_string(), // Vite default
                        "http://127.0.0.1:3000".to_string(),
                    ]);
                }

                origins
            }
            CorsMode::SingleFrontend {
                frontend_url,
                allow_localhost,
            } => {
                let mut origins = vec![frontend_url.clone()];

                if *allow_localhost {
                    origins.extend([
                        "http://localhost:3000".to_string(),
                        "http://localhost:5173".to_string(),
                    ]);
                }

                origins
            }
        }
    }

    fn parse_origins_from_env() -> Vec<String> {
        std::env::var("CORS_ALLOWED_ORIGINS")
            .map(|origins| {
                origins
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect()
            })
            .unwrap_or_else(|_| vec!["http://localhost:3000".to_string()])
    }

    fn parse_list_from_env(env_var: &str) -> Vec<String> {
        std::env::var(env_var)
            .map(|list| {
                list.split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect()
            })
            .unwrap_or_else(|_| match env_var {
                "CORS_ALLOWED_HEADERS" => vec![
                    "authorization".to_string(),
                    "content-type".to_string(),
                    "x-requested-with".to_string(),
                    "accept".to_string(),
                    "x-tenant-id".to_string(),
                ],
                "CORS_ALLOWED_METHODS" => vec![
                    "GET".to_string(),
                    "POST".to_string(),
                    "PUT".to_string(),
                    "DELETE".to_string(),
                    "OPTIONS".to_string(),
                ],
                _ => vec![],
            })
    }

    fn get_active_tenant_domains(&self) -> Vec<String> {
        // Placeholder implementation - you would typically query your database here
        vec![]
    }

    /// Convert this configuration to a tower-http CorsLayer
    pub fn to_cors_layer(&self) -> CorsLayer {
        let origins = self.resolve_allowed_origins();

        let mut cors = CorsLayer::new();

        // Set origins
        if origins.contains(&"*".to_string()) {
            cors = cors.allow_origin(tower_http::cors::Any);
        } else {
            let header_values: Result<Vec<HeaderValue>, _> = origins
                .iter()
                .map(|origin| origin.parse::<HeaderValue>())
                .collect();

            match header_values {
                Ok(values) => cors = cors.allow_origin(values),
                Err(e) => {
                    eprintln!("Invalid origin in CORS config: {}", e);
                    cors = cors.allow_origin(tower_http::cors::Any); // Fallback
                }
            }
        }

        // Set methods
        let methods: Result<Vec<Method>, _> = self
            .allowed_methods
            .iter()
            .map(|m| m.parse::<Method>())
            .collect();

        if let Ok(methods) = methods {
            cors = cors.allow_methods(methods);
        } else {
            eprintln!("Invalid method in CORS config, using defaults");
            cors = cors.allow_methods([
                Method::GET,
                Method::POST,
                Method::PUT,
                Method::DELETE,
                Method::OPTIONS,
            ]);
        }

        // Set headers
        let headers: Result<Vec<HeaderName>, _> = self
            .allowed_headers
            .iter()
            .map(|h| h.parse::<HeaderName>())
            .collect();

        if let Ok(headers) = headers {
            cors = cors.allow_headers(headers);
        }

        // Set credentials
        if self.allow_credentials {
            cors = cors.allow_credentials(true);
        }

        // Set max age
        cors = cors.max_age(std::time::Duration::from_secs(self.max_age_seconds));

        cors
    }
}
