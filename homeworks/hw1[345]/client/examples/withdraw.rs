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

    // Create an account with the name "Hello".
    if let Err(err) = client.create_account("Hello") {
        error!("Failed to create account: {}", err);
        return;
    }

    // Deposit 100.0 into the account with the name "Hello".
    if let Err(err) = client.deposit("Hello", 100.0) {
        error!("Failed to deposit amount: {}", err);
        return;
    }

    // Withdraw 50.0 from the account with the name "Hello".
    if let Err(err) = client.withdraw("Hello", 50.0) {
        error!("Failed to withdraw amount: {}", err);
        return;
    }

    // Withdraw 80.0 from the account with the name "Hello".
    if let Err(err) = client.withdraw("Hello", 80.0) {
        error!("Failed to withdraw amount: {}", err);
    }
}
