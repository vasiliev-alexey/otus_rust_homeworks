use crate::client::ResponseError::UnexpectedResponse;

use log::{debug, error};
use shared::errors::{ConnectError, ConnectResult};
use shared::models::{
    DepositParams, GetBalanceAccountRequestParams, OpenAccountRequestParams, Request,
    RequestPayload, Response, ResponsePayload, TransferParams, WithdrawParams,
};
use shared::{Operation, TransactionId};
use std::fmt::{Display, Formatter};
use std::io;
use std::io::Write;
use std::net::Shutdown::Both;
use std::net::{TcpStream, ToSocketAddrs};
use thiserror::Error;

pub struct BankClient {
    stream: TcpStream,
}

/// Performs any necessary cleanup before the BankClient instance is dropped.
///
/// This method is automatically called when the BankClient instance goes out of scope
/// or is explicitly dropped using the `drop` function.
/// // Do some operations with the client...
///
/// // The `drop` function is automatically called at the end of the scope
/// // to clean up the resources associated with the client.
impl Drop for BankClient {
    fn drop(&mut self) {
        let data_req = Request {
            payload: RequestPayload::CloseConnection,
        };
        let json = serde_json::to_string(&data_req).unwrap();
        self.stream.write_all(json.as_bytes()).unwrap();
        let _ = self.stream.shutdown(Both);
    }
}

impl BankClient {
    /// Establishes a connection to the bank server.
    ///
    /// This method connects the `BankClient` instance to the bank server using the provided address.
    /// It returns a new `BankClient` instance that is ready to perform operations on the server.
    ///
    /// # Arguments
    ///
    /// * `address` - The address of the bank server to connect to, formatted as `host:port`.
    ///
    /// # Returns
    ///
    /// A new `BankClient` instance connected to the bank server.
    ///
    /// # Examples
    ///
    /// use client::client::BankClient;
    ///
    /// let connected_client = BankClient::connect("127.0.0.1:8080");
    pub fn connect<Addrs>(addr: Addrs) -> ConnectResult<Self>
    where
        Addrs: ToSocketAddrs,
    {
        let stream = TcpStream::connect(addr)?;
        BankClient::handshake(stream)
    }
    /// Performs a handshake with the bank server to establish a secure connection.
    /// # Returns
    ///
    /// ConnectResult - Result of the handshake, `Ok` if the handshake was successful, `Err` otherwise.
    fn handshake(mut stream: TcpStream) -> ConnectResult<Self> {
        let data_req = Request {
            payload: RequestPayload::Ping,
        };
        let json = serde_json::to_string(&data_req).unwrap();

        let _ = stream.write(json.as_bytes())?;

        let resp = Response::read(&mut stream)?;
        if resp.payload != ResponsePayload::HandShakeEstablished {
            error!("Handshake error: {:?}", resp.payload);
            let msg = format!("received: {:?}", resp.payload);
            return Err(ConnectError::BadHandshake(msg));
        }

        Ok(Self { stream })
    }
    /// Creates a new bank account for the client with the specified name.
    ///
    /// This method creates a new bank account for the client with the provided name and returns the
    /// account identifier. The account identifier can be used to perform operations such as deposits
    /// and withdrawals on the account.
    ///
    /// # Arguments
    ///
    /// * `code` - The code of the account holder.
    ///
    /// # Returns
    /// * 'ResponseResult' - Result of the operation, TransactionId if the operation was successful, `Err` otherwise.
    ///
    /// # Errors
    /// AccountDuplicationError - If the account already exists.
    /// GenericError - If the response payload is not `AccountCreated`.
    pub fn create_account(&mut self, account: &str) -> ResponseResult<TransactionId> {
        let data_req = Request {
            payload: RequestPayload::OpenAccount(OpenAccountRequestParams {
                account: account.to_string(),
            }),
        };
        debug!("sending: {:?}", &data_req);
        data_req.send(&mut self.stream)?;

        let response = Response::read(&mut self.stream)?;
        debug!("received: {:?}", &response);

        if let ResponsePayload::AccountCreated(transaction_id) = &response.payload {
            Ok((*transaction_id).to_owned())
        } else {
            Err(ResponseError::unexpected_response(&response.payload))
        }
    }
    /// Deposits the specified amount into the specified account.
    ///
    /// # Arguments
    ///
    /// * `account` - The account to deposit the amount into.
    /// * `amount` - The amount to deposit.
    ///
    /// # Returns
    /// * `TransactionId` for the operation
    ///
    /// # Errors
    ///
    /// Returns an GenericError if the deposit fails or if the response payload is not `DepositSuccess`.
    pub fn deposit(&mut self, account: &str, amount: f64) -> ResponseResult<TransactionId> {
        let data_req = Request {
            payload: RequestPayload::Deposit(DepositParams {
                account: account.to_string(),
                amount,
            }),
        };
        debug!("sending: {:?}", &data_req);
        data_req.send(&mut self.stream)?;

        let response = Response::read(&mut self.stream)?;

        if let ResponsePayload::DepositSuccess(transaction_id) = response.payload {
            Ok(transaction_id.to_owned())
        } else {
            Err(ResponseError::unexpected_response(&response.payload))
        }
    }
    /// Withdraws the specified amount from the specified account.
    ///
    /// # Arguments
    ///
    /// * `account` - The account to withdraw the amount from.
    /// * `amount` - The amount to withdraw.
    ///
    /// # Errors
    ///
    /// Returns an error if there is an error response or if the response payload is not `WithdrawSuccess`.
    pub fn withdraw(&mut self, account: &str, amount: f64) -> ResponseResult<TransactionId> {
        let data_req = Request {
            payload: RequestPayload::Withdraw(WithdrawParams {
                account: account.to_string(),
                amount,
            }),
        };
        debug!("sending: {:?}", &data_req);
        data_req.send(&mut self.stream)?;

        let response = Response::read(&mut self.stream)?;
        debug!("received: {:?}", &response);

        if let ResponsePayload::WithdrawSuccess(transaction_id) = response.payload {
            Ok(transaction_id.to_owned())
        } else if let ResponsePayload::WithdrawalError(error_message) = response.payload {
            Err(ResponseError::WithdrawalError(error_message))
        } else {
            Err(ResponseError::unexpected_response(&response.payload))
        }
    }

