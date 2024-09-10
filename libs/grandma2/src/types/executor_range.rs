use crate::{Ma2Error, Result};

use super::executor::{ButtonExecutor, FaderExecutor};

#[derive(Debug, Clone, Copy)]
pub struct ButtonRange {
    pub start_button: ButtonExecutor,
    pub num_buttons: u16,
}

impl ButtonRange {
    pub fn from(button1: ButtonExecutor, button2: ButtonExecutor) -> Result<Self> {
        if button1 > button2 {
            return Err(Box::new(Ma2Error::InvalidButtonRange(button1, button2)).into());
        }
        let num_buttons = button2.id() - button1.id() + 1;
        Ok(Self {
            start_button: button1,
            num_buttons,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FaderRange {
    pub start_fader: FaderExecutor,
    pub num_faders: u16,
}

impl FaderRange {
    pub fn from(fader1: FaderExecutor, fader2: FaderExecutor) -> Result<Self> {
        if fader1 > fader2 {
            return Err(Ma2Error::InvalidFaderRange(fader1, fader2).into());
        }
        let num_faders = fader2.id() - fader1.id() + 1;
        Ok(Self {
            start_fader: fader1,
            num_faders,
        })
    }
}