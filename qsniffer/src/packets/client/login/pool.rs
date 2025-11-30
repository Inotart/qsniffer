use crate::nullpacket::NullPacket;
use crate::packets::client::login::login_start::LoginStart;
use crate::packets::client::login::encryption_begin::EncryptionBegin;
use crate::packets::client::login::login_plugin_response::LoginPluginResponse;
use crate::packets::client::login::login_acknowledged::LoginAcknowledged;
use crate::packets::client::login::cookie_response::CookieResponse;

pub fn id_to_packet(id: u32) -> Box<dyn qexed_tcp_connect::net_types::packet::Packet,> {
    // protocol_version 还没做专门的适配
    match id {
        0x00 =>{Box::new(LoginStart::default())},
        0x01 =>{Box::new(EncryptionBegin::default())},
        0x02 =>{Box::new(LoginPluginResponse::default())},
        0x03 =>{Box::new(LoginAcknowledged::default())},
        0x04 =>{Box::new(CookieResponse::default())},
        _ => {
            // 使用传入的NullPacket类型
            log::warn!("Unknown packet ID: 0x{:X}, returning {} instance", id, stringify!(#null_packet));
            Box::new(NullPacket::new())
        },
    }
}