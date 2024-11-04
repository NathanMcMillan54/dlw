use std::{fs::{read_to_string, remove_file, File}, io::Write, path::Path};

use crate::id::{DId, LId, Port};

#[derive(Deserialize, Serialize, Default)]
pub struct ReceivedMessage {
    pub recv_time: [u8; 3],
    pub message: String,
}

#[derive(Deserialize, Serialize, Default)]
pub struct StreamFile {
    pub id: LId,
    pub port: Port,
    pub did: DId,
    pub encryption_info: [i32; 6],
    pub received: Vec<ReceivedMessage>,
    pub pending: Vec<String>,
}

impl StreamFile {
    pub fn new(id: LId, port: Port, did: DId, info: [i32; 6]) -> Self {
        return StreamFile {
            id,
            port,
            did,
            encryption_info: info,
            received: vec![],
            pending: vec![],
        };
    }

    pub fn exists(&self) -> bool {
        return Path::exists(Path::new(&format!("/tmp/darklight/connections/_dl-{}-{}", self.id, self.port)));
    }

    pub fn create(&self) {
        File::options().read(true).write(true).create(true).open(&format!("/tmp/darklight/connections/_dl-{}-{}", self.id, self.port)).expect("Failed to create file");
    }

    pub fn remove(&self) {
        remove_file(&format!("/tmp/darklight/connections/_dl-{}-{}", self.id, self.port)).expect("Failed to remove stream file");
    }

    pub fn read(&self) -> String {
        let ret = read_to_string(&format!("/tmp/darklight/connections/_dl-{}-{}", self.id, self.port)).expect("Failed to read stream file");
        ret
    }

    pub fn read_and_set(&mut self) {
        let current_contents = self.read();
        let contents_json: StreamFile = serde_json::from_str(&current_contents).expect("Failed to parse stream file");

        self.received = contents_json.received;
        self.pending = contents_json.pending;
    }

    pub fn write(&self) {
        let self_json = serde_json::to_string(self).unwrap();

        if self.exists() == false {
            self.create();
        }

        File::options().write(true).open(&format!("/tmp/darklight/connections/_dl-{}-{}", self.id, self.port)).expect("Failed to open stream file").write_fmt(format_args!("{}", self_json)).expect("Failed to write to stream file");
    }
}
