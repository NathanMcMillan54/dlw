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

use distributors::tcp::TcpDistributor;
use dlwp::{config::DistributorConfig, encryption::EncryptionInfo};
use dlwp::serde_json;
use input::check_user_input;
use lib_dldistributor::{
    connections::{LocalConnections, PendingMessage, PendingMessages}, encryption::DistributorEncryption, get_a_magic_num, info::DistributorInfo
};

use crate::{DISTRIBUTOR, DISTRIBUTOR_ID, DISTRIBUTOR_UID};

pub(crate) mod distributors;
pub(crate) mod input;
pub(crate) mod users;
pub(crate) mod verify_server;

// "Magic number" information
#[cfg(debug_assertions)]
#[path = "magicn_debug.rs"]
pub(crate) mod magicn;

#[cfg(not(debug_assertions))]
#[path = "magicn_release.rs"]
pub(crate) mod magicn;

// Update encrpytion
#[cfg(debug_assertions)]
#[path = "encryption_debug.rs"]
pub(crate) mod encrpytion;

#[cfg(not(debug_assertions))]
#[path = "encryption_release.rs"]
pub(crate) mod encryption;

pub struct DarkLightDistributor {
    pub info: DistributorInfo,
    pub dist_encrption: DistributorEncryption,
    pub user_connections: LocalConnections,
    pub pending_messages: PendingMessages, // This is used to prevent conflicting data in threads
    pub verify_server: SocketAddrV4,
    pub tcp_distributors: Vec<TcpDistributor>,
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
            // Where ``DistributorEncryption`` is initally set:
            // There is no encrpyion inforomation and it's update interval is set to 24 hours
            dist_encrption: DistributorEncryption::new(EncryptionInfo {
                decode_function: encrpytion::libcerpton_decode,
                encode_function: encrpytion::libcerpton_encode,
                info: [0; 6],
            }, Duration::from_secs(24 * 3600), encrpytion::update_encryption),
            user_connections: LocalConnections::empty(),
            pending_messages: HashMap::new(),
            verify_server: SocketAddrV4::from_str("0.0.0.0:0").unwrap(),
            tcp_distributors: vec![]
        };
    }

    pub fn set_config(&mut self, config_path: &'static str) {
        let contents = read_to_string(config_path).expect("Failed to read config file");
        let parsed_file: DistributorConfig =
            serde_json::from_str(&contents).expect("Failed to parse config file");

        self.info.config = parsed_file;
    }

    pub fn set_encrption(&mut self) {
        let s1 = crate::S1.parse().unwrap();
        let s2 = crate::S2.parse().unwrap();
        let s3 = crate::S3.parse().unwrap();

        self.dist_encrption.info.info = [s1, s2, s3, 0, 0, 0];
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
                            PendingMessage::new(false, 0, String::new()),
                        );
                        break;
                    }
                } else if check.starts_with("INIT-DIS") {
                    // Add to list of distributors to be checked
                    self.tcp_distributors.push(TcpDistributor::new(accept.0, check));
                    break;
                } 
                else {
                    accept.0.write(check.as_bytes());
                    accept.0.flush();
                }

                reads += 1;
            }

            // Get rid of client (not "Id")
        }
    }
}
