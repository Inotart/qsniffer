use crate::{
    net_types::{block_entities::BlockEntities, heightmap::Heightmaps, subdata::Subdata},
    packet::{decode::PacketReader, encode::PacketWriter},
};
#[derive(Debug, Default, PartialEq,Clone)]
pub struct Chunk {
    pub heightmaps: Vec<Heightmaps>,
    pub data: Vec<u8>,
    pub block_entities:Vec<BlockEntities>
}
impl Subdata for Chunk {
    fn new() -> Self {
        Chunk {
            heightmaps: vec![],
            data: vec![],
            block_entities:vec![],
        }
    }
    fn serialize(&self, w: &mut PacketWriter) {
        w.serialize(&self.heightmaps);
        w.serialize(&self.data);
        w.serialize(&self.block_entities);
    }

    fn deserialize(&mut self, r: &mut PacketReader) {
        self.heightmaps = r.deserialize();
        self.data = r.deserialize();
        self.block_entities = r.deserialize();
    }
}
