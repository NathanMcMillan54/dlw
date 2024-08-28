use dlwp::message::{MSG_END, MSG_INIT};

use crate::DISTRIBUTOR_ID;

pub fn check_input(input: Vec<u8>) -> String {
    let _read = String::from_utf8(input);
    if _read.is_err() {
        return String::from("INVALID INPUT");
    }

    let trimmed = _read.clone().unwrap().replace(MSG_INIT, "").replace(MSG_END, "");
    let read = trimmed.split(" ").collect::<Vec<&str>>();

    return match read[0] {
        "USR-INIT" => {
            if read.len() == 7 {
                if read[2].parse::<u64>().is_err() || read[3].parse::<u8>().is_err() || read[4].parse::<u8>().is_err() || read[5].parse::<u8>().is_err() || read[6].parse::<u8>().is_err() {
                    String::from("Invalid user values")
                } else {
                    return _read.unwrap();
                }
            } else {
                String::from("Invalid user values")
            }
        },
        "INIT-DIS" => {
            String::new()
        }
        "GET-DIS" => String::from(DISTRIBUTOR_ID),
        _ => String::from("Unknown input")
    };
}
