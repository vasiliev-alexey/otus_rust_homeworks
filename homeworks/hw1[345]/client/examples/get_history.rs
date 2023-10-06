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

    // Get the transaction history.
    let history = client.get_history();

    // Check if there was an error getting the history.
    if let Err(err) = &history {
        println!("Failed to get transaction history: {}", err);
        return;
    }

    // Unwrap the history from the `Result`.
    let history = history.unwrap();

    // Print each transaction in the history.
    for transaction in history {
        info!("{:?}", transaction);
    }
}
