use ractor::Actor;
use server_runtime::controller::Controller;
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Start the runtime controller
    let (actor, _actor_handle) = Actor::spawn(None, Controller, ())
        .await
        .expect("Controller failed to start");

    // Wait here until we receive a CTRL-C Signal break
    signal::ctrl_c().await.expect("Failed to wait");

    Ok(())
}
