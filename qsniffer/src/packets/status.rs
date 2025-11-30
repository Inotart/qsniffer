// 此文件使用脚本自动生成
use std::fmt::Display;
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PacketState {
    Handshaking,
    Status,
    Login,    
}

impl From<&str> for PacketState {
    fn from(value: &str) -> Self {
        match value {
            "handshaking" => PacketState::Handshaking,
            "status" => PacketState::Status,
            "login" => PacketState::Login,
            wrong => panic!("Invalid state: {wrong}. Must be: `configuration`, `handshake`, `login`, `play`, or `status`(旧版本没有 configuration)"),
        }
    }
}
impl Display for PacketState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PacketState::Handshaking => write!(f, "handshaking"),
            PacketState::Status => write!(f, "status"),
            PacketState::Login => write!(f, "login"),
        }
    }
}
