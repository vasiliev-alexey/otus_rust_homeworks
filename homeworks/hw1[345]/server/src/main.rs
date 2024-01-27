use log::{debug, error, info, warn};
use std::io::Read;
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::mpsc;
use std::sync::mpsc::{channel, Receiver, Sender};

use bank_engine::bank::BankResponse::Transaction;
use bank_engine::bank::{Bank, BankError, BankResponse, BankTrait};
use shared::constants::{LOG_LEVEL, MAX_CHUNK_BYTE_SIZE, SERVER_ADDRESS};

use shared::errors::ProcessingErrorsResult;
use shared::errors::ProcessingErrorsResult::TypeMismatchError;
use shared::models::{
    DepositParams, GetBalanceAccountRequestParams, OpenAccountRequestParams, Request,
    RequestPayload, Response, ResponsePayload, ResponseResult, TransferParams, WithdrawParams,
};
use RequestPayload::*;

/// The main function of the program.
///
/// It initializes the logging, creates a new `Bank` object, binds a TCP listener to the specified server path,
/// starts processing thread for Bank
/// and starts accepting incoming connections. For each incoming connection spawns new thread for processing requests.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or(LOG_LEVEL));

    let listener = TcpListener::bind(SERVER_ADDRESS).unwrap();
    info!(
        "Server listening on port {}",
        SERVER_ADDRESS.split(':').nth(1).unwrap_or_default()
    );

    let (tx, rx) = mpsc::channel::<(RequestPayload, Sender<BankResponse>)>();
    create_processing_thread(rx);
    listener.set_nonblocking(true).unwrap();
    loop {
        if let Some(stream) = try_accept(&listener) {
            let tx = tx.clone();
            std::thread::spawn(move || {
                if let Err(ProcessingErrorsResult::Io(data)) = handle_client_requests(stream, tx) {
                    error!("{}", data)
                }
            });
        }
    }
}

/// Creates a processing thread that handles incoming requests from a channel connector.
///
/// # Arguments
///
/// * `channel_connector` - The channel connector that receives requests from other threads.
///
fn create_processing_thread(chanel_connector: Receiver<(RequestPayload, Sender<BankResponse>)>) {
    let mut bank: Bank = Bank::new();
    let _bank_thread = std::thread::spawn(move || loop {
        match chanel_connector.recv() {
            Ok((process, callback_chanel)) => {
                let res = match process {
                    OpenAccount(OpenAccountRequestParams { account }) => {
                        let trans_id = bank.create_account(account.as_str());
                        callback_chanel.send(Transaction(trans_id))
                    }
                    Deposit(DepositParams { account, amount }) => {
                        let trans_id = bank.deposit(account.as_str(), amount);
                        callback_chanel.send(Transaction(trans_id))
                    }
                    Withdraw(WithdrawParams { account, amount }) => {
                        let trans_id = bank.withdraw(account.as_str(), amount);
                        callback_chanel.send(Transaction(trans_id))
                    }
                    Transfer(TransferParams {
                        sender_account,
                        receiver_account,
                        amount,
                    }) => {
                        let trans_id = bank.transfer(
                            sender_account.as_str(),
                            receiver_account.as_str(),
                            amount,
                        );
                        callback_chanel.send(Transaction(trans_id))
                    }
                    GetBalance(GetBalanceAccountRequestParams { account }) => {
                        let balance = bank.get_balance(account.as_str());
                        callback_chanel.send(BankResponse::Balance(balance))
                    }
                    GetHistory => {
                        let history = bank.get_history();
                        callback_chanel.send(BankResponse::History(history))
                    }
                    _ => Ok(()),
                };

                if res.is_err() {
                    error!("{}", res.err().unwrap());
                }
            }
            Err(e) => {
                error!("{}", e);
            }
        }
    });
}

/// Accepts incoming TCP connections on the given listener.
///
/// # Arguments
///
/// * `listener` - The TCP listener to accept connections from.
///
/// # Returns
///
/// Returns an `Option<TcpStream>` representing the accepted TCP stream if successful,
/// or `None` if the operation would block or an error occurred.
///
fn try_accept(listener: &TcpListener) -> Option<TcpStream> {
    match listener.accept() {
        Ok((stream, addr)) => {
            println!("Accepted connection with {}", addr);
            Some(stream)
        }
        Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => None,
        Err(e) => {
            warn!("Failed to accept a connection: {}", e);
            None
        }
    }
}

