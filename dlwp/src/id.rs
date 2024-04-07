use std::fs::read_to_string;
use std::path::Path;

pub type LId = u64;
pub type DId = u32;

pub fn local_user_id() -> Option<LId> {
    return if Path::new("/etc/dlw/local_id").exists() {
        let read = read_to_string("/etc/dlw/local_id")
            .unwrap()
            .replace("\n", "");
        Some(read.parse::<u64>().unwrap())
    } else {
        None
    };
}
