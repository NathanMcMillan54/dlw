use std::{thread::sleep, time::Duration};

use procfs::process::{all_processes, Process};
use tokio::{io::AsyncReadExt, process::Command};

use crate::{config::InstanceConfig, instance::Instance};

// Call this at the end of any tests that started proccesses
fn kill_all_services(pids: Vec<(String, u32)>) {
    for i in 0..pids.len() {
        Command::new("kill")
            .arg(pids[i].1.to_string())
            .spawn()
            .unwrap();
    }
}

#[derive(Debug, Default)]
struct TestInstance {
    pub config: InstanceConfig,
    pub pids: Vec<(String, u32)>,
}

impl Instance for TestInstance {
    const CONFIG_PATH: &'static str = "test_config.json";

    fn set_config(&mut self, _config: InstanceConfig) {
        self.config = _config;
    }

    fn config(&self) -> InstanceConfig {
        self.config.clone()
    }

    fn pids(&self) -> Vec<(String, u32)> {
        self.pids.clone()
    }

    fn start_services(&mut self) {
        for i in 0..self.config.non_essential_services.len() {
            let proc = Command::new(self.config.non_essential_services[i].clone())
                .spawn()
                .unwrap();
            let pid = proc.id();

            self.pids
                .push((self.config.non_essential_services[i].clone(), pid.unwrap()));
        }
    }
}

#[test]
async fn test_start_services() {
    let mut test_instance = TestInstance::default();

    // Config stuff
    assert_eq!(test_instance.config.id, 0);
    test_instance.read_config();
    assert_eq!(test_instance.config.id, 1);

    // Services start
    test_instance.start_services();
    assert_eq!(test_instance.pids.len(), 1);
    assert_eq!(test_instance.pids[0].0, String::from("./runs_forever"));

    kill_all_services(test_instance.pids);
}

#[test]
async fn test_stop_services() {
    let mut test_instance = TestInstance::default();
    test_instance.read_config();

    test_instance.start_services();
    sleep(Duration::from_millis(1));

    kill_all_services(test_instance.pids.clone());

    for p in all_processes().unwrap() {
        if p.is_err() {
            continue;
        }
        let proc = p.unwrap();

        if proc.exe().is_err() {
            continue;
        }

        assert_ne!(
            proc.exe()
                .unwrap()
                .to_str()
                .unwrap()
                .contains(&test_instance.pids[0].0),
            true
        );
    }
}
