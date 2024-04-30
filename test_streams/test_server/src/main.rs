use dlwp::codes::{Code, REQUEST_RESPONSE, STATUS_OK};
use dlwp::message::ReceiveInfo;
use dlwp::stream::{Stream, StreamType};
use std::{thread::sleep, time::Duration};

const PORT: u16 = 5000;

fn main() {
    let mut stream = Stream::new(StreamType::Server { port: PORT }, false);

    println!("starting");
    let ret = stream.start();
    println!("started: {:?}", ret);
    sleep(Duration::from_millis(1500));

    while stream.running() {
        let read = stream.read();
        println!("{}", read.len());
        for r in read {
            if !stream.check_add_connection(r) {
                continue;
            }

            stream.server_write(r.ti, String::from("wsg?"), REQUEST_RESPONSE);
        }

        sleep(Duration::from_millis(80));
    }
}
