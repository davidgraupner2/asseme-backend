use super::api_handlers;
use crate::actors::api::{
    messages::APIMessage,
    state::{ApiActorState, AxumApiState},
    types::APIStartupArguments,
};
use axum::http::{HeaderMap, Request, Response, StatusCode};
use axum::Router;
use axum_server::tls_rustls::RustlsConfig;
use bytes::Bytes;
use http_body_util::Full;
use ractor::{Actor, ActorProcessingErr, ActorRef};
use std::net::{SocketAddr, TcpListener};
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::event;
use tracing::Span;

pub struct APIActor {}

impl APIActor {
    fn router(state: AxumApiState) -> Router {
        Router::new()
            .nest(
                "/api",
                Router::new()
                    .nest("/auth", api_handlers::auth::auth_router())
                    .nest(
                        "/tenant",
                        api_handlers::tenant::tenant_router(state.clone()),
                    )
                    .merge(api_handlers::misc::misc_router()),
            )
            .nest("/agent", api_handlers::agent::agent_router())
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
        let app = Self::router(axum_state.clone()).layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    tracing::info_span!(
                        "http_request",
                        method = %request.method(),
                        uri = %request.uri(),
                        version = ?request.version(),
                        headers = ?request.headers(),
                    )
                })
                .on_request(|request: &Request<_>, _span: &Span| {
                    tracing::info!(
                        "Started processing request: {} {} {:?}",
                        request.method(),
                        request.uri(),
                        request.version()
                    );
                })
                .on_response(|response: &Response<_>, latency: Duration, _span: &Span| {
                    tracing::info!(
                        status = %response.status(),
                        latency = ?latency,
                        headers = ?response.headers(),
                        "Response completed with status {} in {:?}",
                        response.status(),
                        latency
                    );
                })
                .on_body_chunk(|chunk: &Bytes, latency: Duration, _span: &Span| {
                    tracing::debug!(
                        chunk_size = chunk.len(),
                        latency = ?latency,
                        "Sent {} bytes",
                        chunk.len()
                    );
                })
                .on_eos(
                    |trailers: Option<&HeaderMap>, stream_duration: Duration, _span: &Span| {
                        tracing::debug!(
                            trailers = ?trailers,
                            duration = ?stream_duration,
                            "Request stream completed after {:?}",
                            stream_duration
                        );
                    },
                )
                .on_failure(
                    |error: ServerErrorsFailureClass, latency: Duration, _span: &Span| {
                        tracing::error!(
                            error = ?error,
                            latency = ?latency,
                            "Request failed with error: {:?} after {:?}",
                            error,
                            latency
                        );
                    },
                ),
        );

        let server = axum_server::from_tcp_rustls(listener, cert_config)
            .serve(app.into_make_service_with_connect_info::<SocketAddr>());

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
