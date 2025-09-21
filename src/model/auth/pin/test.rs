#[test]
fn test_pin_payload() {
    let pin = super::Pin::new("123456".to_string());

    let payload = crate::payload::Payload::new(crate::payload::action::PayloadAction::Auth, pin);

    let json = payload
        .to_json()
        .expect("Failed to serialize payload to JSON");

    let expected_json = serde_json::json!({
        "action": "auth",
        "data": {
            "code": "123456"
        }
    });

    assert_eq!(json, expected_json);
}
