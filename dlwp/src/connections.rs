use crate::message::ReceiveInfo;

pub struct Connections {
    /// Current client connections
    pub current: Vec<ReceiveInfo>,
    /// Connections that will not be proccessed
    pub not_allowed: Vec<ReceiveInfo>,
}

impl Connections {
    pub fn empty() -> Self {
        return Connections {
            current: vec![],
            not_allowed: vec![],
        };
    }
}
