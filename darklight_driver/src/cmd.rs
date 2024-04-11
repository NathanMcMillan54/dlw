use crate::streams::{StreamInfo, STREAMS_HANDLER};
use dlwp::message::Message;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread::sleep;
use std::time::Duration;

pub fn cmd_input_thread() {
    File::create("/tmp/darklight/cmd_input").unwrap();

    println!("reading");
    loop {
        let reader = BufReader::new(File::open("/tmp/darklight/cmd_input").unwrap());

        for read_line in reader.lines() {
            let read = read_line.unwrap();

            if read.is_empty() {
                continue;
            }

            let inputs = read.split(" ").collect::<Vec<&str>>();

            if inputs[0].is_empty() {
                sleep(Duration::from_millis(250));
                continue;
            }

            unsafe {
                if STREAMS_HANDLER.config.closed == true {
                    STREAMS_HANDLER.config.tcp = false;
                    STREAMS_HANDLER.config.serial = false;
                }
            }

            match inputs[0] {
                "config" => {
                    match inputs[1] {
                        "closed" => {
                            let value = inputs[2].replace("\n", "");
                            println!("closed: {}", value);
                            unsafe {
                                STREAMS_HANDLER.config.closed = {
                                    if value == "true" {
                                        true
                                    } else if value == "false" {
                                        println!("false");
                                        false
                                    } else {
                                        println!("closed must be \"true\" or \"false\"");
                                        continue;
                                    }
                                };
                            }
                        }
                        "tcp" => {
                            let value = inputs[2].replace("\n", "");
                            println!("tcp: {}", value);
                            unsafe {
                                STREAMS_HANDLER.config.tcp = {
                                    if value == "true" {
                                        true
                                    } else if value == "false" {
                                        false
                                    } else {
                                        println!("tcp must be \"true\" or \"false\"");
                                        continue;
                                    }
                                };
                            }
                        }
                        _ => println!("{} is an invalid argument for \"config\"", inputs[1]),
                    };
                }
                "CONNECT" => {
                    let rdid = inputs[1].parse::<u32>().unwrap();
                    let rid = inputs[2].parse::<u64>().unwrap();
                    let port = inputs[3].parse::<u16>().unwrap();
                    let instance = inputs[4].parse::<u32>().unwrap();
                    let day = inputs[5].parse::<i32>().unwrap();
                    let week = inputs[6].parse::<i32>().unwrap();
                    let month = inputs[7].parse::<i32>().unwrap();

                    unsafe {
                        if STREAMS_HANDLER.stream_exists(rid, port) {
                            println!("Stream {}-{} already exists", rid, port);
                            println!(
                                "If this stream is on a different distributor, see issue #(n)"
                            );
                        } else {
                            STREAMS_HANDLER.add_stream(StreamInfo {
                                rid,
                                rdid,
                                port,
                                instance_id: instance,
                                connected: true,
                                sent_connection_request: true,
                                waited: 0,
                                received: vec![],
                                last_minute: 0,
                                pending: vec![],
                                info: [day, week, month, 0, 0, 0],
                            });
                        }
                    }

                    println!("Added {}-{} to stream handler", rid, port);
                }
                "SEND" => {
                    let mut message_str = String::new();
                    for i in 1..inputs.len() {
                        message_str.push_str(inputs[i]);
                        message_str.push(' ');
                    }

                    let rid = inputs[1].parse::<u64>().unwrap();
                    let rdid = inputs[2].parse::<u32>().unwrap();
                    let instance_id = inputs[3].parse::<u32>().unwrap();

                    unsafe {
                        for j in 0..STREAMS_HANDLER.stream_info.len() {
                            if STREAMS_HANDLER.stream_info[j].rid == rid
                                && STREAMS_HANDLER.stream_info[j].rdid == rdid
                                && STREAMS_HANDLER.stream_info[j].instance_id == instance_id
                            {
                                STREAMS_HANDLER.stream_info[j]
                                    .pending
                                    .push(message_str.clone());
                            }
                        }
                    }
                }
                "DISCONNECT" => {
                    println!("disconnect arguments: {:?}", inputs);
                    let rid = inputs[1].parse::<u64>().unwrap();
                    let port = inputs[2].parse::<u16>().unwrap();
                    let rdid = inputs[2].parse::<u32>().unwrap();

                    unsafe {
                        for i in 0..STREAMS_HANDLER.stream_info.len() {
                            if STREAMS_HANDLER.stream_info[i].rid == rid
                                && STREAMS_HANDLER.stream_info[i].port == port
                            {
                                println!("removing...");
                                STREAMS_HANDLER.remove_stream_file(rid, port);
                                STREAMS_HANDLER.stream_info.remove(i);
                            }
                        }
                    }
                }
                _ => {
                    println!("Invalid input: {:?}", inputs[0]);
                }
            }

            //drop(inputs);

            File::create("/tmp/darklight/cmd_input").unwrap();
        }

        sleep(Duration::from_millis(1));
    }
}
