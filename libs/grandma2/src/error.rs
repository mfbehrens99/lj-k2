use std::fmt;

use crate::interface::MaRequest;
use crate::client::{ReceiveMsg, SendMsg};
use crate::types::{ButtonExecutor, FaderExecutor};
use crate::Executor;

pub type Result<T> = std::result::Result<T, Box<Ma2Error>>;

#[derive(Debug)]
pub enum Ma2Error {
    // WS Connection
    FailedToConnect {
        url: String,
        tungstenite_error: tungstenite::Error,
    },
    WebsocketNotYetConnected,
    // Ma Connection Errors
    ConnectedButInvalidSessionId,
    WebRemoteDisabled,
    WebsocketFailedToSend(tungstenite::Error),
    // Messages/Handler not implemented
    CouldNotDeserializeReceiveMsg(String, Box<serde_json::Error>),
    CouldNotSerializeSendMsg(SendMsg),
    MessageHandlerNotImplemented(ReceiveMsg),
    RequestHandlerNotImplemented(MaRequest),
    // Interface Channels closed
    NotYetConnected,
    RequestChannelClosed,
    EventChannelClosed,
    // Ma2 errors
    LoginFailed {
        username: String,
        password: String,
        hashed_password: String,
    },
    InvalidButtonRange(ButtonExecutor, ButtonExecutor),
    InvalidFaderRange(FaderExecutor, FaderExecutor),
    // Serde errors
    SerializeError(SendMsg),
    DeserialzeError(String),
    // Types
    ButtonIdOutOfRange(Executor),
    FaderIdOutOfRange(Executor),
}

impl fmt::Display for Ma2Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GrandMa2 raised an Error: {:?}", self)
    }
}
