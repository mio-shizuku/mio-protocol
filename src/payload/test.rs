use serde::{Deserialize, Serialize};

use crate::payload::{Payload, action::PayloadAction};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct TestProduct {
    id: u32,
    name: String,
    items: Vec<String>,
}

#[test]
fn test_payload_json_serialize() {
    let product = TestProduct {
        id: 1,
        name: "Test Product".to_string(),
        items: vec!["Item1".to_string(), "Item2".to_string()],
    };

    let payload = Payload::new(PayloadAction::Test, product);
    let json = payload
        .to_json()
        .expect("Failed to serialize payload to JSON");

    let expected_json = serde_json::json!({
        "action": "test",
        "data": {
            "id": 1,
            "name": "Test Product",
            "items": ["Item1", "Item2"]
        }
    });

    assert_eq!(json, expected_json);
}

#[test]
fn test_payload_string_serialize() {
    let product = TestProduct {
        id: 2,
        name: "Test Product".to_string(),
        items: vec!["Item1".to_string(), "Item2".to_string()],
    };

    let payload = Payload::new(PayloadAction::Test, product);
    let json_string = payload
        .to_json_string()
        .expect("Failed to serialize payload to JSON string");

    let expected_json_string =
        r#"{"action":"test","data":{"id":2,"name":"Test Product","items":["Item1","Item2"]}}"#;

    assert_eq!(json_string, expected_json_string);
}

#[test]
fn test_payload_json_deserialize() {
    let json_data = serde_json::json!({
        "action": "test",
        "data": {
            "id": 3,
            "name": "Another Product",
            "items": ["ItemA", "ItemB"]
        }
    });

    let payload = Payload::<TestProduct>::from_json(json_data)
        .expect("Failed to deserialize JSON to payload");

    let expected_payload = super::Payload::new(
        PayloadAction::Test,
        TestProduct {
            id: 3,
            name: "Another Product".to_string(),
            items: vec!["ItemA".to_string(), "ItemB".to_string()],
        },
    );

    assert_eq!(payload, expected_payload);
}

#[test]
fn test_payload_string_deserialize() {
    let json_string =
        r#"{"action":"test","data":{"id":4,"name":"String Product","items":["ItemX","ItemY"]}}"#;

    let payload = Payload::<TestProduct>::from_json_string(json_string)
        .expect("Failed to deserialize JSON string to payload");

    let expected_payload = super::Payload::new(
        PayloadAction::Test,
        TestProduct {
            id: 4,
            name: "String Product".to_string(),
            items: vec!["ItemX".to_string(), "ItemY".to_string()],
        },
    );

    assert_eq!(payload, expected_payload);
}
