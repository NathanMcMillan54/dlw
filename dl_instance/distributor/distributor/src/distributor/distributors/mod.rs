// This is for handling external distributors

use std::{io::Write, net::TcpStream, thread::{sleep, spawn}, time::Duration};

use dlwp::{codes::{Code, READ_TIMEDOUT, STATUS_OK}, distributor::READ_AVAILABLE, message::{valid_message_string, ReceiveInfo, MSG_END, MSG_INIT}};
use lib_dldistributor::{connections::PendingMessage, external::ExternalDistributorRW, IDLE_SLEEP};
use tcp::TcpDistributor;

use crate::DISTRIBUTOR;

use super::DarkLightDistributor;

pub mod tcp;

impl DarkLightDistributor {
    fn setup_tcp_distributors(&mut self) {
        for i in 0..self.info.config.tcp_connections.len() {
            let mut test_stream = TcpStream::connect(self.info.config.tcp_connections[i].clone());
            if test_stream.is_err() {
                println!("Cannot connect to {}", self.info.config.tcp_connections[i]);
                continue;
            }

            println!("Connecting to {}...", self.info.config.tcp_connections[i].clone());
            let mut stream = test_stream.unwrap();
            stream.write(b"INIT-DIS-CONN");
            let mut tcp_distributor = TcpDistributor::new(stream, String::new());
            let conn_ret = tcp_distributor.attempt_connect();

            if conn_ret == false {
                continue;
            }

            sleep(Duration::from_millis(600)); // Give external distributor time to move on
            println!("Verifying {}...", self.info.config.tcp_connections[i].clone());
            let verify_ret = tcp_distributor.verify_distributor();
            if verify_ret == false {
                println!("Failed to verify");
                continue;
            }
            
            println!("Adding distirbutor");

            self.tcp_distributors.push(tcp_distributor);
        }
    }

    pub fn tcp_distributor_read(&mut self, tcp_distributor_index: usize) -> String {
        let check_read = self.tcp_distributors[tcp_distributor_index].read();
        if check_read.0.is_empty() && check_read.1 == STATUS_OK {
            for wait in 0..400 {
                if wait % 80 == 0 {
                    self.tcp_distributors[tcp_distributor_index].write(format!("{} {} {} {}", MSG_INIT, READ_AVAILABLE, MSG_END, env!("DIST_IDENT")));
                    sleep(IDLE_SLEEP);
                }

                let read = self.tcp_distributors[tcp_distributor_index].read();

                if !read.0.is_empty() && check_read.1 == STATUS_OK {
                    println!("got read: {}", read.0);
                    return check_read.0.replace(&format!(" {}", env!("DIST_IDENT")), "");
                }
            }
        } else if valid_message_string(&check_read.0.replace(&format!(" {}", env!("DIST_IDENT")), ""), false) {
            return check_read.0.replace(&format!(" {}", env!("DIST_IDENT")), "");
        }

        String::new()
    }

    pub fn tcp_distributor_write(&mut self, tcp_distributor_index: usize, write: String) -> Code {
        let mut read = self.tcp_distributors[tcp_distributor_index].read();

        while !read.0.contains(READ_AVAILABLE) {
            read = self.tcp_distributors[tcp_distributor_index].read();
            sleep(IDLE_SLEEP);
        }

        return if read.0.contains(READ_AVAILABLE) {
            println!("writing...");
            self.tcp_distributors[tcp_distributor_index].write(format!("{} {}", write, env!("DIST_IDENT")))
        } else {
            READ_TIMEDOUT
        };
    }

    pub fn tcp_distributor_handler(&mut self) {
        self.setup_tcp_distributors();

        loop {
            for i in 0..self.tcp_distributors.len() {
                if self.tcp_distributors[i].msg == String::from("skp") {
                    continue;
                } else if self.tcp_distributors[i].msg == String::from("rm") {
                    self.tcp_distributors.remove(i);
                }

                if self.tcp_distributors[i].msg == String::from("INIT-DIS-CONN") {
                    self.tcp_distributors[i].msg = String::from("skp"); // Skip this distributor while the thread below is running
                    // Uses global distributor definition so there isn't interference between threads
                    spawn(move || {
                        unsafe {
                            println!("Verifying external distirbutor connection...");
                            let verify_ret = DISTRIBUTOR.as_mut().unwrap().tcp_distributors[i].verify_distributor();
                            if verify_ret == false {
                                DISTRIBUTOR.as_mut().unwrap().tcp_distributors[i].msg = String::from("rm"); // Remove in the next iteration
                                return;
                            }

                            println!("Connecting to verified distirbutor...");
                            let conn_ret = DISTRIBUTOR.as_mut().unwrap().tcp_distributors[i].attempt_connect();
                            if conn_ret == false {
                                DISTRIBUTOR.as_mut().unwrap().tcp_distributors[i].msg = String::from("rm");
                            } else {
                                DISTRIBUTOR.as_mut().unwrap().tcp_distributors[i].msg = String::new();
                            }
                            return;
                        }
                    });
                    continue;
                }

                let read = self.tcp_distributor_read(i);

                let ri = ReceiveInfo::get_from_message_string(read.clone());
                if ri.rdid == self.info.id {
                    self.pending_messages.insert(ri.rid, PendingMessage::new(true, self.info.id, read.clone()));
                    continue;
                } else {
                    for j in 0..self.tcp_distributors.len() {
                        if self.tcp_distributors[j].info.id == ri.rdid {
                            self.tcp_distributor_write(j, read.clone());
                        }
                    }
                }
                
                sleep(IDLE_SLEEP);
            }
        }
    }
}
