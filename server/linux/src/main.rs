use ractor::Actor;
use server_config::{
    LoadApiConfiguration, LoadCorsConfiguration, LoadLoggingConfiguration,
    LoadRateLimitingConfiguration,
};
use server_config_loaders::env_loader::EnvServerConfigLoader;
use server_runtime::{RuntimeController, RuntimeControllerArguments, RuntimeProperties};
use tokio::signal;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialise the rumtime properties we will be leveraging
    RuntimeProperties::init("Asseme");

    let env_loader = EnvServerConfigLoader::new();

    // Load all configs using type inference
    let api_config = LoadApiConfiguration::load_config(&env_loader);
    let cors_config = LoadCorsConfiguration::load_config(&env_loader);
    let logging_config = LoadLoggingConfiguration::load_config(&env_loader);
    let rate_limit_config = LoadRateLimitingConfiguration::load_config(&env_loader);

    // Use configs to set up your server
    info!("Starting server on port {}", api_config.port);
    println!("Starting server on port {}", api_config.port);
    println!("CORS mode: {:?}", cors_config.mode);

    // Create the arguments we need to pass to the controller runtime
    let runtime_controller_args = RuntimeControllerArguments {
        log_format: logging_config.log_format,
        log_output: logging_config.log_output,
    };

    // Start the runtime controller
    let (_actor, _actor_handle) = Actor::spawn(
        Some("RuntimeController".to_string()),
        RuntimeController,
        runtime_controller_args,
    )
    .await
    .expect("RuntimeController failed to start");

    // Wait here until we receive a CTRL-C Signal break
    signal::ctrl_c().await.expect("Failed to wait");

    Ok(())
}
