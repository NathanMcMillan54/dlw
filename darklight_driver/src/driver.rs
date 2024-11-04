use crate::streams::StreamsHandler;
use dlwp::{
    config::DLConfig, distributor::READ_AVAILABLE, message::contents_to_string,
    serialport::posix::TTYPort,
};
use std::{
    io::{Read, Write},
    net::TcpStream,
};

pub trait Test: Copy + Read + Write {
    fn empty() {}
}

pub struct DarkLightDriver {
    pub streams_handler: StreamsHandler,
    pub config: DLConfig,
    pub tcp_stream: Option<TcpStream>,
    pub serial_port: Option<dlwp::serialport::posix::TTYPort>,
}

impl DarkLightDriver {
    pub fn empty() -> Self {
        return DarkLightDriver {
            streams_handler: StreamsHandler::new(),
            config: DLConfig::empty(),
            tcp_stream: None,
            serial_port: None,
        };
    }

    pub fn new(streams_handler: StreamsHandler, config: DLConfig) -> Self {
        return DarkLightDriver {
            streams_handler,
            config,
            tcp_stream: None,
            serial_port: None,
        };
    }
}

pub fn read<R: Read>(mut stream: &mut R) -> [u8; 4096] {
    let mut buf = [0; 4096];
    stream.read(&mut buf).unwrap_or(0);

    buf
}

// Returns a message if one is received while waiting for send
pub fn write<RW: Read + Write>(mut stream: &mut RW, write: String, wait: bool) -> Option<String> {
    if wait == true {
        let mut read_bytes = read(stream);

        while read_bytes == [0; 4096] {
            read_bytes = read(stream);
        }

        let read_str = contents_to_string(read_bytes);
        if read_str.contains(READ_AVAILABLE) {
            stream.write(write.as_bytes()).unwrap();
            stream.flush().unwrap();
        } else {
            return Some(read_str);
        }
    }

    stream.write(write.as_bytes()).unwrap();
    stream.flush().unwrap();

    None
}
