use log::{debug, error, info};
use std::io::Read;
use std::net::{Shutdown, TcpListener, TcpStream};

use bank_engine::bank::{Bank, BankError, BankTrait, Operation};
use shared::constants::{LOG_LEVEL, MAX_CHUNK_BYTE_SIZE, SERVER_ADDRESS};
use shared::models::{
    DepositParams, GetBalanceAccountRequestParams, Request, RequestPayload, Response,
    ResponsePayload, ResponseResult, TransferParams, WithdrawParams,
};
use RequestPayload::*;

fn try_accept(listener: &TcpListener) -> Option<TcpStream> {
    match listener.accept() {
        Ok((stream, addr)) => {
            println!("Accepted connection with {}", addr);
            Some(stream)
        }
        Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => None,
        Err(e) => {
            println!("Failed to accept a connection: {}", e);
            None
        }
    }
}

/// The main function of the program.
///
/// It initializes the logging, creates a new `Bank` object, binds a TCP listener to the specified server path,
/// and starts accepting incoming connections. For each incoming connection, it handles the client by calling
/// the `handle_client` function.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or(LOG_LEVEL));

    let mut bank: Bank = Bank::new();
    let listener = TcpListener::bind(SERVER_ADDRESS).unwrap();
    info!(
        "Server listening on port {}",
        SERVER_ADDRESS.split(':').nth(1).unwrap_or_default()
    );

    listener.set_nonblocking(true).unwrap();
    let mut connections = Vec::new();
    let mut broken_connections = Vec::new();
    loop {
        if let Some(stream) = try_accept(&listener) {
            stream.set_nonblocking(true).unwrap();
            connections.push(stream);
        }
        for (i, stream) in connections.iter_mut().enumerate() {
            match handle_client_requests(&mut bank, stream.try_clone().unwrap()) {
                Ok(_) => {}
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {}
                Err(e) => {
                    if !e.to_string().contains("Resource temporarily unavailable") {
                        error!("{}", e);
                        broken_connections.push(i);
                    }
                }
            }
        }

        for &idx in &broken_connections {
            connections.swap_remove(idx);
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
/// * `stream` - A mutable reference to a `TcpStream` object.
///
/// ```
fn handle_client_requests(bank: &mut impl BankTrait, mut stream: TcpStream) -> std::io::Result<()> {
    loop {
        let mut received: Vec<u8> = vec![];
        let mut chunk = [0u8; MAX_CHUNK_BYTE_SIZE];
        loop {
            let bytes_read = stream.read(&mut chunk)?;
            received.extend_from_slice(&chunk[..bytes_read]);
            if bytes_read < MAX_CHUNK_BYTE_SIZE {
                break;
            }
        }

        if received.is_empty() {
            return Ok(());
        }
        debug!("handling client");
        let req = serde_json::from_slice::<Request>(received.as_slice());

        if let Err(err) = req {
            let resp = Response {
                payload: ResponsePayload::DeserializeError(err.to_string()),
            };
            error!("Deserialize error: {:?}", err);
            resp.send(&mut stream)?;
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Deserialize error",
            ));
        }
        let req = req.unwrap();

        let resp = match &req.payload {
            OpenAccount(data) => create_account(bank, &data.account),
            Deposit(params) => process_deposit(bank, params),
            Withdraw(data) => process_withdraw(bank, data),
            Transfer(params) => process_transfer(bank, params),
            Ping => process_ping(),
            GetBalance(params) => process_get_balance(bank, params),
            GetHistoryForAccount(account) => process_get_history_for_account(bank, account),
            GetHistory() => process_get_history(bank),
            CloseConnection => {
                info!("Closing connection with {}", stream.peer_addr()?);
                stream.shutdown(Shutdown::Both)?;
                return Ok(());
            }
        }?;
        resp.send(&mut stream)?;
    }
}

/// Process the request to open a new account with the specified account name in the bank and send the response over the TCP stream.
///
/// # Arguments
/// * `bank` - A mutable reference to the `Bank` object.
/// * `account` - The name of the account to be opened.
fn create_account(bank: &mut impl BankTrait, account: &String) -> ResponseResult {
    info!("open account {}", account);
    match bank.create_account(account) {
        Ok(result) => Ok(Response {
            payload: ResponsePayload::AccountCreated(result),
        }),
        Err(error_message) => Ok(Response {
            payload: ResponsePayload::Error(error_message.to_string()),
        }),
    }
}

