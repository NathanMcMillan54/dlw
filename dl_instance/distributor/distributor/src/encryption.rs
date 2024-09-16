use std::time::Duration;

use dlwp::{chrono::{DateTime, NaiveTime, Timelike, Utc}, encryption::EncryptionInfo};

/// Holds encryption info for the distributor that can be updated regularly
pub struct DistributorEncryption {
    pub info: EncryptionInfo,
    pub update_interval: Duration,
    pub last_update: NaiveTime,
    pub update_fn: fn(EncryptionInfo) -> EncryptionInfo,
}

impl DistributorEncryption {
    /// ``update_interval`` should be larger than 1 second
    pub fn new(info: EncryptionInfo, update_interval: Duration, update_fn: fn(EncryptionInfo) -> EncryptionInfo) -> Self {
        let time = Utc::now().time();

        return DistributorEncryption {
            info,
            update_interval,
            last_update: time,
            update_fn
        };
    }

    /// If the time since ``last_update`` is equal to or greater than ``update_interval`` ``update_fn`` gets called and
    /// changes ``info``. Call this function regularaly.
    pub fn check_and_update(&mut self) {
        let current_time = Utc::now().time();
        let diff = current_time - self.last_update;

        if diff.num_seconds() as u64 >= self.update_interval.as_secs() {
            self.info = (self.update_fn)(self.info);
            self.last_update = current_time;
        }
    }
}
