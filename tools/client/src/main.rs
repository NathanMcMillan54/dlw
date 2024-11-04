use std::{
    io::{stdin, stdout, Write},
    thread::{self, sleep},
    time::Duration,
};

use dlcns::get::CNSGet;
use dlwp::{
    cerpton::{libcerpton_decode, libcerpton_encode},
    codes::Code,
    encryption::EncryptionInfo,
    message::contents_to_string,
    stream::{Stream, StreamType},
};

fn cmd_input() -> String {
    print!("> ");
    stdout().flush().unwrap();

    let mut ret = String::new();
    stdin().read_line(&mut ret).unwrap();

    ret = ret.replace("\n", "").replace("\r", "");
    ret
}

fn connect_by_id() -> Stream {
    let parts = vec!["Local Id", "Local Distributor", "Port"];
    let mut id = 0;
    let mut did = 0;
    let mut port = 0;

    for i in 0..3 {
        println!("Enter stream {}\n", parts[i]);

        let mut input = cmd_input();
        while input.parse::<u64>().is_err() {
            println!("Failed to parse input");
            input = cmd_input();
        }

        if i == 0 {
            id = input.parse::<u64>().unwrap();
        } else if i == 1 {
            did = input.parse::<u32>().unwrap();
        } else if i == 2 {
            port = input.parse::<u16>().unwrap();
        }
    }

    Stream::new(
        StreamType::Client {
            rid: id,
            rdid: did,
            port: port,
        },
        false,
    )
}

fn connect_by_name() -> Stream {
    println!("Enter a valid stream name");
    let name = cmd_input();

    let mut cns_get = CNSGet::new();
    let cns_name = cns_get.get_id(name.clone());

    if cns_name.is_none() {
        panic!(
            "Failed to find \"{}\", the name could not be found or does not exist",
            name
        );
    }

    let owner = cns_name.unwrap();
    Stream::new(
        StreamType::Client {
            rid: owner.id,
            rdid: owner.did,
            port: owner.port,
        },
        false,
    )
}

fn main() {
    println!("Connect by Id ('y'/'n')?");
    let mut ret = cmd_input();
    while ret != String::from("y") && ret != String::from("n") {
        println!("Enter 'y' or 'n'");
        ret = cmd_input();
    }

    let mut stream = if ret == String::from("y") {
        connect_by_id()
    } else {
        connect_by_name()
    };

    println!("Enter encryption setting (using Cerpton cipher)");
    let mut setting = [0; 6];
    for i in 0..6 {
        let mut s = cmd_input();

        while s.parse::<i32>().is_err() {
            println!("Must be 32 bit integer");
            s = cmd_input();
        }

        setting[i] = s.parse::<i32>().unwrap();
    }

    stream.add_encryption_info(EncryptionInfo {
        encode_function: libcerpton_encode,
        decode_function: libcerpton_decode,
        info: setting,
    });
    stream.start();
    sleep(Duration::from_millis(150));
    println!("Started");
    println!("Type: \"!R!\" to refresh\n");

    while stream.running() {
        sleep(Duration::from_micros(200));
        for r in stream.read() {
            let contents = contents_to_string(r.contents).replace("\0", "");
            println!("Response: {:?} {}", Code::new(r.ti.code), contents);
        }

        println!("Enter code:");
        let mut code_str = cmd_input();

        if code_str == String::from("!R!") {
            continue;
        }

        while code_str.parse::<u16>().is_err() {
            println!("Enter valid code");
            code_str = cmd_input();
        }

        println!("Enter request:");
        let mut message = cmd_input();
        if message == String::from("!R!") {
            continue;
        }

        while message.len() >= 4096 {
            println!("Message cannot be longer than 4096 bytes");
            message = cmd_input();
        }

        println!("Sending...");
        stream.write(message, Code::new(code_str.parse::<u16>().unwrap()));
    }
}
