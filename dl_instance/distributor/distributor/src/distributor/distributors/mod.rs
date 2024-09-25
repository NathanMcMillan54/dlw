// This is for handling external distributors

use std::{io::Write, net::TcpStream, thread::sleep, time::Duration};

use lib_dldistributor::external::ExternalDistributorRW;
use tcp::TcpDistributor;

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

            println!("Verifying {}...", self.info.config.tcp_connections[i].clone());
            let verify_ret = tcp_distributor.verify_distributor();
            if verify_ret == false {
                continue;
            } else {
                println!("Failed to verify");
            }

            self.tcp_distributors.push(tcp_distributor);
        }
    }

    pub fn tcp_distributor_handler(&mut self) {
        self.setup_tcp_distributors();

        loop {
            for i in 0..self.tcp_distributors.len() {
                if self.tcp_distributors[i].msg == String::from("INIT-DIS-CONN") {
                    println!("Verifying external distirbutor connection...");
                    let verify_ret = self.tcp_distributors[i].verify_distributor();
                    if verify_ret == false {
                        self.tcp_distributors.remove(i);
                        break;
                    }

                    println!("Connecting to verified distirbutor...");
                    let conn_ret = self.tcp_distributors[i].attempt_connect();
                    if conn_ret == false {
                        self.tcp_distributors.remove(i);
                    }
                }
            }
        }
    }
}
