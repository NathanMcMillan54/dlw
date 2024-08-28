use std::{fs::read_to_string, io::{Read, Write}, net::{Ipv4Addr, Shutdown, SocketAddrV4, TcpListener, TcpStream}, str::FromStr};

use dlwp::config::DistributorConfig;
use dlwp::serde_json;
use input::check_input;
use lib_dldistributor::{connections::LocalConnections, info::DistributorInfo};

use crate::{DISTRIBUTOR_ID, DISTRIBUTOR_UID};

pub(crate) mod input;
pub(crate) mod users;
pub(crate) mod verify_server;

pub struct DarkLightDistributor {
    pub info: DistributorInfo,
    pub user_connections: LocalConnections,
    pub verify_server: SocketAddrV4,
}

impl DarkLightDistributor {
    pub fn new() -> Self {
        return DarkLightDistributor {
            info: DistributorInfo::new(
                DISTRIBUTOR_ID
                    .parse()
                    .expect("Failed to parse DIST_ID to u64"),
                DISTRIBUTOR_UID.to_string(),
                DistributorConfig::default(),
            ),
            user_connections: LocalConnections::empty(),
            verify_server: SocketAddrV4::from_str("0.0.0.0:0").unwrap()
        };
    }

    pub fn set_config(&mut self, config_path: &'static str) {
        let contents = read_to_string(config_path).expect("Failed to read config file");
        let parsed_file: DistributorConfig =
            serde_json::from_str(&contents).expect("Failed to parse config file");

        self.info.config = parsed_file;
    }

    // When a user or distributor connects to this distributor this function will verify and add it
    pub fn tcp_check_add(&mut self) {
        let mut listener = TcpListener::bind(self.info.config.bind.clone()).unwrap();

        loop {
            let mut _accept = listener.accept();
            if _accept.is_err() {
                continue;
            }

            let mut accept = _accept.unwrap();
            let mut read = [0; 100];

            if accept.0.read(&mut read).is_err() {
                continue;
            }

            let check = check_input(read.to_vec());

            if check.starts_with("USR-INIT") {
                let verify = self.verify_input(check.as_bytes().to_vec());

                if verify == false {
                    accept.0.write(b"INVALID USER");
                    accept.0.shutdown(Shutdown::Both);
                    continue;
                } else {
                    println!("Added user!");
                    self.user_connections.add_tcp_connection(0, accept.0);
                }
            }
        }
    }
}
