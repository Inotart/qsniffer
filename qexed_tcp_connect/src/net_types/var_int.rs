use serde::de::{Deserialize, Deserializer, Error, SeqAccess, Visitor};
use serde::ser::{Serialize, Serializer, SerializeSeq};
use std::fmt;
#[derive(Debug, Default, PartialEq,Clone)]
// VarInt 结构体定义
pub struct VarInt(pub i32);

// 实现 Serialize 特性
impl Serialize for VarInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // VarInt 编码逻辑：将整数编码为变长字节序列
        let mut value = self.0 as u32;
        let mut bytes = Vec::new();
        
        loop {
            let mut temp = (value & 0x7F) as u8;
            value >>= 7;
            if value != 0 {
                temp |= 0x80;
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

// 实现 Deserialize 特性
impl<'de> Deserialize<'de> for VarInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct VarIntVisitor;

        impl<'de> Visitor<'de> for VarIntVisitor {
            type Value = VarInt;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("VarInt encoded as a sequence of bytes")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut result = 0;
                let mut shift = 0;
                loop {
                    let byte: u8 = match seq.next_element()? {
                        Some(b) => b,
                        None => return Err(Error::custom("unexpected end of sequence")),
                    };
                    result |= ((byte & 0x7F) as i32) << shift;
                    shift += 7;
                    if byte & 0x80 == 0 {
                        break;
                    }
                    if shift >= 32 {
                        return Err(Error::custom("VarInt is too long"));
                    }
                }
                Ok(VarInt(result))
            }
        }

        deserializer.deserialize_seq(VarIntVisitor)
    }
}