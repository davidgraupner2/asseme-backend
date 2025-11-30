use crate::actors::{
    api::{
        actor::{ApiActor, ApiStartupArguments},
        ApiMessage,
    },
    controller::{
        arguments::ControllerArguments, messages::ControllerMessage, state::ControllerState,
        ACTOR_API_SERVER_NAME,
    },
};
use ractor::Actor;
use ractor::{ActorProcessingErr, ActorRef};
// use ractor_supervisor::*;
use config_server::{ApiConfiguration, CorsConfiguration, RateLimitingConfiguration};
use runtime_shared::{initialise_logging, RuntimeProperties};
use tracing::{error, event, info, instrument, warn};

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
        let mut state = ControllerState::new(
            arguments.api_configuration,
            arguments.cors_configuration,
            arguments.rate_limiting_configuration,
        );

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
            state.rate_limiter_config.clone(),
        )
        .await;

        Ok(())
    }

    #[instrument(name = "Controller_Supervision_Handler", level = "trace")]
    async fn handle_supervisor_evt(
        &self,
        myself: ActorRef<Self::Msg>,
        message: ractor::SupervisionEvent,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match message {
            ractor::SupervisionEvent::ActorStarted(actor_cell) => {
                let name = actor_cell
                    .get_name()
                    .unwrap_or_else(|| "unknown".to_string());
                info!(
                    actor = %name,
                    id = %actor_cell.get_id(),
                    "actor started"
                );
            }
            ractor::SupervisionEvent::ActorTerminated(actor_cell, boxed_state, _) => {
                let name = actor_cell
                    .get_name()
                    .unwrap_or_else(|| "unknown".to_string());
                info!(
                    actor = %name,
                    id = %actor_cell.get_id(),
                    status = ?boxed_state,
                    "actor terminated"
                );
            }
            ractor::SupervisionEvent::ActorFailed(actor_cell, error) => {
                let name = actor_cell
                    .get_name()
                    .unwrap_or_else(|| "unknown".to_string());
                warn!(
                    actor = %name,
                    id = %actor_cell.get_id(),
                    error = %error,
                    "actor failed - attempting restart"
                );

                // Log restart attempt, perform restart and log success/failure
                // spawn a task to not block the supervision handler
                let ctrl = myself.clone();
                let api_cfg = state.api_configuration.clone();
                let cors_cfg = state.cors_configuration.clone();
                let rate_cfg = state.rate_limiter_config.clone();

                // Fire-and-forget restart task; capture join result and log later
                if name == ACTOR_API_SERVER_NAME {
                    tokio::spawn(async move {
                        let restarted = start_api_server(ctrl, api_cfg, cors_cfg, rate_cfg).await;
                        match restarted {
                            Some(_) => info!(actor = %name, "actor restart succeeded"),
                            None => error!(actor = %name, "actor restart failed"),
                        }
                    });
                }
            }
            ractor::SupervisionEvent::ProcessGroupChanged(group_change_message) => {
                info!(
                    message = ?group_change_message,
                    "Process group changed"
                )
            }
        }
        Ok(())
    }
}

// Starts the API Server
#[instrument(name = "Controller - Start Api Server", level = "trace")]
async fn start_api_server(
    controller: ActorRef<ControllerMessage>,
    api_config: ApiConfiguration,
    cors_config: CorsConfiguration,
    rate_limiter_config: RateLimitingConfiguration,
) -> Option<ActorRef<ApiMessage>> {
    // Start the API Server as a linked actor i.e. Controller is the supervisor
    match controller
        .spawn_linked(
            Some(ACTOR_API_SERVER_NAME.to_string()),
            ApiActor {},
            ApiStartupArguments {
                api_config: api_config.clone(),
                cors: cors_config.clone(),
                rate_limiting: rate_limiter_config.clone(),
            },
        )
        .await
    {
        Ok(result) => Some(result.0),
        Err(error) => {
            error!(errorMsg = %error, "Error spawning {}", ACTOR_API_SERVER_NAME);
            None
        }
    }
}
