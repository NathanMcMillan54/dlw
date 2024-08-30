use std::{thread::sleep, time::Duration};

use super::DarkLightDistributor;

impl DarkLightDistributor {
    pub fn tcp_user_handler(&mut self) {
        loop {
            if self.user_connections.tcp_connections.len() <= 1 {
                sleep(Duration::from_millis(10)); // Slow down the loop
                continue;
            }
        }
    }
}
