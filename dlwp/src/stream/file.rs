use std::{fs::{read_to_string, remove_file, File}, io::Write, path::Path, thread::sleep, time::Duration};

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

    pub fn read_pending(&mut self) {
        self.wait_for_file("P");

        let mut try_pending_contents = read_to_string(&format!("{}P", self.path()));

        while try_pending_contents.is_err() {
            try_pending_contents = read_to_string(&format!("{}P", self.path()));
        }

        let parsed_pending: Vec<String> = serde_json::from_str(&try_pending_contents.unwrap()).unwrap();

        self.pending = parsed_pending.clone();
    }

    pub fn read_recieved(&mut self) {
        self.wait_for_file("R");

        let mut try_recieved_contents = read_to_string(&format!("{}R", self.path()));

        while try_recieved_contents.is_err() {
            try_recieved_contents = read_to_string(&format!("{}R", self.path()));
        }

        let parsed_recieved: Vec<ReceivedMessage> = serde_json::from_str(&try_recieved_contents.unwrap()).unwrap();

        self.received = parsed_recieved.clone();
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
