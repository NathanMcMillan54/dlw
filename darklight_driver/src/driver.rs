use crate::streams::{StreamInfo, StreamsHandler};
use dlwp::{
    cerpton::{libcerpton_decode, libcerpton_encode},
    chrono::{Timelike, Utc},
    config::DLConfig,
    distributor::{GET_DISTRIBUTOR, READ_AVAILABLE, USER_INIT},
    encryption::EncryptionInfo,
    id::{local_user_id, DId},
    message::{contents_to_string, fmt_message_recv, Message, ReceiveInfo},
    serialport::posix::TTYPort,
    stream::file::ReceivedMessage,
};
use std::{
    borrow::{Borrow, BorrowMut},
    io::{Read, Write},
    net::TcpStream, thread::sleep, time::Duration,
};

pub struct DarkLightDriver {
    pub streams_handler: StreamsHandler,
    pub config: DLConfig,
    pub tcp_stream: Option<TcpStream>,
    pub serial_port: Option<dlwp::serialport::posix::TTYPort>,
}

impl DarkLightDriver {
    pub fn empty() -> Self {
        return DarkLightDriver {
            streams_handler: StreamsHandler::new(),
            config: DLConfig::empty(),
            tcp_stream: None,
            serial_port: None,
        };
    }

    pub fn new(streams_handler: StreamsHandler, config: DLConfig) -> Self {
        return DarkLightDriver {
            streams_handler,
            config,
            tcp_stream: None,
            serial_port: None,
        };
    }

    pub fn connect_to_distributor(&mut self) -> bool {
        // Get the distributor ID
        self.write(GET_DISTRIBUTOR.to_string(), false);
        let mut id_response = String::new();

        while id_response == String::new() {
            id_response = self.read();
        }

        let id = id_response.parse::<DId>();
        if id.is_err() {
            println!("Failed to get Distirbutor ID: {}", id_response);
        }

        // Send user information
        self.write(
            format!(
                "{} {} {} {} {} {} {}",
                USER_INIT,
                env!("DLU_KEY"),
                local_user_id().unwrap(),
                env!("UC_OS"),
                env!("UC_ARCH"),
                env!("UC_DAY"),
                env!("UC_MONTH")
            ),
            false,
        );

        let mut connect_response = String::new();
        while connect_response == String::new() {
            connect_response = self.read();
        }

        if connect_response.contains("CONN") {
            println!("Connected to Distributor {}", id.unwrap());
            return true;
        } else {
            println!("Failed to connect: {}", connect_response);
            return false;
        }
    }

    fn handle_new_message(&mut self, new_msg: &String) {
        let time = Utc::now();
        let ri = ReceiveInfo::get_from_message_string(new_msg.to_string());
        if ri != ReceiveInfo::empty() {
            // Check if a server is running on the port
            let mut streaminfo = StreamInfo {
                id: local_user_id().unwrap(),
                port: ri.port,
                did: ri.rdid,
                local: true,
            };

            if self.streams_handler.streams.contains_key(&streaminfo) {
                //
            } else {
                for (info, file) in self.streams_handler.streams.iter() {
                    if info.port == ri.port {
                        let encryption_info = file.encryption_info;
                        let decoded_message = Message::decode(
                            &new_msg,
                            EncryptionInfo {
                                encode_function: libcerpton_encode,
                                decode_function: libcerpton_decode,
                                info: encryption_info,
                            },
                        );

                        let test_streaminfo = StreamInfo {
                            id: decoded_message.ti.tid,
                            port: ri.port,
                            did: decoded_message.ti.tdid,
                            local: false,
                        };

                        if self.streams_handler.streams.contains_key(&test_streaminfo) {
                            streaminfo = test_streaminfo;
                        }
                    }
                }
            }

            self.streams_handler.add_received_message(
                streaminfo,
                ReceivedMessage {
                    recv_time: [time.hour() as u8, time.minute() as u8, time.second() as u8],
                    message: new_msg.as_str().to_string(),
                },
            );
        }
    }

