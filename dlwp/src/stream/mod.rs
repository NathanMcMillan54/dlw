/// Contains the file structure for a local stream file
pub mod file;
/// Contains ``Stream`` for creating, reading, and writing, to other streams.
pub mod handler;

pub use file::*;
pub use handler::{Stream, StreamType};
