use crate::{
    encryption::EncryptionInfo,
    id::{DId, InstanceID, LId, Port},
};
use cerpton::utf::utf8_to_string;

/// Divides the ``Message`` when it is represented as a ``String``
pub const INFO_SPLIT: &str = "\\|\\";
const INFO_SPLIT_AREA: &str = " \\|\\ ";
pub const CONTENTS_LENGTH: usize = 4096;

/// The start of a message being sent
pub const MSG_INIT: &str = "\\z ";
/// The start of a message being sent on a serial device
pub const SN_MSG_INIT: &str = "\\\\z ";
/// The end of a message being sent
pub const MSG_END: &str = " \\q";

/// "contents" type, 4096 byte array
pub type Contents = [u8; CONTENTS_LENGTH];

/// If the original ``String`` was less than 4096 bytes there will be many null characters (``\0``) at the end
pub fn contents_to_string(contents: Contents) -> String {
    utf8_to_string(contents.to_vec())
}

/// Converts a ``String`` to ``Contents``, will panic if the input ``String`` is larger than 4096 bytes
pub fn string_to_contents(string: String) -> Contents {
    let bytes: &[u8] = string.as_bytes().try_into().unwrap();
    let mut ret = [0; 4096];

    for i in 0..4096 {
        if i >= bytes.len() {
            break;
        }

        ret[i] = bytes[i];
    }

    return ret;
}

/// Splits a ``String`` using ``INFO_SPLIT`` into an ``&str`` array. Has ``inline`` incase it is called regularaly.
#[inline]
pub fn split_from_info(string: &String) -> Vec<&str> {
    string.split(INFO_SPLIT_AREA).collect::<Vec<&str>>()
}

/// Checks if a ``Message`` represented as a ``String`` is valid. This starts by checking if it's "info splits" are in
/// the right places. If it's ``ReceiveInfo`` and ``TransmitInfo`` does not contain empty values.
#[inline]
pub fn valid_message_string(string: &String, encrypted: bool) -> bool {
    let split = split_from_info(&string);

    if split.len() != 3 {
        return false;
    }

    let receive_info = split[0].split(" ").collect::<Vec<&str>>();
    if receive_info.len() != 4 {
        return false;
    }

    for i in 0..receive_info.len() {
        // If it can't be parsed to an integer, then it is not valid
        let parse = receive_info[i].parse::<u64>();
        if parse.is_err() {
            return false;
        } else {
            if i == 0 || i == 1 {
                // If ``rid`` and ``rdid`` are ``0`` it will be assumed that the ``Message`` will be empty
                if parse.unwrap() == 0 {
                    return false;
                }
            }
        }
    }

    // The rest cannot be checked if ``encrypted`` is ture
    if encrypted {
        return true;
    }

    let transmit_info = split[1].split(" ").collect::<Vec<&str>>();
    if transmit_info.len() != 6 {
        return false;
    }

    for i in 0..transmit_info.len() {
        // If it can't be parsed to an integer, then it is not valid
        let parse = transmit_info[i].parse::<u64>();
        if parse.is_err() {
            return false;
        } else {
            if i == 0 || i == 2 {
                // If ``tid`` and ``tdid`` are ``0`` it will be assumed that the ``Message`` will be empty
                if parse.unwrap() == 0 {
                    return false;
                }
            }
        }
    }

    true
}

/// Struct containing information about the receiver
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ReceiveInfo {
    pub rid: LId,
    pub rdid: DId,
    pub instance_id: DId,
    pub port: u16,
}

impl ReceiveInfo {
    pub fn empty() -> Self {
        return ReceiveInfo {
            rid: 0,
            rdid: 0,
            instance_id: 0,
            port: 0,
        };
    }

    pub fn get_from_message_string(message: String) -> Self {
        let msg_split = split_from_info(&message);

        if msg_split.len() != 3 && msg_split.len() != 2 {
            return ReceiveInfo::empty();
        }

        let info_split = msg_split[0].split(' ').collect::<Vec<&str>>();

        if info_split.len() != 4 {
            return ReceiveInfo::empty();
        }

        let rid = info_split[0].parse::<u64>();
        let rdid = info_split[1].parse::<u32>();
        let instance = info_split[2].parse::<u32>();
        let port = info_split[3].parse::<u16>();

        if rid.is_err() || rdid.is_err() || instance.is_err() || port.is_err() {
            return ReceiveInfo::empty();
        }

        return ReceiveInfo {
            rid: rid.unwrap(),
            rdid: rdid.unwrap(),
            instance_id: instance.unwrap(),
            port: port.unwrap(),
        };
    }
}

/// Struct containing information about the transmitter
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TransmitInfo {
    pub tid: LId,
    pub tdid: DId,
    pub code: u16,
}

impl TransmitInfo {
    pub fn empty() -> Self {
        return TransmitInfo {
            tid: 0,
            tdid: 0,
            code: 0,
        };
    }

