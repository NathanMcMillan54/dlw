use std::net::TcpStream;

use dlwp::{codes::Code, id::DId};

/// Interacting with a verify server, the verify server should be a regular TCP server
pub trait VerifyServerRW {
    /// The return code of this function should be sent to the user. If the user is not valid code ``408``
    /// ("REMOVE_CLIENT") should be sent.
    fn verify_user(&self, _inputs: Vec<u8>) -> Code {
        unimplemented!()
    }

    /// Verify that an external distributor is valid
    fn verify_distributor(&self, _inputs: Vec<u8>) -> Code {
        unimplemented!()
    }

    fn write(&self, _inputs: Vec<u8>) -> Code {
        unimplemented!()
    }

    fn read(&self, _inputs: Vec<u8>) -> Result<Vec<u8>, Code> {
        unimplemented!()
    }
}

/// Keeps track of verified distributor information
#[derive(Default)]
pub struct ExternalDistributorInfo {
    pub id: DId,
    pub version: String,
    pub magic_number: u128,
    pub arch: Option<u8>,
    pub os: Option<u8>,
}

/// Interacting with an external distributor
///
/// #### Suggested implementation (TCP distributor):
/// ```
/// use lib_dldistributor::external::{ExternalDistributorInfo, ExternalDistributorRW};
/// # use std::net::TcpStream;
///
/// struct TcpDistributor {
///     pub stream: TcpStream,
///     pub info: ExternalDistributorInfo,
/// }
///
/// impl TcpDistributor {
///     pub fn new_and_check(stream: TcpStream) -> Option<Self> {
///         let mut tcp_dist = TcpDistributor {
///             stream: stream,
///             info: ExternalDistributorInfo::default(),
///         };
///
///         return if tcp_dist.verify_distributor() {
///             Some(tcp_dist)
///         } else {
///             None
///         }
///     }
/// }
///
/// impl ExternalDistributorRW for TcpDistributor {
///     fn verify_distributor(&mut self) -> bool {
///         /*
///             Get information for ``self.info``
///             If returning ``true`` then add ``TcpDistributor`` to a list of distributors
///         */
///         # true
///     }
/// }
///
/// ```
pub trait ExternalDistributorRW {
    /// Encrypt input then send
    fn write(&mut self, _inputs: String) -> Code {
        unimplemented!()
    }

    // Read and decrypt input
    fn read(&mut self) -> (String, Code) {
        unimplemented!()
    }

    fn attempt_connect(&mut self) -> bool {
        unimplemented!()
    }

    /// Verify that the distributor's information is correct
    fn verify_distributor(&mut self) -> bool {
        unimplemented!()
    }
}
