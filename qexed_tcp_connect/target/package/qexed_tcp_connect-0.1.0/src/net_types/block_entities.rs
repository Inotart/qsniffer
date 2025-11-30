use crate::{
    net_types::{subdata::Subdata, var_int::VarInt},
    packet::{decode::PacketReader, encode::PacketWriter},
};

#[derive(Debug, Default, PartialEq,Clone)]
pub struct BlockEntities {
    pub xz: u8,
    pub y: u16,
    pub entity_type: VarInt,
    pub nbt: Vec<crab_nbt::Nbt>,
}
impl Subdata for BlockEntities {
    fn new() -> Self {
        BlockEntities {
            xz:0,
            y:0,
            entity_type:VarInt(0),
            nbt:vec![],
        }
    }
    fn serialize(&self, w: &mut PacketWriter) {
        w.serialize(&self.xz);
        w.serialize(&self.y);
        w.serialize(&self.entity_type);
        w.serialize(&self.nbt);
    }

    fn deserialize(&mut self, r: &mut PacketReader) {
        self.xz = r.deserialize();
        self.y = r.deserialize();
        self.entity_type = r.deserialize();
        self.nbt = r.deserialize();
    }
}
