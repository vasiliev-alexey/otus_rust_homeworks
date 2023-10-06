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

/// Represents an error that occurs while sending data.
#[derive(Debug, Error)]
pub enum SendError {
    /// An IO error with the specified underlying error.
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

/// Represents an error that occurs while receiving data. It includes IO and encoding errors.
#[derive(Debug, Error)]
pub enum ReceiverError {
    /// An IO error with the specified underlying error.
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    /// A bad encoding error.
    #[error("bad encoding")]
    BadEncoding,
}
