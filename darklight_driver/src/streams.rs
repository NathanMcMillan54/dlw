use dlwp::config::DLConfig;
use dlwp::distributor::{GET_DISTRIBUTOR, READ_AVAILABLE, USER_INIT};
use dlwp::id::distributor_id;
use dlwp::id::*;
use dlwp::stream::file::{ReceivedMessage, StreamFile};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::fs::{remove_file, File};

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone)]
pub struct StreamInfo {
    pub id: LId,
    pub port: Port,
    pub did: DId,
    pub local: bool,
}

#[derive(Clone)]
pub struct StreamsHandler {
    pub streams: HashMap<StreamInfo, StreamFile>,
}

impl StreamsHandler {
    pub fn new() -> Self {
        return StreamsHandler {
            streams: HashMap::new(),
        };
    }

    pub fn handle_local_streams(&mut self) {
        for (stream, file) in self.streams.iter_mut() {
            file.read_and_set();
        }
    }

    // Does not do error handling, check if ``streaminfo`` exists first
    pub fn add_received_message(&mut self, streaminfo: StreamInfo, message: ReceivedMessage) {
        self.streams
            .get_mut(&streaminfo)
            .unwrap()
            .received
            .push(message);
    }
}
