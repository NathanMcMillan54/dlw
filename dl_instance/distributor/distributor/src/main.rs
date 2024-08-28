use std::sync::Mutex;

use distributor::DarkLightDistributor;

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

lazy_static::lazy_static! {
    pub(crate) static ref DISTRIBUTOR: Mutex<DarkLightDistributor> = Mutex::new(DarkLightDistributor::new());
}

#[tokio::main]
async fn main() {
    println!("Reading config...");
    DISTRIBUTOR.lock().unwrap().set_config(&CONFIG_PATH);
    println!("Set config file");
    println!("Connecting to verify server...");
    DISTRIBUTOR.lock().unwrap().get_verify_server().await;
    DISTRIBUTOR.lock().unwrap().tcp_check_add();
}
