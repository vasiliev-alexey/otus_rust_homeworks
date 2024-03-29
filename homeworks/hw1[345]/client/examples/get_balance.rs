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

    info!("Successfully connected to the bank server");

    // Create two accounts: "Alice" and "Bob".
    let _ = client.create_account("Alice").await?;
    let _ = client.create_account("Bob").await?;

    // Deposit 100.0 into the account with the name "Alice".
    let _deposit_trid = client.deposit("Alice", 100.0).await?;

    let _ = client.transfer("Alice", "Bob", 25.0).await?;

    // Get the balances of "Alice" and "Bob".
    let alice_balance = client.get_balance("Alice").await?;
    let bob_balance = client.get_balance("Bob").await?;

    // Assert that the balances are correct.
    assert_eq!(alice_balance, 75.0);
    assert_eq!(bob_balance, 25.0);

    // Log the balances of "Alice".
    info!("Alice balance: {}", alice_balance);
    client.shutdown().await;
    Ok(())
}
