use dlwp::cerpton::{libcerpton_decode, libcerpton_encode};
use dlwp::codes::{READ_SUCCESS, WRITE_FAILED, WRITE_TIMEDOUT};
use dlwp::config::DLConfig;
use dlwp::distributor::{GET_DISTRIBUTOR, READ_AVAILABLE, USER_INIT};
use dlwp::id::distributor_id;
use dlwp::id::*;
use dlwp::io::{DLSerialIO, DLIO, DLTCPIO};
use dlwp::message::{fmt_message_recv, fmt_message_recv_rm, fmt_message_send, Message};
use dlwp::serialport::posix::TTYPort;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::fs::{remove_file, File};
use std::io::{Read, Write};
use std::net::SocketAddr;
use std::path::Path;
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;

#[allow(dead_code)]
pub struct Stream {
    pub received: Vec<Message>,
    pub pending: Vec<String>,
    pub info: [i32; 6],
}

impl Stream {
    pub fn new(info: [i32; 6]) -> Self {
        return Stream {
            received: vec![],
            pending: vec![],
            info,
        };
    }
}

#[derive(Debug, Default, Hash, PartialEq, Eq)]
pub struct StreamInfo {
    pub id: LId,
    pub port: Port,
    pub did: DId,
    pub local: bool,
}

impl StreamInfo {
    pub fn create_file(&self) {
        File::options().read(true).write(true).create(true).open(&format!("/tmp/darklight/connections/_dl_{}-{}", self.id, self.port)).expect("Failed to create stream file");
    }

    pub fn remove_file(&self) {
        remove_file(&format!("/tmp/darklight/connections/_dl_{}-{}", self.id, self.port)).expect("Failed to remove file");
    }
}

pub struct StreamsHandler {
    pub stream_info: HashMap<StreamInfo, Stream>,
}

impl StreamsHandler {
    pub fn new() -> Self {
        return StreamsHandler {
            stream_info: HashMap::new(),
        };
    }

}
