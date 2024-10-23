use dlwp::{cerpton::{libcerpton_decode, libcerpton_encode}, codes::{INVALID_RR, READ_FAILED, STATUS_OK, WRITE_FAILED}, distributor::DIST_INIT};
use lib_dldistributor::external::{ExternalDistributorInfo, ExternalDistributorRW};
use std::{borrow::Borrow, io::{Read, Write}, net::TcpStream, time::Duration};

use crate::{distributor::{encrpytion::current_encryption, magicn::get_my_magic_num}, DISTRIBUTOR_ID, VS1, VS2, VS3};

pub struct TcpDistributor {
    pub stream: TcpStream,
    pub info: ExternalDistributorInfo,
    pub msg: String,
}

impl TcpDistributor {
    pub fn new_and_check(stream: TcpStream) -> Option<Self> {
        let mut tcp_dist = TcpDistributor {
            stream: stream,
            info: ExternalDistributorInfo::default(),
            msg: String::new(),
        };

        return if tcp_dist.verify_distributor() {
            Some(tcp_dist)
        } else {
            None
        };
    }

    pub fn new(stream: TcpStream, msg: String) -> Self {
        return TcpDistributor {
            stream,
            info: ExternalDistributorInfo::default(),
            msg: msg,
        };
    }
}

impl ExternalDistributorRW for TcpDistributor {
    fn write(&mut self, _inputs: String) -> dlwp::codes::Code {
        let encryption = current_encryption();
        let input = (encryption.encode_function)(encryption.info, _inputs).replace("\0", "\\0");

        let write_err = io_err_check!(self.stream.write(input.as_bytes()));
        let flush_err = io_err_check!(self.stream.flush());

        return if write_err == true || flush_err == true {
            WRITE_FAILED
        } else {
            STATUS_OK
        };
    }

    fn read(&mut self) -> (String, dlwp::codes::Code) {
        let mut buf = [0; 4096];

        let read_err = io_err_check!(self.stream.read(&mut buf));

        if read_err == true {
            return (String::new(), READ_FAILED);
        }

        let ret = String::from_utf8(buf.to_vec());
        
        if ret.is_err() { // Received invalid characters
            return (String::new(), INVALID_RR);
        }

        let encryption = current_encryption();

        return ((encryption.decode_function)(encryption.info, ret.unwrap()).replace("\0", "").replace("\\0", "\0"), STATUS_OK);
    }

    fn verify_distributor(&mut self) -> bool {
        /*if !unsafe { crate::DISTRIBUTOR.as_ref().unwrap().info.config.tcp_connections.contains(&self.stream.peer_addr().unwrap().ip().to_string()) } {
            return false;
        }*/

        self.stream.set_read_timeout(Some(Duration::from_millis(50)));
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
            let write = dlwp::cerpton::libcerpton_encode([s1, s2, s3, 0, 0, 0], format!("{} {}", checks[i], env!("DIST_IDENT")));
            let mut write_ret = self.write(write.clone());

            while write_ret != STATUS_OK {
                write_ret = self.write(write.clone());
                errors += 1;
            }

            let mut read_ret = self.read();
            read_ret.0 = libcerpton_decode([s1, s2, s3, 0, 0, 0], read_ret.0);

            while read_ret.1 != STATUS_OK || !read_ret.0.contains(env!("DIST_IDENT")) {
                read_ret = self.read();
                read_ret.0 = libcerpton_decode([s1, s2, s3, 0, 0, 0], read_ret.0);
                errors += 1;
            }

            if read_ret.1 == STATUS_OK {
                read_ret.0 = read_ret.0.replace(&format!(" {}", env!("DIST_IDENT")), "").replace("\0", "");
                match i {
                    0 => self.info.version = read_ret.0,
                    1 => self.info.id = read_ret.0.parse().unwrap_or(0),
                    2 => self.info.os = Some(read_ret.0.parse().unwrap_or(255)),
                    3 => self.info.arch = Some(read_ret.0.parse().unwrap_or(255)),
                    4 => self.info.magic_number = read_ret.0.parse().unwrap_or(0),
                    _ => {},
                }
            } else {
                println!("errors");
            }
        }

        // check magic num

        true
    }

    fn attempt_connect(&mut self) -> bool {
        self.stream.set_read_timeout(Some(Duration::from_millis(50)));

        let s1 = VS1.parse::<i32>().unwrap();
        let s2 = VS2.parse::<i32>().unwrap();
        let s3 = VS3.parse::<i32>().unwrap() * 30;

        #[cfg(debug_assertions)]
        let magic_num = get_my_magic_num(vec![]).to_string();

        let mut responses = [
            env!("CARGO_PKG_VERSION"),
            DISTRIBUTOR_ID,
            env!("DIST_OS"),
            env!("DIST_ARCH"),
            magic_num.as_str()
        ];
        let mut errors = 0;

        for i in 0..responses.len() {
            let mut read_ret = self.read();
            read_ret.0 = libcerpton_decode([s1, s2, s3, 0, 0, 0], read_ret.0);
            
            while read_ret.1 != STATUS_OK || !read_ret.0.contains(env!("DIST_IDENT")) {
                read_ret = self.read();
                read_ret.0 = libcerpton_decode([s1, s2, s3, 0, 0, 0], read_ret.0);
                errors += 1;
            }

            read_ret.0 = read_ret.0.replace(&format!(" {}", env!("DIST_IDENT")), "").replace("\0", "");

            match i {
                0 => {
                    if !read_ret.0.contains("VERSION") {
                        return false;
                    }
                },
                1 => {
                    if !read_ret.0.contains("GET_ID") {
                        return false;
                    }
                },
                2 => {
                    if !read_ret.0.contains("GET_OS") {
                        return false;
                    }
                },
                3 => {
                    if !read_ret.0.contains("GET_ARCH") {
                        return false;
                    }
                },
                4 => {
                    if !read_ret.0.contains("GET_MN") {
                        return false;
                    }
                },
                _ => {}
            }

            self.write(libcerpton_encode([s1, s2, s3, 0, 0, 0], format!("{} {}", responses[i], env!("DIST_IDENT"))));
        }

        true
    }
}
