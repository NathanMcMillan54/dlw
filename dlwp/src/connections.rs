use std::collections::HashMap;

use crate::{message::ReceiveInfo, stream::Stream};

pub struct Connections {
    /// Current client connections
    pub current: HashMap<ReceiveInfo, Stream>,
    /// Connections that will not be proccessed
    pub not_allowed: Vec<ReceiveInfo>,
}

impl Connections {
    pub fn empty() -> Self {
        return Connections {
            current: HashMap::new(),
            not_allowed: vec![],
        };
    }
}
