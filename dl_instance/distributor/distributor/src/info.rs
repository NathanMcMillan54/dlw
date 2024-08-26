use dlwp::config::DistributorConfig;

pub struct DistributorInfo {
    pub id: u64,
    pub uid: String,
    pub config: DistributorConfig,
    pub version: String,
}

impl DistributorInfo {
    pub fn new(id: u64, uid: String, config: DistributorConfig) -> Self {
        return DistributorInfo {
            id,
            uid,
            config,
            version: env!("CARGO_PKG_VERSION").to_string(), // Do not change this value
        };
    }
}

/// This should return the responses that an instance would request about the distributor (link)[]
pub trait DistCheckResponses {
    /// Public id
    fn id_response(&self) -> Vec<u8> {
        unimplemented!()
    }

    /// Second id ("key")
    fn key_response(&self) -> Vec<u8> {
        unimplemented!()
    }

    /// Return the value of ``version`` from ``DistributorInfo``
    fn version_response(&self) -> Vec<u8> {
        unimplemented!()
    }
}
