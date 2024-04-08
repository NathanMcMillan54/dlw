pub extern crate cerpton;

#[cfg(feature = "include_chrono")]
pub extern crate chrono;

#[cfg(not(feature = "include_serde"))]
#[macro_use]
extern crate serde;

#[cfg(not(feature = "include_serde"))]
extern crate serde_json;

#[cfg(feature = "include_serde")]
#[macro_use]
pub extern crate serde;

#[cfg(feature = "include_serde")]
pub extern crate serde_json;

#[cfg(feature = "use_io")]
pub extern crate serialport;

#[cfg(test)]
pub mod tests;

/// "codes" for specifying information in a ``Message``
pub mod codes;
/// Configurations for DarkLight
pub mod config;

#[cfg(feature = "use_io")]
pub mod distributor;

pub(crate) mod dlcmd;
/// Type for encryption
pub mod encryption;
/// Contains types and functions for using different types of Ids
pub mod id;

#[cfg(feature = "use_io")]
pub mod io;

/// For a ``Message``
pub mod message;
/// Used for creating DarkLight clients and servers
pub mod stream;
