use crate::id::DId;

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub struct DLConfig {
    /// Using TCP for communication
    pub tcp: bool,
    /// Using a serial device for communication
    pub serial: bool,
    /// Path to serial device
    pub serial_path: &'static str,
    /// Closed from receiving and transmitting
    pub closed: bool,
    /// IP address (if ``tcp`` is ``true``)
    pub ip_address: &'static str,
    pub public_instance_id: u32,
}

impl DLConfig {
    pub const fn empty() -> Self {
        return DLConfig {
            tcp: false,
            serial: false,
            serial_path: "",
            closed: true,
            ip_address: "",
            public_instance_id: 0,
        };
    }
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct DistributorConfig {
    pub tcp_connections: Vec<String>,
    pub serial_connections: Vec<String>,
    pub bind: String,
    pub max_users: u16,
}
