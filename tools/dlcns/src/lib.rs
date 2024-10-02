#![doc = include_str!("../README.md")]
extern crate dlwp;

use dlwp::id::{DId, LId, Port};

pub mod get;
pub mod name;

pub const CNS_ID: LId = 9711410197108107101;
pub const CNS_PORT: Port = 4999;
pub const CNS_DISTRIBUTOR: DId = 3;
pub const OWNERS_LIST: &str = "owners_list.json";
