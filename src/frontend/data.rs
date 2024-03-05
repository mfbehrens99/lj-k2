use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Debug, Clone, Deserialize, Serialize)]
pub enum HoldActionId {}

#[derive(Eq, PartialEq, Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Icon {
    None,
    Off,

    Sun,
    Right,
    Left,
    Rainbow,

    Chill,
    Party,
    Rave,

    #[serde(rename = "led-bars")]
    LEDBars,
    Sunstrip,
    MovingHead,
    CounterFront,
    Hexagon,

    CounterBack,
    Bottles,
    Bulb,
}

#[derive(Eq, PartialEq, Debug, Clone, Deserialize, Serialize)]
pub struct PresetCategory {
    row: u8,
    text: String,
}

impl PresetCategory {
    pub fn new(row: u8, text: &str) -> PresetCategory {
        PresetCategory {
            row,
            text: text.to_owned(),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Deserialize, Serialize)]
pub struct PresetButton {
    text: String,
    pub row: u8,
    pub column: u8,
    icon: Icon,
    color: String,
}

impl PresetButton {
    pub fn new(text: &str, row: u8, column: u8, icon: Icon, color: &str) -> PresetButton {
        PresetButton {
            text: text.to_owned(),
            row,
            column,
            icon,
            color: color.to_owned(),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Deserialize, Serialize)]
pub struct HoldAction {
    text: String,
    row: u8,
    column: u8,
    icon: Icon,
    color: String,
}

impl HoldAction {
    pub fn new(text: &str, row: u8, column: u8, icon: Icon, color: &str) -> Self {
        HoldAction {
            text: text.to_owned(),
            row,
            column,
            icon,
            color: color.to_owned(),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Deserialize, Serialize)]
pub struct Fader {
    pub text: String,
    row: u8,
    column: u8,
    pub icon: Icon,
    pub color: String,
}

impl Fader {
    pub fn new(text: &str, row: u8, column: u8, icon: Icon, color: &str) -> Self {
        Fader {
            text: text.to_owned(),
            row,
            column,
            icon,
            color: color.to_owned(),
        }
    }
}
