pub mod api_handlers;
pub mod messages;
pub mod startup;
pub mod state;

use crate::actors::api::{
    messages::APIMessage,
    state::{ApiActorState, AxumApiState},
};
use axum::Router;
use axum_server::tls_rustls::RustlsConfig;
use ractor::{Actor, ActorProcessingErr, ActorRef};
use startup::APIStartupArguments;
use std::net::TcpListener;
use std::sync::Arc;
use tracing::event;

pub struct API {}

impl API {
    fn router(state: Arc<AxumApiState>) -> Router {
        Router::new()
            .nest(
                "/api",
                Router::new()
                    .nest("/auth", api_handlers::auth::auth_router())
                    .merge(api_handlers::misc::misc_router()),
            )
            .nest("/agent", api_handlers::agent::agent_router())
            .with_state(state)
    }
}

impl Actor for API {
    type State = ApiActorState;
    type Msg = APIMessage;
    type Arguments = APIStartupArguments;

    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        arguments: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        event!(tracing::Level::INFO, "API Server starting");

        let state = ApiActorState::new();

        // Load the TLS Certificates
        let cert_config = RustlsConfig::from_pem_file("./certs/cert.pem", "./certs/key.pem")
            .await
            .unwrap();

        // Init the shared api state
        let axum_state = Arc::new(AxumApiState::new(arguments.database_config).await);

        // Setup the API Routes
        let app = Self::router(axum_state);

        // Start the Web Server
        let listener = TcpListener::bind(format!("0.0.0.0:{}", arguments.api_config.port)).unwrap();
        let server =
            axum_server::from_tcp_rustls(listener, cert_config).serve(app.into_make_service());

        tokio::spawn(async move {
            if let Err(e) = server.await {
                eprintln!("Server error: {}", e);
            }
        });

        Ok(state)
    }

    async fn post_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        _state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        event!(tracing::Level::INFO, "API Server started");

        Ok(())
    }

    async fn post_stop(
        &self,
        _myself: ActorRef<Self::Msg>,
        _state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        event!(tracing::Level::INFO, "API Server stopped");

        Ok(())
    }
}
