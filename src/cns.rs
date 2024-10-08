use std::io::Write;
use std::thread::sleep;
use std::time::Duration;
use std::{fmt::format, fs::File, path::Path};

use dlcns::name::{Name, NamesList, Owner};
use dlcns::OWNERS_LIST;
use dlwp::cerpton::{libcerpton_encode, Encoder};
use dlwp::codes::REQUEST_CONNECTION;
use dlwp::encryption::EncryptionInfo;
use dlwp::message::TransmitInfo;
use dlwp::{
    cerpton::{libcerpton_decode, Decoder},
    codes::{Code, INVALID_RR, REGULAR_RESPONSE},
    message::contents_to_string,
    stream::{Stream, StreamType},
};

#[path = "cns/add.rs"]
mod add;

pub(crate) static mut NAMES_LIST: NamesList = NamesList::empty();

pub fn handle_message(contents: String, ti: TransmitInfo) -> (String, Code) {
    let split = contents.split(" ").collect::<Vec<&str>>();

    return if split[0].contains("GET_ID") {
        (String::from(""), REGULAR_RESPONSE)
    } else if split[0].contains("GET_NAME") {
        (String::from(""), REGULAR_RESPONSE)
    } else if split[0].contains("GET_ALL") {
        (String::from(""), REGULAR_RESPONSE)
    } else if split[0].contains("REQUEST_ADD") {
        unsafe { add::check_allowadd(contents, ti) }
    } else {
        (String::from("INVALID"), INVALID_RR)
    };
}

fn setup() {
    if Path::new(dlcns::OWNERS_LIST).exists() {
        unsafe {
            NAMES_LIST = NamesList::from_file(dlcns::OWNERS_LIST.to_string()).unwrap();
        }
        return;
    } else {
        unsafe {
            NAMES_LIST.list.push(Name {
                owner: Owner {
                    id: 1000,
                    did: 3,
                    port: 4999,
                    name: String::from("info.darklight.org"),
                    name_type: 0,
                },
                requests: 0,
                date: [21, 5, 2024],
                current_dlu_key: String::from("32r234"),
                og_dlu_key: String::from("32r234"),
            });
            let mut file = File::create(dlcns::OWNERS_LIST).unwrap();
            NAMES_LIST.write_to_file(dlcns::OWNERS_LIST.to_string());
        }
    }
}

fn main() {
    setup();

    let mut stream = Stream::new(StreamType::Server { port: 4999 }, false);

    stream.add_encryption_info(EncryptionInfo {
        encode_function: libcerpton_encode,
        decode_function: libcerpton_decode,
        info: [2, 1, 2, 0, 0, 0],
    });

    stream.start();

    sleep(Duration::from_millis(500));

    println!("Started CNS server.");
    while stream.running() {
        let read = stream.read();

        for r in read {
            if stream.check_add_connection(r) == false {
                continue;
            }

            if r.ti.code == REQUEST_CONNECTION.value() {
                continue;
            }

            let contents = contents_to_string(r.contents);
            let response = handle_message(contents, r.ti);

            stream.server_write(r.ti, response.0, response.1);
        }
    }
}
