// This binary needs to be built with six environment variables when testing:
// S1, S2, S3 - Cerpton encryption setting
// REQUEST_KEY1, REQUEST_KEY2, REQUEST_KEY3 - ``keyss`` server uses these to generate new keys
// Build:
// S1=1 S2=2 S3=1 REQUEST_KEY1=A REQUEST_KEY2=B REQUEST_KEY3=C cargo build --bin verify_server
// Make sure that the same values are used for other services while testing. This server will run on :5000 by default.
// When building in release mode BIND_ADDR is needed:
// S1=1 S2=2 S3=1 REQUEST_KEY1=A REQUEST_KEY2=B REQUEST_KEY3=C BIND_ADDR=0.0.0.0:5000 cargo build --bin verify_server --release

extern crate dlwp;
#[macro_use]
extern crate serde;

use std::{
    fs::{read_to_string, File}, io::{Read, Write}, net::{Shutdown, TcpListener, TcpStream}, panic::{set_hook, PanicInfo}, path::Path, process::exit, thread
};

use dlwp::{
    cerpton::{libcerpton_decode, libcerpton_encode},
    serde_json,
};
use signal_hook::{consts::{SIGINT, SIGTERM}, iterator::Signals};

// This should be a proper database someday
const USER_KEYS_FILE: &str = "user_keys.json";
const DISTRIBUTOR_KEYS_FILE: &str = "distributor_keys.json";

const SETTING: [&str; 3] = [env!("S1"), env!("S2"), env!("S3")];
// If these values are received then a new key should be generated. Only keyss should have these values!
const REQUEST_KEY: [&str; 3] = [
    env!("REQUEST_KEY1"),
    env!("REQUEST_KEY2"),
    env!("REQUEST_KEY2"),
];

#[derive(PartialEq, Deserialize, Serialize)]
struct UsedUserKey {
    key: String,
    id: u64,
    os: u8,
    arch: u8,
    date: [u8; 2],
}

impl UsedUserKey {
    pub fn new(key: String, id: u64, os: u8, arch: u8, day: u8, month: u8) -> Self {
        return UsedUserKey {
            key,
            id,
            os,
            arch,
            date: [day, month],
        };
    }
}

#[derive(Deserialize, Serialize)]
struct UserKeys {
    unused_keys: Vec<String>,
    used_keys: Vec<UsedUserKey>,
}

impl UserKeys {
    pub const fn empty() -> Self {
        return UserKeys {
            unused_keys: vec![],
            used_keys: vec![],
        };
    }

    pub fn valid_unused(&self, key: String) -> bool {
        return if self.unused_keys.contains(&key) {
            true
        } else {
            false
        };
    }

    pub fn valid_used(&self, key: String, id: u64, os: u8, arch: u8, day: u8, month: u8) -> bool {
        let user_key = UsedUserKey::new(key, id, os, arch, day, month);

        return if self.used_keys.contains(&user_key) {
            true
        } else {
            false
        };
    }
}

#[derive(Deserialize, Serialize)]
struct DistributorKey {
    key: String,
    id: u64,
}

impl DistributorKey {
    pub fn new(key: String, id: u64) -> Self {
        return DistributorKey {
            key,
            id,
        }
    }
}

#[derive(Deserialize, Serialize)]
struct DistirbutorKeys {
    keys: Vec<DistributorKey>
}

impl DistirbutorKeys {
    pub const fn empty() -> Self {
        return DistirbutorKeys {
            keys: vec![],
        };
    }
}

static mut USER_KEYS: UserKeys = UserKeys::empty();
static mut DISTRIBUTOR_KEYS: DistirbutorKeys = DistirbutorKeys::empty();

