pub use dlwp::cerpton::{libcerpton_encode, libcerpton_decode};
use dlwp::encryption::EncryptionInfo;

pub fn current_encryption() -> EncryptionInfo {
    unsafe { crate::DISTRIBUTOR.as_ref().unwrap().dist_encrption.info }
}

pub fn update_encryption(e: EncryptionInfo) -> EncryptionInfo {
    e
}
