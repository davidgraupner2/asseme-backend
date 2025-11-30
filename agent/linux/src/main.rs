use config_agent::AgentSettings;
use ractor::Actor;
use runtime_agent::{
    actors::controller::arguments::AgentControllerArguments, AgentRuntimeController,
};
use runtime_shared::RuntimeProperties;
use std::collections::HashMap;
use tokio::signal;

use config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialise the runtime properties we will be leveraging
    RuntimeProperties::init("Linux Agent");

    // Add the file names we will need for the agent
    let runtime_properties = RuntimeProperties::global();
    runtime_properties.register_file(
        "config_file",
        runtime_properties
            .folders()
            .home()
            .join("config")
            .join("config.toml"),
    );

    let settings = AgentSettings::new(
        runtime_properties
            .get_file("config_file")
            .unwrap()
            .to_string_lossy()
            .to_string(),
    )?;

    // // Add the file names we will need for the agent
    // let runtime_properties = RuntimeProperties::global();
    // runtime_properties.register_file(
    //     "config_file",
    //     runtime_properties
    //         .folders()
    //         .home()
    //         .join("config")
    //         .join("config.toml"),
    // );

    // let config_file_name = runtime_properties.get_file("config_file").unwrap();

    // let settings = Config::builder()
    //     .add_source(config::File::with_name(&format!(
    //         "{}",
    //         config_file_name.display()
    //     )))
    //     .build()
    //     .unwrap();

    // println!(
    //     "{:?}",
    //     settings
    //         .try_deserialize::<HashMap<String, String>>()
    //         .unwrap()
    // );

    println!("{:?}", settings);

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
