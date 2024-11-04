use dlwp::config::DLConfig;
use dlwp::distributor::{GET_DISTRIBUTOR, READ_AVAILABLE, USER_INIT};
use dlwp::id::distributor_id;
use dlwp::id::*;
use dlwp::stream::file::StreamFile;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::fs::{remove_file, File};

#[derive(Debug, Default, Hash, PartialEq, Eq)]
pub struct StreamInfo {
    pub id: LId,
    pub port: Port,
    pub did: DId,
    pub local: bool,
}

pub struct StreamsHandler {
    pub stream_info: HashMap<StreamInfo, StreamFile>,
}

impl StreamsHandler {
    pub fn new() -> Self {
        return StreamsHandler {
            stream_info: HashMap::new(),
        };
    }

}
