use crate::{nullpacket::NullPacket, packets::server::status::{ping::Ping, server_info::ServerInfo}};

pub fn id_to_packet(id: u32) -> Box<dyn qexed_tcp_connect::net_types::packet::Packet,> {
    // protocol_version 还没做专门的适配
    match id {
        0x00 =>{Box::new(ServerInfo::default())},
        0x01 =>{Box::new(Ping::default())},
        _ => {
            // 使用传入的NullPacket类型
            log::warn!("Unknown packet ID: 0x{:X}, returning {} instance", id, stringify!(#null_packet));
            Box::new(NullPacket::new())
        },
    }
}