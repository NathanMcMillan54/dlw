use dlwp::{codes::{INVALID_RR, STATUS_OK}, distributor::DIST_INIT};
use lib_dldistributor::external::{ExternalDistributorInfo, ExternalDistributorRW};
use std::{borrow::Borrow, io::{Read, Write}, net::TcpStream, time::Duration};

use crate::{distributor::encrpytion::current_encryption, VS1, VS2, VS3};

pub struct TcpDistributor {
    pub stream: TcpStream,
    pub info: ExternalDistributorInfo,
}

impl TcpDistributor {
    pub fn new_and_check(stream: TcpStream) -> Option<Self> {
        let mut tcp_dist = TcpDistributor {
            stream: stream,
            info: ExternalDistributorInfo::default(),
        };

        return if tcp_dist.verify_distributor() {
            Some(tcp_dist)
        } else {
            None
        };
    }

    pub fn new(stream: TcpStream) -> Self {
        return TcpDistributor {
            stream,
            info: ExternalDistributorInfo::default(),
        };
    }
}

impl ExternalDistributorRW for TcpDistributor {
    fn write(&mut self, _inputs: String) -> dlwp::codes::Code {
        let encryption = current_encryption();
        let input = (encryption.encode_function)(encryption.info, _inputs);

        self.stream.write(input.as_bytes());
        self.stream.flush();

        // TODO: depending on what write or flush returns give a proper ``Code``
        STATUS_OK
    }

    fn read(&mut self) -> (String, dlwp::codes::Code) {
        let mut buf = [0; 4096];

        self.stream.read(&mut buf);

        let ret = String::from_utf8(buf.to_vec());
        
        if ret.is_err() { // Received invalid characters
            return (String::new(), INVALID_RR);
        }

        let encryption = current_encryption();

        return ((encryption.decode_function)(encryption.info, ret.unwrap()), STATUS_OK);
    }

    // Gets called when a distributor attempts to connect
    fn verify_distributor(&mut self) -> bool {
        if !unsafe { crate::DISTRIBUTOR.as_ref().unwrap().info.config.tcp_connections.contains(&self.stream.peer_addr().unwrap().to_string()) } {
            return false;
        }

        self.stream.set_read_timeout(Some(Duration::from_millis(500)));
        let checks = [
            "GET_VERSION",
            "GET_ID",
            "GET_OS",
            "GET_ARCH",
            "GET_MN",
        ];

        let s1 = VS1.parse::<i32>().unwrap();
        let s2 = VS2.parse::<i32>().unwrap();
        let s3 = VS3.parse::<i32>().unwrap() * 30;

        let mut errors = 0;
        for i in 0..checks.len() {
            if errors == 9 {
                return false;
            }

            let write = dlwp::cerpton::libcerpton_encode([s1, s2, s3, 0, 0, 0], format!("{} {}", env!("DIST_IDENT"), checks[i]));
            let mut write_ret = self.write(write.clone());

            while write_ret != STATUS_OK || errors < 9 {
                write_ret = self.write(write.clone());
                errors += 1;
            }

            if errors == 9 {
                return false;
            }

            let mut read_ret = self.read();
            
            while read_ret.1 != STATUS_OK || errors < 9 || !read_ret.0.contains(env!("DIST_IDENT")) {
                read_ret = self.read();
                errors += 1;
            }

            if errors == 9 {
                return false;
            }

            if read_ret.1 == STATUS_OK {
                read_ret.0 = read_ret.0.replace(&format!(" {}", env!("DIST_IDENT")), "");
                match i {
                    0 => self.info.version = read_ret.0,
                    1 => self.info.id = read_ret.0.parse().unwrap(),
                    2 => self.info.os = Some(read_ret.0.parse().unwrap()),
                    3 => self.info.arch = Some(read_ret.0.parse().unwrap()),
                    4 => self.info.magic_number = read_ret.0.parse().unwrap(),
                    _ => {},
                }
            }
        }

        // check magic num

        true
    }

    // Call this early when the distributor is being setup
    fn attempt_connect(&mut self) -> bool {
        self.stream.write(DIST_INIT.as_bytes());
        self.stream.flush();

        false
    }
}
