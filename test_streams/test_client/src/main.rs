use dlwp::{
    cerpton::{libcerpton_decode, libcerpton_encode}, codes::REQUEST_RESPONSE, encryption::EncryptionInfo, message::{contents_to_string, Message}, stream::Stream
};
use std::io::{stdin, stdout, Write};

fn main() {
    let mut stream = Stream::new(
        // Add the client/server's ID and the Distributor ID
        dlwp::stream::StreamType::Client {
            rid: 51115109995751,
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
        let mut input = String::new();
        print!("Enter message (\"r\") to refresh) > ");
        stdout().flush().unwrap();
        stdin()
            .read_line(&mut input)
            .expect("Failed to read string");
        if let Some('\n') = input.chars().next_back() {
            input.pop();
        }
        if let Some('\r') = input.chars().next_back() {
            input.pop();
        }

        if input != String::from("r") {
            stream.write(input, REQUEST_RESPONSE);
        }

        let read = stream.read();
        for i in 0..read.len() {
            println!(
                "Read: {}",
                contents_to_string(read[i].contents).replace("\0", "")
            );
        }
    }
}
