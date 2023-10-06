/// The server path for the TCP listener.
///
/// This constant represents the IP address and port number on which the server will listen for incoming connections.
/// It is set to "127.0.0.1:3333" by default.
pub const SERVER_PATH: &str = "127.0.0.1:3333";

/// The buffer size for reading from and writing to the TCP stream.
///
/// This constant represents the size of the buffer used for reading from and writing to the TCP stream.
/// It is set to 1024 bytes by default.
pub const BUFFER_SIZE: usize = 1024;

/// The log level for the logging framework.
///
/// This constant represents the log level for the logging framework used in the program.
/// It is set to "debug" by default.
pub const LOG_LEVEL: &str = "debug";
