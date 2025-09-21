use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct TestProduct {
    id: u32,
    name: String,
    items: Vec<String>,
}

#[test]
fn test_payload_serialize() {
    let product = TestProduct {
        id: 1,
        name: "Test Product".to_string(),
        items: vec!["Item1".to_string(), "Item2".to_string()],
    };

    let payload = super::Payload::new(product);
    let json = match payload.to_json() {
        Ok(json_value) => json_value,
        Err(e) => panic!("Failed to serialize payload to JSON: {:?}", e),
    };

    let expected_json = serde_json::json!({
        "data": {
            "id": 1,
            "name": "Test Product",
            "items": ["Item1", "Item2"]
        }
    });

    assert_eq!(json, expected_json);
}

#[test]
fn test_payload_deserialize() {
    let json_data = serde_json::json!({
        "data": {
            "id": 2,
            "name": "Another Product",
            "items": ["ItemA", "ItemB"]
        }
    });

    let payload: super::Payload<TestProduct> = match super::Payload::from_json(json_data) {
        Ok(p) => p,
        Err(e) => panic!("Failed to deserialize JSON to payload: {:?}", e),
    };

    let expected_payload = super::Payload::new(TestProduct {
        id: 2,
        name: "Another Product".to_string(),
        items: vec!["ItemA".to_string(), "ItemB".to_string()],
    });

    assert_eq!(payload, expected_payload);
}
