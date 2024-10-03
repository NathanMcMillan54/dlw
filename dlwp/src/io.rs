use crate::codes::{
    Code, LENGTH_EXCEEDED, READ_FAILED, READ_SUCCESS, READ_TIMEDOUT, STATUS_OK, UNKNOWN_STATUS,
    WRITE_FAILED, WRITE_SUCCESS,
};
use crate::distributor::{GET_DISTRIBUTOR, READ_AVAILABLE, USER_INIT};
use crate::id::local_user_id;
use crate::message::{MSG_END, MSG_INIT, SN_MSG_INIT};
use cerpton::utf::string_to_utf8;
use core::str::FromStr;
use serialport::posix::TTYPort;
use serialport::{DataBits, FlowControl, Parity, SerialPortSettings, StopBits};
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

pub trait DLIO {
    fn _read(&mut self) -> (String, Code) {
        (String::new(), READ_FAILED)
    }

    #[allow(unused_variables)]
    fn _write(&mut self, write: String) -> Code {
        WRITE_FAILED
    }
}

/// This was written before ``DLTCPIO``, it does not work anymore
pub struct DLSerialIO {
    pub port: Option<TTYPort>,
    pub file: &'static str,
    pub info: (u32, u32),
}

impl DLSerialIO {
    pub fn default_settings() -> SerialPortSettings {
        SerialPortSettings {
            baud_rate: 9600,
            data_bits: DataBits::Eight,
            flow_control: FlowControl::None,
            parity: Parity::None,
            stop_bits: StopBits::One,
            timeout: Duration::from_millis(3411),
        }
    }

    pub fn new(name: &'static str) -> Self {
        return DLSerialIO {
            port: Some(TTYPort::open(Path::new(name), &DLSerialIO::default_settings()).unwrap()),
            file: name,
            info: (0, 0),
        };
    }

    pub fn init(&mut self, user: bool) -> Code {
        if user {
            let id = local_user_id();
            if id.is_none() {
                return UNKNOWN_STATUS;
            }
            {
                let write_ret = self._write(format!(
                    "{} {} {} {}",
                    SN_MSG_INIT,
                    USER_INIT,
                    id.unwrap(),
                    MSG_END
                ));
                if write_ret != WRITE_SUCCESS {
                    return write_ret;
                }
            }

            sleep(Duration::from_millis(1500));

            {
                let write_ret =
                    self._write(format!("{} {} {}", SN_MSG_INIT, GET_DISTRIBUTOR, MSG_END));
                if write_ret != WRITE_SUCCESS {
                    return write_ret;
                }
            }

            sleep(Duration::from_millis(1500));

            let read = self._read();

            if read.1 != READ_SUCCESS {
                return read.1;
            } else {
                self.info.0 = read
                    .0
                    .replace(SN_MSG_INIT, "")
                    .replace(MSG_END, "")
                    .replace(READ_AVAILABLE, "")
                    .replace(" ", "")
                    .parse::<u32>()
                    .unwrap();
                return STATUS_OK;
            }
        } else {
            STATUS_OK
        }
    }
}

impl DLIO for DLSerialIO {
    fn _read(&mut self) -> (String, Code) {
        let mut read = String::new();
        let port = self.port.as_mut().unwrap();

        loop {
            let mut read_chunk = vec![0; 64];
            let read_ret = port.read(read_chunk.as_mut_slice());

            if read_ret.is_err() {
                return (String::new(), READ_TIMEDOUT);
            } else {
                read.push_str(&String::from_utf8(read_chunk).unwrap());
                read = read.replace("\0", "");
            }

            if read.contains("\\q") {
                break;
            }

            sleep(Duration::from_millis(64));
        }

        read = read.replace(SN_MSG_INIT, "").replace(MSG_END, "");

        return (read, READ_SUCCESS);
    }

    fn _write(&mut self, write: String) -> Code {
        let read = self._read();
        let port = self.port.as_mut().unwrap();

        if read.1 != READ_SUCCESS {
            return read.1;
        }

        if read.0.contains(READ_AVAILABLE) {
            let bytes = string_to_utf8(write);

            let mut current_write = vec![];

            if bytes.len() >= 4096 {
                return LENGTH_EXCEEDED;
            }

            for i in 0..bytes.len() {
                current_write.push(bytes[i]);

                if i % 64 == 0 {
                    let write_ret = port.write_all(bytes.as_slice());

                    if write_ret.is_err() {
                        return WRITE_FAILED;
                    }

                    let flush_ret = port.flush();

                    if flush_ret.is_err() {
                        return WRITE_FAILED;
                    }

                    current_write.clear();
                }

                sleep(Duration::from_micros(100));
            }

            if !current_write.is_empty() {
                port.write_all(current_write.as_slice()).unwrap();
            }

            return WRITE_SUCCESS;
        } else {
            return UNKNOWN_STATUS;
        }
    }
}

pub struct DLTCPIO {
    pub connections: Vec<SocketAddr>,
    pub bind: String,
    pub stream: Option<TcpStream>,
    pub listener: Option<TcpListener>,
    pub listener_fn: Option<fn(&mut TcpStream)>,
    pub read: Vec<String>,
    pub check_ready: bool,
}

impl DLTCPIO {
    pub fn new(bind: String) -> Self {
        return DLTCPIO {
            connections: vec![],
            bind: bind,
            stream: None,
            listener: None,
            listener_fn: None,
            read: vec![],
            check_ready: true,
        };
    }

    pub fn init(&mut self) {
        self.connections
            .push(SocketAddr::from_str(&self.bind).unwrap());
        let stream = TcpStream::connect(self.connections[0]).expect("Failed to connect");
        stream
            .set_read_timeout(Some(Duration::from_secs(1)))
            .unwrap();
        stream
            .set_write_timeout(Some(Duration::from_secs(1)))
            .unwrap();
        self.stream = Some(stream);
    }
}

impl DLIO for DLTCPIO {
    fn _read(&mut self) -> (String, Code) {
        let stream = self.stream.as_mut().unwrap();

        let mut read = [0; 4096];

        let read_ret = stream.read(&mut read);

        if read_ret.is_err() || read == [0; 4096] {
            return (String::new(), READ_FAILED);
        }

        let mut read = String::from_utf8(read.to_vec()).unwrap();
        read = read.replace(MSG_INIT, "").replace(MSG_END, "");

        return (read, READ_SUCCESS);
    }

    fn _write(&mut self, write: String) -> Code {
        if self.check_ready {
            let read = self._read();

            if read.1 != READ_SUCCESS {
                return read.1;
            }

            if read.0.contains(READ_AVAILABLE) {
                let stream = self.stream.as_mut().unwrap();
                let ret = stream.write_all(write.as_bytes());
                stream.flush().expect("Failed to flush write");
                return if ret.is_ok() {
                    WRITE_SUCCESS
                } else {
                    WRITE_FAILED
                };
            }
        } else {
            let stream = self.stream.as_mut().unwrap();
            let ret = stream.write_all(write.as_bytes());
            stream.flush().expect("Failed to flush write");

            return if ret.is_err() {
                WRITE_FAILED
            } else {
                WRITE_SUCCESS
            };
        }

        WRITE_FAILED
    }
}
