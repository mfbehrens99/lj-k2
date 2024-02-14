use serde::{Deserialize, Serialize};

use super::data::*;

#[derive(Debug, PartialEq, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum SendMessage {
    SendPresetCategoryDefinition {
        items: Vec<PresetCategory>,
    },
    SendPresetButtonDefinition {
        items: Vec<PresetButton>,
    },
    SendHoldActionDefinitions {
        items: Vec<HoldAction>,
    },
    SendFaderDefinition {
        items: Vec<Fader>,
    },
    SendFaderState {
        row: u8,
        column: u8,
        value: f32,
    },
    SendFaderHighlight {
        row: u8,
        column: u8,
        value: bool,
    },
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum ReceiveMessage {
    RequestPresetCategoryDefinitions,
    RequestPresetButtonDefinitions,
    RequestHoldActionDefinitions,
    SendHoldAction {
        row: u8,
        column: u8,
        value: bool,
    },
    SetPreset {
        row: u8,
        column: u8,
    },
    RequestFaderDefinitions,
    RequestFaderState {
        row: u8,
        column: u8,
    },
    PageLeft,
    PageRight,
}
