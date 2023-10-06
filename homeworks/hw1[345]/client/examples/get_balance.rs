use client::BankClient;
use log::{error, info};

use shared::constants::{LOG_LEVEL, SERVER_PATH};

fn main() {
    // Initialize the logger based on the environment variable `LOG_LEVEL`.
    env_logger::init_from_env(env_logger::Env::default().default_filter_or(LOG_LEVEL));

    // Connect to the bank server.
    let client = BankClient::connect(SERVER_PATH);

    // Check if there was an error connecting to the server.
    if let Err(err) = &client {
        error!("Failed to connect: {}", err);
        return;
    } else {
        info!("Successfully connected to the bank server");
    }

    // Unwrap the client from the `Result`.
    let mut client = client.unwrap();

    // Create two accounts: "Alice" and "Bob".
    if let Err(err) = client.create_account("Alice") {
        error!("Failed to create account: {}", err);
        return;
    }
    if let Err(err) = client.create_account("Bob") {
        error!("Failed to create account: {}", err);
        return;
    }

    // Deposit 100.0 into the account with the name "Alice".
    if let Err(err) = client.deposit("Alice", 100.0) {
        error!("Failed to deposit amount: {}", err);
        return;
    }

    // Transfer 25.0 from "Alice" to "Bob".
    if let Err(err) = client.transfer("Alice", "Bob", 25.0) {
        error!("Failed to transfer amount: {}", err);
        return;
    }

    // Get the balances of "Alice" and "Bob".
    let alice_balance = client.get_balance("Alice").unwrap();
    let bob_balance = client.get_balance("Bob").unwrap();

    // Assert that the balances are correct.
    assert_eq!(alice_balance, 75.0);
    assert_eq!(bob_balance, 25.0);

    // Log the balances of "Alice".
    info!("Alice balance: {}", alice_balance);
}
