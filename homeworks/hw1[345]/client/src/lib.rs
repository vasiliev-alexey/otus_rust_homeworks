use crate::ResponseError::Io;
use log::{debug, error};
use shared::errors::{ConnectError, ConnectResult, ReceiverError, SendError};
use shared::{
    DepositParams, GetBalanceAccountRequestParams, OpenAccountRequestParams, Operation, Request,
    RequestPayload, Response, ResponsePayload, TransferParams, WithdrawParams,
};
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
    pub fn connect<Addrs>(addr: Addrs) -> ConnectResult<Self>
    where
        Addrs: ToSocketAddrs,
    {
        let stream = TcpStream::connect(addr)?;
        BankClient::handshake(stream)
    }

    fn handshake(mut stream: TcpStream) -> ConnectResult<Self> {
        let data_req = Request {
            payload: RequestPayload::Ping,
        };
        let json = serde_json::to_string(&data_req).unwrap();
        let _ = stream.write(json.as_bytes()).unwrap();

        let mut buf = [0; 1024];
        let size = stream.read(&mut buf)?;
        let resp = serde_json::from_str::<Response>(from_utf8(&buf[0..size]).unwrap());
        if resp.is_err() || resp.unwrap().payload != ResponsePayload::HandShakeEstablished {
            let msg = format!("received: {:?}", buf);
            return Err(ConnectError::BadHandshake(msg));
        }
        Ok(Self { stream })
    }

    pub fn create_account(&mut self, account: &str) -> ResponseResult<()> {
        let data_req = Request {
            payload: RequestPayload::OpenAccount(OpenAccountRequestParams {
                account: account.to_string(),
            }),
        };
        let json = serde_json::to_string(&data_req).unwrap();

        debug!("sending: {:?}", &json);

        self.stream.write_all(json.as_bytes()).unwrap();

        let mut buf = [0; 1024];
        let size = self.stream.read(&mut buf);

        if size.is_err() {
            return Err(Io(size.err().unwrap()));
        }
        let size = size.unwrap();
        let txt = from_utf8(&buf[0..size]).unwrap();

        debug!("text {} , size {}", txt, size);

        let resp = serde_json::from_str::<Response>(from_utf8(&buf[0..size]).unwrap());

        debug!("received: {:?}", &resp);

        if resp.is_err() || ResponsePayload::AccountCreated != resp.as_ref().unwrap().payload {
            error!("unexpected response {:?}", resp.unwrap());
            let msg = format!("received: {:?}", buf);
            return Err(GenericErrorData { error_message: msg }.into());
        }
        Ok(())
    }

    pub fn deposit(&mut self, account: &str, amount: f64) -> ResponseResult<()> {
        let data_req = Request {
            payload: RequestPayload::Deposit(DepositParams {
                account: account.to_string(),
                amount,
            }),
        };
        let json = serde_json::to_string(&data_req).unwrap();

        debug!("sending: {:?}", &json);

        self.stream.write_all(json.as_bytes()).unwrap();
        // self.stream.flush().unwrap();

        //  debug!("sented: {}", bbb);
        let mut buf = [0; 1024];
        let size = self.stream.read(&mut buf)?;

        let txt = from_utf8(&buf[0..size]).unwrap();

        debug!("text {} , size {}", txt, size);

        let resp = serde_json::from_str::<Response>(from_utf8(&buf[0..size]).unwrap());

        debug!("received: {:?}", &resp);

        if resp.is_err() || ResponsePayload::DepositSuccess != resp.as_ref().unwrap().payload {
            error!("unexpected response {:?}", resp.unwrap());
            let error_message = format!("received: {:?}", buf);
            return Err(GenericErrorData { error_message }.into());
        }

        Ok(())
    }

    pub fn withdraw(&mut self, account: &str, amount: f64) -> ResponseResult<()> {
        let data_req = Request {
            payload: RequestPayload::Withdraw(WithdrawParams {
                account: account.to_string(),
                amount,
            }),
        };
        let json = serde_json::to_string(&data_req).unwrap();

        debug!("sending: {:?}", &json);

        self.stream.write_all(json.as_bytes()).unwrap();
        // self.stream.flush().unwrap();

        //  debug!("sented: {}", bbb);
        let mut buf = [0; 1024];
        let size = self.stream.read(&mut buf)?;

        let txt = from_utf8(&buf[0..size]).unwrap();

        debug!("text {} , size {}", txt, size);

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
        // self.stream.flush().unwrap();

        //  debug!("sented: {}", bbb);
        let mut buf = [0; 1024];
        let size = self.stream.read(&mut buf)?;

        let txt = from_utf8(&buf[0..size]).unwrap();

        debug!("text {} , size {}", txt, size);

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

    pub fn get_balance(&mut self, account: &str) -> ResponseResult<f64> {
        let data_req = Request {
            payload: RequestPayload::GetBalance(GetBalanceAccountRequestParams {
                account: account.to_string(),
            }),
        };
        let json = serde_json::to_string(&data_req).unwrap();

        debug!("sending: {:?}", &json);

        self.stream.write_all(json.as_bytes()).unwrap();

        let mut buf = [0; 1024];
        let size = self.stream.read(&mut buf)?;

        let txt = from_utf8(&buf[0..size]).unwrap();

        debug!("text {} , size {}", txt, size);

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

    pub fn get_history(&mut self) -> ResponseResult<Vec<Operation>> {
        let data_req = Request {
            payload: RequestPayload::GetHistory(),
        };
        let json = serde_json::to_string(&data_req).unwrap();

        debug!("sending: {:?}", &json);

        self.stream.write_all(json.as_bytes()).unwrap();

        let mut buf = [0; 2048];
        let size = self.stream.read(&mut buf)?;

        let txt = from_utf8(&buf[0..size]).unwrap();

        debug!("text {} , size {}", txt, size);

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

    pub fn get_history_for_account(&mut self, account: &str) -> ResponseResult<Vec<Operation>> {
        let data_req = Request {
            payload: RequestPayload::GetHistoryForAccount(account.to_string()),
        };
        let json = serde_json::to_string(&data_req).unwrap();

        debug!("sending: {:?}", &json);

        self.stream.write_all(json.as_bytes()).unwrap();

        let mut buf = [0; 2048];
        let size = self.stream.read(&mut buf)?;

        let txt = from_utf8(&buf[0..size]).unwrap();

        debug!("text {} , size {}", txt, size);

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

pub type RequestResult = Result<String, RequestError>;
pub type ResponseResult<T> = Result<T, ResponseError>;

/// Error for request sending. It consists from two steps: sending and receiving data.
///
/// `SendError` caused by send data error.
/// `ReceiverError` caused by receive data error.
#[derive(Debug, Error)]
pub enum RequestError {
    #[error(transparent)]
    Send(#[from] SendError),
    #[error(transparent)]
    Receive(#[from] ReceiverError),
}

#[derive(Debug, Error)]
pub enum ResponseError {
    #[error("Generic error: {0}")]
    GenericError(#[from] GenericErrorData),
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Generic error: {0}")]
    UnexpectedResponse(#[from] UnexpectedResponseData),
}

// impl Display for ResponseError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         match self {
//             ResponseError::GenericError(data) => write!(f, "{}", data.error_message),
//         }
//     }
// }

#[derive(Debug, Error)]
pub struct GenericErrorData {
    error_message: String,
}

impl Display for GenericErrorData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error_message)
    }
}

#[derive(Debug, Error)]
pub struct UnexpectedResponseData {
    error_message: String,
}

impl Display for UnexpectedResponseData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error_message)
    }
}
