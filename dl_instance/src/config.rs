use dlwp::config::DistributorConfig;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InstanceConfig {
    pub id: u32,
    pub owners: Vec<String>,
    pub distributors: Vec<DistributorConfig>,
    pub min_distributor_version: String,
    pub min_library_version: String,
    pub verify_server_addresses: Vec<String>,
    pub non_essential_services: Vec<String>,
    pub information_message: String, // Status/information from the instance
}
