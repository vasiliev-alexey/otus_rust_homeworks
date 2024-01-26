/// The server path for the TCP listener.
///
/// This constant represents the IP address and port number on which the server will listen for incoming connections.
/// It is set to "127.0.0.1:3333" by default.
pub const SERVER_ADDRESS: &str = "127.0.0.1:3333";

/// The maximum number of bytes that can be sent in a single chunk.
pub const MAX_CHUNK_BYTE_SIZE: usize = 1024;

/// The log level for the logging framework.
///
/// This constant represents the log level for the logging framework used in the program.
/// It is set to "debug" by default.
pub const LOG_LEVEL: &str = "debug";
