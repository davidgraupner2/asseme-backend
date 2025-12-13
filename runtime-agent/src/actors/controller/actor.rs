use database_agent::models::properties::PropertyValue;
use database_agent::{get_db_connection_pool, SqlitePool};
use ractor::Actor;
use ractor::{ActorProcessingErr, ActorRef};
use tracing::{error, info, instrument};

use crate::actors::api::actor::{ApiActor, ApiStartupArguments};
use crate::actors::api::messages::ApiMessage;
use crate::actors::controller::arguments::AgentControllerArguments;
use crate::actors::controller::messages::AgentControllerMessage;
use crate::actors::controller::state::AgentControllerState;

use crate::{
    ACTOR_AGENT_API_NAME, DATABASE_NAME, DEFAULT_PROPERTY_LOGGING_FORMAT,
    DEFAULT_PROPERTY_LOGGING_LEVEL, PROPERTY_LOGGING_FORMAT, PROPERTY_LOGGING_LEVEL,
};
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

        // Get access to our database pool
        if let Ok(db_pool) = get_db_connection_pool(
            RuntimeProperties::global().folders().supplementary_files(),
            DATABASE_NAME,
        ) {
            state.db_pool = Some(db_pool);
        } else {
            panic!(
                "Database {} does not exist or could not be created!!!",
                DATABASE_NAME
            );
        }

        let rp = RuntimeProperties::global();
        println!("Properties: {:#?}", rp);

        // Load our logging parameters or defaults
        let logging_format = PropertyValue::get_string_or(
            state.db_pool.clone().unwrap().get().unwrap(),
            PROPERTY_LOGGING_FORMAT,
            DEFAULT_PROPERTY_LOGGING_FORMAT.to_string(),
        );

        let logging_level = PropertyValue::get_string_or(
            state.db_pool.clone().unwrap().get().unwrap(),
            PROPERTY_LOGGING_LEVEL,
            DEFAULT_PROPERTY_LOGGING_LEVEL.to_string(),
        );

        // Setup Logging
        let tracing_worker_guards = initialise_logging(
            RuntimeProperties::global().folders().logs(),
            &format!("{}.log", RuntimeProperties::global().app_name()),
            &logging_format,
            "file",
            Some(&logging_level),
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

        // Start the API Server as a linked actor i.e. Controller is the supervisor
        state.spawned_actors.api_server =
            start_agent_api_server(myself, state.db_pool.clone().unwrap()).await;

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
    db_pool: SqlitePool,
) -> Option<ActorRef<ApiMessage>> {
    // Start the API Server as a linked actor i.e. Controller is the supervisor
    match controller
        .spawn_linked(
            Some(ACTOR_AGENT_API_NAME.to_string()),
            ApiActor {},
            ApiStartupArguments { db_pool },
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
