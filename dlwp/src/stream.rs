use crate::{
    codes::{Code, STATUS_OK},
    dlcmd::{send_dlcmd, CONNECT},
    encryption::EncryptionInfo,
    id::{DId, LId},
    message::Message,
};

#[allow(improper_ctypes_definitions)]
extern "C" fn empty_encryption_function(_i: [i32; 6], s: &'static str) -> &'static str {
    s
}

pub(crate) const EMPTY_ENCRYPTIONIFNO: EncryptionInfo = EncryptionInfo {
    function: empty_encryption_function,
    info: [0; 6],
};

/// A type for determining how the ``Stream`` will act (as client or server)
#[derive(Copy, Clone)]
pub enum StreamType {
    Client {
        /// The receiving id of a client/server
        rid: LId,
        /// The client/server's Distributor Id
        rdid: DId,
        /// The port that the client/server is using
        port: u16,
    },
    Server {
        /// Port to bind to
        port: u16,
    },
}

impl StreamType {
    /// Returns ``true`` if the ``StreamType`` is ``Client``
    pub fn is_client(self) -> bool {
        return match self {
            Self::Client { .. } => true,
            Self::Server { .. } => false,
        };
    }

    pub fn rid(self) -> Option<LId> {
        return match self {
            Self::Client {
                rid,
                rdid: _,
                port: _,
            } => Some(rid),
            Self::Server { .. } => None,
        };
    }

    pub fn rdid(self) -> Option<DId> {
        return match self {
            Self::Client {
                rid: _,
                rdid,
                port: _,
            } => Some(rdid),
            Self::Server { .. } => None,
        };
    }

    /// Gets the port of the ``Stream``
    pub fn port(self) -> u16 {
        return match self {
            Self::Client {
                rid: _,
                rdid: _,
                port,
            } => port,
            Self::Server { port } => port,
        };
    }
}

pub struct Stream {
    /// The ``StreamType`` (client or server)
    pub stream_type: StreamType,
    /// Information for encrypting/decrypting messages before and after being sent to a Distributor
    pub encryption: EncryptionInfo,
    /// Store sent and received messages
    pub history: bool,

    received_messages: Vec<Message>,
    sent_messages: Vec<Message>,
    running: bool,
}

impl Stream {
    pub fn new(stream_type: StreamType, history: bool) -> Self {
        return Stream {
            stream_type,
            encryption: EMPTY_ENCRYPTIONIFNO,
            history,
            received_messages: vec![],
            sent_messages: vec![],
            running: false,
        };
    }

    pub fn add_encryption_info(&mut self, info: EncryptionInfo) {
        self.encryption = info;
    }

    /// Clears the messages sent and received
    pub fn clear_history(&mut self) {
        self.received_messages.clear();
        self.sent_messages.clear();
    }

    fn _server_start(&mut self) -> Code {
        STATUS_OK
    }

    fn _client_start(&mut self) -> Code {
        STATUS_OK
    }

    /// Starts the ``Stream``. If ``self.stream_type`` is ``Client`` it will try to connect to server/client, if it is
    /// ``Server`` it will allow connections on the port used in ``Server``.
    pub fn start(&mut self) -> Code {
        let ret = if self.stream_type.is_client() {
            self._client_start()
        } else {
            self._server_start()
        };

        self.running = true;
        ret
    }

    pub fn stop(&mut self) -> Code {
        self.running = false;
        STATUS_OK
    }
}
