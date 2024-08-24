use crate::{config::InstanceConfig, distributors::DistributorInfo};
use dlwp::config::DistributorConfig;
use std::{
    fs::{read_to_string, File},
    io::Write,
    thread,
};
use tokio::{process::Command, task::spawn};

pub trait Instance {
    const CONFIG_PATH: &'static str = unimplemented!();

    /// The implemented struct should have a field called ``config`` which this function will set
    fn set_config(&mut self, _config: InstanceConfig) {
        unimplemented!()
    }

    fn start_services(&mut self) {
        unimplemented!()
    }

    /// Uses the "kill" command to kill the proccesses returned by ``self.pids()``
    fn stop_services(&self) {
        let pids = self.pids();

        for i in 0..pids.len() {
            Command::new("kill")
                .arg(pids[i].1.to_string())
                .spawn()
                .unwrap();
        }
    }

    /// Calls ``set_config`` if reading was successful
    fn read_config(&mut self) -> Option<InstanceConfig> {
        let contents = read_to_string(Self::CONFIG_PATH);

        if contents.is_err() {
            return None;
        }

        let config: Result<InstanceConfig, dlwp::serde_json::Error> =
            dlwp::serde_json::from_str(&contents.unwrap());

        if config.is_err() {
            return None;
        } else {
            self.set_config(config.unwrap());
            return Some(self.config());
        }
    }

    fn write_config(&self) -> bool {
        let file_ = File::options().write(true).open(Self::CONFIG_PATH);

        if file_.is_err() {
            return false;
        }

        let mut file = file_.unwrap();
        let contents = dlwp::serde_json::to_string_pretty(&self.config());

        if contents.is_err() {
            return false;
        }

        let file_write = file.write_fmt(format_args!("{}", contents.unwrap()));
        if file_write.is_err() {
            return false;
        }

        return true;
    }

    /// Should return ``self.config`` (field)
    fn config(&self) -> InstanceConfig {
        unimplemented!()
    }

    /// Should retunr ``self.pids`` (field)
    fn pids(&self) -> Vec<(String, u32)> {
        unimplemented!()
    }

    /// Check all distributors by calling ``distributor_check``
    fn run_distributor_check(&mut self) {
        unimplemented!()
    }
}
