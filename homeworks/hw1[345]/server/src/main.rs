use std::io::Read;
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str::from_utf8;

use log::{debug, error, info};

use bank_engine::bank::{Bank, BankError, BankTrait, Operation};
use shared::constants::{BUFFER_SIZE, SERVER_PATH};
use shared::models::ResponsePayload::TransferSuccess;
use shared::models::ResponsePayload::{Error, History};
use shared::models::{
    DepositParams, GetBalanceAccountRequestParams, Request, RequestPayload, Response,
    ResponsePayload, ResponseResult, TransferParams, WithdrawParams,
};
use RequestPayload::*;

/// The main function of the program.
///
/// It initializes the logging, creates a new `Bank` object, binds a TCP listener to the specified server path,
/// and starts accepting incoming connections. For each incoming connection, it handles the client by calling
/// the `handle_client` function.
fn main() -> Result<(), Box<dyn std::error::Error>> {
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
                debug!("New connection incoming: {}", stream.peer_addr()?);
                handle_client_requests(&mut bank, stream)?;
            }
            Err(e) => {
                error!("Error: {}", e);
            }
        }
    }
    Ok(())
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
fn handle_client_requests(
    bank: &mut impl BankTrait,
    mut stream: TcpStream,
) -> Result<(), Box<dyn std::error::Error>> {
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
                        info!("Closing connection with {}", stream.peer_addr().unwrap());
                        stream.shutdown(Shutdown::Both).unwrap();
                        break;
                    }
                    #[allow(unreachable_patterns)]
                    _ => process_unknown(&req.payload),
                }?;
                resp.send(&mut stream)?;
            }
            Err(_) => {
                error!(
                    "An error occurred, terminating connection with {}",
                    stream.peer_addr().unwrap()
                );
                stream.shutdown(Shutdown::Both).unwrap();
            }
        };
    }
    Ok(())
}

/// Process the request to open a new account with the specified account name in the bank and send the response over the TCP stream.
///
/// # Arguments
/// * `bank` - A mutable reference to the `Bank` object.
/// * `account` - The name of the account to be opened.
fn create_account(bank: &mut impl BankTrait, account: &String) -> ResponseResult {
    info!("open account {}", account);

    let result = bank.create_account(account);

    if let Err(error_message) = result {
        return Ok(Response {
            payload: ResponsePayload::Error(error_message.to_string()),
        });
    }
    Ok(Response {
        payload: ResponsePayload::AccountCreated(result.unwrap()),
    })
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
    let deposit_result = bank.deposit(deposit_params.account.as_str(), deposit_params.amount);

    if let Err(error_message) = deposit_result {
        Ok(Response {
            payload: ResponsePayload::Error(error_message.to_string()),
        })
    } else {
        Ok(Response {
            payload: ResponsePayload::DepositSuccess(deposit_result.unwrap()),
        })
    }
}

/// Process the withdrawal request from the specified account and send the response over the TCP stream.
///
/// # Arguments
/// * `bank` - A mutable reference to the `Bank` object.
/// * `withdraw_params` - The withdrawal parameters containing the account and amount to withdraw.
fn process_withdraw(bank: &mut impl BankTrait, withdraw_params: &WithdrawParams) -> ResponseResult {
    info!("process withdraw for account {}", withdraw_params.account);
    let withdraw_result = bank.withdraw(withdraw_params.account.as_str(), withdraw_params.amount);

    if let Err(msg) = withdraw_result {
        // Process insufficient funds
        if let BankError::InsufficientFunds(info) = &msg {
            Ok(Response {
                payload: ResponsePayload::WithdrawalError(info.to_string()),
            })
        } else {
            Ok(Response {
                payload: ResponsePayload::Error(msg.to_string()),
            })
        }
    } else {
        Ok(Response {
            payload: ResponsePayload::WithdrawSuccess(withdraw_result.unwrap()),
        })
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
    let transfer_result = bank.transfer(
        params.sender_account.as_str(),
        params.receiver_account.as_str(),
        params.amount,
    );

    if let Err(message) = transfer_result {
        Ok(Response {
            payload: Error(message.to_string()),
        })
    } else {
        Ok(Response {
            payload: TransferSuccess(transfer_result.unwrap()),
        })
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
    let account_balance = bank.get_balance(get_balance_params.account.as_str());

    if let Ok(balance) = account_balance {
        Ok(Response {
            payload: ResponsePayload::Balance(balance),
        })
    } else {
        Ok(Response {
            payload: ResponsePayload::Error(account_balance.unwrap_err().to_string()),
        })
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
    let hist_result = bank.get_history();

    if let Ok(history) = hist_result {
        Ok(Response {
            payload: History(history.iter().copied().map(|o| (*o).clone()).collect()),
        })
    } else {
        Ok(Response {
            payload: Error(hist_result.unwrap_err().to_string()),
        })
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

    if let Ok(hist) = hist_result {
        Ok(Response {
            payload: History(
                hist.iter()
                    .map(|o| (*o).clone())
                    .collect::<Vec<Operation>>(),
            ),
        })
    } else {
        Ok(Response {
            payload: Error(hist_result.unwrap_err().to_string()),
        })
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

/// Process the unknown request by sending an error response over the TCP stream.
///
/// # Arguments
/// req_payload: The request payload
///
fn process_unknown(req_payload: &RequestPayload) -> ResponseResult {
    Ok(Response {
        payload: ResponsePayload::Error(format!("unknown request {:?}", req_payload)),
    })
}
