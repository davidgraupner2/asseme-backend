use ractor::Actor;
use runtime_shared::RuntimeProperties;
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialise the rumtime properties we will be leveraging
    RuntimeProperties::init("Linux Agent");

    let env_loader = EnvServerConfigLoader::new();

    // Load all configs using type inference
    let api_config = LoadApiConfiguration::load_config(&env_loader);
    let cors_config = LoadCorsConfiguration::load_config(&env_loader);
    let logging_config = LoadLoggingConfiguration::load_config(&env_loader);
    let rate_limit_config = LoadRateLimitingConfiguration::load_config(&env_loader);

    // Create the arguments we need to pass to the controller runtime
    let runtime_controller_args = RuntimeControllerArguments {
        log_format: logging_config.log_format,
        log_output: logging_config.log_output,
        api_configuration: api_config,
        cors_configuration: cors_config,
        rate_limiting_configuration: rate_limit_config,
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
