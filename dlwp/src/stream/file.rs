use std::{fs::{read_to_string, remove_file, File}, io::Write, path::Path, thread::sleep, time::Duration};

use serde_json::Error;

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
        file.write();
        return file;
    }

    fn path(&self) -> String {
        format!("/tmp/darklight/connections/_dl-{}-{}", self.id, self.port)
    }

    pub fn exists(&self, pr: &str) -> bool {
        return Path::exists(Path::new(&format!("/tmp/darklight/connections/_dl-{}-{}{}", self.id, self.port, pr)));
    }

    pub fn wait_for_file(&self, pr: &str) {
        while self.exists(pr) == false {
            sleep(Duration::from_micros(100));
        }

        sleep(Duration::from_millis(150));
    }

    pub fn create(&self) {
        self.create_pending();
        self.create_recieved();
    }

    pub fn create_pending(&self) {
        File::options().read(true).write(true).create(true).open(&format!("{}P", self.path())).expect("Failed to create file");
    }

    pub fn create_recieved(&self) {
        File::options().read(true).write(true).create(true).open(&format!("{}R", self.path())).expect("Failed to create file");
    }

    pub fn remove_pending(&self) {
        remove_file(&format!("{}P", self.path())).expect("Failed to remove stream file");
    }

    pub fn remove_recieved(&self) {
        remove_file(&format!("{}R", self.path())).expect("Failed to remove stream file");
    }

    pub fn remove_all(&self) {
        self.remove_pending();
        self.remove_recieved();
    }

    fn read(&self, pr: &str) -> String {
        self.wait_for_file(pr);

        let mut file_contents = read_to_string(format!("{}{}", self.path(), pr));

        while file_contents.is_err() {
            file_contents = read_to_string(format!("{}{}", self.path(), pr));
        }

        return file_contents.unwrap();
    }

    pub fn read_pending(&mut self) {
        let mut read = self.read("P");
        let mut parsed_pending: Result<Vec<String>, Error> = serde_json::from_str(&read);

        while read.is_empty() || parsed_pending.is_err() {
            read = self.read("P");
            parsed_pending = serde_json::from_str(&read);
        }

        self.pending = parsed_pending.unwrap();
    }

    pub fn read_recieved(&mut self) {
        let mut read = self.read("R");
        let mut parsed_received: Result<Vec<ReceivedMessage>, Error> = serde_json::from_str(&read);

        while read.is_empty() || parsed_received.is_err() {
            read = self.read("R");
            parsed_received = serde_json::from_str(&read);
        }

        self.received = parsed_received.unwrap();
    }

    pub fn write_pending(&self) {
        self.remove_pending();
        self.create_pending();
        self.wait_for_file("P");

        let mut file = File::options().write(true).open(&format!("{}P", self.path())).unwrap();

        file.write_fmt(format_args!("{}", serde_json::to_string(&self.pending).unwrap())).unwrap();
        file.flush().unwrap();
    }

    pub fn write_received(&self) {
        self.remove_recieved();
        self.create_recieved();
        self.wait_for_file("R");
        let mut file = File::options().write(true).open(&format!("{}R", self.path())).unwrap();

        file.write_fmt(format_args!("{}", serde_json::to_string(&self.received).unwrap())).unwrap();
        file.flush().unwrap();
    }

    pub fn write(&self) {
        self.write_pending();
        self.write_received();
    }
}
