use serde::{Deserialize, Serialize};


#[derive(Eq, PartialEq, Debug, Clone, Deserialize, Serialize)]
pub enum HoldActionId {}


#[derive(Eq, PartialEq, Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Icon {
    None,
    Off,
    Sun,
    Chill,
    Party,
    Rave,
    Rainbow,
    Left,
    Right,
    Hexagon,
    Sunstrip,
    Bulb,
    MovingHead,
}

#[derive(Eq, PartialEq, Debug, Clone, Deserialize, Serialize)]
pub struct PresetCategory {
    row: u8,
    text: String,
}

impl PresetCategory {
    pub fn new(row: u8, text: String) -> Self {
        PresetCategory { row, text }
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Deserialize, Serialize)]
pub struct PresetButton {
    row: u8,
    column: u8,
    pub icon: Icon,
    pub color: String,
    pub text: String,
}

impl PresetButton {
    pub fn new(row: u8, column: u8) -> Self {
        PresetButton {
            row,
            column,
            icon: Icon::None,
            color: "#000000".to_string(),
            text: "".to_string(),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Deserialize, Serialize)]
pub struct HoldAction {
    row: u8,
    column: u8,
    pub icon: Icon,
    pub color: String,
    pub text: String,
}

impl HoldAction {
    pub fn new(row: u8, column: u8) -> Self {
        HoldAction {
            row,
            column,
            icon: Icon::None,
            color: "#000000".to_string(),
            text: "".to_string(),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Deserialize, Serialize)]
pub struct Fader {
    row: u8,
    column: u8,
    pub icon: Icon,
    pub color: String,
    pub text: String,
}

impl Fader {
    pub fn new(row: u8, column: u8) -> Self {
        Fader {
            row,
            column,
            icon: Icon::None,
            color: "#000000".to_string(),
            text: "".to_string(),
        }
    }
}
