use dlwp::chrono::{Timelike, Utc};
use dlwp::codes::{READ_SUCCESS, WRITE_FAILED, WRITE_TIMEDOUT};
use dlwp::config::DLConfig;
use dlwp::distributor::{GET_DISTRIBUTOR, USER_INIT};
use dlwp::id::distributor_id;
use dlwp::id::*;
use dlwp::io::{DLSerialIO, DLIO, DLTCPIO};
use dlwp::message::Message;
use dlwp::serialport::posix::TTYPort;
use std::fmt::{Debug, Formatter};
use std::fs::{remove_file, File};
use std::io::Write;
use std::net::SocketAddr;
use std::path::Path;
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;

#[allow(dead_code)]
pub struct StreamInfo {
    pub rid: LId,
    pub rdid: DId,
    pub port: Port,
    pub instance_id: InstanceID,
    pub connected: bool,
    pub(crate) sent_connection_request: bool,
    pub(crate) waited: u8,
    pub(crate) last_minute: u8,
    pub received: Vec<Message>,
    pub pending: Vec<String>,
    pub info: [i32; 6],
}

impl Debug for StreamInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StreamInfo")
            .field("rid", &self.rid)
            .field("rdid", &self.rdid)
            .field("port", &self.port)
            .field("instance_id", &self.instance_id)
            .field("connected", &self.connected)
            .finish()
    }
}

pub struct StreamsHandler {
    pub config: DLConfig,
    pub stream_info: Vec<StreamInfo>,
    pub io_method: Option<&'static mut dyn DLIO>,
    pub tcp: bool,
    pub serial: bool,
    pub dist_init: bool,
    pub dist_conn: bool,
    pub wait: u16,
    pub updating: bool,
}

impl StreamsHandler {
    pub(crate) fn create_stream_file(&self, rid: u64, port: u16) {
        println!("created file");
        File::options()
            .read(true)
            .write(true)
            .create(true)
            .open(&format!("/tmp/darklight/connections/_dl_{}-{}", rid, port))
            .unwrap();
    }

    fn write_to_stream_file(&self, rid: u64, port: u16, write: &String) {
        println!("rid: {}, port: {}", rid, port);
        File::options()
            .write(true)
            .create(true)
            .open(&format!("/tmp/darklight/connections/_dl_{}-{}", rid, port))
            .expect("Failed to open stream file")
            .write_fmt(format_args!("{}", write))
            .expect("Failed to write to stream file");
    }

    pub fn remove_stream_file(&self, rid: u64, port: u16) {
        remove_file(&format!("/tmp/darklight/connections/_dl_{}-{}", rid, port))
            .expect("Failed to remove stream file");
    }

    // TODO: For some reason, at the current moment (2024/1/7), the darklight driver does not properly receive the
    // transmitting distributor's id adn sets it to zero (between ``self.run`` and ``self.write_to_stream_file``). For
    // now, streams will only be recognized by their Id and port. This means that a driver can hand only one connection
    // of "id" and "port". Having the distributor id can allow two devices to accidentally (or intentionally) have the
    // local id. When that problem is fixed this function will be changed to include a distributor id.
    pub fn stream_exists(&self, rid: u64, port: u16) -> bool {
        return match self.stream_info.len() {
            0 => false,
            1 => {
                if self.stream_info[0].rid == rid && self.stream_info[0].port == port {
                    return true;
                } else {
                    return false;
                }
            }
            _ => {
                for i in 0..self.stream_info.len() {
                    if self.stream_info[i].rid == rid && self.stream_info[i].port == port {
                        return true;
                    }
                }

                return false;
            }
        };
    }

    pub fn add_stream(&mut self, stream_info: StreamInfo) {
        let rid = stream_info.rid;
        let port = stream_info.port;

        if self.stream_info.is_empty() {
            self.stream_info.push(stream_info);

            self.create_stream_file(rid, port);
            return;
        }

        for i in 0..self.stream_info.len() {
            if self.stream_info[i].rid == stream_info.rid
                && self.stream_info[i].port == stream_info.port
            {
                return;
            }
        }

        self.stream_info.push(stream_info);
        self.create_stream_file(rid, port);
    }

