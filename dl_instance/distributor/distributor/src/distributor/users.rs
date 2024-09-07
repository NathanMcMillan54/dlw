use std::{borrow::{Borrow, BorrowMut}, io::{Read, Write}, net::TcpStream, thread::sleep, time::Duration};

use dlwp::{
    distributor::READ_AVAILABLE, id::LId, message::{ReceiveInfo, MSG_END, MSG_INIT}
};

use super::DarkLightDistributor;
use crate::DISTRIBUTOR;

impl DarkLightDistributor {
    fn check_can_read(&self, id: &LId) -> bool {
        return if self.pending_messages.contains_key(id) {
            // For users just recently added
            if self.pending_messages[id].message_str == String::new() {
                sleep(Duration::from_micros(1000));
                true
            } else {
                false
            }
        } else {
            true
        }
    }

    fn tcp_user_read(&self, mut stream: &TcpStream) -> String {
        let mut buf = [0; 4096];
        let mut wait = 0;

        while wait < 400 {
            if wait % 80 == 0 || wait == 0 {
                stream.write(format!("{}{}{}", MSG_INIT, READ_AVAILABLE, MSG_END).as_bytes());
                stream.flush();
                sleep(Duration::from_micros(100));
            }

            stream.read(&mut buf);

            if buf != [0; 4096] {
                break;
            }

            wait += 1;
        }

        let mut read_str = String::from_utf8(buf.to_vec()).unwrap_or(String::new()).replace("\0", "");
        if read_str.starts_with(MSG_INIT) && read_str.ends_with(MSG_END) {
            read_str = read_str.replace(MSG_INIT, "").replace(MSG_END, "");
        }
        return read_str;
    }

    fn tcp_user_write_pending(&self, id: &LId) {

    }

    pub fn tcp_user_handler(&mut self) {
        loop {
            if self.user_connections.tcp_connections.len() <= 0 {
                sleep(Duration::from_millis(15)); // Slow down the loop
                continue;
            }

            for (id, mut stream) in self.user_connections.tcp_connections.iter() {
                if self.check_can_read(id) == false {
                    self.tcp_user_write_pending(id);
                    self.pending_messages.remove(id);
                    continue;
                } else {
                    self.pending_messages.remove(id);
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

                if ri.rdid == self.info.id {
                    // Local message
                } else {
                    // Message for external distributor
                }
            }
        }
    }
}
