use serde::Serialize;

#[derive(Serialize)]
struct TestProduct {
    id: u32,
    name: String,
    items: Vec<String>,
}

unsafe impl Send for TestProduct {}
unsafe impl Sync for TestProduct {}

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
        "id": 1,
        "name": "Test Product",
        "items": ["Item1", "Item2"]
    });

    assert_eq!(json, expected_json);
}
