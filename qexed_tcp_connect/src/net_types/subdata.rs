
use crab_nbt::Nbt;

use crate::{
    net_types::{self, rest_buffer::RestBuffer, var_int::VarInt, var_long::VarLong},
    packet::{decode::PacketReader, encode::PacketWriter},
};

pub trait Subdata {
    fn new() -> Self;
    fn serialize(&self, w: &mut PacketWriter);
    fn deserialize(&mut self, r: &mut PacketReader);
}
impl Subdata for u8 {
    fn new() -> Self {
        0
    }
    fn serialize(&self, w: &mut PacketWriter) {
        w.u8(*self);
    }

    fn deserialize(&mut self, r: &mut PacketReader) {
        *self = r.u8();
    }
}
impl Subdata for i8 {
    fn new() -> Self {
        0
    }
    fn serialize(&self, w: &mut PacketWriter) {
        w.i8(*self);
    }

    fn deserialize(&mut self, r: &mut PacketReader) {
        *self = r.i8();
    }
}
impl Subdata for u16 {
    fn new() -> Self {
        0
    }
    fn serialize(&self, w: &mut PacketWriter) {
        w.u16(*self);
    }

    fn deserialize(&mut self, r: &mut PacketReader) {
        *self = r.u16();
    }
}
impl Subdata for i16 {
    fn new() -> Self {
        0
    }
    fn serialize(&self, w: &mut PacketWriter) {
        w.i16(*self);
    }

    fn deserialize(&mut self, r: &mut PacketReader) {
        *self = r.i16();
    }
}
impl Subdata for u32 {
    fn new() -> Self {
        0
    }
    fn serialize(&self, w: &mut PacketWriter) {
        w.u32(*self);
    }

    fn deserialize(&mut self, r: &mut PacketReader) {
        *self = r.u32();
    }
}
impl Subdata for i32 {
    fn new() -> Self {
        0
    }
    fn serialize(&self, w: &mut PacketWriter) {
        w.i32(*self);
    }

    fn deserialize(&mut self, r: &mut PacketReader) {
        *self = r.i32();
    }
}
impl Subdata for u64 {
    fn new() -> Self {
        0
    }
    fn serialize(&self, w: &mut PacketWriter) {
        w.u64(*self);
    }

    fn deserialize(&mut self, r: &mut PacketReader) {
        *self = r.u64();
    }
}
impl Subdata for i64 {
    fn new() -> Self {
        0
    }
    fn serialize(&self, w: &mut PacketWriter) {
        w.i64(*self);
    }

    fn deserialize(&mut self, r: &mut PacketReader) {
        *self = r.i64();
    }
}
impl Subdata for f32{
    fn new() -> Self {
        0.0
    }
    fn serialize(&self, w: &mut PacketWriter) {
        w.f32(*self);
    }

    fn deserialize(&mut self, r: &mut PacketReader) {
        *self = r.f32();
    }
}
impl Subdata for f64{
    fn new() -> Self {
        0.0
    }
    fn serialize(&self, w: &mut PacketWriter) {
        w.f64(*self);
    }

    fn deserialize(&mut self, r: &mut PacketReader) {
        *self = r.f64();
    }
}
impl Subdata for bool {
    fn new() -> Self {
        false
    }
    fn serialize(&self, w: &mut PacketWriter) {
        w.bool(*self);
    }

    fn deserialize(&mut self, r: &mut PacketReader) {
        *self = r.bool();
    }
}
impl Subdata for String {
    fn new() -> Self {
        "".to_owned()
    }
    fn serialize(&self, w: &mut PacketWriter) {
        w.string(&self);
    }

    fn deserialize(&mut self, r: &mut PacketReader) {
        *self = r.string();
    }
}
impl Subdata for serde_json::Value {
    fn new() -> Self {
        serde_json::Value::Null
    }
    fn serialize(&self, w: &mut PacketWriter) {
        w.json(&self);
    }

    fn deserialize(&mut self, r: &mut PacketReader) {
        *self = r.json();
    }
}
impl Subdata for uuid::Uuid {
    fn new() -> Self {
        uuid::Uuid::nil()
    }
    fn serialize(&self, w: &mut PacketWriter) {
        w.uuid(&self);
    }

    fn deserialize(&mut self, r: &mut PacketReader) {
        *self = r.uuid();
    }
}
impl Subdata for VarInt {
    fn new() -> Self {
        net_types::var_int::VarInt(0)
    }
    fn serialize(&self, w: &mut PacketWriter) {
        w.varint(&self);
    }

    fn deserialize(&mut self, r: &mut PacketReader) {
        *self = r.varint();
    }
}
impl Subdata for crab_nbt::Nbt {
    fn new() -> Self {
        crab_nbt::nbt!("root", {})
    }
    fn serialize(&self, w: &mut PacketWriter) {
        let bytes = self.write_unnamed();
        w.byte_all(bytes.to_vec());
    }

    fn deserialize(&mut self, r: &mut PacketReader) {
        let rs: Result<Nbt, crab_nbt::error::Error> = Nbt::read(&mut *r.buf);
        if rs.is_err() {
            return;
        }
        *self = rs.unwrap();
    }
}

impl Subdata for RestBuffer {
    fn new() -> Self {
        RestBuffer(Vec::new())
    }
    fn serialize(&self, w: &mut crate::packet::encode::PacketWriter) {
        w.byte_all(self.0.clone());
    }

    fn deserialize(&mut self, r: &mut crate::packet::decode::PacketReader) {
        self.0 = r.byte_all();
    }
}

impl<T> Subdata for Option<T> where T: Subdata{
    fn new() -> Self {
        None
    }

    fn serialize(&self, w: &mut PacketWriter) {
        w.option(self.as_ref());
    }

    fn deserialize(&mut self, r: &mut PacketReader) {
        
        let mut is_true:bool = false;
        is_true.deserialize(r);
        if !is_true{
            *self = None;
            return
        }
        let mut v =  T::new();
        v.deserialize(r);
        *self=Some(v);
    }
}
impl<T> Subdata for Vec<T> where T: Subdata,{
    fn new() -> Self {
        Vec::new() // 返回空向量
    }

    fn serialize(&self, w: &mut PacketWriter) {
        // 1. 写入长度（VarInt 编码）
        w.vec(self);
    }

    fn deserialize(&mut self, r: &mut PacketReader) {
        *self = r.vec();
    }
}
impl<const N: usize> Subdata for [u8; N] {
    fn new() -> Self {
        [0u8; N] // 返回空向量
    }

    fn serialize(&self, w: &mut PacketWriter) {
        // 1. 写入长度（VarInt 编码）
        w.fixed_bytes(self);
    }

    fn deserialize(&mut self, r: &mut PacketReader) {
        *self = r.fixed_bytes();
    }
}
impl Subdata for VarLong {
    fn new() -> Self {
        net_types::var_long::VarLong(0)
    }
    fn serialize(&self, w: &mut PacketWriter) {
        w.varlong(&self);
    }

    fn deserialize(&mut self, r: &mut PacketReader) {
        *self = r.varlong();
    }
}
// fixed_bytes(&mut self,value:&[u8; N])