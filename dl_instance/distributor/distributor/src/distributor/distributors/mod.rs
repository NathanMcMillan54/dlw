// This is for handling external distributors

use std::{thread::sleep, time::Duration};

use super::DarkLightDistributor;

pub mod tcp;

impl DarkLightDistributor {
    pub fn tcp_distributor_handler(&mut self) {
        loop {
            sleep_condition!(self.tcp_distributors.len() == 0);
        }
    }
}
