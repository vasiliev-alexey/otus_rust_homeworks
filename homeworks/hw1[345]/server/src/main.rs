use bank_engine::bank::{Bank, BankTrait};
use log::{debug, error, info};
use shared::constants::SERVER_PATH;
use shared::ResponsePayload::{Error, History};
use shared::{
    DepositParams, GetBalanceAccountRequestParams, Request, RequestPayload, Response,
    ResponsePayload, TransferParams, WithdrawParams,
};
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str::from_utf8;
use ResponsePayload::TransferSuccess;

const BUFFER_SIZE: usize = 1024;

fn handle_client(bank: &mut Bank, mut stream: TcpStream) {
    let mut data = [0; BUFFER_SIZE];

    debug!("handling client");

    loop {
        match stream.read(&mut data) {
            Ok(size) => {
                if size == 0 {
                    // Греем процессор - но пока нет асинхронных операций
                    continue;
                }
                debug!("handling client response {size} bytes");
                debug!("Received {} bytes of data", size);
                debug!("{}", from_utf8(&data[0..size]).unwrap());

                let req =
                    serde_json::from_str::<Request>(from_utf8(&data[0..size]).unwrap()).unwrap();
                match &req.payload {
                    RequestPayload::OpenAccount(data) => {
                        open_account(bank, &mut stream, &data.account)
                    }
                    RequestPayload::Withdraw(data) => process_withdraw(bank, &mut stream, data),
                    RequestPayload::Deposit(params) => process_deposit(bank, &mut stream, params),
                    RequestPayload::Transfer(params) => process_transfer(bank, &mut stream, params),
                    RequestPayload::Ping => process_ping(&mut stream),
                    RequestPayload::GetBalance(params) => {
                        process_get_balance(bank, &mut stream, params)
                    }
                    RequestPayload::GetHistoryForAccount(account) => {
                        process_get_history_for_account(bank, &mut stream, account)
                    }
                    RequestPayload::GetHistory() => process_get_history(bank, &mut stream),
                    RequestPayload::CloseConnection => {
                        info!("Closing connection with {}", stream.peer_addr().unwrap());
                        stream.shutdown(Shutdown::Both).unwrap();
                        break;
                    }
                    #[allow(unreachable_patterns)]
                    _ => process_unknown(&mut stream),
                }
                debug!("Load request: {:?}", &req);
            }
            Err(_) => {
                error!(
                    "An error occurred, terminating connection with {}",
                    stream.peer_addr().unwrap()
                );
                stream.shutdown(Shutdown::Both).unwrap();
            }
        }
    }
}

fn process_get_history_for_account(bank: &mut Bank, stream: &mut TcpStream, account: &String) {
    info!("process history for account {account}");
    let hist_result = bank.get_account_history(account);

    if let Ok(hist) = hist_result {
        let data_req = Response {
            payload: History(hist),
        };
        let json = serde_json::to_string(&data_req).unwrap();
        stream.write_all(json.as_bytes()).unwrap()
    } else {
        let data_req = Response {
            payload: Error(hist_result.unwrap_err().to_string()),
        };
        let json = serde_json::to_string(&data_req).unwrap();
        stream.write_all(json.as_bytes()).unwrap();
    }
}

fn process_transfer(bank: &mut Bank, stream: &mut TcpStream, params: &TransferParams) {
    info!(
        "process transfer from account {} to account {} and amount {}",
        params.sender_account, params.receiver_account, params.amount
    );
    let transfer_result = bank.transfer(
        params.sender_account.as_str(),
        params.receiver_account.as_str(),
        params.amount,
    );

    if let Err(message) = transfer_result {
        let data_req = Response {
            payload: Error(message.to_string()),
        };
        let json = serde_json::to_string(&data_req).unwrap();
        stream.write_all(json.as_bytes()).unwrap()
    } else {
        let data_req = Response {
            payload: TransferSuccess,
        };
        let json = serde_json::to_string(&data_req).unwrap();
        stream.write_all(json.as_bytes()).unwrap()
    }

    // if transfer_result.is_ok() {
    //     let data_req = Response {
    //         payload: TransferSuccess,
    //     };
    //     let json = serde_json::to_string(&data_req).unwrap();
    //     stream.write_all(json.as_bytes()).unwrap();
    // } else {
    //     let data_req = Response {
    //         payload: Error(transfer_result.unwrap_err().to_string()),
    //     };
    //     let json = serde_json::to_string(&data_req).unwrap();
    //     stream.write_all(json.as_bytes()).unwrap();
    // }
}

