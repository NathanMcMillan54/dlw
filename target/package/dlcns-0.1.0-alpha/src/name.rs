use std::fs::{read_to_string, File};
use std::io::Write;

use dlwp::id::{DId, LId, Port};
use dlwp::serde::{Deserialize, Serialize};
use dlwp::serde_json;

#[derive(Debug, Deserialize, Serialize)]
pub struct Owner {
    pub id: LId,
    pub did: DId,
    pub port: Port,
    pub name: String,
    pub name_type: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Name {
    pub owner: Owner,
    pub requests: usize,
    /// Date that the ``Name`` was added
    pub date: [i32; 3],
    pub current_dlu_key: String,
    pub og_dlu_key: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NamesList {
    pub list: Vec<Name>,
}

impl NamesList {
    pub const fn empty() -> Self {
        return NamesList { list: vec![] };
    }

    pub fn from_file(file: String) -> Option<Self> {
        let read = read_to_string(file);
        if read.is_err() {
            return None;
        }

        let list: Result<Self, serde_json::Error> = serde_json::from_str(&read.unwrap());
        if list.is_err() {
            return None;
        } else {
            return Some(list.unwrap());
        }
    }

    pub fn write_to_file(&self, file: String) {
        let mut file = File::options().write(true).open(file).unwrap();
        let list = serde_json::to_string_pretty(self).unwrap();
        file.write_fmt(format_args!("{}", list)).unwrap();
    }
}
