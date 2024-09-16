use std::{
    borrow::BorrowMut,
    thread::{self, sleep},
    time::Duration,
};

use distributor::{users, DarkLightDistributor};
use dlwp::encryption::EncryptionInfo;
use lib_dldistributor::encryption::DistributorEncryption;

extern crate dlwp;
#[macro_use]
extern crate lib_dldistributor;
#[macro_use]
extern crate tokio;

// Verify Server encryption Settings
const VS1: &str = env!("VS1");
const VS2: &str = env!("VS2");
const VS3: &str = env!("VS3");

// Distributor encryption setting
const S1: &str = env!("SS1");
const S2: &str = env!("SS2");
const S3: &str = env!("SS3");

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
        DISTRIBUTOR.as_mut().unwrap().set_encrption();

        println!("Starting...");
        thread::spawn(|| {
            DISTRIBUTOR.as_mut().unwrap().tcp_check_add();
        });

        thread::spawn(|| {
            DISTRIBUTOR.as_mut().unwrap().tcp_user_handler();
        });

        thread::spawn(|| {
            DISTRIBUTOR.as_mut().unwrap().tcp_distributor_handler();
        });
    }

    loop {
        sleep(Duration::from_millis(250));
        unsafe { DISTRIBUTOR.as_mut().unwrap().dist_encrption.check_and_update() }
    }
}
