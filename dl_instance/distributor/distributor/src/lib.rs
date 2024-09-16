pub mod connections;
pub mod external;
pub mod encryption;
pub mod info;
pub mod macros;

/// A short time to stop in loops
pub const IDLE_SLEEP: std::time::Duration = std::time::Duration::from_micros(750);

extern "Rust" {
    /// Implement a way to get the "magic number" of the local distributor.
    /// This is only implemented in the distributor binary for debug mode.
    pub fn get_my_magic_num(inputs: Vec<u128>) -> u128;
    /// Implement a way to get the "magic number" of any other distributor.
    /// This is only implemented in the distributor binary for debug mode.
    pub fn get_a_magic_num(inputs: Vec<u128>) -> u128;
}
