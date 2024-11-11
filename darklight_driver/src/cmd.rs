use crate::cns::cns_add;
use crate::driver::DarkLightDriver;
use crate::streams::{StreamInfo, StreamsHandler};
use dlwp::id::{local_user_id, DId, LId, Port};
use dlwp::stream::file::StreamFile;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::thread::{sleep, spawn};
use std::time::Duration;

pub fn check_cmd_input(driver: &mut DarkLightDriver) {
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

        match inputs[0] {
            "config" => {
                match inputs[1] {
                    "closed" => {
                        let value = inputs[2].replace("\n", "");
                        driver.config.closed = {
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
                    "tcp" => {
                        let value = inputs[2].replace("\n", "");
                        driver.config.tcp = {
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

                let local = if rid == local_user_id().unwrap() {
                    true
                } else {
                    false
                };

                let stream_info = StreamInfo {
                    id: rid,
                    port,
                    did: rdid,
                    local,
                };

                if driver.streams_handler.streams.contains_key(&stream_info) {
                    println!("Stream {}-{} already exists", rid, port);
                    println!("If this stream is on a different distributor, see issue #(n)");
                } else {
                    driver.streams_handler.streams.insert(
                        stream_info,
                        StreamFile::new(rid, port, rdid, [day, week, month, 0, 0, 0]),
                    );
                    println!("Added {}-{} to stream handler", rid, port);
                }
            }
            "SEND" => {
                let mut message_str = String::new();
                for i in 1..inputs.len() {
                    message_str.push_str(inputs[i]);
                    message_str.push(' ');
                }

                let rid = inputs[1].parse::<u64>().unwrap();
                let rdid = inputs[2].parse::<u32>().unwrap();
                let port = inputs[4].parse::<u16>().unwrap();
                let local = if rid == local_user_id().unwrap() {
                    true
                } else {
                    false
                };

                let stream_info = StreamInfo {
                    id: rid,
                    port: port,
                    did: rdid,
                    local,
                };

                if driver.streams_handler.streams.contains_key(&stream_info) == false {
                    println!("Stream: {:?} does not exist", stream_info);
                    return;
                }

                let stream = driver
                    .streams_handler
                    .streams
                    .get_mut(&stream_info)
                    .unwrap();
                stream.pending.push(message_str);
            }
            "DISCONNECT" => {
                println!("disconnect arguments: {:?}", inputs);
                let rid = inputs[1].parse::<u64>().unwrap();
                let port = inputs[2].parse::<u16>().unwrap();
                let rdid = inputs[2].parse::<u32>().unwrap();
                let local = if rid == local_user_id().unwrap() {
                    true
                } else {
                    false
                };

                let stream_info = StreamInfo {
                    id: rid,
                    port,
                    did: rdid,
                    local,
                };

                sleep(Duration::from_millis(700));

                if driver.streams_handler.streams.contains_key(&stream_info) {
                    let stream = driver
                        .streams_handler
                        .streams
                        .remove(&stream_info)
                        .expect("Failed to remove");
                    stream.remove_all();
                }
            }
            "CLR_RECV" => {
                println!("here!");
                let did = inputs[1].parse::<DId>();
                let id_parse = inputs[2].parse::<LId>();
                let port = inputs[3].parse::<Port>();

                if did.is_err() || id_parse.is_err() || port.is_err() {
                    println!("Failed to parse arguments: {:?}", inputs);
                }
                let id = id_parse.unwrap();

                let streaminfo = StreamInfo {
                    id: id,
                    port: port.unwrap(),
                    did: did.unwrap(),
                    local: id == local_user_id().unwrap()
                };

                if !driver.streams_handler.streams.contains_key(&streaminfo) {
                    println!("Stream: {:?} does not exist", streaminfo);
                }

                println!("cleared");
                driver.streams_handler.streams.get_mut(&streaminfo).unwrap().received.clear();
                driver.streams_handler.streams.get_mut(&streaminfo).unwrap().write_received();
            }
            "REQUEST-ADD-NAME" => {
                println!("Requested to add a name");
                println!("Shutting down all streams...");

                for (info, stream) in driver.streams_handler.streams.iter() {
                    stream.remove_all();
                }
                driver.streams_handler.streams.clear();

                if inputs.len() != 4 {
                    println!("Invalid arguments {}", inputs.len());
                    continue;
                }

                let arg1 = Box::leak(inputs[1].to_string().into_boxed_str());
                let arg2 = Box::leak(inputs[2].to_string().into_boxed_str());
                let arg3 = Box::leak(inputs[3].to_string().into_boxed_str());
                spawn(move || {
                    cns_add(vec![arg1, arg2, arg3]);
                });
            }
            _ => {
                println!("Invalid input: {:?}, {:?}", inputs[0], inputs);
            }
        }

        File::create("/tmp/darklight/cmd_input").unwrap();
    }
}
