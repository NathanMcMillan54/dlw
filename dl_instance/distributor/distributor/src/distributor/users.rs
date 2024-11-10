use std::{
    borrow::{Borrow, BorrowMut},
    io::{Read, Write},
    net::TcpStream,
    thread::sleep,
    time::Duration,
};

use dlwp::{
    distributor::READ_AVAILABLE,
    id::LId,
    message::{ReceiveInfo, MSG_END, MSG_INIT},
};
use lib_dldistributor::connections::PendingMessage;

use super::DarkLightDistributor;
use crate::DISTRIBUTOR;

impl DarkLightDistributor {
    fn check_can_read(&self, id: &LId) -> bool {
        return if self.local_pending_messages.contains_key(id) {
            if self.local_pending_messages[id].message_str == String::new() { // For users just recently added
                sleep(Duration::from_micros(1000));
                true
            } else {
                false
            }
        } else {
            true
        };
    }
}

impl DarkLightDistributor {
    fn tcp_user_read(&self, mut stream: &TcpStream) -> Option<String> {
        let mut buf = [0; 4096];
        let mut wait = 0;

        while wait < 400 {
            if wait % 80 == 0 || wait == 0 {
                let write_err = io_err_check!(stream.write(format!("{}{}{}", MSG_INIT, READ_AVAILABLE, MSG_END).as_bytes()));
                let flush_err = io_err_check!(stream.flush());

                if write_err == true || flush_err == true {
                    return None;
                }

                sleep(Duration::from_micros(100));
            }

            let read_err = io_err_check!(stream.read(&mut buf));
            if read_err == true {
                return None;
            }

            if buf != [0; 4096] {
                break;
            }

            wait += 1;
        }

        let mut read_str = String::from_utf8(buf.to_vec())
            .unwrap_or(String::new())
            .replace("\0", "");
        if read_str.starts_with(MSG_INIT) && read_str.ends_with(MSG_END) {
            read_str = read_str.replace(MSG_INIT, "").replace(MSG_END, "");
        }
        return Some(read_str);
    }

    fn tcp_user_write(&self, mut stream: &TcpStream, write: String) -> bool {
        let write_err = io_err_check!(stream.write(format!("{}{}{}", MSG_INIT, write, MSG_END).as_bytes()));
        let flush_err = io_err_check!(stream.flush());
        return if write_err == true || flush_err == true {
            false
        } else {
            true
        }
    }

    pub fn tcp_user_handler(&mut self) {
        loop {
            //sleep_condition!(self.user_connections.tcp_connections.len() <= 1); // Loop delays then continues if there <= 1 users

            for (id, mut stream) in self.user_connections.tcp_connections.iter() {
                /*if self.local_pending_messages.contains_key(id) {
                    if self.local_pending_messages[id].can_send == true {
                        println!("i have a message: {}", self.local_pending_messages[id].message_str);
                        self.tcp_user_write(stream, self.local_pending_messages[id].message_str.clone());
                    }
                }*/

                if self.local_pending_messages.contains_key(id) {
                    if self.local_pending_messages[id].message_str == String::new() {
                        self.local_pending_messages.remove(id);
                    } else if self.local_pending_messages[id].message_str == String::from("rm") {
                        continue;
                    } else {
                        self.tcp_user_write(stream, self.local_pending_messages[id].message_str.clone());
                        self.local_pending_messages.remove(id);
                    }
                }

                let try_read = self.tcp_user_read(stream);

                // User read failed
                if try_read.is_none() {
                    self.local_pending_messages.insert(*id, PendingMessage {
                        can_send: false,
                        did: 0,
                        message_str: String::from("rm")
                    });
                    continue;
                }

                let read = try_read.unwrap();

                // User sent nothing
                if read == String::new() {
                    continue;
                }

                let ri = ReceiveInfo::get_from_message_string(read.clone());
                // Message might have been invalid or was completley empty
                if ri == ReceiveInfo::empty() {
                    println!("Failed to parse ri: {}", read);
                    continue;
                }

                if ri.rdid == self.info.id {
                    if !self.user_connections.connection_exists(&ri.rid) {
                        // Notify user that connection doesn't exist
                    } else if self.user_connections.connection_is_tcp(&ri.rid) {
                        // Send immediatley
                        let ret = self
                            .tcp_user_write(&self.user_connections.tcp_connections[&ri.rid], read);
                        if ret == false {
                            self.local_pending_messages.insert(*id, PendingMessage {
                                can_send: false,
                                did: 0,
                                message_str: String::from("rm"),
                            });
                        }
                    } else {
                        // It is a serial connection
                    }
                } else {
                    // Message for external distributor
                    println!("For external distributor");
                    self.external_pending_messages.insert(ri.rdid as u64, PendingMessage::new(true, ri.rdid, read));
                }
            }
            
            for (id, msg) in self.local_pending_messages.iter() {
                if msg.can_send == false && msg.message_str == String::from("rm") { // Remove users
                    self.user_connections.tcp_connections.remove(id);
                }
            }
        }
    }
}
