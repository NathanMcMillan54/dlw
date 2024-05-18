use std::{fmt::format, thread::sleep, time::Duration};

use dlcns::{CNS_DISTRIBUTOR, CNS_ID, CNS_PORT};
use dlwp::{
    cerpton::{alphabet::ALPHABET_LEN, libcerpton_encode, libcerpton_decode, Encoder}, chrono::{Timelike, Utc}, codes::{INVALID_RR, REGULAR_RESPONSE, REQUEST_CONNECTION, REQUEST_RESPONSE}, id::local_user_id, langs::is_human_readable_including, message::contents_to_string, stream::Stream,
    encryption::EncryptionInfo
};
use rand::{thread_rng, Rng};

use crate::ids::first_key;

pub fn setup_cns() -> ([i32; 6], String, String) {
    let mut utc = Utc::now();
    let start_hour = utc.hour();
    let start_minute = utc.minute();
    let start_second = utc.second();

    let mut rng = thread_rng();
    let (mut s1, mut s2) = (
        rng.gen_range(1..ALPHABET_LEN),
        rng.gen_range(1..ALPHABET_LEN),
    );
    let mut encoder = Encoder::new(s1, s2);
    let mut s3 = 0;
    encoder.set_alphabet();
    println!("{} {}", s1, s2);

    while encoder.setting_good() == false {
        println!("here");
        s1 = rng.gen_range(0..ALPHABET_LEN);
        s2 = rng.gen_range(0..ALPHABET_LEN);
        encoder.change_setting(s1, s2);
        encoder.set_alphabet();
        println!("Running");
    }

    s3 = rng.gen_range(0..i32::MAX / 16);

    println!("Encoding your keys...\nThis may take a few minutes. Start time: {}:{}:{}", start_hour, start_minute, start_second);
    if s3 > 3500 * 1000 {
        println!("This encryption setting is large");
    } else {
        println!("Encryption setting is not very large");
    }

    let current_key = libcerpton_encode([s1, s2, s3, 0, 0, 0], env!("DLU_KEY").to_string());
    let first_key = libcerpton_encode([s1, s2, s3, 0, 0, 0], first_key());
    println!("Encoded: {} {}", current_key, first_key);
    utc = Utc::now();
    let hours = (utc.hour() as i32 - start_hour as i32).abs();
    let minutes = (utc.minute() as i32 - start_hour as i32).abs();
    let seconds = (utc.second() as i32 - start_second as i32).abs();

    println!("Time to encode: {}:{}:{}", hours, minutes, seconds);
    if minutes >= 1 {
        println!("Request may take a few minutes");
    }

    return ([s1, s2, s3, 0, 0, 0], current_key, first_key);
}

pub fn cns_add(input: Vec<&str>) {
    let readable = is_human_readable_including(input[0].to_string(), vec!['-', '_']);
    let parseable = input[1].parse::<u16>().is_ok();

    if readable == false {
        println!("Input name: {} is invalid, read (documentation)", input[1]);
    } else if parseable == false {
        println!("Port {} is invalid", input[2]);
    }

    let mut stream = Stream::new(
        dlwp::stream::StreamType::Client {
            rid: CNS_ID,
            rdid: CNS_DISTRIBUTOR,
            port: CNS_PORT,
        },
        false,
    );

    stream.add_encryption_info(EncryptionInfo {
        encode_function: libcerpton_encode,
        decode_function: libcerpton_decode,
        info: [2, 1, 2, 0, 0, 0],
    });

    stream.start();
    sleep(Duration::from_millis(100));

    let (setting, current_key, first_key) = setup_cns();

    let mut send_first = false;
    let mut send_second = false;
    let mut recv_first = false;
    let mut recv_second = false;

    println!("sending...");
    while stream.running() {
        sleep(Duration::from_millis(100));
        if send_first == false {
            println!("Sending first...");
            stream.write(format!("REQUEST_ADD0 {} {} {} {} {}", setting[0], setting[1], setting[2], current_key, first_key), REQUEST_RESPONSE);
            send_first = true;
            continue;
        }

        if recv_first == false {
            let read = stream.read();
            if read.is_empty() {
                continue;
            }
            let contents = contents_to_string(read[0].contents).replace("\0", "");
            if read[0].ti.code != REQUEST_CONNECTION.value() && read[0].ti.code == REGULAR_RESPONSE.value() {
                if contents.contains("ALLOW_ADD0") {
                    println!("receving first");
                    recv_first = true;
                    println!("Name request is allowed, {} names registered", contents.replace("ALLOW_ADD0 ", ""));
                }
            } else if read[0].ti.code == INVALID_RR.value() {
                println!("An error occured1: {}", contents);
                send_first = false;
                recv_first = false;
                continue;
            }
        }

        if recv_first == true && recv_first == true && send_second == false {
            send_second = true;
            println!("sending second");
            stream.write(format!("REQUEST_ADD1 {} {} {} {} {} {} {}", setting[0], setting[1], setting[2], current_key, first_key, input[0], input[1]), REQUEST_RESPONSE);
        }

        if send_second == true && recv_second == false {
            let read = stream.read();
            if read.is_empty() {
                continue;
            }
            let contents = contents_to_string(read[0].contents).replace("\0", "");
            if read[0].ti.code != REQUEST_CONNECTION.value() && read[0].ti.code == REGULAR_RESPONSE.value() {
                if contents.contains("Added name") {
                    println!("receving second");
                    recv_second = true;
                    println!("Name: {} is now asociated with {}-{}", input[0], local_user_id().unwrap(), input[1]);
                }
            } else if read[0].ti.code == INVALID_RR.value() {
                println!("An error occured2: {}", contents);
                return;
            }
        }
    }
}
