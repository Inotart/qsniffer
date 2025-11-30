use crate::nullpacket::NullPacket;
use crate::packets::client::handshaking::set_protocol::SetProtocol;
use crate::packets::client::handshaking::legacy_server_list_ping::LegacyServerListPing;
pub fn id_to_packet(id: u32) -> Box<dyn qexed_tcp_connect::net_types::packet::Packet,> {
    // protocol_version 还没做专门的适配
    match id {
        0x00 =>{Box::new(SetProtocol::default())},
        0xfe =>{Box::new(LegacyServerListPing::default())},
        _ => {
            // 使用传入的NullPacket类型
            log::warn!("Unknown packet ID: 0x{:X}, returning {} instance", id, stringify!(#null_packet));
            Box::new(NullPacket::new())
        },
    }
}