// src/lib.rs
extern crate proc_macro;
use proc_macro::TokenStream;
use syn::parse_macro_input;

// 声明 packet 模块
mod packet;
use packet::{PacketId, PacketConfig, implement_packet};

/// 主属性宏
#[proc_macro_attribute]
pub fn packet(args: TokenStream, input: TokenStream) -> TokenStream {
    // 解析包ID参数
    let packet_id = parse_macro_input!(args as PacketId);
    
    // 解析输入结构体
    let input = parse_macro_input!(input as syn::DeriveInput);
    
    // 实现Packet trait（不再包含Default实现）
    match implement_packet(packet_id.value, &input) {
        Ok(tokens) => tokens.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

/// 函数式过程宏 - 生成id到packet的映射函数，支持自定义NullPacket类型
#[proc_macro]
pub fn id_to_packet(input: TokenStream) -> TokenStream {
    let packet_config = parse_macro_input!(input as PacketConfig);
    
    let expanded = packet::generate_function_with_consts(&packet_config.packets, &packet_config.null_packet);
    
    TokenStream::from(expanded)
}
// 在 src/lib.rs 中添加

/// Subdata trait 实现宏
#[proc_macro_attribute]
pub fn substruct(_args: TokenStream, input: TokenStream) -> TokenStream {
    // 解析输入结构体
    let input = parse_macro_input!(input as syn::DeriveInput);
    
    // 实现Subdata trait
    match packet::implement_subdata(&input) {
        Ok(tokens) => tokens.into(),
        Err(e) => e.to_compile_error().into(),
    }
}
// 在原有的 id_to_packet 宏基础上，增加一个新版本
/// 函数式过程宏 - 生成id到packet的映射函数和ID常量
#[proc_macro]
pub fn id_to_packet_with_consts(input: TokenStream) -> TokenStream {
    let packet_config = parse_macro_input!(input as PacketConfig);
    
    let expanded = packet::generate_function_with_consts(&packet_config.packets, &packet_config.null_packet);
    
    TokenStream::from(expanded)
}
// 在 src/lib.rs 顶部添加
mod packet_match;
/// 包匹配过程宏
#[proc_macro]
pub fn packet_match(input: TokenStream) -> TokenStream {
    packet_match::parse_packet_match(input)
}