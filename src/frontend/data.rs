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
pub struct PresetCategory<'a> {
    row: u8,
    text: &'a str,
}

impl<'a> PresetCategory<'a> {
    pub fn new(row: u8, text: &'a str) -> PresetCategory<'a> {
        PresetCategory { row, text }
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Deserialize, Serialize)]
pub struct PresetButton<'a> {
    text: &'a str,
    pub row: u8,
    pub column: u8,
    icon: Icon,
    color: &'a str,
}

impl<'a> PresetButton<'a> {
    pub fn new(text: &'a str, row: u8, column: u8, icon: Icon, color: &'a str) -> PresetButton<'a> {
        PresetButton {
            text,
            row,
            column,
            icon,
            color,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Deserialize, Serialize)]
pub struct HoldAction<'a> {
    text: &'a str,
    row: u8,
    column: u8,
    icon: Icon,
    color: &'a str,
}

impl<'a> HoldAction<'a> {
    pub fn new(text: &'a str, row: u8, column: u8, icon: Icon, color: &'a str) -> HoldAction<'a> {
        HoldAction {
            text,
            row,
            column,
            icon,
            color,
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
