pub mod constants;
pub mod errors;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
pub struct Request {
    pub payload: RequestPayload,
}
#[derive(Serialize, Debug, Deserialize)]
pub enum RequestPayload {
    /// Represents a ping request.
    Ping,

    /// Represents an open account request with the specified parameters.
    OpenAccount(OpenAccountRequestParams),

    /// Represents a withdraw request with the specified parameters.
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
    GetHistory(),

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
    AccountCreated,

    /// Indicates an error occurred while creating an account with the specified error message.
    AccountCreatedError(String),

    /// Indicates that a deposit was successful.
    DepositSuccess,

    /// Indicates that a withdrawal was successful.
    WithdrawSuccess,

    /// Indicates that a transfer was successful.
    TransferSuccess,

    /// Represents the balance of an account with the specified amount.
    Balance(f64),

    /// Represents the history of operations for an account with the specified list of operations.
    History(Vec<Operation>),
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

/// Represents the parameters for a withdraw request.
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
pub use bank_engine::bank::Operation;
pub use bank_engine::bank::OperationType;
