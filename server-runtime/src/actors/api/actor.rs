use crate::actors::api::{
    messages::ApiMessage,
    state::{ApiActorState, ApiState, V1ApiState},
    utils::load_certs,
    v1::routes::api_router,
};
use axum::Router;
use ractor::{Actor, ActorProcessingErr, ActorRef};
use server_config::{ApiConfiguration, CorsConfiguration};
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::{event, instrument};

#[derive(Debug)]
pub struct ApiStartupArguments {
    pub api_config: ApiConfiguration,
    pub cors: CorsConfiguration,
}

#[derive(Debug)]
pub struct ApiActor {}

impl ApiActor {
    fn router(state: ApiState, cors: CorsConfiguration) -> Router {
        let v1_state = Arc::new(V1ApiState::new());

        Router::new()
            .merge(api_router(state.clone(), v1_state.clone()))
            .with_state(state.into())
    }
}

impl Actor for ApiActor {
    type State = ApiActorState;
    type Msg = ApiMessage;
    type Arguments = ApiStartupArguments;

    #[instrument(name = "API_Pre_Start", level = "trace")]
    async fn pre_start(
        &self,
        myself: ActorRef<Self::Msg>,
        args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        let state = ApiActorState::new();

        // Load the TSL certificates
        // - Panics if certificates cannot be found
        let cert_config = load_certs().await;

        //Initialise the shared Axum State
        let api_state = ApiState::new();

        // Start the Web Server
        let listener =
            std::net::TcpListener::bind(format!("0.0.0.0:{}", args.api_config.port.clone()))
                .unwrap();

        // Create the API Router
        // - Ensuring we pass in the required shared state and cors configuration
        let app = Self::router(api_state.clone(), args.cors.clone());

        // Use axum-server with TLS and connection info for IP extraction
        let server = axum_server::from_tcp_rustls(listener, cert_config)
            .serve(app.into_make_service_with_connect_info::<SocketAddr>());

        tokio::spawn(async move {
            if let Err(e) = server.await {
                eprintln!("Server error: {}", e);
            }
        });

        Ok(state)
    }

    #[instrument(name = "API_Post_Start", level = "trace")]
    async fn post_start(
        &self,
        myself: ActorRef<Self::Msg>,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        event!(tracing::Level::INFO, "API Server started");

        Ok(())
    }

    #[instrument(name = "API_Post_Stop", level = "trace")]
    async fn post_stop(
        &self,
        myself: ActorRef<Self::Msg>,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        event!(tracing::Level::INFO, "API Server stopped");

        Ok(())
    }
}
