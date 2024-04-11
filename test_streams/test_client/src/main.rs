use dlwp::{
    codes::REQUEST_RESPONSE,
    message::{contents_to_string, Message},
    stream::Stream,
};
use std::io::{stdin, stdout, Write};

fn main() {
    let mut stream = Stream::new(
        dlwp::stream::StreamType::Client {
            rid: 0,
            rdid: 0,
            port: 5000,
        },
        false,
    );

    stream.start();

    while stream.running() {
        let mut input = String::new();
        print!("Enter message (\"r\") to refresh) > ");
        stdout().flush().unwrap();
        stdin()
            .read_line(&mut input)
            .expect("Did not enter a correct string");
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
