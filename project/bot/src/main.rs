mod bot_service;
mod config_service;
mod data_service;

use log::debug;
use std::error::Error;

use crate::bot_service::run_bot;
use crate::config_service::read_config;
use crate::data_service::Service;

use tokio::task;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();
    log::info!("Starting buttons bot...");

    let config = read_config()?;
    let serv = Service::new(&config);
    let data_serv = serv.clone();

    debug!(
        "load data to redis from source file: {}",
        config.source_file
    );

    task::spawn(async move {
        let _ = serv.clone().load_sched();
    });

    run_bot(data_serv).await
}
