use std::io;
use thiserror::Error;

pub type ConnectResult<T> = Result<T, ConnectError>;

/// Represents an error that occurs during a connection. It includes IO and handshake errors.
#[derive(Debug, Error)]
pub enum ConnectError {
    /// An unexpected handshake response with the specified error message.
    #[error("Unexpected handshake response: {0}")]
    BadHandshake(String),

    /// An IO error with the specified underlying error.
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}
