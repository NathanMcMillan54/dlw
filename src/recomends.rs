use std::{fs::read_to_string, thread::sleep, time::Duration};

use dlwp::{cerpton::{libcerpton_decode, libcerpton_encode}, codes::REGULAR_RESPONSE, encryption::EncryptionInfo, stream::{Stream, StreamType}};
use serde::{Deserialize, Serialize};
use dlwp::serde_json;

#[derive(Deserialize, Serialize)]
struct Version {
    pub minimum: String,
    pub obsolete_after: [u16; 3],
    pub latest: String,
}

#[derive(Deserialize, Serialize)]
struct Recomendations {
    pub dlwp_version: Version,
    pub driver_version: Version,
    pub user_encryption: Vec<String>,
}

fn main() {
    let mut stream = Stream::new(StreamType::Server { port: 4997 }, false);
    stream.add_encryption_info(EncryptionInfo {
        encode_function: libcerpton_encode,
        decode_function: libcerpton_decode,
        info: [2, 1, 2, 0, 0, 0],
    });

    let current_file = read_to_string("recomendations.json").unwrap();
    let verify_file_valid: Result<Recomendations, serde_json::Error> = serde_json::from_str(&current_file);

    if verify_file_valid.is_err() {
        panic!("Recomendations file is not valid json");
    }

    stream.start();
    sleep(Duration::from_millis(100));

    while stream.running() {
        for r in stream.read() {
            if stream.check_add_connection(r) == true {
                stream.server_write(r.ti, current_file.clone(), REGULAR_RESPONSE);
            }

            sleep(Duration::from_millis(150));
            stream.remove_connection(r.ri);
        }
    }
}
