use config_agent::AgentSettings;
use ractor::Actor;
use runtime_agent::{
    actors::controller::arguments::AgentControllerArguments, AgentRuntimeController,
};
use runtime_shared::RuntimeProperties;
use tokio::signal;

const AGENT_NAME: &str = "Linux Agent";
const AGENT_CONFIG_FOLDER: &str = "config";
const AGENT_CONFIG_FILE: &str = "agent.toml";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialise the runtime properties we will be leveraging
    RuntimeProperties::init(AGENT_NAME);

    // Add the file names we will need for the agent
    let runtime_properties = RuntimeProperties::global();
    runtime_properties.register_file(
        "config_file",
        runtime_properties
            .folders()
            .home()
            .join(AGENT_CONFIG_FOLDER)
            .join(AGENT_CONFIG_FILE),
    );

    let settings = AgentSettings::new(
        runtime_properties
            .get_file("config_file")
            .unwrap()
            .to_string_lossy()
            .to_string(),
    )?;

    let agent_runtime_controller_arguments = AgentControllerArguments {
        api_config: settings.api,
        log_config: settings.logging,
    };

    // Start the runtime controller
    let (_actor, _actor_handle) = Actor::spawn(
        Some("AgentRuntimeController".to_string()),
        AgentRuntimeController,
        agent_runtime_controller_arguments,
    )
    .await
    .expect("Agent RuntimeController failed to start");

    // Wait here until we receive a CTRL-C Signal break
    signal::ctrl_c().await.expect("Failed to wait");

    Ok(())
}
