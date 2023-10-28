use client::client::BankClient;
use log::{error, info};
use rand::Rng;
use std::error::Error;

use shared::constants::{LOG_LEVEL, SERVER_ADDRESS};

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or(LOG_LEVEL));
    let mut hanlers = vec![];
    for i in 0..5 {
        hanlers.push(std::thread::spawn(move || {
            let connect_result = BankClient::connect(SERVER_ADDRESS);

            if let Err(msg) = connect_result {
                error!("Connection error: {}", msg);
                return;
            }
            let mut client = connect_result.unwrap();

            info!("Successfully connected to the bank server");
            let client_name = format!("Client{i}");

            client.create_account(client_name.as_str()).unwrap();
            std::thread::sleep(std::time::Duration::from_secs(i));
            let mut n = 100;
            loop {
                std::thread::sleep(std::time::Duration::from_secs(10));
                client.deposit(client_name.as_str(), 100.0).unwrap();

                client.withdraw(client_name.as_str(), 50.0).unwrap();

                let rec_num = rand::thread_rng().gen_range(0..5);
                let rec_client_name = format!("Client{rec_num}");
                match client.transfer(client_name.as_str(), rec_client_name.as_str(), 10.0) {
                    Ok(..) => {}
                    Err(e) => {
                        error!("Error {}: {}", i, e);
                    }
                }

                n -= 1;
                if n == 0 {
                    return;
                }
            }
        }));
    }
    for x in hanlers {
        x.join().unwrap();
    }

    Ok(())
}
