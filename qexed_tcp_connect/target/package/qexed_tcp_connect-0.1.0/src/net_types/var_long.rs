use serde::de::{Deserialize, Deserializer, Error, SeqAccess, Visitor};
use serde::ser::{Serialize, Serializer, SerializeSeq};
use std::fmt;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct VarLong(pub i64);

// ZigZag 编码辅助函数
pub fn zigzag_encode(n: i64) -> u64 {
    ((n << 1) ^ (n >> 63)) as u64
}

pub fn zigzag_decode(n: u64) -> i64 {
    ((n >> 1) as i64) ^ (-((n & 1) as i64))
}

impl Serialize for VarLong {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 先进行 ZigZag 编码处理负数
        let mut value = zigzag_encode(self.0);
        let mut bytes = Vec::new();
        
        loop {
            let mut temp = (value & 0x7F) as u8;
            value >>= 7;
            if value != 0 {
                temp |= 0x80;  // 设置续位标志
            }
            bytes.push(temp);
            if value == 0 {
                break;
            }
        }
        
        // 将字节序列序列化为序列
        let mut seq = serializer.serialize_seq(Some(bytes.len()))?;
        for byte in bytes {
            seq.serialize_element(&byte)?;
        }
        seq.end()
    }
}

impl<'de> Deserialize<'de> for VarLong {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct VarLongVisitor;

        impl<'de> Visitor<'de> for VarLongVisitor {
            type Value = VarLong;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("VarLong encoded as a sequence of bytes")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut result: u64 = 0;
                let mut shift = 0;
                
                loop {
                    let byte: u8 = match seq.next_element()? {
                        Some(b) => b,
                        None => return Err(Error::custom("unexpected end of sequence")),
                    };
                    
                    // 检查是否超出 64 位范围
                    if shift >= 64 {
                        return Err(Error::custom("VarLong is too long"));
                    }
                    
                    result |= ((byte & 0x7F) as u64) << shift;
                    shift += 7;
                    
                    // 如果续位标志为0，结束解码
                    if byte & 0x80 == 0 {
                        break;
                    }
                }
                
                // ZigZag 解码
                let decoded_value = zigzag_decode(result);
                Ok(VarLong(decoded_value))
            }
        }

        deserializer.deserialize_seq(VarLongVisitor)
    }
}

// 为 VarLong 实现一些实用方法
impl VarLong {
    pub fn new(value: i64) -> Self {
        VarLong(value)
    }
    
    pub fn value(&self) -> i64 {
        self.0
    }
    
    pub fn set_value(&mut self, value: i64) {
        self.0 = value;
    }
}

// 实现从 i64 到 VarLong 的转换
impl From<i64> for VarLong {
    fn from(value: i64) -> Self {
        VarLong(value)
    }
}

// 实现从 VarLong 到 i64 的转换
impl From<VarLong> for i64 {
    fn from(var_long: VarLong) -> Self {
        var_long.0
    }
}
