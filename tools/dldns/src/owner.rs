use dlwp::id::{DId, LId, Port};

pub struct Owner {
    pub id: LId,
    pub did: DId,
    pub port: Port,
    pub name: String,
    pub name_type: usize,
}
