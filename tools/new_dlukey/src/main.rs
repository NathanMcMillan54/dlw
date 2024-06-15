use std::{thread::sleep, time::Duration};

use dlwp::{cerpton::{libcerpton_decode, libcerpton_encode}, codes::{CONNECTION_ACCEPTED, INVALID_RR, REGULAR_RESPONSE, REQUEST_CONNECTION, REQUEST_RESPONSE}, encryption::EncryptionInfo, message::contents_to_string, stream::{Stream, StreamType}};

fn main() {
    let mut stream = Stream::new(StreamType::Client { rid: 505051114, rdid: 3, port: 4998 }, false);
    stream.add_encryption_info(EncryptionInfo {
        encode_function: libcerpton_encode,
        decode_function: libcerpton_decode,
        info: [2, 1, 2, 0, 0, 0],
    });
    stream.start();

    sleep(Duration::from_millis(100));

    let mut wait = 0;
    while stream.running() {
        if wait == 1000 {
            println!("Sending another request");
            stream.write(String::from("check"), REQUEST_RESPONSE);
            wait = 0;
        }

        for r in stream.read() {
            let contents = contents_to_string(r.contents);
            if r.ti.code == INVALID_RR.value() || r.ti.code == REGULAR_RESPONSE.value() {
                println!("{}", contents);
            } else if r.ti.code == CONNECTION_ACCEPTED.value() || r.ti.code == REQUEST_CONNECTION.value() {
                continue;
            }

            println!("Key: '{}'", contents);
            stream.stop();
            break;
        }

        wait += 1;
        sleep(Duration::from_micros(750));
    }
}
