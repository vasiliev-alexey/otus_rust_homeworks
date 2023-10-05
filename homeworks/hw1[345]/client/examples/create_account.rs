use client::BankClient;
use log::{debug, error, info};
use shared::{OpenAccountRequestParams, Request, RequestPayload};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

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
    let _ = client.create_account("Hello");
}
// fn main_2() {
//     env_logger::init_from_env(env_logger::Env::default().default_filter_or(LOG_LEVEL));
//
//     let data_req = Request {
//         payload: RequestPayload::OpenAccount(OpenAccountRequestParams {
//             account: "Hello".to_string(),
//         }),
//     };
//     let json = serde_json::to_string(&data_req).unwrap();
//     debug!("{}", json);
//
//     match TcpStream::connect(SERVER_PATH) {
//         Ok(mut stream) => {
//             debug!("Successfully connected to server  {}", SERVER_PATH);
//             let _ = stream.write(json.as_bytes()).unwrap();
//
//             let mut data = vec![];
//
//             match stream.read_to_end(&mut data) {
//                 Ok(size) => {
//                     debug!("Read byte from responce: {}", size);
//                     if from_utf8(&data).unwrap().trim_end() == json {
//                         debug!("Reply is ok!");
//                         debug!("{}", from_utf8(&data).unwrap());
//                     } else {
//                         let text = from_utf8(&data).unwrap();
//                         debug!("Unexpected reply: {}", text);
//                     }
//                 }
//                 Err(e) => {
//                     error!("Failed to receive data: {}", e);
//                 }
//             }
//         }
//         Err(e) => {
//             error!("Failed to connect: {}", e);
//         }
//     }
//     info!("Terminated.");
// }
