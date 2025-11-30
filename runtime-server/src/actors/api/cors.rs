use axum::http::{HeaderName, HeaderValue, Method};
use config_server::{CorsConfiguration, CorsMode};
use tower_http::cors::CorsLayer;
use tracing::warn;

/// Convert this configuration to a tower-http CorsLayer
pub fn to_cors_layer(config: CorsConfiguration, api_port: u16) -> CorsLayer {
    let origins = resolve_allowed_origins(&config, api_port);

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
                warn!("Invalid origin in CORS config: {}", e);
                cors = cors.allow_origin(tower_http::cors::Any); // Fallback
            }
        }
    }

    // Set methods
    let methods: Result<Vec<Method>, _> = config
        .allowed_methods
        .iter()
        .map(|m| m.parse::<Method>())
        .collect();

    if let Ok(methods) = methods {
        cors = cors.allow_methods(methods);
    } else {
        warn!("Invalid method in CORS config, using defaults");
        cors = cors.allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ]);
    }

    // Set headers
    let headers: Result<Vec<HeaderName>, _> = config
        .allowed_headers
        .iter()
        .map(|h| h.parse::<HeaderName>())
        .collect();

    if let Ok(headers) = headers {
        cors = cors.allow_headers(headers);
    }

    // Set credentials
    if config.allow_credentials {
        cors = cors.allow_credentials(true);
    }

    // Set max age
    cors = cors.max_age(std::time::Duration::from_secs(config.max_age_seconds));

    cors
}

fn resolve_allowed_origins(config: &CorsConfiguration, api_port: u16) -> Vec<String> {
    match &config.mode {
        CorsMode::Permissive => vec!["*".to_string()],
        CorsMode::Restrictive => config.allowed_origins.clone(),
        CorsMode::MultiTenant {
            admin_domain,
            tenant_pattern: _,
            allow_localhost,
        } => {
            let mut origins = vec![format!("https://{}", admin_domain)];

            if *allow_localhost {
                origins.extend([format!("http://localhost:{}", api_port)]);
            }

            // Add tenant subdomains (#TODO)
            // This would query our database for active tenants
            origins.extend(get_active_tenant_domains());

            origins
        }
        CorsMode::SingleFrontend {
            frontend_url,
            allow_localhost,
        } => {
            let mut origins = vec![frontend_url.clone()];

            if *allow_localhost {
                origins.extend([format!("http://localhost:{}", api_port)]);
            }

            origins
        }
    }
}

fn get_active_tenant_domains() -> Vec<String> {
    // Placeholder implementation - you would typically query your database here
    vec![]
}
