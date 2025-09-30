#[test]
fn test_system_info_payload() {
    let system_info = super::SystemInfo::new(
        8,
        (4, 1, 2),
        super::CpuInfo::new(2, vec![10, 20], vec![1000, 2000], 15, 1500),
        super::MemoryInfo::new(8192, 4096, 4096, 2048),
        vec![super::DiskInfo::new("/dev/sda1".to_string(), 256, 128)],
        vec![super::NetworkInfo::new("eth0".to_string(), 1024, 512)],
    );

    let payload =
        crate::payload::Payload::new(crate::payload::action::PayloadAction::Info, system_info);

    let json = payload
        .to_json()
        .expect("Failed to serialize payload to JSON");

    let expected_json = serde_json::json!({
        "action": "info",
        "data": {
            "uptime": 8,
            "load_avg": [4, 1, 2],
            "cpu": {
                "cores": 2,
                "usages": [10, 20],
                "freqs": [1000, 2000],
                "usage_avg": 15,
                "freq_avg": 1500
            },
            "memory": {
                "ram": 8192,
                "ram_used": 4096,
                "swap": 4096,
                "swap_used": 2048
            },
            "disks": [
                {
                    "mount": "/dev/sda1",
                    "total": 256,
                    "free": 128,
                }
            ],
            "networks": [
                {
                    "interface": "eth0",
                    "rx": 1024,
                    "tx": 512
                }
            ]
        }
    });

    assert_eq!(json, expected_json);
}
