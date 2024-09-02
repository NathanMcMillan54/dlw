use std::{io::Write, net::TcpStream, thread::sleep, time::Duration};

use dlwp::{
    distributor::READ_AVAILABLE,
    message::{MSG_END, MSG_INIT},
};

use super::DarkLightDistributor;
use crate::DISTRIBUTOR;

impl DarkLightDistributor {
    fn tcp_user_read(&self, mut stream: &TcpStream) -> String {
        let mut buf = [0; 4096];
        let mut wait = 0;

        while wait < 400 {
            if wait % 40 == 0 || wait == 0 {
                println!("Sending {}", wait);
                stream.write(format!("{}{}{}", MSG_INIT, READ_AVAILABLE, MSG_END).as_bytes());
                stream.flush();
            }

            wait += 1;
        }

        String::new()
    }

    pub fn tcp_user_handler(&mut self) {
        loop {
            if self.user_connections.tcp_connections.len() <= 1 {
                sleep(Duration::from_millis(15)); // Slow down the loop
                continue;
            }

            for (id, mut stream) in self.user_connections.tcp_connections.iter() {
                let read = self.tcp_user_read(stream);

                if read.is_empty() {
                    continue;
                }
            }
        }
    }
}
