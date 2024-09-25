// This is for handling external distributors

use std::{io::Write, net::TcpStream, thread::{sleep, spawn}, time::Duration};

use lib_dldistributor::{external::ExternalDistributorRW, IDLE_SLEEP};
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

    pub fn tcp_distributor_handler(&mut self) {
        self.setup_tcp_distributors();
        self.tcp_distributors.push(TcpDistributor::new(TcpStream::connect("127.0.0.1:6000").unwrap(), String::new()));

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
                
                sleep(IDLE_SLEEP);
            }
        }
    }
}
