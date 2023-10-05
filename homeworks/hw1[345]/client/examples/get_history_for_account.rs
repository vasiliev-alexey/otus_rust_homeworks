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

    let history = client.get_history_for_account("Alice");

    if history.is_err() {
        println!("History: {:?}", history.err().unwrap());
        return;
    }
    let history = history.unwrap();

    for his in history {
        info!("{:?}", his);
    }
}
