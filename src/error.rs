//! Error types for the MQTT client

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("WebSocket connection error: {0}")]
    WebSocketError(String),

    #[error("MQTT protocol error: {0}")]
    ProtocolError(String),

    #[error("Connection closed")]
    ConnectionClosed,

    #[error("Connection not established")]
    NotConnected,

    #[error("Invalid packet format")]
    InvalidPacket,

    #[error("Buffer overflow")]
    BufferOverflow,

    #[error("IO error: {0}")]
    Io(String),

    #[error("Other error: {0}")]
    Other(String),
}

// Removed ewebsock dependency
