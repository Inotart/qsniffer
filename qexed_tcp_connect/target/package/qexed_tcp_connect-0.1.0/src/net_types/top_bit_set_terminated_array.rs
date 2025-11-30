use bytes::{ BytesMut};

use crate::{
    net_types::subdata::Subdata,
    packet::{encode::PacketWriter},
};

// 定义 topBitSetTerminatedArray 的包装类型
pub struct TopBitSetTerminatedArray<T>(pub Vec<T>);

impl<T> TopBitSetTerminatedArray<T> {
    pub fn new() -> Self {
        TopBitSetTerminatedArray(Vec::new())
    }
}

// 为 TopBitSetTerminatedArray 实现 Subdata trait
impl<T> Subdata for TopBitSetTerminatedArray<T>
where
    T: Subdata + Clone, // 需要 Clone 因为我们要修改第一个字节
{
    fn new() -> Self {
        TopBitSetTerminatedArray(Vec::new())
    }

    fn serialize(&self, w: &mut crate::packet::encode::PacketWriter) {
        let len = self.0.len();

        for (index, item) in self.0.iter().enumerate() {
            let mut buf = BytesMut::new();
            let mut wb = PacketWriter::new(&mut buf);
            // 序列化元素
            item.serialize(&mut wb);

            if !buf.is_empty() {
                // 修改第一个字节的最高位
                if index < len - 1 {
                    // 设置最高位，表示还有更多元素
                    buf[0] |= 0x80;
                } else {
                    // 清除最高位，表示这是最后一个元素
                    buf[0] &= 0x7F;
                }

                // 将修改后的数据写入主 PacketWriter
                w.byte_all((&buf).to_vec());
            }
        }
    }

    fn deserialize(&mut self, _r: &mut crate::packet::decode::PacketReader) {
        let result = Vec::new();
        // 不打算实现通用类型了
        // loop {  
        //     // 我们先读完内容
        //     let mut buf = BytesMut::new();
        //     // 从 r.buf 读取
        //     // 读取第一个字节
        //     let rb = PacketReader::new(Box::new(&mut buf));
        //     let first_byte = rb.u8();
        //     let has_more = (first_byte & 0x80) != 0;

        //     // 清除最高位
        //     let clean_first_byte = first_byte & 0x7F;
        //     // 读完内容
        //     let bytes = r.byte_all();
        //     // 写入回去
        //     let mut buf2 = Buf::new
        //     (**r.buf).put_u8(first_byte);        // 原来是 r.buf.put_u8(first_byte);
        //     (**r.buf).put_slice(&bytes);         // 原来是 r.buf.put_slice(bytes);
            
        //     // 读取一个元素
        //     let mut item: T = Subdata::new();
        //     item.deserialize(r);

        //     result.push(item);

        //     // 如果最高位为0，表示这是最后一个元素
        //     if !has_more {
        //         return;
        //     }
        // }

        *self = TopBitSetTerminatedArray(result);
    }
}
