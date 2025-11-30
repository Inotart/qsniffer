#[qexed_packet_macros::packet(id = 0x03)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Compress {
    pub threshold:qexed_tcp_connect::net_types::var_int::VarInt,
}
impl Compress {
    pub fn new() -> Self {
        Compress {
            threshold:qexed_tcp_connect::net_types::var_int::VarInt(0),
        }
    }
}
