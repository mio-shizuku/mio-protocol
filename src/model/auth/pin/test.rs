#[test]
fn test_pin_payload() {
    let pin = super::Pin::new("123456".to_string());

    let payload = crate::payload::Payload::new(crate::payload::action::PayloadAction::Auth, pin);

    let json = match payload.to_json() {
        Ok(json_value) => json_value,
        Err(e) => panic!("Failed to serialize payload to JSON: {:?}", e),
    };

    let expected_json = serde_json::json!({
        "action": "auth",
        "data": {
            "code": "123456"
        }
    });

    assert_eq!(json, expected_json);
}
