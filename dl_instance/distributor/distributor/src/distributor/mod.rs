use std::{fs::read_to_string, net::TcpStream};

use dlwp::config::DistributorConfig;
use dlwp::serde_json;
use lib_dldistributor::{connections::LocalConnections, info::DistributorInfo};

use crate::{DISTRIBUTOR_ID, DISTRIBUTOR_UID};

pub(crate) mod verify_server;

pub struct DarkLightDistributor {
    pub info: DistributorInfo,
    pub user_connections: LocalConnections,
    pub verify_server: Option<TcpStream>,
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
            verify_server: None,
        };
    }

    pub fn set_config(&mut self, config_path: &'static str) {
        let contents = read_to_string(config_path).expect("Failed to read config file");
        let parsed_file: DistributorConfig =
            serde_json::from_str(&contents).expect("Failed to parse config file");

        self.info.config = parsed_file;
    }
}
