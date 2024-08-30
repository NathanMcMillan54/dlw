// This is the "New key server" that is described in ``documentation/information_servers.md``
// 

use std::{io::{Read, Write}, net::TcpStream, thread::sleep, time::Duration};

use dlwp::{cerpton::{libcerpton_decode, libcerpton_encode}, codes::REGULAR_RESPONSE, encryption::EncryptionInfo, message::TransmitInfo, stream::{Stream, StreamType}};
use fernet::Fernet;
use rand::{thread_rng, Rng};

const REQUEST_KEY: [&str; 3] = [env!("REQUEST_KEY1"), env!("REQUEST_KEY2"), env!("REQUEST_KEY3")];
const SETTING: [&str; 3] = [env!("S1"), env!("S2"), env!("S3")];

#[cfg(not(debug_assertions))]
const VERIFY_SERVER: &str = env!("VS_ADDR");

#[cfg(debug_assertions)]
const VERIFY_SERVER: &str = "127.0.0.1:5000";

fn key() -> String {
    // Slightly increases randomness
    let mut rng = thread_rng();
    let random1 = rng.gen_range(100..1000);
    let random2 = rng.gen_range(1..random1);

    sleep(Duration::from_micros(random2));

    Fernet::generate_key()
}

fn main() {
    let mut stream = Stream::new(StreamType::Server { port: 4998 }, false);
    
    stream.add_encryption_info(EncryptionInfo {
        info: [2, 1, 2, 0, 0, 0],
        encode_function: libcerpton_encode,
        decode_function: libcerpton_decode,
    });

    stream.start();
    sleep(Duration::from_millis(100));

    while stream.running() {
        for read in stream.read() {
            if !stream.check_add_connection(read) {
                continue;
            }

            // TODO: Properly check that the client did not recently request a new key
            let mut verify_server = TcpStream::connect(VERIFY_SERVER).unwrap();
            let new_key = key();
            let write = libcerpton_encode([SETTING[0].parse().unwrap(), SETTING[1].parse().unwrap(), SETTING[2].parse().unwrap(), 0, 0, 0], format!("{} {} {} {}", REQUEST_KEY[0], REQUEST_KEY[1], REQUEST_KEY[2], new_key));
            verify_server.write(write.as_bytes());
            verify_server.flush();

            let mut buf = [0; 100];
            verify_server.read(&mut buf);

            let vs_read = String::from_utf8(buf.to_vec()).unwrap().replace("\0", "");
            if vs_read.is_empty() || vs_read == String::from("INVALID") {
                continue;
            }

            stream.server_write(read.ti, new_key, REGULAR_RESPONSE);
        }
    }
}
