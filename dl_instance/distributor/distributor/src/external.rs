use dlwp::codes::Code;

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

/// Interacting with an external distributor
pub trait ExternalDistributor {
    fn write(&self, _inputs: Vec<u8>) -> Code {
        unimplemented!()
    }

    fn read(&self, _inputs: Vec<u8>) -> Result<Vec<u8>, Code> {
        unimplemented!()
    }
}
