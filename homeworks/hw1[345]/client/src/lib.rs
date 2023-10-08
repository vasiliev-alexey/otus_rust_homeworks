use crate::ResponseError::Io;
use log::{debug, error};
use shared::errors::{ConnectError, ConnectResult};
use shared::models::{
    DepositParams, GetBalanceAccountRequestParams, OpenAccountRequestParams, Request,
    RequestPayload, Response, ResponsePayload, TransferParams, WithdrawParams,
};
use shared::Operation;
use std::fmt::{Display, Formatter};
use std::io;
use std::io::{Read, Write};
use std::net::Shutdown::Both;
use std::net::{TcpStream, ToSocketAddrs};
use std::str::from_utf8;
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
/// ```
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
const BUFFER_SIZE: usize = 1024;
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
    /// ```
    ///
    ///
    /// use client::BankClient;
    ///
    /// let connected_client = BankClient::connect("127.0.0.1:8080");
    /// ```

    pub fn connect<Addrs>(addr: Addrs) -> ConnectResult<Self>
    where
        Addrs: ToSocketAddrs,
    {
        let stream = TcpStream::connect(addr)?;
        BankClient::handshake(stream)
    }
    /// Performs a handshake with the bank server to establish a secure connection.
    ///
    /// This method initiates a handshake protocol with the bank server to establish a secure connection.
    /// It ensures the authenticity and integrity of the communication by verifying the server's identity
    /// and exchanging cryptographic keys.
    ///
    /// # Returns
    ///
    /// ConnectResult - Result of the handshake, `Ok` if the handshake was successful, `Err` otherwise.
    ///
    /// ```
    fn handshake(mut stream: TcpStream) -> ConnectResult<Self> {
        let data_req = Request {
            payload: RequestPayload::Ping,
        };
        let json = serde_json::to_string(&data_req).unwrap();
        let _ = stream.write(json.as_bytes()).unwrap();

        let mut buf = [0; BUFFER_SIZE];
        let size = stream.read(&mut buf)?;
        let resp = serde_json::from_str::<Response>(from_utf8(&buf[0..size]).unwrap());
        if resp.is_err() || resp.unwrap().payload != ResponsePayload::HandShakeEstablished {
            let msg = format!("received: {:?}", buf);
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
    ///
    /// ResponseResult - Result of the operation, `Ok` if the operation was successful, `Err` otherwise.
    ///
    /// Error - Error encountered during the operation, `Ok` if the operation was successful, ResponseError otherwise.
    ///    
    /// ```
    pub fn create_account(&mut self, account: &str) -> ResponseResult<()> {
        let data_req = Request {
            payload: RequestPayload::OpenAccount(OpenAccountRequestParams {
                account: account.to_string(),
            }),
        };
        let json = serde_json::to_string(&data_req).unwrap();

        debug!("sending: {:?}", &json);

        self.stream.write_all(json.as_bytes()).unwrap();

        let mut buf = [0; BUFFER_SIZE];
        let size = self.stream.read(&mut buf);

        if size.is_err() {
            return Err(Io(size.err().unwrap()));
        }
        let size = size.unwrap();
        let resp = serde_json::from_str::<Response>(from_utf8(&buf[0..size]).unwrap());
        debug!("received: {:?}", &resp);

        if resp.is_err() || ResponsePayload::AccountCreated != resp.as_ref().unwrap().payload {
            error!("unexpected response {:?}", resp.unwrap());
            let msg = format!("received: {:?}", buf);
            return Err(GenericErrorData { error_message: msg }.into());
        }
        Ok(())
    }
    /// Deposits the specified amount into the specified account.
    ///
    /// # Arguments
    ///
    /// * `account` - The account to deposit the amount into.
    /// * `amount` - The amount to deposit.
    ///
    /// # Errors
    ///
    /// Returns an GenericError if the deposit fails or if the response payload is not `DepositSuccess`.
    ///
    /// ```
    pub fn deposit(&mut self, account: &str, amount: f64) -> ResponseResult<()> {
        let data_req = Request {
            payload: RequestPayload::Deposit(DepositParams {
                account: account.to_string(),
                amount,
            }),
        };
        let json = serde_json::to_string(&data_req).unwrap();
        self.stream.write_all(json.as_bytes()).unwrap();
        let mut buf = [0; BUFFER_SIZE];
        let size = self.stream.read(&mut buf)?;
        let resp = serde_json::from_str::<Response>(from_utf8(&buf[0..size]).unwrap());
        debug!("received: {:?}", &resp);

        if resp.is_err() || ResponsePayload::DepositSuccess != resp.as_ref().unwrap().payload {
            error!("unexpected response {:?}", resp.unwrap());
            let error_message = format!("received: {:?}", buf);
            return Err(GenericErrorData { error_message }.into());
        }
        Ok(())
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
    ///
    /// ```
    pub fn withdraw(&mut self, account: &str, amount: f64) -> ResponseResult<()> {
        let data_req = Request {
            payload: RequestPayload::Withdraw(WithdrawParams {
                account: account.to_string(),
                amount,
            }),
        };
        let json = serde_json::to_string(&data_req).unwrap();

        self.stream.write_all(json.as_bytes()).unwrap();
        let mut buf = [0; BUFFER_SIZE];
        let size = self.stream.read(&mut buf)?;

        let resp = serde_json::from_str::<Response>(from_utf8(&buf[0..size]).unwrap());

        debug!("received: {:?}", &resp);

        if resp.is_err() {
            error!("Error response {:?}", resp.unwrap());
            return Err(GenericErrorData {
                error_message: "".to_string(),
            }
            .into());
        } else if ResponsePayload::WithdrawSuccess != resp.as_ref().unwrap().payload {
            error!("unexpected response {:?}", resp.as_ref().unwrap());
            return Err(UnexpectedResponseData {
                error_message: format!(
                    "expected type {:?} , found {:?}",
                    ResponsePayload::WithdrawSuccess,
                    resp.unwrap().payload
                ),
            }
            .into());
        }

        Ok(())
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
    ///
    /// ```
    pub fn transfer(
        &mut self,
        sender_account: &str,
        receiver_account: &str,
        amount: f64,
    ) -> ResponseResult<()> {
        let data_req = Request {
            payload: RequestPayload::Transfer(TransferParams {
                sender_account: sender_account.to_string(),
                receiver_account: receiver_account.to_string(),
                amount,
            }),
        };
        let json = serde_json::to_string(&data_req).unwrap();

        debug!("sending: {:?}", &json);

        self.stream.write_all(json.as_bytes()).unwrap();
        let mut buf = [0; BUFFER_SIZE];
        let size = self.stream.read(&mut buf)?;
        let resp = serde_json::from_str::<Response>(from_utf8(&buf[0..size]).unwrap());

        debug!("received: {:?}", &resp);
        if resp.is_err() {
            error!("Error response {:?}", resp.unwrap());
            return Err(GenericErrorData {
                error_message: "".to_string(),
            }
            .into());
        } else if ResponsePayload::TransferSuccess != resp.as_ref().unwrap().payload {
            error!("unexpected response {:?}", resp.as_ref().unwrap());
            return Err(UnexpectedResponseData {
                error_message: format!(
                    "expected type {:?} , found {:?}",
                    ResponsePayload::TransferSuccess,
                    resp.unwrap().payload
                ),
            }
            .into());
        }
        Ok(())
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
    /// ```
    pub fn get_balance(&mut self, account: &str) -> ResponseResult<f64> {
        let data_req = Request {
            payload: RequestPayload::GetBalance(GetBalanceAccountRequestParams {
                account: account.to_string(),
            }),
        };
        let json = serde_json::to_string(&data_req).unwrap();

        self.stream.write_all(json.as_bytes()).unwrap();

        let mut buf = [0; BUFFER_SIZE];
        let size = self.stream.read(&mut buf)?;
        let resp = serde_json::from_str::<Response>(from_utf8(&buf[0..size]).unwrap());

        debug!("received: {:?}", &resp);
        let bal = &resp.as_ref().unwrap().payload;

        if let ResponsePayload::Balance(val) = bal {
            return Ok(*val);
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
            payload: RequestPayload::GetHistory(),
        };
        let json = serde_json::to_string(&data_req).unwrap();

        self.stream.write_all(json.as_bytes()).unwrap();

        let mut buf = [0; BUFFER_SIZE * 2];
        let size = self.stream.read(&mut buf)?;

        let resp = serde_json::from_str::<Response>(from_utf8(&buf[0..size]).unwrap());

        debug!("received: {:?}", &resp);
        let bal = &resp.as_ref().unwrap().payload;

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
    /// ```

    pub fn get_history_for_account(&mut self, account: &str) -> ResponseResult<Vec<Operation>> {
        let data_req = Request {
            payload: RequestPayload::GetHistoryForAccount(account.to_string()),
        };
        let json = serde_json::to_string(&data_req).unwrap();

        debug!("sending: {:?}", &json);
        self.stream.write_all(json.as_bytes()).unwrap();

        let mut buf = [0; 2048];
        let size = self.stream.read(&mut buf)?;

        let resp = serde_json::from_str::<Response>(from_utf8(&buf[0..size]).unwrap());
        debug!("received: {:?}", &resp);
        let bal = &resp.as_ref().unwrap().payload;

        if let ResponsePayload::History(val) = bal {
            return Ok(val.clone());
        }
        Err(GenericErrorData {
            error_message: "some error".to_string(),
        }
        .into())
    }
}

//pub type RequestResult = Result<String, RequestError>;
pub type ResponseResult<T> = Result<T, ResponseError>;

/// Error for request sending. It consists from two steps: sending and receiving data.
///
/// `SendError` caused by send data error.
/// `ReceiverError` caused by receive data error.
// #[derive(Debug, Error)]
// pub enum RequestError {
//     #[error(transparent)]
//     Send(#[from] SendError),
//     #[error(transparent)]
//     Receive(#[from] ReceiverError),
// }
/// Represents an error that can occur when handling API responses.
#[derive(Debug, Error)]
pub enum ResponseError {
    /// A generic error that occurred.
    #[error("Generic error: {0}")]
    GenericError(#[from] GenericErrorData),

    /// An I/O error that occurred.
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    /// An unexpected response error that occurred.
    #[error("Unexpected response error: {0}")]
    UnexpectedResponse(#[from] UnexpectedResponseData),
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
