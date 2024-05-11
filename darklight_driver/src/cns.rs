use dlcns::{CNS_DISTRIBUTOR, CNS_ID, CNS_PORT};
use dlwp::{
    cerpton::{alphabet::ALPHABET_LEN, libcerpton_encode, Encoder}, chrono::{Timelike, Utc}, stream::Stream
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
    let (setting, current_key, first_key) = setup_cns();

    let mut stream = Stream::new(
        dlwp::stream::StreamType::Client {
            rid: CNS_ID,
            rdid: CNS_DISTRIBUTOR,
            port: CNS_PORT,
        },
        false,
    );
}
