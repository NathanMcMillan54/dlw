use std::collections::HashMap;

use crate::{message::ReceiveInfo, stream::Stream};

/// This uses the ``ReceiveInfo`` struct to keep track of connections because it contains less information than
/// ``TransmitInfo``, these new values can be created by using the ``TransmitInfo.into_ri()`` method.
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

    /// Checks from ``self.not_allowed`` if a conenction is allowed
    pub fn is_allowed(&self, ri: ReceiveInfo) -> bool {
        return if self.not_allowed.contains(&ri) {
            false
        } else {
            true
        };
    }

    /// Add an ID to ``self.not_allowed``
    pub fn add_not_allowed(&mut self, ri: ReceiveInfo) {
        self.not_allowed.push(ri);
    }
}
