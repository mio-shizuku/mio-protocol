#[test]
fn test_system_info_payload() {
    let system_info = super::SystemInfo::new(
        8,
        (4, 1, 2),
        super::CpuInfo::new(2, vec![10, 20], vec![1000, 2000], 15, 1500),
        super::MemoryInfo::new(8192, 4096, 4096, 2048),
        vec![super::DiskInfo::new(
            "/dev/sda1".to_string(),
            256,
            128
        )],
        vec![super::NetworkInfo::new("eth0".to_string(), 1024, 512)],
    );
    let payload = crate::payload::Payload::new(crate::payload::action::PayloadAction::Info, system_info);
    let json = payload
        .to_json()
        .expect("Failed to serialize payload to JSON");
    let expected_json = serde_json::json!({
        "action": "info",
        "data": {
            "uptime_secs": 8,
            "load_avg": [4, 1, 2],
            "cpu": {
                "cpu_cores": 2,
                "cpu_usage": [10, 20],
                "cpu_freq": [1000, 2000],
                "average_cpu_usage": 15,
                "average_cpu_freq": 1500
            },
            "memory": {
                "total_memory_gb": 8192,
                "used_memory_gb": 4096,
                "total_swap_gb": 4096,
                "used_swap_gb": 2048
            },
            "disks": [
                {
                    "mount_point": "/dev/sda1",
                    "total_gb": 256,
                    "available_gb": 128,
                }
            ],
            "networks": [
                {
                    "name": "eth0",
                    "received_mb": 1024,
                    "transmitted_mb": 512
                }
            ]
        }
    });
    assert_eq!(json, expected_json);
}