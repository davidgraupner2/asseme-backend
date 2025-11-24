use std::error::Error;

use crate::{
    actors::{
        api::{
            actor::{ApiActor, ApiStartupArguments},
            ApiMessage,
        },
        controller::{
            arguments::ControllerArguments, messages::ControllerMessage, state::ControllerState,
        },
    },
    logging::initialise_logging,
    RuntimeProperties,
};
use ractor::concurrency::Duration;
use ractor::Actor;
use ractor::{ActorProcessingErr, ActorRef};
// use ractor_supervisor::*;
use server_config::{ApiConfiguration, CorsConfiguration};
use tokio::time::Instant;
use tracing::{debug, error, event, info, instrument, trace, warn, Level};

#[derive(Debug)]
pub struct Controller;

impl Actor for Controller {
    type State = ControllerState;
    type Msg = ControllerMessage;
    type Arguments = ControllerArguments;

    // Invoked when the controller is being started
    // Panics in pre_start do not invoke the supervision strategy and the actor won’t be started
    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        arguments: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        // Initialize Rustls crypto provider
        rustls::crypto::ring::default_provider()
            .install_default()
            .map_err(|_| ())
            .ok(); // Ignore error if already installed

        // Initialise our state
        let mut state =
            ControllerState::new(arguments.api_configuration, arguments.cors_configuration);

        // Setup Logging
        let tracing_worker_guards = initialise_logging(
            RuntimeProperties::global().folders().logs(),
            &format!("{}.log", RuntimeProperties::global().app_name()),
            &arguments.log_format,
            &arguments.log_output,
        );

        state.tracing_worker_guards = tracing_worker_guards;

        Ok(state)
    }

    // TODO Documentation
    // Invoked after an actor has started.
    // Any post initialization can be performed here, such as writing to a log file, emitting metrics.
    #[instrument(name = "Controller_Post_Start", level = "trace")]
    async fn post_start(
        &self,
        myself: ActorRef<Self::Msg>,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        event!(
            tracing::Level::INFO,
            "Runtime Controller starting API Server"
        );

        // Start the API Server as a linked actor i.e. Controller is the supervisor
        state.spawned_actors.api_server = start_api_server(
            myself,
            state.api_configuration.clone(),
            state.cors_configuration.clone(),
        )
        .await;

        Ok(())
    }

    async fn handle_supervisor_evt(
        &self,
        myself: ActorRef<Self::Msg>,
        message: ractor::SupervisionEvent,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match message {
            _ => {
                dbg!("Supervisor event received: {:?}", message);
                // warn!("Supervisor event received: {:?}", message);
            }
        }
        Ok(())
    }
}

// Starts the API Server
async fn start_api_server(
    controller: ActorRef<ControllerMessage>,
    api_config: ApiConfiguration,
    cors_config: CorsConfiguration,
) -> Option<ActorRef<ApiMessage>> {
    // Start the API Server as a linked actor i.e. Controller is the supervisor
    let api_server = controller
        .spawn_linked(
            Some("Api Server".to_string()),
            ApiActor {},
            ApiStartupArguments {
                api_config: api_config.clone(),
                cors: cors_config.clone(),
            },
        )
        .await;

    match api_server {
        Ok((actor_ref, join_handle)) => Some(actor_ref),
        Err(error) => {
            event!(
                Level::ERROR,
                "Api Server could not be started! - {:#?}",
                error
            );
            None
        }
    }
}

// A child-level backoff function that implements exponential backoff after the second failure.
// Return Some(delay) to make the supervisor wait before restarting this child.
// fn actor_supervision_backoff_strategy() -> ChildBackoffFn {
//     ChildBackoffFn::new(
//         |_child_id: &str,
//          restart_count: usize,
//          last_fail: Instant,
//          child_reset_after: Option<Duration>| {
//             // On the first failure, restart immediately (None).
//             // After the second failure, double the delay each time (exponential).
//             if restart_count <= 1 {
//                 None
//             } else {
//                 Some(Duration::from_secs(1 << restart_count))
//             }
//         },
//     )
// }

// // This actor supervision specification describes exactly how to manage our single API actor.
// fn api_actor_restart_strategy() -> ChildSpec {
//     ChildSpec {
//         id: "api_server".into(), // Unique identifier for meltdown logs and debugging.
//         restart: Restart::Transient, // Only restart if the child fails abnormally.
//         spawn_fn: SpawnFn::new(|cell, id| spawn_api_server(cell, id)),
//         backoff_fn: Some(actor_supervision_backoff_strategy()), // Apply our custom exponential backoff on restarts.
//         // If the child remains up for 60s, its individual failure counter resets to 0 next time it fails.
//         reset_after: Some(Duration::from_secs(60)),
//     }
// }

// // Supervisor-level meltdown configuration. If more than 5 restarts occur within a 10s window, meltdown is triggered.
// // Also, if we stay quiet for 30s (no restarts), the meltdown log resets.
// fn supervision_meltdown_strategy() -> SupervisorOptions {
//     SupervisorOptions {
//         strategy: SupervisorStrategy::OneForOne, // If one child fails, only that child is restarted.
//         max_restarts: 5,                         // Permit up to 5 restarts in the meltdown window.
//         max_window: Duration::from_secs(10),     // The meltdown window.
//         reset_after: Some(Duration::from_secs(30)), // If no failures for 30s, meltdown log is cleared.
//     }
// }

// // Group all child actor specifications and meltdown options together:
// fn supervisor_arguments() -> SupervisorArguments {
//     // Group all Child Specifications together
//     let child_specs = vec![api_actor_restart_strategy()];

//     SupervisorArguments {
//         child_specs: child_specs,
//         options: supervision_meltdown_strategy(),
//     }
// }

// // Spawn the actor supervisor
// async fn spawn_actor_supervisor() -> Result<ActorRef<SupervisorMsg>, Box<dyn Error>> {
//     let (supervisor, _join_handle) = Supervisor::spawn(
//         "root".into(), // name for the supervisor
//         supervisor_arguments(),
//     )
//     .await?;
//     Ok(supervisor)
// }
