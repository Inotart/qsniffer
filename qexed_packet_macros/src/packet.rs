// src/packet.rs
use proc_macro2;
use quote::{quote};
use syn::{DeriveInput, Ident};
use syn::parse::{Parse, ParseStream};

/// 解析包ID参数
pub struct PacketId {
    pub value: u32,
}

impl Parse for PacketId {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // 解析 id = 0x00 格式
        let ident: syn::Ident = input.parse()?;
        if ident != "id" {
            return Err(syn::Error::new(ident.span(), "expected `id`"));
        }
        
        input.parse::<syn::Token![=]>()?;
        
        let lit: syn::LitInt = input.parse()?;
        let value = lit.base10_parse::<u32>()?;
        
        Ok(PacketId { value })
    }
}

/// 解析包配置结构，包括包列表和NullPacket类型
pub struct PacketConfig {
    pub packets: Vec<Ident>,
    pub null_packet: Ident,
}

impl Parse for PacketConfig {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // 解析包列表 [Handshake, StatusRequest, ...]
        let content;
        syn::bracketed!(content in input);
        let packets = content.parse_terminated(Ident::parse, syn::Token![,])?;
        
        // 解析逗号分隔符
        input.parse::<syn::Token![,]>()?;
        
        // 解析NullPacket类型
        let null_packet = input.parse::<Ident>()?;
        
        Ok(PacketConfig {
            packets: packets.into_iter().collect(),
            null_packet,
        })
    }
}

/// 实现Packet trait
pub fn implement_packet(packet_id: u32, input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let struct_name = &input.ident;
    
    // 移除默认值解析，只保留序列化字段
    let serialize_fields = parse_serialize_fields(input)?;
    
    // 只生成Packet trait实现，移除Default实现
    let packet_impl = generate_packet_impl(struct_name, packet_id, &serialize_fields);
    
    Ok(quote! {
        #input
        
        #packet_impl
    })
}

/// 只解析用于序列化的字段名（移除默认值处理）
fn parse_serialize_fields(input: &DeriveInput) -> syn::Result<Vec<proc_macro2::TokenStream>> {
    let mut serialize_fields = Vec::new();
    
    if let syn::Data::Struct(data_struct) = &input.data {
        if let syn::Fields::Named(fields_named) = &data_struct.fields {
            for field in &fields_named.named {
                let field_name = field.ident.as_ref().unwrap();
                
                serialize_fields.push(quote! {
                    #field_name
                });
            }
        }
    }
    
    Ok(serialize_fields)
}

fn generate_packet_impl(struct_name: &Ident, packet_id: u32, serialize_fields: &[proc_macro2::TokenStream]) -> proc_macro2::TokenStream {
    quote! {
        impl qexed_tcp_connect::net_types::packet::Packet for #struct_name {
            fn id(&self) -> u32 {
                #packet_id
            }
            
            fn serialize(&self, w: &mut qexed_tcp_connect::packet::encode::PacketWriter) {
                #(w.serialize(&self.#serialize_fields);)*
            }

            fn deserialize(&mut self, r: &mut qexed_tcp_connect::packet::decode::PacketReader) {
                #(self.#serialize_fields = r.deserialize();)*
            }
            
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    }
}

/// 为每个数据包生成匹配分支
pub fn generate_match_arms(packets: &[Ident]) -> Vec<proc_macro2::TokenStream> {
    packets.iter().enumerate().map(|(index, packet_name)| {
        let packet_id = index as u32;
        quote! {
            #packet_id => Box::new(#packet_name::new()),
        }
    }).collect()
}



/// 实现 Subdata trait
pub fn implement_subdata(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let struct_name = &input.ident;
    
    // 解析序列化字段
    let serialize_fields = parse_serialize_fields(input)?;
    
    // 生成 Subdata trait 实现
    let subdata_impl = generate_subdata_impl(struct_name, &serialize_fields);
    
    Ok(quote! {
        #input
        
        #subdata_impl
    })
}

fn generate_subdata_impl(struct_name: &Ident, serialize_fields: &[proc_macro2::TokenStream]) -> proc_macro2::TokenStream {
    quote! {
        impl qexed_tcp_connect::net_types::subdata::Subdata for #struct_name {
            fn new() -> Self {
                Self::default()
            }

            fn serialize(&self, w: &mut qexed_tcp_connect::packet::encode::PacketWriter) {
                #(w.serialize(&self.#serialize_fields);)*
            }

            fn deserialize(&mut self, r: &mut qexed_tcp_connect::packet::decode::PacketReader) {
                #(self.#serialize_fields = r.deserialize();)*
            }
        }
    }
}

// 在原有的 generate_function 基础上，增加常量生成
pub fn generate_function_with_consts(
    packets: &[Ident],
    null_packet: &Ident
) -> proc_macro2::TokenStream {
    let match_arms = generate_match_arms(packets);
    let const_defs = generate_const_definitions(packets);
    
    quote! {
        #(#const_defs)*
        
        /// 根据数据包ID创建对应的Packet实例
        pub fn id_to_packet(id: u32) -> Box<dyn qexed_tcp_connect::net_types::packet::Packet> {
            match id {
                #(#match_arms)*
                _ => {
                    // 使用传入的NullPacket类型
                    log::warn!("Unknown packet ID: 0x{:X}, returning {} instance", id, stringify!(#null_packet));
                    Box::new(#null_packet::new())
                },
            }
        }
    }
}

/// 为每个数据包生成常量定义
fn generate_const_definitions(packets: &[Ident]) -> Vec<proc_macro2::TokenStream> {
    packets.iter().enumerate().map(|(index, packet_name)| {
        let packet_id = index as u32;
        let const_name = format!("ID_{}", packet_name);
        let const_ident = Ident::new(&const_name, packet_name.span());
        
        quote! {
            /// 数据包ID常量
            #[allow(dead_code)]
            pub const #const_ident: u32 = #packet_id;
        }
    }).collect()
}