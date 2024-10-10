use std::fs::read_to_string;

use dlwp::{cerpton::{libcerpton_decode, libcerpton_encode}, codes::{FILE_RESPONSE, REQUEST_CONNECTION, REQUEST_FILE}, encryption::EncryptionInfo, id::Port, message::contents_to_string, stream::Stream};

#[cfg(feature = "visu_dl")]
const PORT: Port = 5000;

#[cfg(feature = "info_dl")]
const PORT: Port = 5001;

const HTML_FILES: [&str; 1] = ["main.html"];

const TXT_FILE: [&str; 1] = ["main.txt"];

fn main() {
    println!("Darklight web server");

    let mut stream = Stream::new(dlwp::stream::StreamType::Server { port: PORT }, false);
    stream.add_encryption_info(EncryptionInfo {
        encode_function: libcerpton_encode,
        decode_function: libcerpton_decode,
        info: [2, 1, 2, 0, 0, 0],
    });

    stream.start();

    while stream.running() {
        for read in stream.read() {
            if !stream.check_add_connection(read) {
                continue;
            }

            if read.ti.code == REQUEST_CONNECTION.value() {
                #[cfg(feature = "visu_dl")]
                let main = read_to_string("darklight-text/main.html").unwrap();

                #[cfg(feature = "info_dl")]
                let main = read_to_string("darklight-text/main.txt").unwrap();

                stream.server_write(read.ti, main, FILE_RESPONSE);
                continue;
            }

            let contents = contents_to_string(read.contents).replace("\0", "");
            if contents.contains("../") || contents.starts_with("/") {
                continue;
            }

            if read.ti.code == REQUEST_FILE.value() {
                if HTML_FILES.contains(&contents.as_str()) || TXT_FILES.contains(&contents.as_str()) {
                    let file = read_to_string(format!("darklight-text/{}", contents)).unwrap();
                    stream.server_write(read.ti, file, FILE_RESPONSE);
                }
            }
        }
    }
}