    pub fn into_ri(&self, instance: InstanceID, port: Port) -> ReceiveInfo {
        ReceiveInfo {
            rid: self.tid,
            rdid: self.tdid,
            instance_id: instance,
            port,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Message {
    /// The receiving information
    pub ri: ReceiveInfo,
    /// The transmitting information
    pub ti: TransmitInfo,
    /// The contents of the message >~4096 bytes
    pub contents: [u8; 4096],
    /// ``day``
    pub day: i32,
    /// ``week``
    pub week: i32,
    /// ``month``
    pub month: i32,
}

impl Message {
    pub fn empty() -> Self {
        return Message {
            ri: ReceiveInfo::empty(),
            ti: TransmitInfo::empty(),
            contents: [0; 4096],
            day: -1,
            week: -1,
            month: -1,
        };
    }

    /// Converts the ``Message`` to a ``String``. This returns a ``String`` following this format:
    /// When a ``Message`` is encrypted, the transmit information and contents should be encrypted
    pub fn as_string(&self) -> String {
        format!(
            "{} {} {} {} {} {} {} {} {} {} {} {} {}",
            self.ri.rid,
            self.ri.rdid,
            self.ri.instance_id,
            self.ri.port,
            INFO_SPLIT,
            self.ti.tid,
            self.ti.code,
            self.ti.tdid,
            self.day,
            self.week,
            self.month,
            INFO_SPLIT,
            contents_to_string(self.contents)
        )
    }

    pub fn get_ri_from_encoded(string: &String) -> ReceiveInfo {
        let split_message = split_from_info(string);
        if split_message.len() != 3 {
            return ReceiveInfo::empty();
        }

        let split_ri = split_message[0].split(" ").collect::<Vec<&str>>();

        if split_ri.len() != 4 {
            return ReceiveInfo::empty();
        }

        ReceiveInfo {
            rid: split_ri[0].parse().unwrap_or(0),
            rdid: split_ri[1].parse().unwrap_or(0),
            instance_id: split_ri[2].parse().unwrap_or(0),
            port: split_ri[3].parse().unwrap_or(0),
        }
    }

    #[inline]
    pub fn ri_as_string(&self) -> String {
        format!(
            "{} {} {} {}",
            self.ri.rid, self.ri.rdid, self.ri.instance_id, self.ri.port
        )
    }

    #[inline]
    pub fn ti_as_string(&self) -> String {
        format!(
            "{} {} {} {} {} {}",
            self.ti.tid, self.ti.code, self.ti.tdid, self.day, self.week, self.month
        )
    }

    pub fn encode(&self, encryption: EncryptionInfo) -> String {
        let mut ret = String::new();

        let ri = self.ri_as_string();
        let ti = self.ti_as_string();
        let contents = contents_to_string(self.contents);

        ret.push_str(&ri);
        ret.push_str(INFO_SPLIT_AREA);
        // Find better way of doing this
        ret.push_str(&(encryption.encode_function)(encryption.info, ti));
        ret.push_str(INFO_SPLIT_AREA);
        // Find better way of doing this
        ret.push_str(&(encryption.encode_function)(encryption.info, contents));

        ret
    }

    pub fn decode(string: &String, encryption: EncryptionInfo) -> Self {
        let split = split_from_info(string);

        // This is always plain text
        let ri = split[0];
        let ti = (encryption.decode_function)(encryption.info, split[1].to_string());
        let contents = (encryption.decode_function)(encryption.info, split[2].to_string());

        let ret = Message::from_string(&format!(
            "{}{}{}{}{}",
            ri, INFO_SPLIT_AREA, ti, INFO_SPLIT_AREA, contents
        ));

        ret
    }

    /// Converts an unencrypted ``String`` formatted for ``Message``s to a new ``Message``.
    pub fn from_string(string: &String) -> Self {
        let valid = valid_message_string(string, false);

        if valid == false {
            return Message::empty();
        }

        let mut ret_message = Message::empty();
        let split = split_from_info(string);
        let ri = split[0].split(" ").collect::<Vec<&str>>();
        let ti = split[1].split(" ").collect::<Vec<&str>>();
        let contents = split[2];

        for i in 0..ri.len() {
            let parse = ri[i].parse::<u64>();
            if parse.is_err() {
                return Message::empty();
            }

            match i {
                0 => ret_message.ri.rid = parse.unwrap(),
                1 => ret_message.ri.rdid = parse.unwrap() as u32,
                2 => ret_message.ri.instance_id = parse.unwrap() as u32,
                3 => ret_message.ri.port = parse.unwrap() as u16,
                _ => break,
            };
        }

        for i in 0..ti.len() {
            let parse = ti[i].parse::<u64>();
            if parse.is_err() {
                return Message::empty();
            }

            match i {
                0 => ret_message.ti.tid = parse.unwrap(),
                1 => ret_message.ti.code = parse.unwrap() as u16,
                2 => ret_message.ti.tdid = parse.unwrap() as u32,
                3 => ret_message.day = parse.unwrap() as i32,
                4 => ret_message.week = parse.unwrap() as i32,
                5 => ret_message.month = parse.unwrap() as i32,
                _ => break,
            };
        }

        ret_message.contents = string_to_contents(contents.to_owned());

        ret_message
    }
}