fn process_get_balance(
    bank: &mut Bank,
    stream: &mut TcpStream,
    get_balance_params: &GetBalanceAccountRequestParams,
) {
    info!(
        "process balance for account {} ",
        get_balance_params.account
    );
    let account_balance = bank.get_balance(get_balance_params.account.as_str());

    if let Ok(balance) = account_balance {
        let data_req = Response {
            payload: ResponsePayload::Balance(balance),
        };
        let json = serde_json::to_string(&data_req).unwrap();
        stream.write_all(json.as_bytes()).unwrap()
    } else {
        let data_req = Response {
            payload: ResponsePayload::Error(account_balance.unwrap_err().to_string()),
        };
        let json = serde_json::to_string(&data_req).unwrap();
        stream.write_all(json.as_bytes()).unwrap();
    }
}

fn process_get_history(bank: &mut Bank, stream: &mut TcpStream) {
    info!("process history for hw1[345]",);
    let hist_result = bank.get_history();

    if let Ok(history) = hist_result {
        let data_req = Response {
            payload: History(history),
        };
        let json = serde_json::to_string(&data_req).unwrap();
        stream.write_all(json.as_bytes()).unwrap()
    } else {
        let data_req = Response {
            payload: Error(hist_result.unwrap_err().to_string()),
        };
        let json = serde_json::to_string(&data_req).unwrap();
        stream.write_all(json.as_bytes()).unwrap();
    }
}

fn process_deposit(bank: &mut Bank, stream: &mut TcpStream, deposit_params: &DepositParams) {
    info!(
        "process deposit for account {}  and amount {}",
        deposit_params.account, deposit_params.account
    );
    let data_req = Response {
        payload: ResponsePayload::DepositSuccess,
    };
    let json = serde_json::to_string(&data_req).unwrap();
    debug!("sending: {}", json);
    let deposit_result = bank.deposit(deposit_params.account.as_str(), deposit_params.amount);

    if let Err(error_message) = deposit_result {
        let data_req = Response {
            payload: ResponsePayload::Error(error_message.to_string()),
        };
        let json = serde_json::to_string(&data_req).unwrap();
        stream.write_all(json.as_bytes()).unwrap();
    } else {
        let data_req = Response {
            payload: ResponsePayload::DepositSuccess,
        };
        let json = serde_json::to_string(&data_req).unwrap();
        stream.write_all(json.as_bytes()).unwrap();
    }
}

fn process_unknown(stream: &mut TcpStream) {
    let data_req = Response {
        payload: ResponsePayload::Error("unknown".to_string()),
    };
    let json = serde_json::to_string(&data_req).unwrap();
    stream.write_all(json.as_bytes()).unwrap()
}

fn process_withdraw(bank: &mut Bank, stream: &mut TcpStream, withdraw_params: &WithdrawParams) {
    info!("process withdraw for account {}", withdraw_params.account);
    let withdraw_result = bank.withdraw(withdraw_params.account.as_str(), withdraw_params.amount);
    if let Err(msg) = withdraw_result {
        let data_req = Response {
            payload: ResponsePayload::Error(msg.to_string()),
        };
        let json = serde_json::to_string(&data_req).unwrap();
        stream.write_all(json.as_bytes()).unwrap();
        debug!("send response success");
    } else {
        let data_req = Response {
            payload: ResponsePayload::WithdrawSuccess,
        };
        let json = serde_json::to_string(&data_req).unwrap();
        debug!("send response failure");
        stream.write_all(json.as_bytes()).unwrap();
    }
}

fn main() {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    let mut bank = Bank::new();
    let listener = TcpListener::bind(SERVER_PATH).unwrap();
    info!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                debug!("New connection incoming: {}", stream.peer_addr().unwrap());
                handle_client(&mut bank, stream)
            }
            Err(e) => {
                error!("Error: {}", e);
                break;
            }
        }
    }
    drop(listener);
}

fn open_account(bank: &mut Bank, stream: &mut TcpStream, account: &String) {
    info!("open account {}", account);

    let result = bank.create_account(account);

    if let Err(error_message) = result {
        let data_req = Response {
            payload: ResponsePayload::Error(error_message.to_string()),
        };
        let json = serde_json::to_string(&data_req).unwrap();
        stream.write_all(json.as_bytes()).unwrap();
    } else {
        let data_req = Response {
            payload: ResponsePayload::AccountCreated,
        };
        let json = serde_json::to_string(&data_req).unwrap();
        debug!("sending: {}", json);
        stream.write_all(json.as_bytes()).unwrap();
    }
}

fn process_ping(stream: &mut TcpStream) {
    let data_req = Response {
        payload: ResponsePayload::HandShakeEstablished,
    };
    let json = serde_json::to_string(&data_req).unwrap();
    stream.write_all(json.as_bytes()).unwrap();
}
