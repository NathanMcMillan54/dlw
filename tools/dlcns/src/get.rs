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
        };
    }

    /// Returns the owner of a name
    pub fn get_owner_name(&mut self, name: String) -> Option<Owner> {
        if self.stream.running() == false {
            return None;
        }

        let mut owner = Owner {
            id: 0,
            did: 0,
            port: 0,
            name: String::new(),
            name_type: 0,
        };
        let mut read = vec![];
        while read.is_empty() {
            self.stream
                .write(String::from(format_name_request(&name)), REQUEST_RESPONSE);
            read = self.stream.read();

            if read.is_empty() == false {
                if contents_to_string(read[0].contents)
                    .replace("\0", "")
                    .is_empty()
                {
                    read.clear();
                }
            }
        }

        let response = contents_to_string(read[0].contents).replace("\0", "");
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
    pub fn get_id(&mut self, id: LId, did: DId, port: Port) -> String {
        unimplemented!()
    }
}
