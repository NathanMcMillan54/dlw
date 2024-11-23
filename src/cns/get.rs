use std::{thread::sleep, time::Duration};

use dlwp::{codes::{FILE_RESPONSE, REGULAR_RESPONSE}, message::{TransmitInfo, MAX_CONTENTS_LENGTH}, stream::Stream};

use crate::NAMES_LIST;

pub fn get_cns_all_info(inputs: Vec<&str>, mut stream: &mut Stream, ti: TransmitInfo) {
    let location = if inputs[0] == "GET_ALL_NAMES_ID" {
        let did = inputs[1].parse::<u32>();
        let id = inputs[2].parse::<u64>();

        if did.is_err() || id.is_err() {
            return;
        }

        vec![did.unwrap() as u64, id.unwrap()]
    } else {
        vec![]
    };

    let mut ret = String::new();
    unsafe {
        for i in 0..NAMES_LIST.list.len() {
            match inputs[0] {
                "GET_ALL_NAMES" => {
                    ret.push_str(&format!("{} {} {} {} {},", NAMES_LIST.list[i].owner.id, NAMES_LIST.list[i].owner.did, NAMES_LIST.list[i].owner.port, NAMES_LIST.list[i].owner.name, NAMES_LIST.list[i].owner.name_type));
                }
                "GET_ALL_NAMES_ID" => {
                    if NAMES_LIST.list[i].owner.did == location[0] as u32 && NAMES_LIST.list[i].owner.id == location[1] {
                        ret.push_str(&format!("{} {} {} {} {},", NAMES_LIST.list[i].owner.id, NAMES_LIST.list[i].owner.did, NAMES_LIST.list[i].owner.port, NAMES_LIST.list[i].owner.name, NAMES_LIST.list[i].owner.name_type));
                    }
                }
                _ => {
                    return;
                }
            }
        }
    }

    if ret.len() > MAX_CONTENTS_LENGTH - 2 {
        let chunks = ret.len() / (MAX_CONTENTS_LENGTH - 2);
        stream.server_write(ti, chunks.to_string(), FILE_RESPONSE);
        sleep(Duration::from_millis(100));

        // TODO: Send in chunks
    } else {
        stream.server_write(ti, ret, REGULAR_RESPONSE);
    }
}

pub fn get_cns_info(inputs: Vec<&str>) -> String {
    let location = if inputs[0] == "GET_NAME" {
        let did = inputs[1].parse::<u32>();
        let id = inputs[2].parse::<u64>();
        let port = inputs[3].parse::<u16>();

        if did.is_err() || id.is_err() || port.is_err() {
            return format!("Parsing error");
        }

        vec![did.unwrap() as u64, id.unwrap(), port.unwrap() as u64]
    } else {
        vec![]
    };

    let mut ret = String::new();
    unsafe {
        for i in 0..NAMES_LIST.list.len() {
            match inputs[0] {
                "GET_ID" => {
                    if &NAMES_LIST.list[i].owner.name == inputs[1] {
                        return format!(
                            "{} {} {} {} {}",
                            NAMES_LIST.list[i].owner.id,
                            NAMES_LIST.list[i].owner.did,
                            NAMES_LIST.list[i].owner.port,
                            NAMES_LIST.list[i].owner.name,
                            NAMES_LIST.list[i].owner.name_type
                        );
                    }
                }
                "GET_NAME" => {
                    if NAMES_LIST.list[i].owner.did == location[0] as u32
                        && NAMES_LIST.list[i].owner.id == location[1] as u64
                        && NAMES_LIST.list[i].owner.port == location[2] as u16
                    {
                        return format!(
                            "{} {}",
                            NAMES_LIST.list[i].owner.name, NAMES_LIST.list[i].owner.name_type
                        );
                    }
                }
                _ => {}
            }
        }
    }
    
    if ret.len() <= MAX_CONTENTS_LENGTH {
        let chunks = ret.len() / MAX_CONTENTS_LENGTH;
        // TODO: 
    } else {
        return ret;
    }

    String::new()
}