    /// Transfers the specified amount from the sender's account to the receiver's account.
    ///
    /// # Arguments
    ///
    /// * `sender_account` - The account from which the amount will be transferred.
    /// * `receiver_account` - The account to which the amount will be transferred.
    /// * `amount` - The amount to be transferred.
    ///
    /// # Errors
    ///
    /// Returns an error if there is an error response or if the response payload is not `TransferSuccess`.
    pub fn transfer(
        &mut self,
        sender_account: &str,
        receiver_account: &str,
        amount: f64,
    ) -> ResponseResult<TransactionId> {
        let data_req = Request {
            payload: RequestPayload::Transfer(TransferParams {
                sender_account: sender_account.to_string(),
                receiver_account: receiver_account.to_string(),
                amount,
            }),
        };

        debug!("sending: {:?}", &data_req);
        data_req.send(&mut self.stream)?;

        let response = Response::read(&mut self.stream)?;
        debug!("received: {:?}", &response);

        if let ResponsePayload::TransferSuccess(transaction_id) = response.payload {
            Ok(transaction_id.to_owned())
        } else if let ResponsePayload::SomeAccountError(error_message) = response.payload {
            error!("Transfer error {:?}", error_message);
            Err(UnexpectedResponseData { error_message }.into())
        } else {
            error!("unexpected response {:?}", response);
            Err(UnexpectedResponseData {
                error_message: format!(
                    "expected type {:?} , found {:?}",
                    ResponsePayload::TransferSuccess(TransactionId::default()),
                    response
                ),
            }
            .into())
        }
    }

