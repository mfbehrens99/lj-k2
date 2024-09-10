use crate::{
    types::{ButtonData, FaderData},
    types::{ButtonExecutor, FaderExecutor},
    Ma2Error,
};

/// A request that can be send to the GrandMa2 client in order to trigger an action.
#[derive(Debug)]
pub enum MaRequest {
    Disconnect,
    SubscribeButton(ButtonExecutor, ButtonExecutor),
    SubscribeFader(FaderExecutor, FaderExecutor),
    SetButton(ButtonExecutor, ButtonData),
    SetFader(FaderExecutor, FaderData),
}

/// A event that can be read by the user to trigger further actions.
#[derive(Debug)]
pub enum MaEvent {
    Disconnected,
    LoginSuccessful(bool),
    FaderChanged(FaderData),
    ButtonChanged(ButtonData),
    Error(Ma2Error),
}
