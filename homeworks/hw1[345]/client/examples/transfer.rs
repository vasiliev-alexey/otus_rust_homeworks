use client::client::BankClient;
use log::info;
use std::error::Error;

use shared::constants::{LOG_LEVEL, SERVER_ADDRESS};

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the logger based on the environment variable `LOG_LEVEL`.
    env_logger::init_from_env(env_logger::Env::default().default_filter_or(LOG_LEVEL));

    // Connect to the bank server.
    let mut client = BankClient::connect(SERVER_ADDRESS)?;

    info!("Successfully connected to the bank server");

    // Create two accounts: "Alice" and "Bob".
    client.create_account("Alice")?;

    client.create_account("Bob")?;

    // Deposit 100.0 into the account with the name "Alice".
    client.deposit("Alice", 100.0)?;

    // Transfer 100.0 from "Alice" to "Bob".
    client.transfer("Alice", "Bob", 100.0)?;
    Ok(())
}
