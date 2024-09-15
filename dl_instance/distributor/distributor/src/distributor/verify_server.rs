use std::{
    io::{Read, Write},
    net::{SocketAddrV4, TcpStream},
    str::FromStr,
};

use dlwp::cerpton::{libcerpton_decode, libcerpton_encode};

use crate::{VS1, VS2, VS3};

use super::DarkLightDistributor;

impl DarkLightDistributor {
    pub async fn get_verify_server(&mut self) {
        #[cfg(not(debug_assertions))]
        let file = reqwest::get(
            "https://nathanmcmillan54.github.io/dlw/v0.1.0-alpha/erguorheoww83yr3297f2eq3f.txt",
        ) // This has the address of the verify server
        .await
        .expect("Failed to get URL")
        .text()
        .await
        .expect("Failed to get text")
        .replace("\n", "")
        .replace(" ", "");

        #[cfg(not(debug_assertions))]
        let decoded = libcerpton_decode(
            [
                VS1.parse().unwrap(),
                VS2.parse().unwrap(),
                VS3.parse().unwrap(),
                0,
                0,
                0,
            ],
            file,
        );

        #[cfg(not(debug_assertions))]
        let test_address =
            SocketAddrV4::from_str(&decoded).expect("Failed to parse verify server address");

        #[cfg(debug_assertions)]
        let test_address = SocketAddrV4::from_str("127.0.0.1:5000").unwrap();

        self.verify_server = test_address;
    }

    pub fn verify_input(&self, input: Vec<u8>) -> bool {
        if input.len() > 100 {
            return false;
        }

        let mut _verify_server = TcpStream::connect(self.verify_server);

        if _verify_server.is_err() {
            println!("Cannot connect to verify_server");
            return false;
        }

        let mut verify_server = _verify_server.unwrap();

        let encrypted = libcerpton_encode(
            [
                VS1.parse().unwrap(),
                VS2.parse().unwrap(),
                VS3.parse().unwrap(),
                0,
                0,
                0,
            ],
            String::from_utf8(input).unwrap().replace("INIT-USR ", ""),
        );
        verify_server.write(encrypted.as_bytes());
        verify_server.flush();

        let mut response_buf = [0; 10];

        while response_buf == [0; 10] {
            verify_server.read(&mut response_buf);
        }

        let response = String::from_utf8(response_buf.to_vec())
            .unwrap_or(String::new())
            .replace("\0", "");
        if response == String::from("VALID") {
            return true;
        } else {
            return false;
        }
    }
}
