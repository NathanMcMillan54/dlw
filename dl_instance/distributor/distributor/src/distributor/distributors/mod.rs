// This is for handling external distributors

use std::{thread::sleep, time::Duration};

use lib_dldistributor::external::ExternalDistributorRW;

use super::DarkLightDistributor;

pub mod tcp;

impl DarkLightDistributor {
    pub fn tcp_distributor_handler(&mut self) {
        loop {
            sleep_condition!(self.tcp_distributors.len() == 0);

            for i in 0..self.tcp_distributors.len() {
                if self.tcp_distributors[i].info.id == 0 {
                    let verify_ret = self.tcp_distributors[i].verify_distributor();
                    if verify_ret == false {
                        self.tcp_distributors.remove(i);
                        break;
                    }
                }
            }
        }
    }
}
