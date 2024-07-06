use crate::config::DLConfig;

#[test]
fn test_read_dlconfig() {
    let config = DLConfig::empty();
    let json = serde_json::to_string_pretty(&config).unwrap();
    let json_string = "{\n  \"tcp\": false,\n  \"serial\": false,\n  \"serial_path\": \"\",\n  \"closed\": true,\n  \"ip_address\": \"\",\n  \"public_instance_id\": 0\n}";

    assert_eq!(json_string, json);
}
