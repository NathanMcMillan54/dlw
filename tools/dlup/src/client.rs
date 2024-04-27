use std::thread::sleep;
use std::time::Duration;

use dlwp::codes::*;
use dlwp::encryption::EncryptionInfo;
use dlwp::message::contents_to_string;
use dlwp::stream::{Stream, StreamType};

fn main() {
    // The DarkLight information servers should always be on id 1000 and distributor 3
    let mut stream = Stream::new(
        StreamType::Client {
            rid: 1000,
            rdid: 3,
            port: 5000,
        },
        false,
    );

    stream.add_encryption_info(EncryptionInfo {
        decode_function: dlwp::cerpton::libcerpton_decode,
        encode_function: dlwp::cerpton::libcerpton_encode,
        // The DarkLight information servers (at the moment) will use this encryption information
        info: [2, 1, 0, 0, 0, 0],
    });

    stream.start();

    while stream.running() {
        let read = stream.read();
        for r in read {
            let string = contents_to_string(r.contents);

            if r.ti.code == STATUS_OK.value() && string.contains("YES") {
                println!("DarkLight is running");
                stream.stop();
            } else {
                println!("Waiting...");
                sleep(Duration::from_millis(700));
            }
        }

        stream.write(String::from("is this working?"), REQUEST_RESPONSE);
    }
}
