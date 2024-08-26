use std::{net::{SocketAddrV4, TcpStream}, str::FromStr};

use dlwp::cerpton::libcerpton_decode;

use crate::{VS1, VS2, VS3};

use super::DarkLightDistributor;

impl DarkLightDistributor {
    pub async fn get_verify_server(&mut self) {
        #[cfg(not(debug_assertions))]
        let file = reqwest::get("https://nathanmcmillan54.github.io/dlw/v0.1.0-alpha/erguorheoww83yr3297f2eq3f.txt") // This has the address of the verify server
        .await.expect("Failed to get URL")
        .text().await
        .expect("Failed to get text").replace("\n", "").replace(" ", "");
        
        #[cfg(not(debug_assertions))]
        let decoded = libcerpton_decode([VS1.parse().unwrap(), VS2.parse().unwrap(), VS3.parse().unwrap(), 0, 0, 0], file);
        
        #[cfg(not(debug_assertions))]
        let test_address = SocketAddrV4::from_str(&decoded).expect("Failed to parse verify server address");
    
        #[cfg(debug_assertions)]
        let test_address = SocketAddrV4::from_str("127.0.0.1:5000").unwrap();

        self.verify_server = Some(TcpStream::connect(test_address).expect("Failed to connect to verify server"));
    }
}
