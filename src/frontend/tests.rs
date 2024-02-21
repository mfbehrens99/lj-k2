use serde_json::Value;

use super::*;
use super::data::*;

#[test]
fn test_receive_message() {
    let str = r#"{"type": "requestPresetCategoryDefinitions"}"#;
    let json: ReceiveMessage = serde_json::from_str(str).unwrap();
    assert_eq!(json, ReceiveMessage::RequestPresetCategoryDefinitions);

    let str = r#"{"type": "requestPresetButtonDefinitions"}"#;
    let json: ReceiveMessage = serde_json::from_str(str).unwrap();
    assert_eq!(json, ReceiveMessage::RequestPresetButtonDefinitions);

    let str = r#"{"type": "setPreset", "row": 3, "column": 1}"#;
    let json: ReceiveMessage = serde_json::from_str(str).unwrap();
    assert_eq!(json, ReceiveMessage::SetPreset { row: 3, column: 1 });

    let str = r#"{"type": "requestHoldActionDefinitions"}"#;
    let json: ReceiveMessage = serde_json::from_str(str).unwrap();
    assert_eq!(json, ReceiveMessage::RequestHoldActionDefinitions);

    let str = r#"{"type": "sendHoldAction", "row": 2, "column": 0, "value": true}"#;
    let json: ReceiveMessage = serde_json::from_str(str).unwrap();
    assert_eq!(json, ReceiveMessage::SendHoldAction { row: 2, column: 0, value: true});

    let str = r#"{"type": "requestFaderDefinitions"}"#;
    let json: ReceiveMessage = serde_json::from_str(str).unwrap();
    assert_eq!(json, ReceiveMessage::RequestFaderDefinitions);

    let str = r#"{"type": "requestFaderState", "row": 3, "column": 2}"#;
    let json: ReceiveMessage = serde_json::from_str(str).unwrap();
    assert_eq!(
        json,
        ReceiveMessage::RequestFaderState { row: 3, column: 2 }
    );
}

#[test]
fn test_send_message() {
    let message = SendMessage::SendPresetCategoryDefinitions {
        items: &[data::PresetCategory::new(7, "Bar")],
    };
    let json_string = serde_json::to_string(&message).unwrap();
    let json: Value = serde_json::from_str(&json_string).unwrap();
    let json_test = serde_json::json!({
        "type": "sendPresetCategoryDefinitions",
        "items": [{
            "row": 7,
            "text": "Bar",
        }]
    });
    assert_eq!(json_test, json);

    let message = SendMessage::SendPresetButtonDefinitions {
        items: &[PresetButton::new("Hallo Welt!", 2, 4, Icon::Chill, "#000000")],
    };
    let json_string = serde_json::to_string(&message).unwrap();
    let json: Value = serde_json::from_str(&json_string).unwrap();
    let json_test = serde_json::json!({
        "type": "sendPresetButtonDefinitions",
        "items": [{
            "row": 2,
            "column": 4,
            "icon": "chill",
            "color": "#000000",
            "text": "Hallo Welt!",
        }],
    });
    assert_eq!(json_test, json);

    let message = SendMessage::SendHoldActionDefinitions {
        items: &[HoldAction::new("Test", 2, 3, Icon::Party, "#111111")],
    };
    let json_string = serde_json::to_string(&message).unwrap();
    let json: Value = serde_json::from_str(&json_string).unwrap();
    let json_test = serde_json::json!({
        "type": "sendHoldActionDefinitions",
        "items": [{
            "row": 2,
            "column": 3,
            "icon": "party",
            "color": "#111111",
            "text": "Test",
        }],
    });
    assert_eq!(json_test, json);

    let message = SendMessage::SendFaderDefinitions {
        items: &[Fader::new(5, 1)],
    };
    let json_string = serde_json::to_string(&message).unwrap();
    let json: Value = serde_json::from_str(&json_string).unwrap();
    let json_test = serde_json::json!({
        "type": "sendFaderDefinitions",
        "items": [{
            "row": 5,
            "column": 1,
            "icon": "none",
            "color": "#000000",
            "text": "",
        }],
    });
    assert_eq!(json_test, json);

    let message = SendMessage::SendFaderState {
        row: 0,
        column: 5,
        value: 0.4,
    };
    let json_string = serde_json::to_string(&message).unwrap();
    let json: Value = serde_json::from_str(&json_string).unwrap();
    let json_test = serde_json::json!({
        "type": "sendFaderState",
        "row": 0,
        "column": 5,
        "value": 0.4,
    });
    assert_eq!(json_test, json);

    let message = SendMessage::SendFaderHighlight {
        row: 1,
        column: 4,
        value: true,
    };
    let json_string = serde_json::to_string(&message).unwrap();
    let json: Value = serde_json::from_str(&json_string).unwrap();
    let json_test = serde_json::json!({
        "type": "sendFaderHighlight",
        "row": 1,
        "column": 4,
        "value": true,
    });
    assert_eq!(json_test, json);
}