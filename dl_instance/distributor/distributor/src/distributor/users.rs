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
    fn tcp_user_read(&self, mut stream: &TcpStream) -> String {
        let mut buf = [0; 4096];
        let mut wait = 0;

        while wait < 400 {
            if wait % 80 == 0 || wait == 0 {
                stream.write(format!("{}{}{}", MSG_INIT, READ_AVAILABLE, MSG_END).as_bytes());
                stream.flush();
                sleep(Duration::from_micros(100));
            }

            let ret = stream.read(&mut buf);
            if ret.is_err() {
                wait += 1;
                continue; // Write proper error handler
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
        return read_str;
    }

    fn tcp_user_write(&self, mut stream: &TcpStream, write: String) -> bool {
        let ret = stream.write(format!("{}{}{}", MSG_INIT, write, MSG_END).as_bytes());
        return if ret.is_ok() {
            stream.flush();
            true
        } else {
            false
        };
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
                //println!("on user: {}", id);

                if self.local_pending_messages.contains_key(id) {
                    if self.local_pending_messages[id].message_str == String::new() {
                        self.local_pending_messages.remove(id);
                    } else {
                        self.tcp_user_write(stream, self.local_pending_messages[id].message_str.clone());
                        self.local_pending_messages.remove(id);
                    }
                }

                let read = self.tcp_user_read(stream);

                // User sent nothing
                if read.is_empty() {
                    continue;
                }

                let ri = ReceiveInfo::get_from_message_string(read.clone());
                // Message might have been invalid
                if ri == ReceiveInfo::empty() {
                    continue;
                }

                //println!("read: {}", read);
                if ri.rdid == self.info.id {
                    if !self.user_connections.connection_exists(&ri.rid) {
                        // Notify user that connection doesn't exist
                    } else if self.user_connections.connection_is_tcp(&ri.rid) {
                        // Send immediatley
                        let ret = self
                            .tcp_user_write(&self.user_connections.tcp_connections[&ri.rid], read);
                        // if ret == false, notify the user
                    } else {
                        // It is a serial connection
                    }
                } else {
                    // Message for external distributor
                    self.external_pending_messages.insert(ri.rdid as u64, PendingMessage::new(true, ri.rdid, read));
                }
            }
        }
    }
}
