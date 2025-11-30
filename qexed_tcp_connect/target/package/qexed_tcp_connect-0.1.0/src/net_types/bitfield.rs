use crate::{
    net_types::{subdata::Subdata},
    packet::{decode::PacketReader, encode::PacketWriter},
};
#[derive(Debug, Default, PartialEq,Clone)]
pub struct Bitfield(pub Vec<u8>);
impl Subdata for Bitfield {
    fn new() -> Self {
        Bitfield(vec![])
    }
    fn serialize(&self, w: &mut PacketWriter) {
        w.serialize(&self.0);
    }

    fn deserialize(&mut self, r: &mut PacketReader) {
        *self = r.deserialize()
    }
}