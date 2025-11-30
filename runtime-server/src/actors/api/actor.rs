use crate::actors::api::{state::ApiActorState, utils::get_request_id_header_name};
use crate::actors::{
    api::{
        cors::to_cors_layer,
        messages::ApiMessage,
        state::{ApiState, V1ApiState},
        utils::load_certs,
        v1::routes::api_router,
    },
    controller::ACTOR_API_SERVER_NAME,
};
use axum::Router;
use axum_server::tls_rustls::RustlsConfig;
use config_server::{ApiConfiguration, CorsConfiguration, RateLimitingConfiguration};
use ractor::{Actor, ActorProcessingErr, ActorRef};
use std::net::SocketAddr;
use std::sync::Arc;
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
use tracing::{error, info, instrument};

#[derive(Debug)]
pub struct ApiStartupArguments {
    pub api_config: ApiConfiguration,
    pub cors: CorsConfiguration,
    pub rate_limiting: RateLimitingConfiguration,
}

#[derive(Debug)]
pub struct ApiActor {}

impl ApiActor {
    fn router(
        state: ApiState,
        cors: CorsConfiguration,
        port: u16,
        rate_limiting: RateLimitingConfiguration,
        api_request_timeout_seconds: u64,
        agent_ping_interval: u64,
        agent_ping_timeout: u64,
    ) -> Router {
        // Create the initial API Version 1 state - as we create more version, there could be more of these
        let v1_state = V1ApiState::new(agent_ping_interval, agent_ping_timeout);

        // Create the CORS layer based on passed in configuration
        let cors_layer = to_cors_layer(cors, port);

        // Setup the standard rate limiting configuration
        // - We are utilising [tower-governor](https://github.com/benwis/tower-governor/)
        // - This is limited per client IP Address
        let rate_limiter_config = GovernorConfigBuilder::default()
            .per_second(rate_limiting.per_second)
            .burst_size(rate_limiting.burst_size)
            .key_extractor(SmartIpKeyExtractor)
            .use_headers()
            .finish()
            .unwrap();

        // Initialise a seperate background task to cleanup old tracking entries in RAM
        // - This avoids over consumption of RAM when tracking previous API requests
        let rate_limiter = rate_limiter_config.limiter().clone();
        let interval = Duration::from_secs(rate_limiting.cleanup_duration);
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
            .merge(api_router(state.clone().into(), v1_state))
            .layer(middleware_stack)
            // Add a timeout layer to timeout requests if they take too long
            .layer(TimeoutLayer::new(Duration::from_secs(
                api_request_timeout_seconds,
            )))
            .with_state(state.into())
    }
}

impl Actor for ApiActor {
    type State = ApiActorState;
    type Msg = ApiMessage;
    type Arguments = ApiStartupArguments;

    #[instrument(name = "API Server - Pre Start", level = "trace")]
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
        let listener = match tokio::net::TcpListener::bind(format!(
            "0.0.0.0:{}",
            args.api_config.port.clone()
        ))
        .await
        {
            Ok(listener) => listener.into_std().unwrap(),
            Err(error) => {
                error!(error=%error,actor=ACTOR_API_SERVER_NAME,"Failed to start");
                panic!("{}", error)
            }
        };

        // Create the API Router
        // - Ensuring we pass in the required shared state and cors configuration
        let app = Self::router(
            api_state.clone(),
            args.cors.clone(),
            args.api_config.port.clone(),
            args.rate_limiting,
            args.api_config.request_timeout_secs,
            args.api_config.agent_ping_interval,
            args.api_config.agent_ping_timeout,
        );

        let server = axum_server::from_tcp_rustls(listener, cert_config)
            .serve(app.into_make_service_with_connect_info::<SocketAddr>());

        tokio::spawn(async move {
            if let Err(e) = server.await {
                eprintln!("Server error: {}", e);
            }
        });

        Ok(state)
    }

    #[instrument(name = "API Server - Post Start", level = "trace")]
    async fn post_start(
        &self,
        myself: ActorRef<Self::Msg>,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        info!(name = ACTOR_API_SERVER_NAME, "started successfully");

        Ok(())
    }

    #[instrument(name = "API Server - Post Stop", level = "trace")]
    async fn post_stop(
        &self,
        myself: ActorRef<Self::Msg>,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        // signal server to shutdown and drop the listener
        if let Some(tx) = state.shutdown_tx.take() {
            let _ = tx.send(());
        }

        info!(name = ACTOR_API_SERVER_NAME, "stopped");

        Ok(())
    }

    #[instrument(name = "API Server - Process Message", level = "trace")]
    async fn handle(
        &self,
        myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match message {
            ApiMessage::TriggerPanic => panic!("Test: deliberate panic from ApiActor"),
        }

        Ok(())
    }

    fn handle_supervisor_evt(
        &self,
        myself: ActorRef<Self::Msg>,
        message: ractor::SupervisionEvent,
        state: &mut Self::State,
    ) -> impl std::prelude::rust_2024::Future<Output = Result<(), ActorProcessingErr>> + Send {
        async move {
            match message {
                ractor::SupervisionEvent::ActorTerminated(who, _, _)
                | ractor::SupervisionEvent::ActorFailed(who, _) => {
                    myself.stop(None);
                }
                _ => {}
            }
            Ok(())
        }
    }

    fn spawn(
        name: Option<ractor::ActorName>,
        handler: Self,
        startup_args: Self::Arguments,
    ) -> impl std::prelude::rust_2024::Future<
        Output = Result<
            (ActorRef<Self::Msg>, ractor::concurrency::JoinHandle<()>),
            ractor::SpawnErr,
        >,
    > + Send {
        ractor::ActorRuntime::<Self>::spawn(name, handler, startup_args)
    }

    fn spawn_linked(
        name: Option<ractor::ActorName>,
        handler: Self,
        startup_args: Self::Arguments,
        supervisor: ractor::ActorCell,
    ) -> impl std::prelude::rust_2024::Future<
        Output = Result<
            (ActorRef<Self::Msg>, ractor::concurrency::JoinHandle<()>),
            ractor::SpawnErr,
        >,
    > + Send {
        ractor::ActorRuntime::<Self>::spawn_linked(name, handler, startup_args, supervisor)
    }
}
