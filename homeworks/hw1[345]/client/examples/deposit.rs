use client::client::BankClient;
use log::info;
use std::error::Error;

use shared::constants::{LOG_LEVEL, SERVER_PATH};

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the logger based on the environment variable `LOG_LEVEL`.
    env_logger::init_from_env(env_logger::Env::default().default_filter_or(LOG_LEVEL));
    // Connect to the bank server.
    let mut client = BankClient::connect(SERVER_PATH)?;
    // Check if there was an error connecting to the server.
    info!("Successfully connected to the bank server");
    // Create an account with the name "Hello".
    let _ = client.create_account("Hello")?;
    let deposit_transaction_id = client.deposit("Hello", 100.0)?;
    info!(
        "successfully deposited with transaction id: {}",
        deposit_transaction_id
    );
    Ok(())
}
