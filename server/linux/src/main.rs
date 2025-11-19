use server_config::{
    LoadApiConfiguration, LoadCorsConfiguration, LoadLoggingConfiguration,
    LoadRateLimitingConfiguration,
};
use server_config_loaders::env_loader::EnvServerConfigLoader;
use server_runtime::RuntimeProperties;
use tokio::signal;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialise the rumtime properties we will be leveraging
    RuntimeProperties::init("Asseme");

    println!(
        "ID: {:?}, name: {:?}, host_name: {:?}, version: {:?}, exe: {:?}, jobs: {:?}, log_file_name: {:?}",
        RuntimeProperties::global().id(),
        RuntimeProperties::global().name(),
        RuntimeProperties::global().host_name(),
        RuntimeProperties::global().version(),
        RuntimeProperties::global().exe_name(),
        RuntimeProperties::global().folders().jobs(),
        RuntimeProperties::global().files().log_file_name()
    );

    let env_loader = EnvServerConfigLoader::new();

    // Load all configs using type inference
    let api_config = LoadApiConfiguration::load_config(&env_loader);
    let cors_config = LoadCorsConfiguration::load_config(&env_loader);
    let logging_config = LoadLoggingConfiguration::load_config(&env_loader);
    let rate_limit_config = LoadRateLimitingConfiguration::load_config(&env_loader);

    // Use configs to set up your server
    info!("Starting server on port {}", api_config.port);
    println!("Starting server on port {}", api_config.port);
    println!("Log level: {:?}", logging_config.log_level);
    println!("CORS mode: {:?}", cors_config.mode);

    // Wait here until we receive a CTRL-C Signal break
    signal::ctrl_c().await.expect("Failed to wait");

    Ok(())
}
