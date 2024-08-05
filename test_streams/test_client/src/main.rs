use dlwp::{
    cerpton::{libcerpton_decode, libcerpton_encode},
    codes::REQUEST_RESPONSE,
        chrono::{Utc, Timelike},
    encryption::EncryptionInfo,
    message::{contents_to_string, Message},
    stream::Stream,
};
use std::io::{stdin, stdout, Write};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut stream = Stream::new(
        // Add the client/server's ID and the Distributor ID
        dlwp::stream::StreamType::Client {
            rid: 505051114,
            rdid: 3,
            port: 5000,
        },
        false,
    );

    stream.add_encryption_info(EncryptionInfo {
        encode_function: libcerpton_encode,
        decode_function: libcerpton_decode,
        info: [2, 1, 2, 0, 0, 0],
    });
    stream.start();

    while stream.running() {
        let utc = Utc::now();
        let mut input = format!("Stuff2 {}:{}", utc.hour(), utc.minute());

        stream.write(input, REQUEST_RESPONSE);

        let read = stream.read();
        for i in 0..read.len() {
            println!(
                "Read: {}",
                contents_to_string(read[i].contents).replace("\0", "")
            );
        }

        sleep(Duration::new(15, 0));
    }
}
