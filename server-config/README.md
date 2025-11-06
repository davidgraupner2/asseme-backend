# Server Configuration

This module provides configuration management for the Asseme MSP backend server. It handles API, database, logging, and CORS configuration through environment variables and configuration files.

## Table of Contents

- [Overview](#overview)
- [Configuration Modules](#configuration-modules)
- [Environment Variables](#environment-variables)
- [CORS Configuration](#cors-configuration)
- [Deployment Scenarios](#deployment-scenarios)

## Overview

The server-config module provides a centralized way to manage all server configuration through environment variables, making it easy to deploy the same codebase across different environments.

## Configuration Modules

### API Configuration (`api.rs`)

Handles HTTP server settings, CORS, and general API behavior.

```rust
pub struct ApiConfiguration {
    pub host: String,
    pub port: u16,
    pub behind_proxy: bool,
    pub cors: CorsConfiguration,
}
```

### Database Configuration (`database.rs`)

Manages SurrealDB connection settings and authentication.

```rust
pub struct DatabaseConfiguration {
    pub url: String,
    pub namespace: String,
    pub database: String,
    pub username: String,
    pub password: String,
    pub access_method: String,
}
```

### Logging Configuration (`logging.rs`)

Controls log levels

```rust
pub struct LoggingConfiguration {
    pub level: String,
}
```

## Environment Variables

### Core Server Settings

```bash
# API Server
API_HOST=0.0.0.0                    # Server bind address
API_PORT=8080                       # Server port
API_BEHIND_PROXY=false              # Whether behind reverse proxy

# Database
DB_URL=ws://localhost:8000          # SurrealDB connection URL
DB_NAMESPACE=production             # Database namespace
DB_DATABASE=asseme                  # Database name
DB_USERNAME=root                    # Database username
DB_PASSWORD=root                    # Database password
DB_ACCESS_METHOD=user               # Access method for authentication

# Logging
LOG_LEVEL=info                      # Log level (trace, debug, info, warn, error)
```

## CORS Configuration

The CORS system supports multiple deployment modes to handle different use cases.

### CORS Modes

#### 1. Permissive Mode (Development)

Allows all origins - **use only for development**.

```bash
CORS_MODE=permissive
```

#### 2. Restrictive Mode

Allows only explicitly specified origins.

```bash
CORS_MODE=restrictive
CORS_ALLOWED_ORIGINS=https://app1.com,https://app2.com,http://localhost:3000
```

#### 3. Multi-Tenant MSP Mode

Designed for MSP platforms with multiple tenant subdomains.

```bash
CORS_MODE=multi_tenant
MSP_ADMIN_DOMAIN=admin.yourmsp.com
MSP_TENANT_PATTERN=*.yourmsp.com
MSP_ALLOW_LOCALHOST=true
```

#### 4. Single Frontend Mode

For dedicated client deployments with one frontend.

```bash
CORS_MODE=single_frontend
FRONTEND_URL=https://app.yourclient.com
SINGLE_FRONTEND_ALLOW_LOCALHOST=false
```

### Additional CORS Settings

```bash
# General CORS Configuration
CORS_ALLOW_CREDENTIALS=true                                    # Allow cookies/auth headers
CORS_MAX_AGE=3600                                             # Preflight cache time (seconds)
CORS_ALLOWED_HEADERS=authorization,content-type,x-tenant-id   # Allowed request headers
CORS_ALLOWED_METHODS=GET,POST,PUT,DELETE,OPTIONS              # Allowed HTTP methods

# Dynamic Configuration (optional)
CORS_CONFIG_RELOAD_INTERVAL=300                               # Config reload interval (seconds)
CORS_ENABLE_DYNAMIC_RELOAD=true                               # Enable runtime config reload
```

## Deployment Scenarios

### Development Environment

Perfect for local development with hot reloading frontends.

```bash
# .env.development
CORS_MODE=permissive
API_HOST=127.0.0.1
API_PORT=8080
API_BEHIND_PROXY=false
MSP_ALLOW_LOCALHOST=true
LOG_LEVEL=debug
```

### MSP Production Environment

For multi-tenant MSP platforms serving multiple clients.

```bash
# .env.production
CORS_MODE=multi_tenant
MSP_ADMIN_DOMAIN=admin.yourmsp.com
MSP_TENANT_PATTERN=*.yourmsp.com
MSP_ALLOW_LOCALHOST=false

API_HOST=0.0.0.0
API_PORT=8080
API_BEHIND_PROXY=true

CORS_ALLOW_CREDENTIALS=true
CORS_MAX_AGE=86400

DB_URL=wss://db.yourmsp.com
DB_NAMESPACE=production
DB_DATABASE=msp_platform

LOG_LEVEL=info
```

### Single Client Deployment

For dedicated installations serving one client's frontend.

```bash
# .env.client-specific
CORS_MODE=single_frontend
FRONTEND_URL=https://portal.specificclient.com
SINGLE_FRONTEND_ALLOW_LOCALHOST=false

API_HOST=0.0.0.0
API_PORT=8080
API_BEHIND_PROXY=true

CORS_ALLOW_CREDENTIALS=true
CORS_MAX_AGE=86400

DB_URL=wss://db-client1.yourmsp.com
DB_NAMESPACE=client1
DB_DATABASE=client_portal

LOG_LEVEL=warn
```

### Staging Environment

For testing with production-like settings but additional debugging.

```bash
# .env.staging
CORS_MODE=restrictive
CORS_ALLOWED_ORIGINS=https://staging-app.yourmsp.com,http://localhost:3000

API_HOST=0.0.0.0
API_PORT=8080
API_BEHIND_PROXY=true

DB_URL=wss://staging-db.yourmsp.com
DB_NAMESPACE=staging
DB_DATABASE=asseme_staging

LOG_LEVEL=debug
```

### High Security Environment

For sensitive deployments requiring maximum security.

```bash
# .env.secure
CORS_MODE=restrictive
CORS_ALLOWED_ORIGINS=https://secure-portal.client.com
CORS_ALLOW_CREDENTIALS=true
CORS_MAX_AGE=300  # Short cache time

API_HOST=127.0.0.1  # Local only, behind secure proxy
API_PORT=8080
API_BEHIND_PROXY=true

# Minimal headers for security
CORS_ALLOWED_HEADERS=authorization,content-type
CORS_ALLOWED_METHODS=GET,POST,PUT,DELETE

LOG_LEVEL=warn
```

### Environment-Specific Configurations

```bash
# Docker Compose development
# docker-compose.yml
environment:
  - CORS_MODE=permissive
  - API_HOST=0.0.0.0
  - API_PORT=8080
  - DB_URL=ws://surrealdb:8000

# Kubernetes production
# k8s-configmap.yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: server-config
data:
  CORS_MODE: "multi_tenant"
  MSP_ADMIN_DOMAIN: "admin.yourmsp.com"
  API_BEHIND_PROXY: "true"
  LOG_LEVEL: "info"
```

### Testing Configuration

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_development_config() {
        std::env::set_var("CORS_MODE", "permissive");
        std::env::set_var("API_PORT", "8080");

        let config = ServerConfig::from_env().unwrap();
        assert_eq!(config.api.port, 8080);

        config.validate().unwrap();
    }

    #[test]
    fn test_production_config() {
        std::env::set_var("CORS_MODE", "multi_tenant");
        std::env::set_var("MSP_ADMIN_DOMAIN", "admin.test.com");

        let config = ServerConfig::from_env().unwrap();
        config.validate().unwrap();
    }
}
```

## Security Considerations

1. **Never use permissive CORS in production**
2. **Always use HTTPS origins in production**
3. **Validate tenant domains against your database**
4. **Monitor CORS violations in logs**
5. **Use minimal required headers and methods**
6. **Consider rate limiting per origin**
7. **Regularly audit allowed origins**

## Troubleshooting

### Common Issues

**CORS Preflight Failures**

```bash
# Check allowed methods include OPTIONS
CORS_ALLOWED_METHODS=GET,POST,PUT,DELETE,OPTIONS

# Verify max-age is reasonable
CORS_MAX_AGE=3600
```

**Credential Issues**

```bash
# Ensure credentials are allowed and origins are specific
CORS_ALLOW_CREDENTIALS=true
# Don't use * with credentials - specify exact origins
```