/// Process the deposit request for the specified account and amount in the bank and send the response over the TCP stream.
///
/// # Arguments
/// * `bank` - A mutable reference to the `Bank` object.
/// * `deposit_params` - The deposit parameters containing the account and amount to deposit.
fn process_deposit(bank: &mut impl BankTrait, deposit_params: &DepositParams) -> ResponseResult {
    info!(
        "process deposit for account {}  and amount {}",
        deposit_params.account, deposit_params.account
    );
    match bank.deposit(deposit_params.account.as_str(), deposit_params.amount) {
        Ok(res) => Ok(Response {
            payload: ResponsePayload::DepositSuccess(res),
        }),
        Err(error_message) => Ok(Response {
            payload: ResponsePayload::DepositError(error_message.to_string()),
        }),
    }
}

/// Process the withdrawal request from the specified account and send the response over the TCP stream.
///
/// # Arguments
/// * `bank` - A mutable reference to the `Bank` object.
/// * `withdraw_params` - The withdrawal parameters containing the account and amount to withdraw.
fn process_withdraw(bank: &mut impl BankTrait, withdraw_params: &WithdrawParams) -> ResponseResult {
    info!("process withdraw for account {}", withdraw_params.account);
    match bank.withdraw(withdraw_params.account.as_str(), withdraw_params.amount) {
        Ok(res) => Ok(Response {
            payload: ResponsePayload::WithdrawSuccess(res),
        }),
        Err(error_message) => {
            if let BankError::InsufficientFunds(info) = &error_message {
                Ok(Response {
                    payload: ResponsePayload::WithdrawalError(info.to_string()),
                })
            } else {
                Ok(Response {
                    payload: ResponsePayload::Error(error_message.to_string()),
                })
            }
        }
    }
}

/// Process the transfer request from the sender account to the receiver account with the specified amount, and send the response over the TCP stream.
///
/// # Arguments
/// * `bank` - A mutable reference to the `Bank` object.
/// * `params` - The transfer parameters containing the sender account, receiver account, and amount to transfer.
fn process_transfer(bank: &mut impl BankTrait, params: &TransferParams) -> ResponseResult {
    info!(
        "process transfer from account {} to account {} and amount {}",
        params.sender_account, params.receiver_account, params.amount
    );
    match bank.transfer(
        params.sender_account.as_str(),
        params.receiver_account.as_str(),
        params.amount,
    ) {
        Ok(transaction_id) => Ok(Response {
            payload: ResponsePayload::TransferSuccess(transaction_id),
        }),
        Err(message) => Ok(Response {
            payload: ResponsePayload::Error(message.to_string()),
        }),
    }
}

/// Process the request to get the balance of the specified account from the bank and send the response over the TCP stream.
///
/// # Arguments
/// * `bank` - A mutable reference to the `Bank` object.
/// * `get_balance_params` - The parameters for the get balance request containing the account to get the balance for.
fn process_get_balance(
    bank: &mut impl BankTrait,
    get_balance_params: &GetBalanceAccountRequestParams,
) -> ResponseResult {
    info!(
        "process balance for account {} ",
        get_balance_params.account
    );

    match bank.get_balance(get_balance_params.account.as_str()) {
        Ok(balance) => Ok(Response {
            payload: ResponsePayload::Balance(balance),
        }),
        Err(error_message) => Ok(Response {
            payload: ResponsePayload::Error(error_message.to_string()),
        }),
    }
}
/// Process the request to get the transaction history from the bank and send the response over the TCP stream.
///
/// # Arguments
/// * `bank` - A mutable reference to the `Bank` object.
///
/// # Returns
/// ()
///```

fn process_get_history(bank: &mut impl BankTrait) -> ResponseResult {
    info!("process history for hw1[345]",);

    match bank.get_history() {
        Ok(history) => Ok(Response {
            payload: ResponsePayload::History(
                history.iter().copied().map(|o| (*o).clone()).collect(),
            ),
        }),
        Err(error_message) => Ok(Response {
            payload: ResponsePayload::Error(error_message.to_string()),
        }),
    }
}

/// Process the request to get the transaction history for the specified account from the bank and send the response over the TCP stream.
///
/// # Arguments
/// * `bank` - A mutable reference to the `Bank` object.
/// * `account` - The name of the account to get the transaction history for.
fn process_get_history_for_account(bank: &mut impl BankTrait, account: &String) -> ResponseResult {
    info!("process history for account {account}");
    let hist_result = bank.get_account_history(account);
    match hist_result {
        Ok(hist) => Ok(Response {
            payload: ResponsePayload::History(
                hist.iter()
                    .map(|o| (*o).clone())
                    .collect::<Vec<Operation>>(),
            ),
        }),
        Err(error_message) => Ok(Response {
            payload: ResponsePayload::Error(error_message.to_string()),
        }),
    }
}

/// Process the ping request by sending a handshake response over the TCP stream.
///
/// # Arguments
///
fn process_ping() -> ResponseResult {
    Ok(Response {
        payload: ResponsePayload::HandShakeEstablished,
    })
}