    /// Retrieves the balance of the specified account.
    ///
    /// # Arguments
    ///
    /// * `account` - The account for which to retrieve the balance.
    ///
    /// # Errors
    ///
    /// Returns an error if there is an error response or if the response payload does not contain the balance.
    ///
    /// # Returns
    ///
    /// The balance of the specified account.
    pub fn get_balance(&mut self, account: &str) -> ResponseResult<f64> {
        let data_req = Request {
            payload: RequestPayload::GetBalance(GetBalanceAccountRequestParams {
                account: account.to_string(),
            }),
        };

        debug!("sending: {:?}", &data_req);
        data_req.send(&mut self.stream)?;

        let response = Response::read(&mut self.stream)?;
        debug!("received: {:?}", &response);

        let bal = &response.payload;

        if let ResponsePayload::Balance(aviable_balance) = bal {
            return Ok(*aviable_balance);
        }

        Err(GenericErrorData {
            error_message: "some error".to_string(),
        }
        .into())
    }
    /// Retrieves the transaction history.
    ///
    /// # Errors
    ///
    /// Returns an error if there is an error response or if the response payload does not contain the transaction history.
    ///
    /// # Returns
    ///
    /// The transaction history as a vector of `Operation` objects.
    ///
    /// ```
    pub fn get_history(&mut self) -> ResponseResult<Vec<Operation>> {
        let data_req = Request {
            payload: RequestPayload::GetHistory,
        };
        data_req.send(&mut self.stream)?;

        let response = Response::read(&mut self.stream)?;
        debug!("received: {:?}", &response);

        let bal = &response.payload;

        if let ResponsePayload::History(val) = bal {
            return Ok(val.clone());
        }

        Err(GenericErrorData {
            error_message: "some error".to_string(),
        }
        .into())
    }
    /// Retrieves the transaction history for the specified account.
    ///
    /// # Arguments
    ///
    /// * `account` - The account for which to retrieve the transaction history.
    ///
    /// # Errors
    ///
    /// Returns an error if there is an error response or if the response payload does not contain the transaction history.
    ///
    /// # Returns
    ///
    /// The transaction history for the specified account as a vector of `Operation` objects.
    pub fn get_history_for_account(&mut self, account: &str) -> ResponseResult<Vec<Operation>> {
        let data_req = Request {
            payload: RequestPayload::GetHistoryForAccount(account.to_string()),
        };
        debug!("sending: {:?}", &data_req);
        data_req.send(&mut self.stream)?;

        let response = Response::read(&mut self.stream)?;
        debug!("received: {:?}", &response);

        if let ResponsePayload::History(account_history) = &response.payload {
            return Ok(account_history.clone());
        }
        Err(GenericErrorData {
            error_message: "some error".to_string(),
        }
        .into())
    }
}

pub type ResponseResult<T> = Result<T, ResponseError>;

/// Represents an error that can occur when handling API responses.
#[derive(Debug, Error)]
pub enum ResponseError {
    /// A generic error that occurred.
    #[error("Generic error: {0}")]
    GenericError(#[from] GenericErrorData),

    /// A UTF-8 error that occurred.
    #[error("Utf8Error error: {0}")]
    Utf8Error(#[from] core::str::Utf8Error),

    #[error("DeserializationError error: {0}")]
    DeserializationError(#[from] serde_json::Error),

    /// An I/O error that occurred.
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    /// An unexpected response error that occurred.
    #[error("Unexpected response error: {0}")]
    UnexpectedResponse(#[from] UnexpectedResponseData),

    #[error("Withdrawal error: {0}")]
    WithdrawalError(String),
}

impl ResponseError {
    fn unexpected_response(payload: &ResponsePayload) -> Self {
        error!("Unexpected response payload: {:?} ", payload);
        UnexpectedResponse(UnexpectedResponseData {
            error_message: format!("Unexpected response: {:?}", payload),
        })
    }
}

/// Represents generic error data.
#[derive(Debug, Error)]
pub struct GenericErrorData {
    /// The error message associated with the generic error.
    error_message: String,
}
impl Display for GenericErrorData {
    /// Formats the `GenericErrorData` as a string.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to write the formatted string to.
    ///
    /// # Returns
    ///
    /// A `std::fmt::Result` indicating the success or failure of the formatting operation.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error_message)
    }
}

/// Represents unexpected response data.
#[derive(Debug, Error)]
pub struct UnexpectedResponseData {
    /// The error message associated with the unexpected response.
    error_message: String,
}
impl Display for UnexpectedResponseData {
    /// Formats the `UnexpectedResponseData` as a string.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to write the formatted string to.
    ///
    /// # Returns
    ///
    /// A `std::fmt::Result` indicating the success or failure of the formatting operation.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error_message)
    }
}
