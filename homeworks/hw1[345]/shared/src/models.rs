use crate::constants::MAX_CHUNK_BYTE_SIZE;
use crate::errors::ProcessingErrorsResult;
use bank_engine::bank::{Operation, TransactionId};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::net::TcpStream;

#[derive(Serialize, Debug, Deserialize)]
pub struct Request {
    pub payload: RequestPayload,
}

impl Request {
    pub fn send(&self, stream: &mut TcpStream) -> Result<(), std::io::Error> {
        let json = serde_json::to_vec(&self)?;
        stream.write_all(&json)?;
        Ok(())
    }
}

#[derive(Serialize, Debug, Deserialize)]
pub enum RequestPayload {
    /// Represents a ping request.
    Ping,

    /// Represents an open account request with the specified parameters.
    OpenAccount(OpenAccountRequestParams),

    /// Represents a withdrawal request with the specified parameters.
    Withdraw(WithdrawParams),

    /// Represents a deposit request with the specified parameters.
    Deposit(DepositParams),

    /// Represents a get balance request with the specified parameters.
    GetBalance(GetBalanceAccountRequestParams),

    /// Represents a transfer request with the specified parameters.
    Transfer(TransferParams),

    /// Represents a close connection request.
    CloseConnection,

    /// Represents a get history request without any parameters.
    GetHistory,

    /// Represents a get history for account request with the specified account identifier.
    GetHistoryForAccount(String),
}

/// Represents the payload of a response.
#[derive(Serialize, Debug, Deserialize, PartialEq)]
pub enum ResponsePayload {
    /// Indicates that a handshake has been established.
    HandShakeEstablished,

    /// Indicates an error occurred with the specified error message.
    Error(String),

    /// Indicates that an account was successfully created.
    AccountCreated(TransactionId),

    /// Indicates an error occurred while creating an account with the specified error message.
    AccountCreatedError(String),

    /// Indicates that a deposit was successful.
    DepositSuccess(TransactionId),
    /// Indicates an error occurred while making a deposit with the specified error message.
    DepositError(String),

    /// Indicates that a withdrawal was successful.
    WithdrawSuccess(TransactionId),
    /// Indicates an error occurred while making a withdrawal with the specified error message.
    WithdrawalError(String),

    /// Indicates that a transfer was successful.
    TransferSuccess(TransactionId),
    /// Indicates an error occurred while making a transfer to same account
    SomeAccountError(String),

    /// Represents the balance of an account with the specified amount.
    Balance(f64),

    /// Represents the history of operations for an account with the specified list of operations.
    History(Vec<Operation>),
    /// Represents an error occurred while getting the history with the specified error message.
    DeserializeError(String),
}

/// Represents the parameters for an open account request.
#[derive(Serialize, Debug, Deserialize)]
pub struct OpenAccountRequestParams {
    /// The account identifier for the new account.
    pub account: String,
}

/// Represents the parameters for a get balance request.
#[derive(Serialize, Debug, Deserialize)]
pub struct GetBalanceAccountRequestParams {
    /// The account identifier for which the balance is requested.
    pub account: String,
}

/// Represents the parameters for a deposit request.
#[derive(Serialize, Debug, Deserialize)]
pub struct DepositParams {
    /// The account identifier where the deposit will be made.
    pub account: String,

    /// The amount to be deposited.
    pub amount: f64,
}

/// Represents the parameters for a withdrawal request.
#[derive(Serialize, Debug, Deserialize)]
pub struct WithdrawParams {
    /// The account identifier from which the withdrawal will be made.
    pub account: String,

    /// The amount to be withdrawn.
    pub amount: f64,
}

/// Represents the parameters for a transfer request.
#[derive(Serialize, Debug, Deserialize)]
pub struct TransferParams {
    /// The account identifier of the sender.
    pub sender_account: String,

    /// The account identifier of the receiver.
    pub receiver_account: String,

    /// The amount to be transferred.
    pub amount: f64,
}

/// Represents a response from the server.
#[derive(Serialize, Debug, Deserialize)]
pub struct Response {
    /// The payload of the response.
    pub payload: ResponsePayload,
}

// pub type ResponseResult = Result<Response, std::io::Error>;
pub type ResponseResult = Result<Response, ProcessingErrorsResult>;

impl Response {
    pub fn read(stream: &mut impl Read) -> Result<Self, std::io::Error> {
        let mut received: Vec<u8> = Vec::with_capacity(MAX_CHUNK_BYTE_SIZE);
        let mut chunk = [0u8; MAX_CHUNK_BYTE_SIZE];
        loop {
            let bytes_read = stream.read(&mut chunk)?;
            received.extend_from_slice(&chunk[..bytes_read]);
            if bytes_read < MAX_CHUNK_BYTE_SIZE {
                break;
            }
        }
        let resp = serde_json::from_slice::<Response>(received.as_slice())?;
        Ok(resp)
    }

    pub fn send(&self, stream: &mut TcpStream) -> Result<(), std::io::Error> {
        let json = serde_json::to_vec(&self)?;
        stream.write_all(&json)?;
        Ok(())
    }
}
