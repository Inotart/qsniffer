use bytes::{BufMut, BytesMut};

use crate::net_types::{subdata::Subdata, var_int::VarInt};

pub struct PacketWriter<'a> {
    buf: &'a mut BytesMut,
}

impl<'a> PacketWriter<'a> {
    pub fn new(buf: &'a mut BytesMut) -> Self {
        Self { buf }
    }

    pub fn u8(&mut self, value: u8) {
        self.buf.put_u8(value);
    }

    pub fn bool(&mut self, value: bool) {
        self.u8(if value { 1 } else { 0 });
    }

    pub fn u16(&mut self, value: u16) {
        self.buf.put_u16(value);
    }

    pub fn u32(&mut self, value: u32) {
        self.buf.put_u32(value);
    }
    pub fn u64(&mut self, value: u64) {
        self.buf.put_u64(value);
    }
    pub fn i8(&mut self, value: i8) {
        self.buf.put_i8(value);
    }
    pub fn i16(&mut self, value: i16) {
        self.buf.put_i16(value);
    }
    pub fn i32(&mut self, value: i32) {
        self.buf.put_i32(value);
    }
    pub fn i64(&mut self, value: i64) {
        self.buf.put_i64(value);
    }
    pub fn f32(&mut self, value: f32){
        self.buf.put_f32(value);
    }
    pub fn f64(&mut self, value: f64){
        self.buf.put_f64(value);
    }
    pub fn string(&mut self, value: &str) {
        self.varint(&VarInt(value.len() as i32));
        self.buf.put_slice(value.as_bytes());
    }
    pub fn byte_all(&mut self, value: Vec<u8>) {
        self.buf.put_slice(&value);
    }

    pub fn option_string(&mut self, value: Option<&str>) {
        if let Some(v) = value {
            self.bool(true);
            self.string(v);
        } else {
            self.bool(false);
        }
    }

    pub fn json(&mut self, value: &serde_json::Value) {
        let json_str = value.to_string();
        self.varint(&VarInt(json_str.len() as i32));
        self.buf.put_slice(json_str.as_bytes());
    }
    pub fn uuid(&mut self, value: &uuid::Uuid) {
        self.buf.put_slice(value.as_bytes());
    }
    pub fn varint(&mut self, value: &VarInt) {
        let mut val = value.0 as u32;
        loop {
            let mut temp = (val & 0x7F) as u8;
            val >>= 7;
            if val != 0 {
                temp |= 0x80;
            }
            self.buf.put_u8(temp);
            if val == 0 {
                break;
            }
        }
    }
    /// 读取固定长度的字节数组
    pub fn fixed_bytes<const N: usize>(&mut self,value:&[u8; N]) {
        // 检查是否有足够的数据
        for i in 0..N {
            self.u8(value[i]);
        }
    }
    pub fn vec<T:Subdata>(&mut self,value: &Vec<T>){
        let l = value.len();
        self.varint(&VarInt(l as i32));
        for prop in value {
            prop.serialize(self);
        }
    }
    pub fn serialize<T:Subdata>(&mut self,value: &T){
        value.serialize(self);
    }
    pub fn option<T:Subdata>(&mut self, value: Option<&T>){
        if let Some(v) = value {
            self.bool(true);
            v.serialize(self);
        } else {
            self.bool(false);
        }
    }
}