/// Handles client requests by reading data from the provided TCP stream and processing the requests accordingly.
///
/// The function takes a mutable reference to a `TcpStream` object and a `Sender<(RequestPayload, Sender<BankResponse>)>` object as arguments.
/// It returns a `Result<(), ProcessingErrorsResult>`, indicating success or failure of the handling process.
///
/// The function enters an infinite loop and waits for incoming data from the client. It reads the data from the stream in chunks and appends it to a vector.
/// If no data is received, the function returns `Ok(())` to indicate that the connection has been closed.
/// Otherwise, it tries to deserialize the received data into a `Request` object. If deserialization fails, it sends a deserialization error response to the client.
///
/// After successful deserialization, the function matches the payload of the `Request` object and calls the corresponding processing function.
/// The processing functions are responsible for handling different types of requests such as ping, account creation, deposit, withdrawal, transfer, balance retrieval, history retrieval, and connection closure.
///
/// Once the request is processed, the function sends the response back to the client using the `send` method of the `Response` object.
/// The loop continues until the connection is closed by the client.
fn handle_client_requests(
    mut stream: TcpStream,
    processing_sender: Sender<(RequestPayload, Sender<BankResponse>)>,
) -> Result<(), ProcessingErrorsResult> {
    loop {
        debug!("waiting for client {:?}", stream.peer_addr()?);
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
        let req = serde_json::from_slice::<Request>(received.as_slice());

        if let Err(ref err) = req {
            let resp = Response {
                payload: ResponsePayload::DeserializeError(err.to_string()),
            };
            error!("Deserialize error: {:?}", err);
            resp.send(&mut stream)?;
        }
        let req = req.unwrap();
        let resp = match &req.payload {
            Ping => process_ping(),
            OpenAccount(_) => create_account(req.payload, &processing_sender),
            Deposit(_) => process_deposit(req.payload, &processing_sender),
            Withdraw(_) => process_withdraw(req.payload, &processing_sender),
            Transfer(_) => process_transfer(req.payload, &processing_sender),
            GetBalance(_) => process_get_balance(req.payload, &processing_sender),
            GetHistory => process_get_history(req.payload, &processing_sender),
            GetHistoryForAccount(_) => process_history_for_account(req.payload, &processing_sender),
            CloseConnection => {
                info!("Closing connection with {}", stream.peer_addr()?);
                stream.shutdown(Shutdown::Both)?;
                return Ok(());
            }
        }?;
        debug!("send data to client");
        resp.send(&mut stream)?;
    }
}

/// Creates a new account by processing the given request payload and sending it to the processing thread.
///
/// # Arguments
///
/// * `payload` - The request payload containing the account information.
/// * `processing_sender` - The sender for sending the request payload to the processing thread.
///
/// # Returns
///
/// Returns a `ResponseResult` representing the result of the account creation process.
///
fn create_account(
    payload: RequestPayload,
    processing_sender: &Sender<(RequestPayload, Sender<BankResponse>)>,
) -> ResponseResult {
    info!("open account {:?}", payload);
    let processing_response = processing(payload, processing_sender)?;

    if let Transaction(result) = processing_response {
        return match result {
            Ok(trans_id) => Ok(Response {
                payload: ResponsePayload::AccountCreated(trans_id),
            }),
            Err(error_message) => Ok(Response {
                payload: ResponsePayload::AccountCreatedError(error_message.to_string()),
            }),
        };
    };

    Err(TypeMismatchError(
        "Expected Transaction, found unexpected".to_string(),
    ))
}

/// Processes a deposit request by sending it to the processing thread and handling the response.
///
/// # Arguments
///
/// * `deposit_params` - The request payload containing the deposit information.
/// * `processing_sender` - The sender for sending the deposit request to the processing thread.
///
/// # Returns
///
/// Returns a `ResponseResult` representing the result of the deposit process.
///
fn process_deposit(
    deposit_params: RequestPayload,
    processing_sender: &Sender<(RequestPayload, Sender<BankResponse>)>,
) -> ResponseResult {
    info!("process deposit for {:?}", deposit_params);
    let processing_response = processing(deposit_params, processing_sender)?;

    if let Transaction(result) = processing_response {
        return match result {
            Ok(trans_id) => Ok(Response {
                payload: ResponsePayload::DepositSuccess(trans_id),
            }),
            Err(error_message) => Ok(Response {
                payload: ResponsePayload::DepositError(error_message.to_string()),
            }),
        };
    };
    Err(TypeMismatchError("Expected Transaction".to_string()))
}

