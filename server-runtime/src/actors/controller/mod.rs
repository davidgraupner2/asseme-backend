pub mod arguments;
pub mod messages;
mod state;

use std::error;

use crate::{
    actors::controller::{
        arguments::ControllerArguments, messages::ControllerMessage, state::ControllerState,
    },
    logging::initialise_logging,
    RuntimeProperties,
};
use ractor::{Actor, ActorProcessingErr, ActorRef};
use tracing::{error, info, instrument, trace, warn};

#[derive(Debug)]
pub struct Controller;

impl Actor for Controller {
    type State = ControllerState;
    type Msg = ControllerMessage;
    type Arguments = ControllerArguments;

    // Invoked when the controller is being started
    // Panics in pre_start do not invoke the supervision strategy and the actor won’t be started
    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        arguments: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        // Initialize Rustls crypto provider
        rustls::crypto::ring::default_provider()
            .install_default()
            .map_err(|_| ())
            .ok(); // Ignore error if already installed

        // Initialise our state
        let mut state = ControllerState::new();

        println!("Arguments: {:#?}", arguments);

        // Setup Logging
        let tracing_worker_guards = initialise_logging(
            RuntimeProperties::global().folders().logs(),
            &format!("{}.log", RuntimeProperties::global().app_name()),
            &arguments.log_format,
            &arguments.log_output,
        );

        state.tracing_worker_guards = tracing_worker_guards;

        println!("{:#?}", state.tracing_worker_guards);
        eprintln!("guards.len = {}", state.tracing_worker_guards.len());

        info!("Info Here");
        warn!("Warining here");
        error!("Error Heer");
        trace!("Trce here");

        std::thread::sleep(std::time::Duration::from_millis(200));

        Ok(state)
    }

    // TODO Documentation
    // Invoked after an actor has started.
    // Any post initialization can be performed here, such as writing to a log file, emitting metrics.
    #[instrument(name = "Controller_Post_Start", level = "trace")]
    async fn post_start(
        &self,
        myself: ActorRef<Self::Msg>,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        info!("{:?} Started Successfully", myself.get_name());

        Ok(())
    }
}
