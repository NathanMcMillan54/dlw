use std::{borrow::{Borrow, BorrowMut}, collections::{hash_map::{IntoIter, IntoKeys}, HashMap}, io::Write, net::TcpStream};

use dlwp::{
    id::{DId, LId},
    io::DLSerialIO,
};

pub struct LocalConnections {
    pub tcp_connections: HashMap<LId, TcpStream>,
    pub serial_connections: HashMap<LId, DLSerialIO>,
}

impl LocalConnections {
    pub fn empty() -> Self {
        return LocalConnections {
            tcp_connections: HashMap::new(),
            serial_connections: HashMap::new(),
        };
    }

    // If other forms of communication are directly supported then this will need to return something else to indicate
    // how a user is connected
    pub fn connection_is_tcp(&self, id: &LId) -> bool {
        self.tcp_connections.contains_key(id)
    }

    pub fn connection_exists(&self, id: &LId) -> bool {
        return if self.tcp_connections.contains_key(id) || self.serial_connections.contains_key(id)
        {
            true
        } else {
            false
        };
    }

    pub fn add_tcp_connection(&mut self, id: LId, stream: TcpStream) -> bool {
        return if self.connection_exists(&id) {
            false
        } else {
            self.tcp_connections.insert(id, stream);
            true
        };
    }

    pub fn add_serial_connection(&mut self, id: LId, serial: DLSerialIO) -> bool {
        return if self.connection_exists(&id) {
            false
        } else {
            self.serial_connections.insert(id, serial);
            true
        };
    }

    pub fn remove_connection(&mut self, id: &LId) -> bool {
        return if self.serial_connections.contains_key(&id) {
            self.serial_connections.remove(id);
            true
        } else if self.tcp_connections.contains_key(&id) {
            self.tcp_connections.remove(id);
            true
        } else {
            false
        };
    }
}
