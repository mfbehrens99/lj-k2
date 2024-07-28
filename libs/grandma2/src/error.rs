use std::fmt;

use crate::{
    interface_msg::{Button, Fader, MaRequest},
    ma2_msg::{ReceiveMsg, SendMsg},
};

pub type Result<T> = std::result::Result<T, Ma2Error>;

#[derive(Debug)]
pub enum Ma2Error {
    // WS Connection
    FailedToConnect {
        url: String,
        tungstenite_error: tungstenite::Error,
    },
    NotYetConnected,
    // Ma Connection Errors
    ConnectedButInvalidSessionId,
    WebRemoteDisabled,
    FailedToSend(tungstenite::Error),
    // Handler not implemented
    MessageHandlerNotImplemented(ReceiveMsg),
    RequestHandlerNotImplemented(MaRequest),
    // Interface Channels closed
    RequestChannelClosed,
    EventChannelClosed,
    // Ma2 errors
    LoginFailed {
        username: String,
        password: String,
        hashed_password: String,
    },
    InvalidButtonRange(Button, Button),
    InvalidFaderRange(Fader, Fader),
    // Serde errors
    SerializeError(SendMsg),
    DeserialzeError(String),
}

impl fmt::Display for Ma2Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GrandMa2 raised an Error: {:?}", self)
    }
}
