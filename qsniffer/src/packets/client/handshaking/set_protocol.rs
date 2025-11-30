#[qexed_packet_macros::packet(id = 0x00)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct SetProtocol {
    pub protocol_version:qexed_tcp_connect::net_types::var_int::VarInt,
    pub server_host:String,
    pub server_port:u16,
    pub next_state:qexed_tcp_connect::net_types::var_int::VarInt,
}
