use crate::actors::api::routes::routes::api_router;
use crate::actors::api::{
    messages::APIMessage,
    state::{ApiActorState, AxumApiState},
    types::APIStartupArguments,
    utils::get_request_id_header_name,
};
use axum::Router;
use axum_server::tls_rustls::RustlsConfig;
use ractor::{Actor, ActorProcessingErr, ActorRef};
use server_config::cors::CorsConfiguration;
use std::net::SocketAddr;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_governor::{
    governor::GovernorConfigBuilder, key_extractor::SmartIpKeyExtractor, GovernorLayer,
};
use tower_http::request_id::MakeRequestUuid;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tower_http::{
    compression::CompressionLayer,
    request_id::{PropagateRequestIdLayer, SetRequestIdLayer},
};
use tracing::event;

pub struct APIActor {}

impl APIActor {
    fn router(state: AxumApiState, cors: CorsConfiguration) -> Router {
        let cors_layer = cors.to_cors_layer();

        // Setup the standard rate limiting configuration
        // - We are utilising [tower-governor](https://github.com/benwis/tower-governor/)
        // - This is limited per client IP Address
        let rate_limiter_config = GovernorConfigBuilder::default()
            .per_second(state.rate_limiting_per_second)
            .burst_size(state.rate_limiting_burst_size)
            .key_extractor(SmartIpKeyExtractor)
            .use_headers()
            .finish()
            .unwrap();

        // Initialise a seperate background task to cleanup old tracking entries in RAM
        // - This avoids over consumption of RAM when tracking previous API requests
        let rate_limiter = rate_limiter_config.limiter().clone();
        let interval = Duration::from_secs(state.rate_limiting_cleanup_duration);
        std::thread::spawn(move || loop {
            std::thread::sleep(interval);
            tracing::info!("rate limiting storage size: {}", rate_limiter.len());
            rate_limiter.retain_recent();
        });

        // Leverage servicebuilder for our common middleware across all routes
        let middleware_stack = ServiceBuilder::new()
            // Set `x-request-id` header on all requests
            // Won't override the header if a request id is already set
            .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid).clone())
            // Log requests and response using tracing
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::new().include_headers(true))
                    .on_response(DefaultOnResponse::new().include_headers(true)),
            )
            // Propogate the `x-request-id` headers from request to response
            .layer(PropagateRequestIdLayer::new(get_request_id_header_name()))
            // Ensures our server is adequately protected
            .layer(cors_layer)
            // Compress responses based on the `Accept-Encoding` header
            .layer(CompressionLayer::new())
            // Add rate limiting
            .layer(GovernorLayer::new(rate_limiter_config));

        Router::new()
            .merge(api_router(state.clone()))
            // .merge(api_handlers::api::api_router(state.clone()))
            // .merge(api_handlers::agent::agent_router())
            .layer(middleware_stack)
            // Apply timeout separately to avoid trait bound issues
            // - This layer ensures all request complete within a specified time or they are timed out
            .layer(TimeoutLayer::new(Duration::from_secs(
                state.request_timeout_secs,
            )))
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
            std::net::TcpListener::bind(format!("0.0.0.0:{}", arguments.api_config.port.clone()))
                .unwrap();

        // Create the API Router
        // - Ensuring we pass in the required shared state
        // - Ensuring Tracing is enabled appropriately
        let app = Self::router(axum_state.clone(), arguments.cors.clone());

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
