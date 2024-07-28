use crate::{Ma2Error, Result};

#[derive(Debug)]
pub enum MaRequest {
    Disconnect,
    SubscribeButton(Button, Button),
    SubscribeFader(Fader, Fader),
    SetButton(Button, ButtonState),
    SetFader(Fader, FaderState),
}

#[derive(Debug)]
pub enum MaEvent {
    Disconnected,
    LoginSuccessful(bool),
    ButtonChanged(Button, ButtonState),
    FaderChanged(Fader, FaderState),
    Error(Ma2Error),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Button(u8);

impl Button {
    pub fn id(&self) -> u8 {
        self.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ButtonRange {
    pub start_button: Button,
    pub num_buttons: u8,
}

impl ButtonRange {
    pub fn from(button1: Button, button2: Button) -> Result<Self> {
        if button1 > button2 {
            return Err(Ma2Error::InvalidButtonRange(button1, button2));
        }
        let num_buttons = button2.id() - button2.id() + 1;
        Ok(Self {
            start_button: button1,
            num_buttons,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Fader(u8);

impl Fader {
    pub fn id(&self) -> u8 {
        self.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FaderRange {
    pub start_fader: Fader,
    pub num_faders: u8,
}

impl FaderRange {
    pub fn from(fader1: Fader, fader2: Fader) -> Result<Self> {
        if fader1 > fader2 {
            return Err(Ma2Error::InvalidFaderRange(fader1, fader2));
        }
        let num_faders = fader2.id() - fader2.id() + 1;
        Ok(Self {
            start_fader: fader1,
            num_faders,
        })
    }
}

#[derive(Debug)]
pub enum ButtonState {
    Pressed,
    Released,
}

#[derive(Debug)]
pub struct FaderState {
    value: f32,
    touched: bool,
}
