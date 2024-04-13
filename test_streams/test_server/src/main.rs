use dlwp::codes::{Code, STATUS_OK};
use dlwp::message::ReceiveInfo;
use dlwp::stream::{Stream, StreamType};
use std::{thread::sleep, time::Duration};

const PORT: u16 = 5000;

fn handle_client(tid: u64, tdid: u32) {
    let mut client = Stream::new(
        StreamType::Client {
            rid: tid,
            rdid: tdid,
            port: PORT,
        },
        false,
    );

    println!("handling....");

    client.start();
    sleep(Duration::from_millis(500));

    client.write(String::from("serving stuff"), STATUS_OK);
    sleep(Duration::from_millis(100));
    client.stop();

    return;
}

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
            println!("{:?}", Code::new(r.ti.code));
            handle_client(r.ti.tid, r.ti.tdid);
            stream.remove_connection(ReceiveInfo {
                rid: r.ti.tid,
                rdid: r.ti.tdid,
                instance_id: r.ri.instance_id,
                port: r.ri.port,
            });
        }

        sleep(Duration::from_millis(80));
    }
}
