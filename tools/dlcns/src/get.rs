use std::{thread::sleep, time::Duration};

use dlwp::{
    cerpton::{libcerpton_decode, libcerpton_encode},
    codes::{
        FILE_RESPONSE, REGULAR_RESPONSE, REQUEST_CHUNK, REQUEST_CONNECTION, REQUEST_FILE,
        REQUEST_RESPONSE,
    },
    encryption::EncryptionInfo,
    id::{DId, LId, Port},
    message::contents_to_string,
    stream::Stream,
};

use crate::{name::Owner, CNS_DISTRIBUTOR, CNS_ID, CNS_PORT, OWNERS_LIST};

/// Getting the location (owner) of a name
pub fn format_name_request(name: &String) -> String {
    return format!("GET_ID {}", name);
}

/// Getting all names by location (owner)
pub fn format_id_request(did: DId, id: LId) -> String {
    return format!("GET_ALL_NAMES {} {}", did, id);
}

/// Getting a single name from a location
pub fn format_specific_id_request(did: DId, id: LId, port: Port) -> String {
    return format!("GET_NAME {} {} {}", did, id, port);
}

pub struct CNSGet {
    stream: Stream,
    pub received: Vec<Owner>,
    pub timeout: Duration,
}

impl CNSGet {
    pub fn new() -> Self {
        let mut stream = Stream::new(
            dlwp::stream::StreamType::Client {
                rid: CNS_ID,
                rdid: CNS_DISTRIBUTOR,
                port: CNS_PORT,
            },
            false,
        );
        stream.add_encryption_info(EncryptionInfo {
            info: [2, 1, 2, 0, 0, 0],
            encode_function: libcerpton_encode,
            decode_function: libcerpton_decode,
        });
        stream.start();

        return CNSGet {
            stream,
            received: vec![],
            timeout: Duration::from_millis(5000),
        };
    }

    fn write_read(&mut self, write: String) -> String {
        let mut waited = Duration::from_millis(0);
        let mut read = vec![];
        while read.is_empty() {
            if waited >= self.timeout {
                break;
            }

            self.stream.write(write.clone(), REQUEST_RESPONSE);
            read = self.stream.read();

            if read.is_empty() == false {
                if contents_to_string(read[0].contents)
                    .replace("\0", "")
                    .is_empty()
                    || read[0].ti.code == REQUEST_CONNECTION.value()
                {
                    read.clear();
                } else {
                    break;
                }
            }

            let delay = Duration::from_millis(self.timeout.as_millis() as u64 / 10);
            sleep(delay);
            waited += delay;
        }

        if read.is_empty() {
            return String::new();
        }

        contents_to_string(read[0].contents).replace("\0", "")
    }

    /// Disconnects CNS, call after being used
    pub fn disconnect(&mut self) {
        self.stream.stop();
    }

    /// Returns the Id of a name
    pub fn get_id(&mut self, name: String) -> Option<Owner> {
        if self.stream.running() == false {
            return None;
        }
        sleep(Duration::from_millis(100));

        let mut owner = Owner {
            id: 0,
            did: 0,
            port: 0,
            name: String::new(),
            name_type: 0,
        };

        let response = self.write_read(format_name_request(&name));
        if response.is_empty() {
            return None;
        }

        let split_response = response.split(" ").collect::<Vec<&str>>();

        if split_response.len() < 5 {
            return None;
        }

        let id = split_response[0].parse::<u64>();
        let did = split_response[1].parse::<u32>();
        let port = split_response[2].parse::<u16>();
        let name = split_response[3];
        let name_type = split_response[4].parse::<usize>();

        if id.is_err() || did.is_err() || port.is_err() || name_type.is_err() || name.is_empty() {
            return None;
        }

        owner.id = id.unwrap();
        owner.did = did.unwrap();
        owner.port = port.unwrap();
        owner.name = name.to_string();

        return Some(owner);
    }

    /// Return a list of all owners and their names
    pub fn get_all(&mut self) -> String {
        unimplemented!();
    }

    /// Returns the name of an onwer
    pub fn get_name(&mut self, id: LId, did: DId, port: Port) -> Option<Owner> {
        if self.stream.running() == false {
            return None;
        }
        sleep(Duration::from_millis(100));

        let mut owner = Owner {
            id: 0,
            did: 0,
            port: 0,
            name: String::new(),
            name_type: 0,
        };

        let response = self.write_read(format_specific_id_request(did, id, port));

        if response.is_empty() {
            return None;
        }

        let split = response.split(" ").collect::<Vec<&str>>();

        if split.len() < 2 {
            return None;
        }

        let name_type = split[1].parse::<usize>();

        if name_type.is_err() {
            return None;
        }

        owner.id = id;
        owner.did = did;
        owner.port = port;
        owner.name = split[0].to_string();
        owner.name_type = name_type.unwrap();

        return Some(owner);
    }
}
