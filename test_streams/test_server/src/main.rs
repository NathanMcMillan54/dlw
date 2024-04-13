use dlwp::codes::STATUS_OK;
use dlwp::stream::{Stream, StreamType};
use std::{thread::sleep, time::Duration};

const PORT: u16 = 5000;

fn handle_client(tid: u64, tdid: u32) {
    let mut client = Stream::new(StreamType::Client {
        rid: tid,
        rdid: tdid,
        port: PORT,
    }, false);

    println!("handling....");

    client.start();
    sleep(Duration::from_millis(500));

    client.write(String::from("serving stuff"), STATUS_OK);
    println!("{:?}", client.stop());

    return;
}

fn main() {
    let mut stream = Stream::new(StreamType::Server { 
        port: PORT
    }, false);

    println!("starting");
    let ret = stream.start();
    println!("started: {:?}", ret);
    sleep(Duration::from_millis(1500));

    while stream.running() {
        let read = stream.read();
        println!("{}", read.len());
        for r in read {
            handle_client(r.ti.tid, r.ti.tdid);
        }

        sleep(Duration::from_millis(80));
    }
}
