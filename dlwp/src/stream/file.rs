use std::{fs::{read_to_string, remove_file, File}, io::Write, path::Path};

use crate::{id::{DId, LId, Port}, message::Message};

#[derive(Deserialize, Serialize, Default, Clone)]
pub struct ReceivedMessage {
    pub recv_time: [u8; 3],
    pub message: String,
}

/// Used by ``Stream``
pub struct ReadMessage {
    pub recv_time: [u8; 3],
    pub message: Message,
}

#[derive(Deserialize, Serialize, Default, Clone)]
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
        let file = StreamFile {
            id,
            port,
            did,
            encryption_info: info,
            received: vec![],
            pending: vec![],
        };

        file.create();
        return file;
    }

    pub fn exists(&self) -> bool {
        return Path::exists(Path::new(&format!("/tmp/darklight/connections/_dl-{}-{}", self.id, self.port)));
    }

    pub fn create(&self) {
        File::options().read(true).write(true).create(true).open(&format!("/tmp/darklight/connections/_dl-{}-{}", self.id, self.port)).expect("Failed to create file").write_fmt(format_args!("{}", serde_json::to_string(&self).expect("Failed to parse"))).expect("Failed to write to stream file");
    }

    pub fn remove(&self) {
        remove_file(&format!("/tmp/darklight/connections/_dl-{}-{}", self.id, self.port)).expect("Failed to remove stream file");
    }

    pub fn read(&self) -> String {
        let ret = read_to_string(&format!("/tmp/darklight/connections/_dl-{}-{}", self.id, self.port)).expect("Failed to read stream file");
        ret
    }

    pub fn read_and_parse(&self) -> StreamFile {
        let current_contents = self.read();
        let contents_json: StreamFile = serde_json::from_str(&current_contents).expect("Failed to parse stream file");

        return contents_json;
    }

    pub fn read_and_set(&mut self) {
        let contents_json = self.read_and_parse();

        self.received = contents_json.received;
        self.pending = contents_json.pending;
    }

    pub fn write_pending(&self) {
        let mut current_json = self.read_and_parse();
        self.remove();
        self.create();
        current_json.pending = self.pending.clone();

        let mut file = File::options().write(true).open(&format!("/tmp/darklight/connections/_dl-{}-{}", self.id, self.port)).unwrap();

        file.write_fmt(format_args!("{}", serde_json::to_string_pretty(&current_json).unwrap())).unwrap();
        file.flush().unwrap();
    }

    pub fn write_received(&self) {
        let mut current_json = self.read_and_parse();
        self.remove();
        self.create();
        current_json.received = self.received.clone();

        let mut file = File::options().write(true).open(&format!("/tmp/darklight/connections/_dl-{}-{}", self.id, self.port)).unwrap();

        file.write_fmt(format_args!("{}", serde_json::to_string_pretty(&current_json).unwrap())).unwrap();
        file.flush().unwrap();
    }

    pub fn write(&self) {
        self.write_pending();
        self.write_received();
    }
}
