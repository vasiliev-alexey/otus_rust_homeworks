use client::client::BankClient;
use log::{debug, error, info};
use rand::Rng;
use std::error::Error;

use shared::constants::{LOG_LEVEL, SERVER_ADDRESS};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or(LOG_LEVEL));
    let mut hanlers = vec![];
    const NUM_THREADS: usize = 5;
    for i in 0..NUM_THREADS {
        hanlers.push(tokio::spawn(async move {
            let connect_result = BankClient::connect(SERVER_ADDRESS).await;

            if let Err(msg) = connect_result {
                error!("Connection error: {}", msg);
                return;
            }
            let mut client = connect_result.unwrap();

            info!("Successfully connected to the bank server");
            let client_name = format!("Client{i}");

            client.create_account(client_name.as_str()).await.unwrap();
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            let mut n = 100;
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                client.deposit(client_name.as_str(), 100.0).await.unwrap();

                client.withdraw(client_name.as_str(), 50.0).await.unwrap();

                let rec_num = rand::thread_rng().gen_range(0..NUM_THREADS);
                if rec_num == i {
                    continue;
                }
                let rec_client_name = format!("Client{rec_num}");
                let _trans_id = client
                    .transfer(client_name.as_str(), rec_client_name.as_str(), 10.0)
                    .await
                    .unwrap();

                n -= 1;
                if n == 0 {
                    return;
                }
            }
        }));
    }
    for x in hanlers {
        x.await.unwrap();
    }

    Ok(())
}
