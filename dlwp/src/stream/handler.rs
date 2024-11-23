use crate::{
    codes::{
        Code, DISCONNECT as DISCONNECT_, REMOVE_CLIENT, REQUEST_CONNECTION, STATUS_OK,
        STREAM_FILE_NOT_FOUND, UNKNOWN_STATUS,
    },
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

use super::file::{ReadMessage, ReceivedMessage, StreamFile};

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
    pub file: StreamFile,
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
            file: StreamFile::default(),
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

    pub fn check_add_connection(&mut self, message: Message) -> bool {
        if self.stream_type.is_client() {
            return false;
        }

        let ri = message.ti.into_ri(message.ri.instance_id, message.ri.port);

        if !self.connections.is_allowed(ri) {
            return false;
        }

        if self.connections.current.get(&ri).is_some() {
            return true;
        } else {
            self.connections.current.insert(
                ri,
                Stream::new(
                    StreamType::Client {
                        rid: ri.rid,
                        rdid: ri.rdid,
                        port: ri.port,
                    },
                    self.history,
                ),
            );
            self.connections
                .current
                .get_mut(&ri)
                .unwrap()
                .add_encryption_info(self.encryption);
            self.connections.current.get_mut(&ri).unwrap().start();
            return true;
        }
    }

    pub fn remove_connection(&mut self, ri: ReceiveInfo) -> Code {
        if self.stream_type.is_client() {
            return UNKNOWN_STATUS;
        }

        let stream = self.connections.current.remove(&ri);
        if stream.is_none() {
            return UNKNOWN_STATUS;
        } else {
            stream.unwrap().write(String::new(), DISCONNECT_);
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

    fn wait_for_stream_file(&self, pr: &str) -> Code {
        let mut wait = 0;


        while wait < 400 {
            if self.file.exists(pr) {
                return STATUS_OK;
            }

            wait += 1;
        }

        return STREAM_FILE_NOT_FOUND;
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

    pub fn _read(&mut self) -> Vec<ReceivedMessage> {
        self.file.read_recieved();
        let received = self.file.received.clone();

        while self.file.received.is_empty() == false {
            self.file.received.clear();
            self.file.write_received();
            self.file.read_recieved();
        }
        received
    }

    pub fn read(&mut self) -> Vec<Message> {
        let mut ret = vec![];
        let received_messages = self._read();
        if received_messages.is_empty() {
            return ret;
        }

        for i in 0..received_messages.len() {
            let message = Message::decode(&received_messages[i].message, self.encryption);
            if message == Message::empty() {
                continue;
            }

            ret.push(message);
        }

        ret
    }

    // Returns all messages with the time they were received (hour, minute, second)
    pub fn read_with_timestamp(&mut self) -> Vec<ReadMessage> {
        let mut ret = vec![];
        let received_messages = self._read();
        if received_messages.is_empty() {
            return ret;
        }

        for i in 0..received_messages.len() {
            let message = Message::decode(&received_messages[i].message, self.encryption);
            if message == Message::empty() {
                continue;
            }

            ret.push(ReadMessage {
                recv_time: received_messages[i].recv_time,
                message
            });
        }

        ret
    }

    /// Writes a ``Message`` to the stream (client)
    pub fn write_message(&mut self, message: Message) {
        let encoded = message.encode(self.encryption);

        self.file.read_pending();
        self.file.pending.push(encoded.replace("\0", ""));
        self.file.write_pending();
    }

    /// When a server receives a message, it should use the transmit info to respond by calling this function
    pub fn server_write(&mut self, ti: TransmitInfo, write: String, code: Code) {
        if self.stream_type.is_client() {
            return;
        }

        let ri = ti.into_ri(self.instance_id, self.stream_type.port());

        self.connections
            .current
            .get_mut(&ri)
            .unwrap()
            .write(write, code);
    }

    // Write a ``String`` to a client
    pub fn write(&mut self, write: String, code: Code) {
        if self.stream_type.is_client() {
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
                contents: string_to_contents(write.replace("\0", "\\0")),
            });
        }
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

        if self.wait_for_stream_file("P") == STREAM_FILE_NOT_FOUND {
            return STREAM_FILE_NOT_FOUND;
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
        let id = self.stream_type.rid();
        self.file.id = id;
        self.file.did = self.stream_type.rdid();
        self.file.port = self.stream_type.port();
        self.file.encryption_info = self.encryption.info;

        let ret = if self.stream_type.is_client() {
            self._client_start()
        } else {
            self._server_start()
        };

        self.running = true;
        ret
    }

    /// Stops the current stream.
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
        sleep(Duration::from_micros(500));

        REMOVE_CLIENT
    }
}
