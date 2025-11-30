use config_agent::AgentSettings;
use database_agent::database::Database;
use ractor::Actor;
use ractor::{ActorProcessingErr, ActorRef};
use tracing::{error, event, info, instrument, warn};

use crate::actors::controller::arguments::AgentControllerArguments;
use crate::actors::controller::messages::AgentControllerMessage;
use crate::actors::controller::state::AgentControllerState;
use runtime_shared::{initialise_logging, RuntimeProperties};

#[derive(Debug)]
pub struct Controller;

impl Actor for Controller {
    type State = AgentControllerState;
    type Msg = AgentControllerMessage;
    type Arguments = AgentControllerArguments;

    // Invoked when the controller is being started
    // Panics in pre_start do not invoke the supervision strategy and the actor won’t be started
    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        arguments: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        // Initialise our state
        let mut state = AgentControllerState::new();

        // Setup Logging
        let tracing_worker_guards = initialise_logging(
            RuntimeProperties::global().folders().logs(),
            &format!("{}.log", RuntimeProperties::global().app_name()),
            &arguments.log_config.format,
            "file",
        );

        state.tracing_worker_guards = tracing_worker_guards;

        let mut database = Database::new("test.db".to_string());
        database.initialise().await;

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
        info!("Controller has started");

        Ok(())
    }

    #[instrument(name = "Controller_Supervision_Handler", level = "trace")]
    async fn handle_supervisor_evt(
        &self,
        myself: ActorRef<Self::Msg>,
        message: ractor::SupervisionEvent,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        Ok(())
    }
}