/// Processes a withdrawal request by sending it to the processing thread and handling the response.
///
/// # Arguments
///
/// * `withdraw_payload` - The request payload containing the withdrawal information.
/// * `processing_sender` - The sender for sending the withdrawal request to the processing thread.
///
/// # Returns
///
/// Returns a `ResponseResult` representing the result of the withdrawal process.
///
fn process_withdraw(
    withdraw_payload: RequestPayload,
    processing_sender: &Sender<(RequestPayload, Sender<BankResponse>)>,
) -> ResponseResult {
    info!("process withdraw for account {:?}", withdraw_payload);

    let processing_response = processing(withdraw_payload, processing_sender)?;
    if let Transaction(result) = processing_response {
        return match result {
            Ok(trans_id) => Ok(Response {
                payload: ResponsePayload::WithdrawSuccess(trans_id),
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
        };
    };
    Err(TypeMismatchError("Expected Transaction".to_string()))
}

/// Processes a transfer request by sending it to the processing thread and handling the response.
///
/// # Arguments
///
/// * `transfer_payload` - The request payload containing the transfer information.
/// * `processing_sender` - The sender for sending the transfer request to the processing thread.
///
/// # Returns
///
/// Returns a `ResponseResult` representing the result of the transfer process.
///
fn process_transfer(
    transfer_payload: RequestPayload,
    processing_sender: &Sender<(RequestPayload, Sender<BankResponse>)>,
) -> ResponseResult {
    info!("process transfer from account {:?}  ", transfer_payload);
    let processing_response = processing(transfer_payload, processing_sender)?;

    if let Transaction(result) = processing_response {
        return match result {
            Ok(trans_id) => Ok(Response {
                payload: ResponsePayload::TransferSuccess(trans_id),
            }),

            Err(error_message) => {
                if let BankError::SomeAccountTransfer(info) = &error_message {
                    return Ok(Response {
                        payload: ResponsePayload::SomeAccountError(info.to_string()),
                    });
                } else {
                    return Ok(Response {
                        payload: ResponsePayload::Error(error_message.to_string()),
                    });
                }
            }
        };
    };
    Err(TypeMismatchError("Expected Transaction".to_string()))
}

/// Processes a balance request by sending it to the processing thread and handling the response.
///
/// # Arguments
///
/// * `balance_req_payload` - The request payload containing the balance information.
/// * `processing_sender` - The sender for sending the balance request to the processing thread.
///
/// # Returns
///
/// Returns a `ResponseResult` representing the result of the balance request.
///
fn process_get_balance(
    balance_req_payload: RequestPayload,
    processing_sender: &Sender<(RequestPayload, Sender<BankResponse>)>,
) -> ResponseResult {
    info!("process balance for account {:?} ", balance_req_payload);
    let processing_response = processing(balance_req_payload, processing_sender)?;

    if let BankResponse::Balance(result) = processing_response {
        return match result {
            Ok(balance) => Ok(Response {
                payload: ResponsePayload::Balance(balance),
            }),
            Err(error_message) => Ok(Response {
                payload: ResponsePayload::Error(error_message.to_string()),
            }),
        };
    };
    Err(TypeMismatchError("Expected Transaction".to_string()))
}

/// Processes a history request by sending it to the processing thread and handling the response.
///
/// # Arguments
///
/// * `history_req_payload` - The request payload containing the history information.
/// * `processing_sender` - The sender for sending the history request to the processing thread.
///
/// # Returns
///
/// Returns a `ResponseResult` representing the result of the history request.
///
fn process_get_history(
    history_req_payload: RequestPayload,
    processing_sender: &Sender<(RequestPayload, Sender<BankResponse>)>,
) -> ResponseResult {
    info!("process history  ");

    let processing_response = processing(history_req_payload, processing_sender)?;

    if let BankResponse::History(result) = processing_response {
        return match result {
            Ok(history) => Ok(Response {
                payload: ResponsePayload::History(history.iter().map(|o| (*o).clone()).collect()),
            }),
            Err(error_message) => Ok(Response {
                payload: ResponsePayload::Error(error_message.to_string()),
            }),
        };
    };

    Err(TypeMismatchError("Expected Transaction".to_string()))
}

/// Processes a history request for a specific account by sending it to the processing thread and handling the response.
///
/// # Arguments
///
/// * `history_req_payload` - The request payload containing the history information for a specific account.
/// * `processing_sender` - The sender for sending the history request to the processing thread.
///
/// # Returns
///
fn process_history_for_account(
    history_req_payload: RequestPayload,
    processing_sender: &Sender<(RequestPayload, Sender<BankResponse>)>,
) -> ResponseResult {
    info!("process history for account {history_req_payload:?}");

    if let BankResponse::History(result) = processing(history_req_payload, processing_sender)? {
        return match result {
            Ok(history) => Ok(Response {
                payload: ResponsePayload::History(history.iter().map(|o| (*o).clone()).collect()),
            }),
            Err(error_message) => Ok(Response {
                payload: ResponsePayload::Error(error_message.to_string()),
            }),
        };
    };

    Err(TypeMismatchError("Expected Transaction".to_string()))
}

/// Processes a request by sending it to the processing thread and receiving the response.
///
/// # Arguments
///
/// * `generic_params` - The request payload containing the generic parameters.
/// * `processing_sender` - The sender for sending the request to the processing thread.
///
/// # Returns
///
/// Returns a `Result` representing the result of the processing.
///
fn processing(
    generic_params: RequestPayload,
    processing_sender: &Sender<(RequestPayload, Sender<BankResponse>)>,
) -> Result<BankResponse, ProcessingErrorsResult> {
    let (response_sender, receiver_from_processing) = channel::<BankResponse>();
    processing_sender
        .send((generic_params, response_sender.clone()))
        .unwrap();
    let resp = receiver_from_processing.recv()?;
    Ok(resp)
}

/// Processes the ping request.
///
/// # Returns
///
/// Returns a `Result` representing the result of the ping processing.
///
fn process_ping() -> ResponseResult {
    debug!("pinging");
    Ok(Response {
        payload: ResponsePayload::HandShakeEstablished,
    })
}
