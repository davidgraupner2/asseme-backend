pub mod messages;
mod state;

use crate::actors::api::actor::APIActor;
use crate::actors::api::types::APIStartupArguments;
// use crate::common::check_db_health;
use crate::{common::bootstrap_runtime, properties::runtime_id};
use messages::ControllerMessage;
use ractor::{Actor, ActorProcessingErr, ActorRef};
use state::ControllerState;
use tracing::event;

// https://github.com/slawlor/ractor
// https://slawlor.github.io/ractor/quickstart/

pub struct Controller;

impl Actor for Controller {
    type State = ControllerState;
    type Msg = ControllerMessage;
    type Arguments = ();

    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        _arguments: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        event!(
            tracing::Level::INFO,
            "Runtime Controller {} started",
            runtime_id()
        );

        // Initialize Rustls crypto provider
        rustls::crypto::ring::default_provider()
            .install_default()
            .map_err(|_| ())
            .ok(); // Ignore error if already installed

        let mut state = ControllerState::new();

        // Bootstrap the runtime
        // - This ensures that the required folders exist and logging is started
        let tracing_worker_guards = bootstrap_runtime(
            format!("{:?}", state.config.logging.log_level)
                .as_str()
                .to_lowercase()
                .as_str(),
        )
        .await;
        state.tracing_worker_guards = tracing_worker_guards;

        Ok(state)
    }

    async fn post_start(
        &self,
        myself: ActorRef<Self::Msg>,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        event!(
            tracing::Level::INFO,
            "Runtime Controller starting API Server"
        );

        // Start the API Server
        let api_startup_args = APIStartupArguments {
            api_config: state.config.api.clone(),
            database_config: state.config.database.clone(),
        };

        let api_server = myself
            .spawn_linked(
                Some("API Server".to_string()),
                APIActor {},
                api_startup_args,
            )
            .await;
        match api_server {
            Ok((actor_ref, _join_handle)) => state.api_server = Some(actor_ref),
            Err(error) => event!(
                tracing::Level::ERROR,
                "API Server could not be started - {:#?}",
                error
            ),
        }

        Ok(())
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match message {
            // ControllerMessage::GetStatus(reply) => {
            //     let db_status = check_db_health(state.config.database.url.clone()).await;

            //     let status = format!(
            //         "Controller Status: Running\nAPI Server: {}\nDatabase: {}",
            //         if state.api_server.is_some() {
            //             "Running"
            //         } else {
            //             "Stopped"
            //         },
            //         db_status
            //     );
            //     let _ = reply.send(status);
            // }
            ControllerMessage::GetConfig(reply) => {
                let config_str = format!("{:#?}", state.config);
                let _ = reply.send(config_str);
            }
            // ControllerMessage::GetDatabaseStatus(reply) => {
            //     let db_status = check_db_health(state.config.database.url.clone()).await;

            //     let status = format!("Database: {}", db_status);
            //     let _ = reply.send(status);
            // }
            ControllerMessage::ReloadApiServer => {
                event!(tracing::Level::INFO, "Reloading API server...");
                if let Some(api_ref) = &state.api_server {
                    api_ref.stop(None);
                    // Restart logic would go here
                }
            }
            ControllerMessage::RestartApiServer => {
                event!(tracing::Level::INFO, "Restarting API server...");
                if let Some(api_ref) = &state.api_server {
                    api_ref.stop(None);
                    // Restart logic would go here
                    // You might want to implement this as a separate function
                }
            }
            ControllerMessage::ReloadCertificates => {
                event!(tracing::Level::INFO, "Reloading certificates...");
                // Implement certificate reload logic
            }
            ControllerMessage::ReloadConfig => {
                event!(tracing::Level::INFO, "Reloading configuration...");
                // Reload configuration from file
                state.config = server_config::Config::load("./config/config.toml");
            }
            ControllerMessage::RecreateDatabase => {
                event!(tracing::Level::WARN, "Recreating database...");
                // Implement database recreation logic
                // This should be very careful as it destroys data
            }
            ControllerMessage::RunMigrations => {
                event!(tracing::Level::INFO, "Running database migrations...");
                // Implement migration logic
            }
            ControllerMessage::SetLogLevel(level) => {
                event!(tracing::Level::INFO, "Setting log level to: {:?}", level);
                // You might need to implement dynamic log level changing
                // This is more complex and might require rebuilding the tracing subscriber
            }
        }
        Ok(())
    }
}
