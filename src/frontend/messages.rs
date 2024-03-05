use serde::{Deserialize, Serialize};

use super::data::*;

#[derive(Debug, PartialEq, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum SendMessage {
    SendPresetCategoryDefinitions { items: Box<[PresetCategory]> },
    SendPresetButtonDefinitions { items: Box<[PresetButton]> },
    SendHoldActionDefinitions { items: Box<[HoldAction]> },
    SendFaderDefinitions { items: Box<[Fader]> },
    SendFaderState { row: u8, column: u8, value: f32 },
    SendFaderHighlight { row: u8, column: u8, value: bool },
    Heartbeat,
}

#[derive(Debug, PartialEq, Clone, Copy, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum ReceiveMessage {
    RequestPresetCategoryDefinitions,
    RequestPresetButtonDefinitions,
    RequestHoldActionDefinitions,
    RequestFaderDefinitions,
    SendHoldAction { row: u8, column: u8, value: bool },
    SetPreset { row: u8, column: u8 },
    SendFaderState { row: u8, column: u8, state: f32 },
    RequestFaderState { row: u8, column: u8 },
    PageLeft,
    PageRight,
}