fn verify_user_key(mut client: TcpStream, input: Vec<&str>) {
    println!("Verifying user...");
    let key = input[0];
    let id = input[1].parse::<u64>();
    let os = input[2].parse::<u8>();
    let arch = input[3].parse::<u8>();
    let day = input[4].parse::<u8>();
    let month = input[5].parse::<u8>();

    if id.is_err() || os.is_err() || arch.is_err() || day.is_err() || month.is_err() {
        client.write(b"INVALID");
        client.shutdown(Shutdown::Both);
        return;
    }

    unsafe {
        if USER_KEYS.unused_keys.contains(&key.to_string()) {
            let pos = USER_KEYS.unused_keys.iter().position(|k| k == key).unwrap();
            USER_KEYS.unused_keys.remove(pos);
            USER_KEYS.used_keys.push(UsedUserKey::new(
                key.to_string(),
                id.unwrap(),
                os.unwrap(),
                arch.unwrap(),
                day.unwrap(),
                month.unwrap(),
            ));
            client.write(b"VALID");
        } else if USER_KEYS.valid_used(
            key.to_string(),
            id.unwrap(),
            os.unwrap(),
            arch.unwrap(),
            day.unwrap(),
            month.unwrap(),
        ) {
            client.write(b"VALID");
        } else {
            client.write(b"INVALID");
        }
    }
    client.shutdown(Shutdown::Both);
}

fn add_key(mut client: TcpStream, input: Vec<&str>) {
    println!("Creating new key");
}

fn handle_client(mut client: TcpStream) {
    let mut buf = [0; 100];
    let ret = client.read(&mut buf);

    if ret.is_err() {
        client.shutdown(Shutdown::Both);
        return;
    }

    let read_str = String::from_utf8(buf.to_vec());
    if read_str.is_err() {
        client.shutdown(Shutdown::Both);
        return;
    }

    let read = libcerpton_decode(
        [
            SETTING[0].parse().unwrap(),
            SETTING[1].parse().unwrap(),
            SETTING[2].parse().unwrap(),
            0,
            0,
            0,
        ],
        read_str.unwrap(),
    );
    let split = read.split(" ").collect::<Vec<&str>>();

    if split.len() == 6 {
        verify_user_key(client, split);
    } else if split.len() == 5 {
        add_key(client, split);
    } else {
        client.write(b"INVALID");
        client.shutdown(Shutdown::Both);
    }
}

// Save all information before termination
fn panic_handler(info: &PanicInfo) {
    unsafe {
        File::options().write(true).open(USER_KEYS_FILE).unwrap().write_fmt(format_args!("{}", serde_json::to_string_pretty(&USER_KEYS).unwrap()));
        File::options().write(true).open(DISTRIBUTOR_KEYS_FILE).unwrap().write_fmt(format_args!("{}", serde_json::to_string_pretty(&DISTRIBUTOR_KEYS).unwrap()));
    }

    println!("{:?}", info);
    exit(0);
}

fn setup() {
    if Path::new(USER_KEYS_FILE).exists() {
        let file_contents = read_to_string(USER_KEYS_FILE).unwrap();
        let user_keys: UserKeys = serde_json::from_str(&file_contents).unwrap();
        unsafe { USER_KEYS = user_keys; }
    } else {
        File::create(USER_KEYS_FILE).unwrap();
    }

    if Path::new(DISTRIBUTOR_KEYS_FILE).exists() {
        let file_contents = read_to_string(DISTRIBUTOR_KEYS_FILE).unwrap();
        let distributor_keys = serde_json::from_str(&file_contents).unwrap();
        unsafe { DISTRIBUTOR_KEYS = distributor_keys; }
    } else {
        File::create(DISTRIBUTOR_KEYS_FILE).unwrap();
    }

    set_hook(Box::new(panic_handler));
}

fn main() {
    println!("Reading key files...");
    setup();

    // Handle intentional or unexpected termination
    let mut signals = Signals::new(&[SIGTERM, SIGINT]).unwrap();
    thread::spawn(move || {
        for signal in signals.forever() {
            if signal == SIGTERM || signal == SIGTERM {
                panic!("Stopping verify_server...");
            }
        }
    });

    println!("Starting...");
    #[cfg(not(debug_assertions))]
    let mut tcp_listener = TcpListener::bind(env!("BIND_ADDR")).unwrap();

    #[cfg(debug_assertions)]
    let mut tcp_listener = TcpListener::bind("0.0.0.0:5000").unwrap();

    for stream in tcp_listener.incoming() {
        handle_client(stream.unwrap());
        continue;
    }
}
