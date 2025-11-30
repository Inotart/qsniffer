use proc_macro2::TokenStream as TokenStream2;
use quote::{quote};
use syn::{parse_macro_input, Ident, Type, Block, Token, Path};
use syn::parse::{Parse, ParseStream, Result};

/// 解析 packet_match 宏的输入
pub struct PacketMatchInput {
    packet_var: Ident,
    base_path: Path,  // 新增：基础路径
    cases: Vec<PacketCase>,
    default_case: Option<Block>,
}

/// 解析单个包匹配分支
pub struct PacketCase {
    packet_id: Ident,  // 改为 Ident，因为现在只需要简化的常量名
    packet_type: Type, // 保持 Type，但现在是相对路径
    var_name: Ident,
    handler: Block,
}

impl Parse for PacketMatchInput {
    fn parse(input: ParseStream) -> Result<Self> {
        // 解析包变量名
        let packet_var: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        
        // 解析基础路径
        let base_path: Path = input.parse()?;
        input.parse::<Token![,]>()?;
        
        // 解析大括号内的内容
        let content;
        syn::braced!(content in input);
        
        let mut cases = Vec::new();
        let mut default_case = None;
        
        // 解析所有 case 和 default
        while !content.is_empty() {
            let lookahead = content.lookahead1();
            
            if lookahead.peek(syn::Ident) {
                let ident: Ident = content.parse()?;
                
                match ident.to_string().as_str() {
                    "case" => {
                        content.parse::<Token![!]>()?;
                        let case = parse_packet_case(&content, &base_path)?;
                        cases.push(case);
                    }
                    "default" => {
                        content.parse::<Token![!]>()?;
                        let handler_block: Block = content.parse()?;
                        default_case = Some(handler_block);
                    }
                    _ => {
                        return Err(syn::Error::new(
                            ident.span(),
                            "expected 'case' or 'default'"
                        ));
                    }
                }
            } else {
                return Err(lookahead.error());
            }
            
            // 可选的分隔符
            if content.peek(Token![,]) {
                content.parse::<Token![,]>()?;
            }
        }
        
        Ok(PacketMatchInput {
            packet_var,
            base_path,
            cases,
            default_case,
        })
    }
}

/// 解析单个 case 分支，现在需要基础路径来构建完整路径
fn parse_packet_case(input: ParseStream, _base_path: &Path) -> Result<PacketCase> {
    // 解析包ID（现在只需要简化的常量名）
    let packet_id: Ident = input.parse()?;
    input.parse::<Token![=>]>()?;
    
    // 解析包类型（相对路径）
    let packet_type: Type = input.parse()?;
    input.parse::<Token![as]>()?;
    
    // 解析变量名
    let var_name: Ident = input.parse()?;
    
    // 解析处理块
    let handler: Block = input.parse()?;
    
    Ok(PacketCase {
        packet_id,
        packet_type,
        var_name,
        handler,
    })
}

/// 生成包匹配代码
fn generate_packet_match(input: PacketMatchInput) -> TokenStream2 {
    let packet_var = &input.packet_var;
    let base_path = &input.base_path;
    let cases = &input.cases;
    let default_case = &input.default_case;
    
    // 生成所有 case 分支
    let case_arms: Vec<_> = cases.iter().map(|case| {
        let packet_id = &case.packet_id;
        let packet_type = &case.packet_type;
        let var_name = &case.var_name;
        let handler = &case.handler;
        
        // 构建完整的包ID路径（基础路径 + pool + 包ID）
        let full_packet_id = quote! { #base_path::pool::#packet_id };
        
        // 构建完整的包类型路径（基础路径 + 包类型）
        let full_packet_type = if let syn::Type::Path(type_path) = packet_type {
            if type_path.path.segments.is_empty() {
                packet_type.clone()
            } else {
                // 如果是相对路径，就加上基础路径
                let mut new_path = base_path.clone();
                for segment in &type_path.path.segments {
                    new_path.segments.push(segment.clone());
                }
                syn::Type::Path(syn::TypePath {
                    path: new_path,
                    qself: None,
                })
            }
        } else {
            packet_type.clone()
        };
        
        quote! {
            #full_packet_id => {
                if let Some(#var_name) = #packet_var.as_any().downcast_ref::<#full_packet_type>() {
                    #handler
                }
            }
        }
    }).collect();
    
    // 生成默认分支
    let default_arm = if let Some(default_handler) = default_case {
        quote! { _ => #default_handler }
    } else {
        quote! { _ => {} }
    };
    
    quote! {
        {
            let #packet_var = &#packet_var;
            match #packet_var.id() {
                #(#case_arms)*
                #default_arm
            }
        }
    }
}

/// 解析 packet_match 宏的主函数
pub fn parse_packet_match(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as PacketMatchInput);
    generate_packet_match(input).into()
}