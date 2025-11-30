use dyn_clone::DynClone;

use crate::packet::decode::PacketReader;
use crate::packet::encode::PacketWriter;

// 定义 Packet trait 作为所有数据包的公共接口

pub trait Packet: DynClone+ std::fmt::Debug + Send + Sync{
    fn id(&self)->u32;
    fn serialize(&self, w: &mut PacketWriter);
    fn deserialize(&mut self, r: &mut PacketReader);
    fn as_any(&self) -> &dyn std::any::Any;
}
dyn_clone::clone_trait_object!(Packet);