    pub fn run(&mut self) {
        loop {
            sleep(Duration::from_millis(80));

            // Check the I/O method
            if self.io_method.is_none()
                || self.tcp != self.config.tcp
                || self.serial != self.config.serial
            {
                if self.config.tcp == true && self.tcp == false {
                    static mut _TCP: DLTCPIO = DLTCPIO {
                        connections: vec![],
                        bind: String::new(),
                        stream: None,
                        listener: None,
                        listener_fn: None,
                        read: vec![],
                        check_ready: false,
                    };

                    println!("Using tcp...");
                    unsafe {
                        _TCP = DLTCPIO::new(String::new());
                        _TCP.bind = self.config.ip_address.to_string();
                        _TCP.connections.push(
                            SocketAddr::from_str(self.config.ip_address)
                                .expect("Failed to read \"ip_address\" from config"),
                        );
                        _TCP.init();
                        _TCP.check_ready = false;
                        self.io_method = Some(&mut _TCP as &mut dyn DLIO);
                    }

                    self.tcp = true;
                    self.serial = false;
                } else if self.config.serial == true && self.serial == false {
                    static mut _SERIAL: DLSerialIO = DLSerialIO {
                        port: None,
                        file: "",
                        info: (0, 0),
                    };

                    unsafe {
                        _SERIAL.port = Some(
                            TTYPort::open(
                                &Path::new(self.config.serial_path),
                                &DLSerialIO::default_settings(),
                            )
                            .unwrap(),
                        );
                        _SERIAL.init(true);

                        self.io_method = Some(&mut _SERIAL as &mut dyn DLIO);
                    }

                    self.serial = true;
                    self.tcp = false;
                } else {
                    return;
                }
            }

            // Get distributor information
            if self.dist_init == false {
                let ret = self
                    .io_method
                    .as_mut()
                    .unwrap()
                    ._write(format!("{}", GET_DISTRIBUTOR));
                if ret == WRITE_FAILED {
                    println!("Failed to write to distributor");
                    continue;
                } else {
                    println!("Successfully wrote to distributor");
                }

                let id = self.io_method.as_mut().unwrap()._read().0.replace("\0", "").replace(" ", "");

                if id.parse::<u64>().is_ok() {
                    if id != distributor_id().unwrap_or(0).to_string() {
                        println!("Setting new Distributor ID");
                    } else {
                        self.dist_init = true;
                        continue;
                    }

                    println!("Received distributor ID, writing to file...");
                    let mut did_file = File::options()
                        .write(true)
                        .open("/etc/dlw/local_did")
                        .unwrap();
                    did_file.write_fmt(format_args!("{}", id)).unwrap();
                    self.dist_init = true;
                } else {
                    println!("Failed to read Distributor ID")
                }

                continue;
            } else if self.dist_init == true && self.dist_conn == false {
                println!("Connecting Stream to Distributor...");
                let ret = self.io_method.as_mut().unwrap()._write(format!(
                    "{} {} {} {} {} {} {}",
                    USER_INIT,
                    env!("DLU_KEY"),
                    local_user_id().unwrap(),
                    env!("UC_OS"),
                    env!("UC_ARCH"),
                    env!("UC_DAY"),
                    env!("UC_MONTH")
                ));

                if ret != WRITE_FAILED || ret != WRITE_TIMEDOUT {
                    let mut read = String::new();
                    println!("Waiting for response from Distributor...");
                    while read.is_empty() {
                        read = self.io_method.as_mut().unwrap()._read().0;
                    }

                    if read.contains("CONN") {
                        println!("Connected");
                        self.dist_conn = true;
                    }
                }

                continue;
            }

            if self.stream_info.is_empty() {
                sleep(Duration::from_millis(250));
            }

            let utc = Utc::now();
            let minute = utc.minute() as u8;

            for i in 0..self.stream_info.len() {
                // TODO: Check if stream received 200 within a certain amount of time
                /*if self.stream_info[i].connected == false {
                    if self.stream_info[i].waited == 5 {
                        println!("Stream: {:?} connection timed out", self.stream_info[i]);
                        self.stream_info.remove(i);
                    }

                    if self.stream_info[i].last_minute != minute {
                        self.stream_info[i].last_minute = minute;
                        self.stream_info[i].waited += 1;
                    }
                } else {*/
                if !self.stream_info[i].received.is_empty() {
                    let stream_info = &self.stream_info[i];
                    self.write_to_stream_file(
                        self.stream_info[i].rid,
                        self.stream_info[i].port,
                        &stream_info.received[0].as_string(),
                    );
                    self.stream_info[i].received.remove(0);
                }

                if !self.stream_info[i].pending.is_empty() {
                    if self.io_method.is_none() {
                        println!("io_method is not set");
                        return;
                    } else {
                        let io_method = self.io_method.as_mut().unwrap();
                        println!("writing to distributor");
                        io_method._write(format!(
                            "\\z {} \\q",
                            self.stream_info[i].pending.remove(0).replace("\0", "")
                        ));
                    }
                }
                //}
            }

            let io_method = self.io_method.as_mut().unwrap();
            let mut read = io_method._read();
            read.0 = read.0.replace("\0", "");

            if read.1 == READ_SUCCESS {
                let message = Message::from_string(&read.0);

                if message.ri.rid == local_user_id().unwrap() {
                    for i in 0..self.stream_info.len() {
                        if self.stream_info[i].port == message.ri.port {
                            let mut waiting = true;
                            let mut waited = 0;

                            while waiting {
                                let read = self.io_method.as_mut().unwrap()._read();

                                if read.0.contains("SSS") {
                                    waiting = false;
                                } else if waited == 400 {
                                    break;
                                } else {
                                    waited += 1;
                                    sleep(Duration::from_millis(1));
                                }
                            }

                            if waiting == true {
                                break;
                            }

                            //self.stream_info[i].received.push(Cell::new(message));
                            if message.ti.code == 200 {
                                if self.stream_info[i].connected == false {
                                    self.stream_info[i].connected = true;
                                }
                            }

                            if Path::new(&format!(
                                "/tmp/darklight/connections/_dl_{}-{}",
                                local_user_id().unwrap(),
                                *&message.ri.port
                            ))
                            .exists()
                            {
                                self.write_to_stream_file(
                                    *&message.ri.rid,
                                    *&message.ri.port,
                                    &message.as_string(),
                                );
                            } else {
                                self.write_to_stream_file(
                                    *&message.ti.tid,
                                    *&message.ri.port,
                                    &message.as_string(),
                                );
                            }
                            break;
                        }
                    }
                }
            }
        }
    }
}

pub(crate) static mut STREAMS_HANDLER: StreamsHandler = StreamsHandler {
    config: DLConfig::empty(),
    stream_info: vec![],
    io_method: None,
    tcp: false,
    serial: false,
    dist_init: false,
    dist_conn: false,
    wait: 0,
    updating: false,
};

pub fn handle_streams() {
    unsafe {
        STREAMS_HANDLER.run();
    }
}
