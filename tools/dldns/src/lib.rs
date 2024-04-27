use dlwp::id::{DId, LId, Port};

pub mod get;
pub mod owner;

pub const DNS_ID: LId = 1000;
pub const DNS_PORT: Port = 4999;
pub const DNS_DISTRIBUTOR: DId = 3;
pub const OWNERS_LIST: &str = "owners_list.json";
