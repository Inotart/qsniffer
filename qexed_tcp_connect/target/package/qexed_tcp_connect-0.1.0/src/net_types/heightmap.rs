use crate::{
    net_types::{subdata::Subdata, var_int::VarInt},
    packet::{decode::PacketReader, encode::PacketWriter},
};
use serde::{Deserialize, Serialize};
#[derive(Debug, Default, PartialEq, Serialize, Deserialize,Clone)]
pub struct Heightmaps {
    pub type_id: VarInt,
    pub data: Vec<u64>,
}
impl Subdata for Heightmaps {
    fn new() -> Self {
        Heightmaps {
            type_id: VarInt(0),
            data: vec![],
        }
    }
    fn serialize(&self, w: &mut PacketWriter) {
        w.serialize(&self.type_id);
        w.serialize(&self.data);
    }

    fn deserialize(&mut self, r: &mut PacketReader) {
        self.type_id = r.deserialize();
        self.data = r.deserialize();
    }
}
