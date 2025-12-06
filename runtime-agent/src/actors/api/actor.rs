use axum::Router;
use database_agent::SqlitePool;
use ractor::{Actor, ActorProcessingErr, ActorRef};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tracing::{info, instrument};

use crate::actors::{
    api::{
        messages::ApiMessage,
        routes::api_router,
        state::{ApiActorState, ApiState},
    },
    ACTOR_AGENT_API_NAME,
};
use runtime_shared::api_server::APIServer;

#[derive(Debug)]
pub struct ApiStartupArguments {
    pub port: u16,
    pub db_pool: SqlitePool,
}

#[derive(Debug)]
pub struct ApiActor {}

impl ApiActor {
    fn router(state: ApiState) -> Router {
        Router::new().merge(api_router()).with_state(state.into())
    }
}

impl Actor for ApiActor {
    type State = ApiActorState;
    type Msg = ApiMessage;
    type Arguments = ApiStartupArguments;

    #[instrument(name = "Agent API - Pre Start", level = "trace")]
    async fn pre_start(
        &self,
        myself: ActorRef<Self::Msg>,
        args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        let mut state = ApiActorState::new();

        //Initialise the shared Axum State
        let api_state = ApiState::new(args.db_pool);

        let app = Self::router(api_state.clone());

        // let addr = SocketAddr::new(Ipv4Addr::new(127,0,0,1),8014);
        let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), args.port);

        match APIServer::new(socket, app).start().await {
            Ok(server_shutdown_handle) => {
                state.server_handle = Some(server_shutdown_handle);

                Ok(state)
            }
            Err(error) => Err(error.into()),
        }
    }

    #[instrument(name = "API Server - Post Start", level = "trace")]
    async fn post_start(
        &self,
        myself: ActorRef<Self::Msg>,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        info!(name = ACTOR_AGENT_API_NAME, "started successfully");

        Ok(())
    }

    #[instrument(name = "API Server - Post Stop", level = "trace")]
    async fn post_stop(
        &self,
        myself: ActorRef<Self::Msg>,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        // Signal the API Server to shutdown
        if let Some(handle) = &state.server_handle {
            handle.shutdown();
            info!("signaled server shutdown");
        }
        info!(name = ACTOR_AGENT_API_NAME, "stopped");

        Ok(())
    }

    #[instrument(name = "API Server - Process Message", level = "trace")]
    async fn handle(
        &self,
        myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match message {}

        Ok(())
    }
}
