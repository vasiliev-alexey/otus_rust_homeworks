use client::client::BankClient;
use log::info;
use std::error::Error;

use shared::constants::{LOG_LEVEL, SERVER_ADDRESS};

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the logger based on the environment variable `LOG_LEVEL`.
    env_logger::init_from_env(env_logger::Env::default().default_filter_or(LOG_LEVEL));
    // Connect to the bank server.
    let mut client = BankClient::connect(SERVER_ADDRESS)?;
    // Create an account with the name "Hello".
    let transaction_id = client.create_account("Hello")?;
    info!(
        "successfully created account with transaction id: {}",
        transaction_id
    );
    client.shutdown();
    Ok(())
}
