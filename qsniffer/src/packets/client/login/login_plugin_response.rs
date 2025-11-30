#[qexed_packet_macros::packet(id = 0x02)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct LoginPluginResponse {
    pub message_id:qexed_tcp_connect::net_types::var_int::VarInt,
    pub data:Option<qexed_tcp_connect::net_types::rest_buffer::RestBuffer>,
}
