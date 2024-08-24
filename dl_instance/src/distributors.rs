use dlwp::{config::DistributorConfig, encryption::EncryptionInfo};

pub struct DistributorInfo {
    pub config: DistributorConfig,
    pub version: String,
    pub running: bool,
}

pub trait DistributorCheck {
    /// Encrypt (or don't) any information being sent to the distributor
    fn write_distributor(&mut self, _write: Vec<u8>, _info: DistributorInfo) {
        todo!()
    }

    /// Decrypt the information received from the distributor
    fn read_distributor(&mut self, _info: DistributorInfo) -> Vec<u8> {
        todo!()
    }

    fn check_id(&self) -> u32 {
        unimplemented!()
    }

    fn check_key(&self) -> String {
        unimplemented!()
    }

    fn check_version(&self) -> String {
        unimplemented!()
    }

    fn check_running(&self) -> bool {
        unimplemented!()
    }
}
