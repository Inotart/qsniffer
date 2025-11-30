use bytes::Buf;

use crate::net_types::{subdata::Subdata, var_int::VarInt};

pub struct PacketReader<'a> {
    pub buf: Box<&'a mut dyn Buf>,
}

impl<'a> PacketReader<'a> {
    pub fn new(buf: Box<&'a mut dyn Buf>) -> Self {
        Self { buf }
    }
    pub fn u8(&mut self) -> u8 {
        self.buf.get_u8()
    }
    pub fn i8(&mut self) -> i8 {
        self.buf.get_i8()
    }
    pub fn bool(&mut self) -> bool {
        self.u8() != 0
    }
    pub fn u16(&mut self) -> u16 {
        self.buf.get_u16()
    }
    pub fn u32(&mut self) -> u32 {
        self.buf.get_u32()
    }
    pub fn u64(&mut self) -> u64 {
        self.buf.get_u64()
    }
    pub fn i16(&mut self) -> i16 {
        self.buf.get_i16()
    }
    pub fn i32(&mut self) -> i32 {
        self.buf.get_i32()
    }
    pub fn i64(&mut self) -> i64 {
        self.buf.get_i64()
    }
    pub fn f32(&mut self) -> f32 {
        self.buf.get_f32()
    }
    pub fn f64(&mut self) -> f64 {
        self.buf.get_f64()
    }
    pub fn string(&mut self) -> String {
        let len = self.varint().0 as usize;
        let bytes = self.buf.copy_to_bytes(len);
        String::from_utf8_lossy(&bytes).to_string()
    }
    pub fn byte_all(&mut self) -> Vec<u8> {
        let len = self.buf.remaining();
        let bytes = self.buf.copy_to_bytes(len);
        bytes.to_vec()
    }
    pub fn option_string(&mut self) -> Option<String> {
        let is_have = self.bool();
        if is_have {
            return Some(self.string());
        } else {
            return None;
        }
    }
    pub fn json(&mut self) -> serde_json::Value {
        let word = self.string();
        return serde_json::json!(word);
    }
    pub fn uuid(&mut self) -> uuid::Uuid {
        // 创建 16 字节数组
        let mut bytes = [0u8; 16];
        self.buf.copy_to_slice(&mut bytes);
        uuid::Uuid::from_bytes(bytes)
    }
    pub fn varint(&mut self) -> VarInt {
        let mut value = 0;
        let mut position = 0;

        for _ in 0..5 {
            let byte = self.buf.get_u8();
            value |= (byte as i32 & 0x7F) << (7 * position);

            if (byte & 0x80) == 0 {
                return VarInt(value);
            }

            position += 1;
        }

        panic!("Invalid VarInt");
    }
    /// 读取固定长度的字节数组
    pub fn fixed_bytes<const N: usize>(&mut self) -> [u8; N] {
        // 检查是否有足够的数据
        let mut result = [0u8; N];
        for i in 0..N {
            result[i] = self.u8();
        }
        result
    }
    pub fn vec<T: Subdata>(&mut self) -> Vec<T> {
        let len = self.varint().0 as usize; // 获取 VarInt 的值
        // 清空现有属性并预分配空间
        let mut value: Vec<T> = vec![];
        value.reserve(len);
        // 遍历读取每个属性
        for _ in 0..len {
            let mut a = T::new();
            a.deserialize(self);
            value.push(a);
        }
        value
    }
    pub fn deserialize<T: Subdata>(&mut self) -> T {
        let mut t = T::new();
        t.deserialize(self);
        t
    }
    pub fn option<T: Subdata>(&mut self) -> Option<T> {
        let is_true = self.bool();
        if !is_true {
            return None;
        }
        let mut v = T::new();
        v.deserialize(self);
        return Some(v);
    }
}
