use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str::from_utf8;

use log::{debug, error, info};

use bank_engine::bank::{Bank, BankTrait};
use shared::constants::{BUFFER_SIZE, SERVER_PATH};
use shared::models::ResponsePayload::TransferSuccess;
use shared::models::ResponsePayload::{Error, History};
use shared::models::{
    DepositParams, GetBalanceAccountRequestParams, Request, RequestPayload, Response,
    ResponsePayload, TransferParams, WithdrawParams,
};
use RequestPayload::*;

/// The main function of the program.
///
/// It initializes the logging, creates a new `Bank` object, binds a TCP listener to the specified server path,
/// and starts accepting incoming connections. For each incoming connection, it handles the client by calling
/// the `handle_client` function.
fn main() {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    let mut bank: Bank = Bank::new();
    let listener = TcpListener::bind(SERVER_PATH).unwrap();
    info!(
        "Server listening on port {}",
        SERVER_PATH.split(':').nth(1).unwrap_or_default()
    );
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
}

/// Handles a client connection.
///
/// This function takes a mutable reference to a `Bank` object and a `TcpStream` object,
/// and performs some actions to handle the client connection.
///
/// # Arguments
///
/// * `bank` - A mutable reference to a `Bank` object.
/// * `stream` - A `TcpStream` object representing the client connection.
///
/// ```
fn handle_client(bank: &mut impl BankTrait, mut stream: TcpStream) {
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
                    OpenAccount(data) => open_account(bank, &mut stream, &data.account),
                    Deposit(params) => process_deposit(bank, &mut stream, params),
                    Withdraw(data) => process_withdraw(bank, &mut stream, data),
                    Transfer(params) => process_transfer(bank, &mut stream, params),
                    Ping => process_ping(&mut stream),
                    GetBalance(params) => process_get_balance(bank, &mut stream, params),
                    GetHistoryForAccount(account) => {
                        process_get_history_for_account(bank, &mut stream, account)
                    }
                    GetHistory() => process_get_history(bank, &mut stream),
                    CloseConnection => {
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

/// Process the request to open a new account with the specified account name in the bank and send the response over the TCP stream.
///
/// # Arguments
/// * `bank` - A mutable reference to the `Bank` object.
/// * `stream` - A mutable reference to the `TcpStream` object for communication.
/// * `account` - The name of the account to be opened.

fn open_account(bank: &mut impl BankTrait, stream: &mut TcpStream, account: &String) {
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
/// Process the deposit request for the specified account and amount in the bank and send the response over the TCP stream.
///
/// # Arguments
/// * `bank` - A mutable reference to the `Bank` object.
/// * `stream` - A mutable reference to the `TcpStream` object for communication.
/// * `deposit_params` - The deposit parameters containing the account and amount to deposit.
fn process_deposit(
    bank: &mut impl BankTrait,
    stream: &mut TcpStream,
    deposit_params: &DepositParams,
) {
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

/// Process the withdrawal request from the specified account and send the response over the TCP stream.
///
/// # Arguments
/// * `bank` - A mutable reference to the `Bank` object.
/// * `stream` - A mutable reference to the `TcpStream` object for communication.
/// * `withdraw_params` - The withdrawal parameters containing the account and amount to withdraw.

fn process_withdraw(
    bank: &mut impl BankTrait,
    stream: &mut TcpStream,
    withdraw_params: &WithdrawParams,
) {
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
/// Process the transfer request from the sender account to the receiver account with the specified amount, and send the response over the TCP stream.
///
/// # Arguments
/// * `bank` - A mutable reference to the `Bank` object.
/// * `stream` - A mutable reference to the `TcpStream` object for communication.
/// * `params` - The transfer parameters containing the sender account, receiver account, and amount to transfer.

fn process_transfer(bank: &mut impl BankTrait, stream: &mut TcpStream, params: &TransferParams) {
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
}

/// Process the request to get the balance of the specified account from the bank and send the response over the TCP stream.
///
/// # Arguments
/// * `bank` - A mutable reference to the `Bank` object.
/// * `stream` - A mutable reference to the `TcpStream` object for communication.
/// * `get_balance_params` - The parameters for the get balance request containing the account to get the balance for.
fn process_get_balance(
    bank: &mut impl BankTrait,
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
/// Process the request to get the transaction history from the bank and send the response over the TCP stream.
///
/// # Arguments
/// * `bank` - A mutable reference to the `Bank` object.
/// * `stream` - A mutable reference to the `TcpStream` object for communication.
///
/// # Returns
/// ()
///```

fn process_get_history(bank: &mut impl BankTrait, stream: &mut TcpStream) {
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

/// Process the request to get the transaction history for the specified account from the bank and send the response over the TCP stream.
///
/// # Arguments
/// * `bank` - A mutable reference to the `Bank` object.
/// * `stream` - A mutable reference to the `TcpStream` object for communication.
/// * `account` - The name of the account to get the transaction history for.

fn process_get_history_for_account(
    bank: &mut impl BankTrait,
    stream: &mut TcpStream,
    account: &String,
) {
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

/// Process the ping request by sending a handshake response over the TCP stream.
///
/// # Arguments
/// * `stream` - A mutable reference to the `TcpStream` object for communication.

fn process_ping(stream: &mut TcpStream) {
    let data_req = Response {
        payload: ResponsePayload::HandShakeEstablished,
    };
    let json = serde_json::to_string(&data_req).unwrap();
    stream.write_all(json.as_bytes()).unwrap();
}
/// Process an unknown request by sending an error response over the TCP stream.
///
/// This function takes a mutable reference to a `TcpStream` object and sends an error response
/// to the client indicating that the request payload is unknown.
///
/// # Arguments
///
/// * `stream` - A mutable reference to a `TcpStream` object for communication.
///
/// # Example
///
/// ```
/// use std::net::TcpStream;
///
/// let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
/// process_unknown(&mut stream);
/// ```
fn process_unknown(stream: &mut TcpStream) {
    let data_req = Response {
        payload: ResponsePayload::Error("unknown".to_string()),
    };
    let json = serde_json::to_string(&data_req).unwrap();
    stream.write_all(json.as_bytes()).unwrap()
}
