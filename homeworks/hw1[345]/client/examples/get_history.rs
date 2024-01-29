use client::client::BankClient;
use log::info;
use std::error::Error;

use shared::constants::{LOG_LEVEL, SERVER_ADDRESS};
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the logger based on the environment variable `LOG_LEVEL`.
    env_logger::init_from_env(env_logger::Env::default().default_filter_or(LOG_LEVEL));

    // Connect to the bank server.
    let mut client = BankClient::connect(SERVER_ADDRESS).await?;

    // Check if there was an error connecting to the server.
    info!("Successfully connected to the bank server");

    // Create two accounts: "Alice" and "Bob".
    client.create_account("Alice").await?;

    client.create_account("Bob").await?;

    // Deposit 100.0 into the account with the name "Alice".
    client.deposit("Alice", 100.0).await?;

    // Transfer 25.0 from "Alice" to "Bob".
    client.transfer("Alice", "Bob", 25.0).await?;

    // Get the transaction history.
    let history = client.get_history().await?;

    // Print each transaction in the history.
    history
        .iter()
        .for_each(|transaction| info!("{:?}", transaction));
    client.shutdown().await;
    Ok(())
}
