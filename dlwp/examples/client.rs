use dlwp::{
    cerpton::{libcerpton_decode, libcerpton_encode},
    chrono::{Timelike, Utc},
    codes::REQUEST_RESPONSE,
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
            rid: 9711410197108107101,
            rdid: 4,
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
        let mut input = format!("Stuff1 {}:{}:{}", utc.hour(), utc.minute(), utc.second());

        stream.write(input, REQUEST_RESPONSE);

        let read = stream.read_with_timestamp();
        for i in 0..read.len() {
            println!(
                "Read: {} At: {:?}",
                contents_to_string(read[i].message.contents).replace("\0", ""),
                read[i].recv_time
            );
        }

        sleep(Duration::new(5, 0));
    }
}
