use dlwp::{codes::{FILE_RESPONSE, REQUEST_CHUNK, REQUEST_FILE, REQUEST_RESPONSE}, id::{DId, LId, Port}, message::contents_to_string, stream::Stream};

use crate::{owner::Owner, DNS_DISTRIBUTOR, DNS_ID, DNS_PORT, OWNERS_LIST};

/// Getting the location of a name
pub fn format_name_request(name: String) -> String {
    return format!("GET_ID {}", name);
}

/// Getting all names by location
pub fn format_id_request(did: DId, id: LId) -> String {
    return format!("GET_ALL_NAMES {} {}", did, id);
}

/// Getting a single name from a location
pub fn format_specific_id_request(did: DId, id: LId, port: Port) -> String {
    return format!("GET_NAME {} {} {}", did, id, port);
}

pub struct DNSGet {
    stream: Stream,
    pub received: Vec<Owner>,
}

impl DNSGet {
    pub fn new() -> Self {
        let mut stream = Stream::new(
            dlwp::stream::StreamType::Client {
                rid: DNS_ID,
                rdid: DNS_DISTRIBUTOR,
                port: DNS_PORT,
            },
            false,
        );
        stream.start();

        return DNSGet {
            stream,
            received: vec![],
        };
    }

    pub fn get_owner_name(&mut self, name: String) -> Option<Owner> {
        if self.stream.running() == false {
            return None;
        }

        self.stream.write(String::from(format_name_request(name)), REQUEST_RESPONSE);

        let mut read = vec![];
        while read.is_empty() {
            read = self.stream.read();
        }

        let mut owner = Owner {
            id: 0,
            did: 0,
            port: 0,
            name: String::new(),
            name_type: 0,
        };

        let response = contents_to_string(read[0].contents);
        let split_response = response.split(" ").collect::<Vec<&str>>();
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

    pub fn get_all(&mut self) -> String {
        self.stream.write(String::from(OWNERS_LIST), REQUEST_FILE);

        let mut read = vec![];
        while read.is_empty() {
            read = self.stream.read();
        }

        let message = read[0];
        let contents = contents_to_string(message.contents);

        if message.ti.code == FILE_RESPONSE.value() && contents.parse::<u64>().is_ok() {
            let mut ret = String::new();
            for i in 0..contents.parse::<u64>().unwrap() {
                self.stream.write(format!("{}", i), REQUEST_CHUNK);

                let mut chunk_read = vec![];
                while chunk_read.is_empty() {
                    chunk_read = self.stream.read();
                }

                ret.push_str(&contents_to_string(chunk_read[0].contents));
            }

            return ret;
        } else if message.ti.code == FILE_RESPONSE.value() {
            return contents;
        } else {
            return String::new()
        }
    }
}
