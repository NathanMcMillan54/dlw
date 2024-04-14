use crate::{
    codes::{Code, REMOVE_CLIENT, REQUEST_CONNECTION, STATUS_OK, UNKNOWN_STATUS},
    connections::Connections,
    dlcmd::{send_dlcmd, CONNECT, DISCONNECT, SEND},
    encryption::EncryptionInfo,
    id::*,
    message::{string_to_contents, Message, ReceiveInfo, TransmitInfo},
};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    thread::sleep,
    time::Duration,
};

#[allow(improper_ctypes_definitions)]
fn empty_encryption_function(_i: [i32; 6], s: String) -> String {
    s
}

pub(crate) const EMPTY_ENCRYPTIONIFNO: EncryptionInfo = EncryptionInfo {
    encode_function: empty_encryption_function,
    decode_function: empty_encryption_function,
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

    /// If ``StreamType`` is a ``Client`` this returns the stream's receiving ID, if it is a ``Server`` it returns the
    /// local ID
    pub fn rid(self) -> LId {
        return match self {
            Self::Client {
                rid,
                rdid: _,
                port: _,
            } => rid,
            Self::Server { .. } => local_user_id().unwrap_or(0),
        };
    }

    /// If ``StreamType`` is a ``Client`` this returns the stream's receiving distributor ID, if it is a ``Server`` it
    /// returns the local distributor ID
    pub fn rdid(self) -> DId {
        return match self {
            Self::Client {
                rid: _,
                rdid,
                port: _,
            } => rdid,
            Self::Server { .. } => distributor_id().unwrap_or(0),
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
    // For servers
    connections: Connections,
    instance_id: InstanceID,
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
            connections: Connections::empty(),
            instance_id: 0,
            received_messages: vec![],
            sent_messages: vec![],
            running: false,
        };
    }

    pub fn add_not_allowed_connections(&mut self, not_allowed_connections: Vec<ReceiveInfo>) {
        if self.stream_type.is_client() {
            return;
        }

        if self.connections.not_allowed.is_empty() {
            self.connections.not_allowed = not_allowed_connections;
        } else {
            for i in 0..not_allowed_connections.len() {
                self.connections
                    .not_allowed
                    .push(not_allowed_connections[i]);
            }
        }
    }

    fn check_add_connection(&mut self, message: Message) -> bool {
        let ri = ReceiveInfo {
            rid: message.ti.tid,
            rdid: message.ti.tdid,
            instance_id: message.ri.instance_id,
            port: message.ri.port,
        };

        if self.connections.not_allowed.contains(&ri) {
            return false;
        }

        if self.connections.current.contains(&ri) {
            return true;
        } else {
            if message.ti.code == REQUEST_CONNECTION.value() {
                self.connections.current.push(ri);
                return true;
            } else {
                return false;
            }
        }
    }

    pub fn remove_connection(&mut self, ri: ReceiveInfo) -> Code {
        if self.stream_type.is_client() {
            return UNKNOWN_STATUS;
        }

        let index = self.connections.current.iter().position(|&iri| iri == ri);
        if index.is_none() {
            return UNKNOWN_STATUS;
        } else {
            self.connections.current.remove(index.unwrap());
            return STATUS_OK;
        }
    }

    fn stream_file_exists(&self) -> bool {
        Path::new(&format!(
            "/tmp/darklight/connections/_dl_{}-{}",
            self.stream_type.rid(),
            self.stream_type.port()
        ))
        .exists()
    }

    pub fn add_encryption_info(&mut self, info: EncryptionInfo) {
        self.encryption = info;
    }

    pub fn running(&self) -> bool {
        self.running
    }

    /// Clears the messages sent and received
    pub fn clear_history(&mut self) {
        self.received_messages.clear();
        self.sent_messages.clear();
    }

    fn _read(&self) -> Vec<String> {
        sleep(Duration::from_micros(15));
        let reader = BufReader::new(
            File::options()
                .read(true)
                .open(&format!(
                    "/tmp/darklight/connections/_dl_{}-{}",
                    self.stream_type.rid(),
                    self.stream_type.port()
                ))
                .unwrap(),
        );
        let mut ret = vec![];

        for line in reader.lines() {
            if line.is_ok() {
                ret.push(line.unwrap());
            }
        }

        ret
    }

    pub fn read(&mut self) -> Vec<Message> {
        let mut ret = vec![];
        let strings = self._read();

        for i in 0..strings.len() {
            let received_message = Message::from_string(&(self.encryption.decode_function)(self.encryption.info, strings[i].to_owned()).to_string());
            if self.check_add_connection(received_message) {
                ret.push(received_message);
            }
        }

        ret
    }

    /// Writes a ``Message`` to the stream
    pub fn write_message(&self, message: Message) {
        send_dlcmd(
            SEND,
            message
                .encode(self.encryption)
                .split(" ")
                .collect::<Vec<&str>>(),
        );
    }

    pub fn write(&self, write: String, code: Code) {
        self.write_message(Message {
            ri: ReceiveInfo {
                rid: self.stream_type.rid(),
                rdid: self.stream_type.rdid(),
                port: self.stream_type.port(),
                instance_id: self.instance_id,
            },
            ti: TransmitInfo {
                tid: local_user_id().unwrap(),
                tdid: distributor_id().unwrap(),
                code: code.value(),
            },
            day: self.encryption.info[0],
            week: self.encryption.info[1],
            month: self.encryption.info[2],
            contents: string_to_contents(write),
        });
    }

    fn _server_start(&mut self) -> Code {
        let decode_info = self.encryption.info;
        let local_did = distributor_id().expect("Local Distributor Id is not set");
        let local_id = local_user_id().expect("Failed to get Local Id");

        // Creates a stream that's "connects" your device to itself
        send_dlcmd(
            CONNECT,
            vec![
                &local_did.to_string(),
                &local_id.to_string(),
                &self.stream_type.port().to_string(),
                &self.instance_id.to_string(),
                &decode_info[0].to_string(),
                &decode_info[1].to_string(),
                &decode_info[2].to_string(),
            ],
        );

        self.running = true;
        STATUS_OK
    }

    fn _client_start(&mut self) -> Code {
        let decode_info = self.encryption.info;

        // Create a stream
        send_dlcmd(
            CONNECT,
            vec![
                &self.stream_type.rdid().to_string(),
                &self.stream_type.rid().to_string(),
                &self.stream_type.port().to_string(),
                &self.instance_id.to_string(),
                &decode_info[0].to_string(),
                &decode_info[1].to_string(),
                &decode_info[2].to_string(),
            ],
        );

        // Delay to ensure the stream has been created by now
        sleep(Duration::from_millis(100));

        if self.stream_file_exists() == false {
            return UNKNOWN_STATUS;
        }

        // Request connection to the client/server
        self.write_message(Message {
            ti: TransmitInfo {
                tdid: distributor_id().expect("Failed to get local Distributor ID"),
                tid: local_user_id().expect("Failed to get local ID"),
                code: REQUEST_CONNECTION.value(),
            },
            ri: ReceiveInfo {
                rid: self.stream_type.rid(),
                rdid: self.stream_type.rdid(),
                port: self.stream_type.port(),
                instance_id: self.instance_id,
            },
            contents: [0; 4096],
            day: 0,
            week: 0,
            month: 0,
        });

        self.running = true;
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

        self.write(String::new(), REMOVE_CLIENT);

        send_dlcmd(
            DISCONNECT,
            vec![
                &self.stream_type.rid().to_string(),
                &self.stream_type.port().to_string(),
                &self.stream_type.rdid().to_string(),
            ],
        );

        // Wait for darklight_driver

        REMOVE_CLIENT
    }
}
