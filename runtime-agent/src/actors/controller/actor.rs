use database_agent::{ensure_database_schema, get_db_connection_pool, SqlitePool};
use ractor::Actor;
use ractor::{ActorProcessingErr, ActorRef};
use tracing::{error, info, instrument};

use crate::actors::api::actor::{ApiActor, ApiStartupArguments};
use crate::actors::api::messages::ApiMessage;
use crate::actors::controller::arguments::AgentControllerArguments;
use crate::actors::controller::messages::AgentControllerMessage;
use crate::actors::controller::state::AgentControllerState;
use crate::actors::{ACTOR_AGENT_API_NAME, DATABASE_NAME};
use config_agent::api::ApiConfiguration;
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
        let mut state =
            AgentControllerState::new(arguments.api_config, arguments.log_config.clone());

        // Setup Logging
        let tracing_worker_guards = initialise_logging(
            RuntimeProperties::global().folders().logs(),
            &format!("{}.log", RuntimeProperties::global().app_name()),
            &arguments.log_config.format,
            "file",
            Some(&state.log_config.level),
        );

        state.tracing_worker_guards = tracing_worker_guards;

        Ok(state)
    }

    #[instrument(name = "Agent Controller Post Start", level = "trace")]
    async fn post_start(
        &self,
        myself: ActorRef<Self::Msg>,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        info!("Agent Controller has started");

        // Generate the agent database file name
        let supplementary_files_folder =
            RuntimeProperties::global().folders().supplementary_files();
        let db_file_name = supplementary_files_folder
            .join(DATABASE_NAME)
            .to_string_lossy()
            .to_string();

        // Ensure database migrations are run to create and update the database schema (if needed)
        let _ = ensure_database_schema(db_file_name.clone());

        // Create the database pool to use for the api
        let db_pool = get_db_connection_pool(db_file_name)?;

        // Start the API Server as a linked actor i.e. Controller is the supervisor
        state.spawned_actors.api_server =
            start_agent_api_server(myself, state.api_config.clone(), db_pool).await;

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

#[instrument(name = "Agent Controller - Start Api Server", level = "trace")]
async fn start_agent_api_server(
    controller: ActorRef<AgentControllerMessage>,
    api_config: ApiConfiguration,
    db_pool: SqlitePool,
) -> Option<ActorRef<ApiMessage>> {
    // Start the API Server as a linked actor i.e. Controller is the supervisor
    match controller
        .spawn_linked(
            Some(ACTOR_AGENT_API_NAME.to_string()),
            ApiActor {},
            ApiStartupArguments {
                port: api_config.port,
                db_pool,
            },
        )
        .await
    {
        Ok(result) => Some(result.0),

        Err(error) => {
            error!(errorMsg = %error, "Error spawning {}", ACTOR_AGENT_API_NAME);
            None
        }
    }
}
