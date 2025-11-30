#[qexed_packet_macros::packet(id = 0x04)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct LoginPluginRequest {
    pub message_id:qexed_tcp_connect::net_types::var_int::VarInt,
    pub channel:String,
    pub data:qexed_tcp_connect::net_types::rest_buffer::RestBuffer,
}
impl LoginPluginRequest {
    pub fn new() -> Self {
        LoginPluginRequest {
            message_id:qexed_tcp_connect::net_types::var_int::VarInt(0),
            channel:"".to_string(),
            data:qexed_tcp_connect::net_types::rest_buffer::RestBuffer::new(),
        }
    }
}
