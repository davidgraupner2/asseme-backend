use config_server::{
    ApiConfiguration, CorsConfiguration, CorsMode, LoadApiConfiguration, LoadCorsConfiguration,
    LoadLoggingConfiguration, LoadRateLimitingConfiguration, LoggingConfiguration,
    RateLimitingConfiguration,
};
use std::env;

pub struct EnvServerConfigLoader;

impl EnvServerConfigLoader {
    pub fn new() -> Self {
        load_env();
        Self {}
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
            .unwrap_or_else(|_| vec!["http://localhost:8000".to_string()])
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
}

impl LoadApiConfiguration for EnvServerConfigLoader {
    fn load_config(&self) -> ApiConfiguration {
        let mut api_configuration = ApiConfiguration::default();

        // Load the config from environment variables defaulting if env vars not found
        api_configuration.port = env::var("API_PORT")
            .unwrap_or(api_configuration.port.to_string())
            .parse()
            .unwrap_or(api_configuration.port);
        api_configuration.behind_proxy = env::var("API_BEHIND_PROXY")
            .unwrap_or(api_configuration.behind_proxy.to_string())
            .parse()
            .unwrap_or(api_configuration.behind_proxy);

        api_configuration.request_timeout_secs = env::var("API_REQUEST_TIMEOUT_SECS")
            .unwrap_or(api_configuration.request_timeout_secs.to_string())
            .parse()
            .unwrap_or(api_configuration.request_timeout_secs);

        api_configuration.agent_ping_interval = env::var("API_AGENT_PING_INTERVAL")
            .unwrap_or(api_configuration.agent_ping_interval.to_string())
            .parse()
            .unwrap_or(api_configuration.agent_ping_interval);

        api_configuration.agent_ping_timeout = env::var("API_AGENT_PING_TIMEOUT")
            .unwrap_or(api_configuration.agent_ping_timeout.to_string())
            .parse()
            .unwrap_or(api_configuration.agent_ping_timeout);

        api_configuration.agent_jwt_secret = env::var("API_AGENT_JWT_SECRET")
            .unwrap_or(api_configuration.agent_jwt_secret.to_string())
            .parse()
            .unwrap_or(api_configuration.agent_jwt_secret);

        api_configuration.server_jwt_secret = env::var("API_SERVER_JWT_SECRET")
            .unwrap_or(api_configuration.server_jwt_secret.to_string())
            .parse()
            .unwrap_or(api_configuration.server_jwt_secret);

        api_configuration
    }
}

impl LoadCorsConfiguration for EnvServerConfigLoader {
    fn load_config(&self) -> CorsConfiguration {
        let mut cors_configuration = CorsConfiguration::default();

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
                    .unwrap_or_else(|_| "http://localhost:8000".to_string()),
                allow_localhost: std::env::var("SINGLE_FRONTEND_ALLOW_LOCALHOST")
                    .map(|v| v.parse().unwrap_or(false))
                    .unwrap_or(false),
            },
            _ => CorsMode::Permissive, // Default for development
        };

        cors_configuration.mode = mode;
        cors_configuration.allowed_origins = Self::parse_origins_from_env();
        cors_configuration.allow_credentials = env::var("CORS_ALLOW_CREDENTIALS")
            .map(|v| v.parse().unwrap_or(true))
            .unwrap_or(true);
        cors_configuration.max_age_seconds = env::var("CORS_MAX_AGE")
            .map(|v| v.parse().unwrap_or(3600))
            .unwrap_or(3600);
        cors_configuration.allowed_headers = Self::parse_list_from_env("CORS_ALLOWED_HEADERS");
        cors_configuration.allowed_methods = Self::parse_list_from_env("CORS_ALLOWED_METHODS");

        cors_configuration
    }
}

impl LoadLoggingConfiguration for EnvServerConfigLoader {
    fn load_config(&self) -> LoggingConfiguration {
        let mut logging_configuration = LoggingConfiguration::default();

        logging_configuration.log_format = env::var("LOG_FORMAT").unwrap_or("pretty".to_string());
        logging_configuration.log_output = env::var("LOG_OUTPUT").unwrap_or("console".to_string());

        logging_configuration
    }
}

impl LoadRateLimitingConfiguration for EnvServerConfigLoader {
    fn load_config(&self) -> RateLimitingConfiguration {
        let mut rate_limiting = RateLimitingConfiguration::default();

        rate_limiting.burst_size = env::var("RATE_LIMITING_BURST_SIZE")
            .unwrap_or("200".to_owned())
            .parse()
            .unwrap_or(200);
        rate_limiting.per_second = env::var("RATE_LIMITING_PER_SECOND")
            .unwrap_or("5".to_owned())
            .parse()
            .unwrap_or(5);
        rate_limiting.cleanup_duration = env::var("RATE_LIMITING_CLEANUP_DURATION")
            .unwrap_or("60".to_owned())
            .parse()
            .unwrap_or(60);

        rate_limiting
    }
}

fn load_env() {
    // Load any environment variables from a .env file
    match dotenvy::dotenv() {
        Ok(_) => {}
        Err(_) => {
            println!("EnvServerConfigLoader WARNING: .env file not found, assuming defaults or configured another way!")
        }
    }
}
