use crate::{
    net_types::{bitset::Bitset, subdata::Subdata},
    packet::{decode::PacketReader, encode::PacketWriter},
};
#[derive(Debug, Default, PartialEq,Clone)]
pub struct Light {
    pub sky_light_mask: Bitset,
    pub block_light_mask: Bitset,
    pub empty_sky_light_mask: Bitset,
    pub empty_block_light_mask: Bitset,
    pub sky_light_arrays: Vec<Vec<u8>>,
    pub block_light_arrays: Vec<Vec<u8>>,
}
impl Subdata for Light {
    fn new() -> Self {
        Light {
            sky_light_mask: Bitset(vec![]),
            block_light_mask: Bitset(vec![]),
            empty_sky_light_mask: Bitset(vec![]),
            empty_block_light_mask: Bitset(vec![]),
            sky_light_arrays: vec![],
            block_light_arrays: vec![],
        }
    }
    fn serialize(&self, w: &mut PacketWriter) {
        w.serialize(&self.sky_light_mask);
        w.serialize(&self.block_light_mask);
        w.serialize(&self.empty_sky_light_mask);
        w.serialize(&self.empty_block_light_mask);
        w.serialize(&self.sky_light_arrays);
        w.serialize(&self.block_light_arrays);
    }

    fn deserialize(&mut self, r: &mut PacketReader) {
        self.sky_light_mask = r.deserialize();
        self.block_light_mask = r.deserialize();
        self.empty_sky_light_mask = r.deserialize();
        self.empty_block_light_mask = r.deserialize();
        self.sky_light_arrays = r.deserialize();
        self.block_light_arrays = r.deserialize();
    }
}
