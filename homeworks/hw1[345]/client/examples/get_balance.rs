use client::BankClient;
use log::{debug, error, info};

use shared::constants::{LOG_LEVEL, SERVER_PATH};

fn main() {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or(LOG_LEVEL));

    let client = BankClient::connect(SERVER_PATH);

    if client.is_err() {
        error!("Failed to connect: {}", &client.err().unwrap());
        return;
    } else {
        info!("Successfully connected");
    }
    let mut client = client.unwrap();
    let _ = client.create_account("Alice");
    let _ = client.create_account("Bob");
    let _x = client.deposit("Alice", 100.0);
    let _x = client.transfer("Alice", "Bob", 25.0);

    let alice_balance = client.get_balance("Alice").unwrap();
    let bob_balance = client.get_balance("Bob").unwrap();
    assert_eq!(alice_balance, 75.0);
    assert_eq!(bob_balance, 25.0);
    info!("Alice balance: {}", alice_balance);
}
