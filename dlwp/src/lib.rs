pub extern crate cerpton;

#[cfg(not(feature = "include_serde"))]
#[macro_use]
extern crate serde;

#[cfg(not(feature = "include_serde"))]
extern crate serde_json;

#[cfg(feature = "include_serde")]
pub extern crate serde;

#[cfg(feature = "include_serde")]
pub extern crate serde_json;

#[cfg(test)]
pub mod tests;

/// "codes" for specifying information in a ``Message``
pub mod codes;
/// Configurations for DarkLight
pub mod config;
pub(crate) mod dlcmd;
/// Type for encryption
pub mod encryption;
/// Contains types and functions for using different types of Ids
pub mod id;
/// For a ``Message``
pub mod message;
