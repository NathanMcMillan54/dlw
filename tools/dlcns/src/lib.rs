use dlwp::id::{DId, LId, Port};

pub mod get;
pub mod owner;

pub const CNS_ID: LId = 505051114;
pub const CNS_PORT: Port = 4999;
pub const CNS_DISTRIBUTOR: DId = 3;
pub const OWNERS_LIST: &str = "owners_list.json";
