/// 空数据包,处理报错的
#[derive(Debug, Default, PartialEq,Clone)]
pub struct NullPacket {}
impl NullPacket {
    pub fn new() -> Self {
        NullPacket {}
    }
}
impl qexed_tcp_connect::net_types::packet::Packet for NullPacket {
    fn id(&self) -> u32 {
        0xfff
    }
    fn serialize(&self, _w: &mut qexed_tcp_connect::packet::encode::PacketWriter) {}
    fn deserialize(&mut self, _r: &mut qexed_tcp_connect::packet::decode::PacketReader) {}
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
