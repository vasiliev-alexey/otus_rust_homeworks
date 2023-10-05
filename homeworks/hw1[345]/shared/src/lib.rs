pub mod constants;
pub mod errors;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
pub struct Request {
    pub payload: RequestPayload,
}
#[derive(Serialize, Debug, Deserialize)]
pub enum RequestPayload {
    Ping,
    OpenAccount(OpenAccountRequestParams),
    Withdraw(WithdrawParams),
    Deposit(DepositParams),
    GetBalance(GetBalanceAccountRequestParams),
    Transfer(TransferParams),
    CloseConnection,
    GetHistory(),
    GetHistoryForAccount(String),
}

#[derive(Serialize, Debug, Deserialize, PartialEq)]
pub enum ResponsePayload {
    HandShakeEstablished,
    Error(String),
    AccountCreated,
    AccountCreatedError(String),
    DepositSuccess,
    WithdrawSuccess,
    TransferSuccess,
    Balance(f64),
    History(Vec<Operation>),
}

#[derive(Serialize, Debug, Deserialize)]
pub struct OpenAccountRequestParams {
    pub account: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct GetBalanceAccountRequestParams {
    pub account: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct DepositParams {
    pub account: String,
    pub amount: f64,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct WithdrawParams {
    pub account: String,
    pub amount: f64,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct TransferParams {
    pub sender_account: String,
    pub receiver_account: String,
    pub amount: f64,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Response {
    pub payload: ResponsePayload,
}

pub use bank_engine::bank::Operation;
pub use bank_engine::bank::OperationType;
