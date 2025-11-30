use crate::{nullpacket::NullPacket, packets::server::login::{compress, cookie_request, disconnect, encryption_begin, login_plugin_request, success}};


pub fn id_to_packet(id: u32,_protocol_version: i32) -> Box<dyn qexed_tcp_connect::net_types::packet::Packet,> {
    // protocol_version 还没做专门的适配
    match id {
        0x00 =>{Box::new(disconnect::Disconnect::default())},
        0x01 =>{Box::new(encryption_begin::EncryptionBegin::default())},
        0x02 =>{Box::new(success::Success::default())},
        0x03 =>{Box::new(compress::Compress::new())},
        0x04 =>{Box::new(login_plugin_request::LoginPluginRequest::new())},
        0x05 =>{Box::new(cookie_request::CookieRequest::new())},
        _ => {
            // 使用传入的NullPacket类型
            log::warn!("Unknown packet ID: 0x{:X}, returning {} instance", id, stringify!(#null_packet));
            Box::new(NullPacket::new())
        },
    }
}