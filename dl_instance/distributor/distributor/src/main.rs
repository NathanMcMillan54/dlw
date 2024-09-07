use std::{
    borrow::BorrowMut,
    thread::{self, sleep},
    time::Duration,
};

use distributor::{users, DarkLightDistributor};

extern crate dlwp;
extern crate lib_dldistributor;
#[macro_use]
extern crate tokio;

// Verify Server Information Settings
const VS1: &str = env!("VS1");
const VS2: &str = env!("VS2");
const VS3: &str = env!("VS3");

// Given distributor ID
const DISTRIBUTOR_ID: &str = env!("DIST_ID");
// Second distributor ID
const DISTRIBUTOR_UID: &str = env!("DIST_UID");

// Path to config file
const CONFIG_PATH: &str = "distributor_config.json";

mod distributor;

static mut DISTRIBUTOR: Option<DarkLightDistributor> = None;

#[tokio::main]
async fn main() {
    println!("Reading config...");
    let mut distributor = DarkLightDistributor::new();
    unsafe {
        DISTRIBUTOR = Some(distributor);
        DISTRIBUTOR.as_mut().unwrap().set_config(&CONFIG_PATH);
        println!("Set config file");
        println!("Connecting to verify server...");
        DISTRIBUTOR.as_mut().unwrap().get_verify_server().await;

        println!("Starting...");
        thread::spawn(|| {
            DISTRIBUTOR.as_mut().unwrap().tcp_check_add();
        });

        thread::spawn(|| {
            DISTRIBUTOR.as_mut().unwrap().tcp_user_handler();
        });
    }

    loop {
        sleep(Duration::from_millis(1000))
    }
}
