use ractor::Actor;
use server_runtime::controller::Controller;
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load any environment variables from a .env file
    match dotenvy::dotenv() {
        Ok(_) => {}
        Err(_) => {
            println!("WARNING: .env file not found, assuming env variables are already configured!")
        }
    }

    // Start the runtime controller
    let (_actor, _actor_handle) = Actor::spawn(None, Controller, ())
        .await
        .expect("Controller failed to start");

    // Wait here until we receive a CTRL-C Signal break
    signal::ctrl_c().await.expect("Failed to wait");

    Ok(())
}