    // Returns ``true`` if the message was sent without receiving a new message
    fn write(&mut self, send: String, wait: bool) -> bool {
        if self.config.closed {
            // TODO: Write to local streams
            return true;
        }

        let write_ret = if self.config.tcp == true {
            write(self.tcp_stream.as_mut().unwrap(), send, wait)
        } else {
            write(self.serial_port.as_mut().unwrap(), send, wait)
        };

        if write_ret.is_none() {
            return true;
        } else {
            self.handle_new_message(&write_ret.unwrap());
            return false;
        }
    }

    fn read(&mut self) -> String {
        let mut read_ret = [0; 4096];

        while read_ret == [0; 4096] {
            read_ret = if self.config.serial == true {
                read(self.serial_port.as_mut().unwrap())
            } else {
                read(self.tcp_stream.as_mut().unwrap())
            };
        }

        String::from_utf8(read_ret.to_vec()).unwrap_or(String::new()).replace("\0", "")
    }

    pub fn send_to_distributor(&mut self) {
        for streaminfo in self.streams_handler.streams.clone().into_keys() {
            if self.streams_handler.streams[&streaminfo].pending.is_empty() {
                continue;
            }

            // First check if there's a new message or if a read is available
            let distributor_read = if self.config.serial == true {
                wait_for_distributor(self.serial_port.as_mut().unwrap())
            } else {
                wait_for_distributor(self.tcp_stream.as_mut().unwrap())
            };

            // If a read is available send the oldest message, if not the handle the new message
            if distributor_read.is_none() {
                self.write(
                    self.streams_handler.streams[&streaminfo].pending[0].clone(),
                    false,
                );
                self.streams_handler
                    .streams
                    .get_mut(&streaminfo)
                    .unwrap()
                    .pending
                    .remove(0);
                self.streams_handler.streams[&streaminfo].write_pending();
            } else {
                self.handle_new_message(&distributor_read.unwrap());
            }

            // If there are any other messages that can be sent or nothing was sent before then another message can be sent
            if self.streams_handler.streams[&streaminfo].pending.is_empty() {
                continue;
            } else {
                while self.write(
                    self.streams_handler.streams[&streaminfo].pending[0].clone(),
                    true,
                ) == false
                {
                    // Wait for write
                }

                self.streams_handler
                    .streams
                    .get_mut(&streaminfo)
                    .unwrap()
                    .pending
                    .remove(0);
                self.streams_handler.streams[&streaminfo].write_pending();
            }
        }
    }

    pub fn read_from_distributor(&mut self) {
        for _ in 0..self.streams_handler.streams.len() {
            let mut waiting_for_message = true;

            while waiting_for_message {
                let dist_read = fmt_message_recv(&self.read());

                if dist_read.is_empty() {
                    continue;
                }

                if ReceiveInfo::get_from_message_string(dist_read.clone()) != ReceiveInfo::empty() {
                    self.handle_new_message(&dist_read);
                    waiting_for_message = false;
                }
            }
        }
    }
}

pub fn read<R: Read>(mut stream: &mut R) -> [u8; 4096] {
    let mut buf = [0; 4096];
    stream.read(&mut buf).unwrap_or(0);

    buf
}

pub fn wait_for_distributor<R: Read>(mut stream: &mut R) -> Option<String> {
    let mut read_bytes = read(stream);

    while read_bytes == [0; 4096] {
        read_bytes = read(stream);
        sleep(Duration::from_micros(500));
    }

    let read_str = contents_to_string(read_bytes);
    if read_str.contains(READ_AVAILABLE) {
        return None;
    } else {
        return Some(read_str);
    }
}

// Returns a message if one is received while waiting for send
pub fn write<RW: Read + Write>(mut stream: &mut RW, write: String, wait: bool) -> Option<String> {
    if wait == true {
        let wait_ret = wait_for_distributor(stream);
        if wait_ret.is_some() {
            return wait_ret;
        }
    }

    stream.write(write.as_bytes()).unwrap();
    stream.flush().unwrap();

    None
}
