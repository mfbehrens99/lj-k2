use serde::{Deserialize, Serialize};

use super::data::*;

#[derive(Debug, PartialEq, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum SendMessage<'a> {
    SendPresetCategoryDefinitions {
        items: &'a [PresetCategory],
    },
    SendPresetButtonDefinitions {
        items: &'a [PresetButton],
    },
    SendHoldActionDefinitions {
        items: &'a [HoldAction],
    },
    SendFaderDefinitions {
        items: &'a [Fader],
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
    Heartbeat,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum ReceiveMessage {
    RequestPresetCategoryDefinitions,
    RequestPresetButtonDefinitions,
    RequestHoldActionDefinitions,
    RequestFaderDefinitions,
    SendHoldAction {
        row: u8,
        column: u8,
        value: bool,
    },
    SetPreset {
        row: u8,
        column: u8,
    },
    SendFaderState {
        row: u8,
        column: u8,
        state: f32,
    },
    RequestFaderState {
        row: u8,
        column: u8,
    },
    PageLeft,
    PageRight,
}
