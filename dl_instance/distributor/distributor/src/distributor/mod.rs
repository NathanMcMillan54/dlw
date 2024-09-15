use std::{
    collections::HashMap,
    fs::read_to_string,
    io::{Read, Write},
    net::{Ipv4Addr, Shutdown, SocketAddrV4, TcpListener, TcpStream},
    str::FromStr,
    sync::Arc,
    thread::sleep,
    time::Duration,
};

use dlwp::config::DistributorConfig;
use dlwp::serde_json;
use input::check_user_input;
use lib_dldistributor::{
    connections::{LocalConnections, PendingMessage, PendingMessages},
    info::DistributorInfo,
};

use crate::{DISTRIBUTOR, DISTRIBUTOR_ID, DISTRIBUTOR_UID};

pub(crate) mod distributors;
pub(crate) mod input;
pub(crate) mod users;
pub(crate) mod verify_server;

pub struct DarkLightDistributor {
    pub info: DistributorInfo,
    pub user_connections: LocalConnections,
    pub pending_messages: PendingMessages, // This is used to prevent conflicting data in threads
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
            pending_messages: HashMap::new(),
            verify_server: SocketAddrV4::from_str("0.0.0.0:0").unwrap(),
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
            sleep(Duration::from_millis(40));
            let mut _accept = listener.accept();
            if _accept.is_err() {
                continue;
            }

            let mut accept = _accept.unwrap();
            accept.0.set_read_timeout(Some(Duration::from_millis(1)));

            let mut read = [0; 100];

            let mut accepted = false;
            let mut reads = 0;

            // Allows for distributor information to be collected before properly connecting
            while reads < 5 && accepted == false {
                if accept.0.read(&mut read).is_err() {
                    reads += 1;
                    continue;
                }

                let check = check_user_input(read.to_vec());

                if check.starts_with("INIT-USR") {
                    let verify = self.verify_input(check.as_bytes().to_vec());

                    if verify == false {
                        accept.0.write(b"INVALID USER");
                        accept.0.shutdown(Shutdown::Both);
                        continue;
                    } else {
                        accepted = true;
                        let id = check.split(" ").collect::<Vec<&str>>()[2];
                        accept.0.write(b"CONN");
                        accept.0.flush();
                        self.user_connections
                            .add_tcp_connection(id.parse().unwrap(), accept.0);

                        self.pending_messages.insert(
                            id.parse().unwrap(),
                            PendingMessage::new(false, String::new()),
                        );
                        break;
                    }
                } else {
                    accept.0.write(check.as_bytes());
                    accept.0.flush();
                }

                reads += 1;
            }

            // Get rid of client (not "Id")
        }
    }
}
