use super::api_handlers;
use crate::actors::api::{
    self,
    messages::APIMessage,
    state::{ApiActorState, AxumApiState},
    types::APIStartupArguments,
};
use axum::http::{HeaderMap, HeaderName, Request, Response, StatusCode};
use axum::Router;
use axum_server::tls_rustls::RustlsConfig;
use bytes::Bytes;
use ractor::{Actor, ActorProcessingErr, ActorRef};
use serde::ser;
use server_config::cors::CorsConfiguration;
use std::time::Duration;
use std::{
    net::{SocketAddr, TcpListener},
    sync::Arc,
};
use tower::ServiceBuilder;
use tower_http::request_id::{
    MakeRequestId, PropagateRequestIdLayer, RequestId, SetRequestIdLayer,
};
use tower_http::request_id::{MakeRequestUuid, PropagateRequestId};
use tower_http::{
    classify::ServerErrorsFailureClass,
    trace::{DefaultMakeSpan, DefaultOnResponse, Trace, TraceLayer},
};
use tracing::event;

pub struct APIActor {}

impl APIActor {
    fn router(state: AxumApiState, cors: CorsConfiguration) -> Router {
        let cors_layer = cors.to_cors_layer();

        // Header name for where we place the unique request id
        let x_request_id = HeaderName::from_static("x-request-id");

        // Leverage servicebuilder for our common middleware across all routes
        let service_builder = ServiceBuilder::new()
            // Set `x-request-id` header on all requests
            .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid).clone())
            // Log requests and response using tracing
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::new().include_headers(true))
                    .on_response(DefaultOnResponse::new().include_headers(true)),
            )
            // Propogate the `x-request-id` headers from request to response
            .layer(PropagateRequestIdLayer::new(x_request_id))
            // Ensures our server is adequately protected
            .layer(cors_layer);

        Router::new()
            .merge(api_handlers::api::api_router(state.clone()))
            .merge(api_handlers::agent::agent_router())
            .layer(service_builder)
            .with_state(state.into())
    }
}

impl Actor for APIActor {
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
        // - If certificates
        let cert_config = match RustlsConfig::from_pem_file("./certs/cert.pem", "./certs/key.pem")
            .await
        {
            Ok(config) => config,
            Err(e) => {
                eprintln!("Failed to load TLS certificates:");
                eprintln!("  Expected certificate file: ./certs/cert.pem");
                eprintln!("  Expected private key file: ./certs/key.pem");
                eprintln!(
                    "  Current working directory: {:?}",
                    std::env::current_dir().unwrap_or_default()
                );
                eprintln!("  The server is not allowed to operate in an insecure manner.");
                eprintln!("  Error details: {}", e);
                panic!("TLS certificate configuration failed - server cannot start without proper certificates");
            }
        };

        // Init the shared api state
        let axum_state =
            AxumApiState::new(arguments.database_config, arguments.api_config.clone()).await;

        // Start the Web Server
        let listener =
            TcpListener::bind(format!("0.0.0.0:{}", arguments.api_config.port.clone())).unwrap();

        // Create the API Router
        // - Ensuring we pass in the required shared state
        // - Ensuring Tracing is enabled appropriately
        let app = Self::router(axum_state.clone(), arguments.cors.clone());

        let server =
            axum_server::from_tcp_rustls(listener, cert_config).serve(app.into_make_service());

        tokio::spawn(async move {
            if let Err(e) = server.await {
                eprintln!("Server error: {}", e);
            }
            6
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
