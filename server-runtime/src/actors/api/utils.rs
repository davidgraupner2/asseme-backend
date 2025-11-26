use axum::http::{HeaderMap, HeaderName};
use axum_server::tls_rustls::RustlsConfig;

pub(crate) async fn load_certs() -> RustlsConfig {
    let cert_config = match RustlsConfig::from_pem_file("./.certs/cert.pem", "./.certs/key.pem")
        .await
    {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed to load TLS certificates:");
            eprintln!("  Expected certificate file: ./certs/cert.pem");
            eprintln!("  Expected private key file: ./certs/key.pem");
            eprintln!(
                "  Current working directory: {:?}",
                std::env::current_dir().unwrap_or_default()
            );
            eprintln!("  The server is not allowed to operate in an insecure manner.");
            eprintln!("  Error details: {}", e);
            panic!("TLS certificate configuration failed - server cannot start without proper certificates");
        }
    };

    cert_config
}

// Return the header name we will use to store the request id
// for each call to the api
pub fn get_request_id_header_name() -> HeaderName {
    HeaderName::from_static("x-request-id")
}
