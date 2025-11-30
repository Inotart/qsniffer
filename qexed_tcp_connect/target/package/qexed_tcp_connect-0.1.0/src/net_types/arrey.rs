use crate::{
    net_types::subdata::Subdata,
    packet::{decode::PacketReader, encode::PacketWriter},
};

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Array<L, T> {
    length: L,
    data: Vec<T>,
}

impl<L, T> Subdata for Array<L, T>
where
    L: Subdata + Default + Copy + Into<usize>,
    T: Subdata,
{
    fn new() -> Self {
        Self {
            length: L::new(),
            data: Vec::new(),
        }
    }

    fn serialize(&self, w: &mut PacketWriter) {
        // 1. 写入长度
        self.length.serialize(w);
        // 2. 写入每个元素
        for item in &self.data {
            item.serialize(w);
        }
    }

    fn deserialize(&mut self, r: &mut PacketReader) {
        // 1. 读取长度
        self.length.deserialize(r);
        
        // 2. 清空现有数据并预分配空间
        let len_usize: usize = self.length.into();
        self.data.clear();
        self.data.reserve(len_usize);
        
        // 3. 读取每个元素
        for _ in 0..len_usize {
            let mut item = T::new();
            item.deserialize(r);
            self.data.push(item);
        }
    }
}

// 为 Array 添加一些实用方法
impl<L, T> Array<L, T> {
    /// 从 Vec 创建 Array
    pub fn from_vec(data: Vec<T>) -> Self
    where
        L: Subdata + From<usize>,
    {
        let length = L::from(data.len());
        Self { length, data }
    }

    /// 获取数据引用
    pub fn data(&self) -> &Vec<T> {
        &self.data
    }

    /// 获取长度
    pub fn len(&self) -> L
    where
        L: Copy,
    {
        self.length
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// 转换为内部的 Vec
    pub fn into_inner(self) -> Vec<T> {
        self.data
    }
}

// 便捷的类型别名
pub type ArrayU8<T> = Array<u8, T>;
pub type ArrayU16<T> = Array<u16, T>;
pub type ArrayU32<T> = Array<u32, T>;
pub type ArrayVarInt<T> = Array<crate::net_types::var_int::VarInt, T>;