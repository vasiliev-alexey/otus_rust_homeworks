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

#[derive(Debug, Error)]
pub enum ProcessingErrorsResult {
    /// An RecvError error with the specified underlying error.
    #[error("RecvError error: {0}")]
    RecvError(#[from] std::sync::mpsc::RecvError),
    /// An IO error with the specified underlying error.
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("TypeMismatchError error: {0}")]
    TypeMismatchError(String),
}
