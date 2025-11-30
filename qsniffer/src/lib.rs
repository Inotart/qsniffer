use anyhow::Result;
use bytes::{Bytes, BytesMut};
use std::sync::Arc;
use tokio::sync::Mutex;

mod nullpacket;
mod packets;

use qexed_tcp_connect::{net_types::packet::Packet, packet::decode::PacketReader};
use packets::server::login::disconnect::Disconnect;

// 共享状态结构体
struct SharedState {
    packet_state: packets::status::PacketState,
    is_finish: bool,
    protocol_version: i32,
}

impl SharedState {
    fn new() -> Self {
        Self {
            packet_state: packets::status::PacketState::Handshaking,
            is_finish: false,
            protocol_version: -1,
        }
    }
}

// 数据包校验函数类型
pub type PacketValidator = dyn Fn(&Bytes, i32) -> Result<()> + Send + Sync;

/// 运行 Minecraft 代理服务器
/// 
/// # 参数
/// - `proxy_bind_addr`: 代理服务器绑定的地址 (例如: "0.0.0.0:25565")
/// - `server_addr`: 目标服务器地址 (例如: "127.0.0.1:25566")
/// - `client_validator`: 客户端数据包校验函数
/// - `server_validator`: 服务端数据包校验函数
/// 
/// # 返回
/// - `Result<()>`: 如果运行成功返回Ok(()), 否则返回错误
pub async fn run_proxy(
    proxy_bind_addr: &str,
    server_addr: &str,
    client_validator: Option<Arc<PacketValidator>>,
    server_validator: Option<Arc<PacketValidator>>,
) -> Result<()> {
    let tcplistener = tokio::net::TcpListener::bind(proxy_bind_addr).await?;
    println!("代理服务器启动在: {}", proxy_bind_addr);
    
    while let std::result::Result::Ok((socket, socketaddr)) = tcplistener.accept().await {
        println!("新的客户端连接: {}", socketaddr);
        
        let network_compression_threshold = 256;
        let (socket_read, socket_write) = tokio::io::split(socket);
        let packet_socket = qexed_tcp_connect::PacketListener::new(
            socket_read,
            socket_write,
            network_compression_threshold,
        );

        // 克隆验证器以便在异步任务中使用
        let client_validator_clone = client_validator.clone();
        let server_validator_clone = server_validator.clone();
        let server_addr = server_addr.to_string();
        
        tokio::spawn(async move {
            if let Err(e) = client_handle(
                packet_socket, 
                network_compression_threshold, 
                &server_addr,
                client_validator_clone,
                server_validator_clone,
            ).await {
                eprintln!("客户端处理错误: {}", e);
            }
        });
    }
    Ok(())
}

async fn client_handle(
    packet_socket: qexed_tcp_connect::PacketListener,
    network_compression_threshold: usize,
    server_addr: &str,
    client_validator: Option<Arc<PacketValidator>>,
    server_validator: Option<Arc<PacketValidator>>,
) -> Result<()> {
    let (mut packet_read, packet_write) = packet_socket.split();
    let client_socket = tokio::net::TcpStream::connect(server_addr).await?;
    println!("连接到目标服务器: {}", server_addr);
    
    let (client_socket_read, client_socket_write) = tokio::io::split(client_socket);
    let client_packet_socket = qexed_tcp_connect::PacketListener::new(
        client_socket_read,
        client_socket_write,
        network_compression_threshold,
    );
    let (mut client_packet_read, client_packet_write) = client_packet_socket.split();
    
    // 使用Arc和Mutex来共享状态和PacketSend对象
    let shared_state = Arc::new(Mutex::new(SharedState::new()));
    let client_packet_write_shared = Arc::new(Mutex::new(client_packet_write));
    let packet_write_shared = Arc::new(Mutex::new(packet_write));

    // (C->P->S) 客户端到服务端的数据流
    let state_clone1 = Arc::clone(&shared_state);
    let client_packet_write_clone = Arc::clone(&client_packet_write_shared);
    let client_validator_clone = client_validator.clone();
    
    let client_to_server_handle = tokio::spawn(async move {
        loop {
            let raw_packets_result = packet_read.read().await;
            let packets = match raw_packets_result {
                Ok(packets) => Bytes::from(packets),
                Err(_) => break,
            };
            
            // 获取当前状态
            let (current_state, is_finish, protocol_version) = {
                let state = state_clone1.lock().await;
                (state.packet_state, state.is_finish, state.protocol_version)
            };
            
            // 如果有客户端验证器，执行验证
            if let Some(validator) = &client_validator_clone {
                if let Err(e) = validator(&packets, protocol_version) {
                    eprintln!("客户端数据包验证失败: {}", e);
                    break;
                }
            }
            
            if !is_finish {
                if let Ok(packet) = read_packet_client(packets.clone(), current_state) {
                    match current_state {
                        packets::status::PacketState::Handshaking => {
                            if packet.id() == 0x00 {
                                if let Some(handshake) = packet.as_any().downcast_ref::<packets::client::handshaking::set_protocol::SetProtocol>() {
                                    // 更新共享状态
                                    let mut state = state_clone1.lock().await;
                                    state.protocol_version = handshake.protocol_version.0;
                                    match handshake.next_state.0 {
                                        1 => state.packet_state = packets::status::PacketState::Status,
                                        2 => state.packet_state = packets::status::PacketState::Login,
                                        _ => {}
                                    }
                                }
                            }
                        },
                        packets::status::PacketState::Status => {
                            // 更新完成状态
                            let mut state = state_clone1.lock().await;
                            state.is_finish = true;
                        }
                        packets::status::PacketState::Login => {
                            if packet.id() == 0x03 {
                                let mut state = state_clone1.lock().await;
                                state.is_finish = true;
                            }
                        }
                    }
                }
            }
            
            // 使用锁来发送数据
            let mut write_guard = client_packet_write_clone.lock().await;
            if write_guard.send_raw(packets).await.is_err() {
                break;
            }
        }
        println!("客户端到服务端的数据流结束");
    });

    // (S->P->C) 服务端到客户端的数据流
    let state_clone2 = Arc::clone(&shared_state);
    let client_packet_write_clone2 = Arc::clone(&client_packet_write_shared);
    let packet_write_clone = Arc::clone(&packet_write_shared);
    let server_validator_clone = server_validator.clone();
    
    let server_to_client_handle = tokio::spawn(async move {
        loop {
            let raw_packets_result = client_packet_read.read().await;
            let packets = match raw_packets_result {
                Ok(packets) => Bytes::from(packets),
                Err(_) => break,
            };
            
            // 获取当前状态
            let (current_state, is_finish, protocol_version) = {
                let state = state_clone2.lock().await;
                (state.packet_state, state.is_finish, state.protocol_version)
            };
            
            // 如果有服务端验证器，执行验证
            if let Some(validator) = &server_validator_clone {
                if let Err(e) = validator(&packets,  protocol_version) {
                    eprintln!("服务端数据包验证失败: {}", e);
                    break;
                }
            }
            
            if !is_finish {
                if let Ok(packet) = read_packet_server(packets.clone(), current_state, protocol_version) {
                    match current_state {
                        packets::status::PacketState::Login => {
                            match packet.id() {
                                0x01 => {
                                    // 检测到加密请求，发送错误消息并关闭连接
                                    let mut disconnect = Disconnect::default();
                                    disconnect.reason = serde_json::json!({
                                        "text": "Qsniffer 不支持加密的服务端的数据包校验！请关闭服务端加密",
                                        "color": "red",
                                        "bold": true
                                    });
                                    
                                    let mut packet_write_guard = packet_write_clone.lock().await;
                                    if packet_write_guard.send(&disconnect).await.is_err() {
                                        break;
                                    }
                                    let _ = packet_write_guard.shutdown().await;
                                    continue;
                                }
                                0x03 => {
                                    if let Some(compress) = packet.as_any().downcast_ref::<packets::server::login::compress::Compress>() {
                                        let network_compression_threshold = compress.threshold.0 as usize;
                                        
                                        // 先发送压缩包给客户端
                                        let mut packet_write_guard = packet_write_clone.lock().await;
                                        if packet_write_guard.send_raw(packets.clone()).await.is_err() {
                                            break;
                                        }
                                        
                                        // 设置代理端->客户端的压缩
                                        packet_write_guard.set_compression_value(network_compression_threshold);
                                        packet_write_guard.set_compression(true);
                                        
                                        // 设置代理端->服务端的压缩
                                        let client_write_guard = client_packet_write_clone2.lock().await;
                                        client_write_guard.set_compression_value(network_compression_threshold);
                                        client_write_guard.set_compression(true);
                                        
                                        println!("已启用压缩，阈值: {}", network_compression_threshold);
                                        continue;
                                    }
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
            }
            
            // 正常发送其他数据包
            let mut packet_write_guard = packet_write_clone.lock().await;
            if packet_write_guard.send_raw(packets).await.is_err() {
                break;
            }
        }
        println!("服务端到客户端的数据流结束");
    });

    // 等待两个任务完成
    let (client_result, server_result) = tokio::join!(client_to_server_handle, server_to_client_handle);
    
    // 检查任务是否正常结束
    if let Err(e) = client_result {
        eprintln!("客户端到服务端任务错误: {}", e);
    }
    if let Err(e) = server_result {
        eprintln!("服务端到客户端任务错误: {}", e);
    }
    
    Ok(())
}

// 数据包读取函数
fn read_packet_client(
    data: Bytes,
    status: packets::status::PacketState,
) -> Result<Box<dyn Packet>> {
    let mut buf = BytesMut::new();
    buf.extend_from_slice(&data);
    let mut reader = PacketReader::new(Box::new(&mut buf));
    let id = reader.varint().0 as u32;
    
    let mut decoded = match status {
        packets::status::PacketState::Handshaking => {
            packets::client::handshaking::pool::id_to_packet(id)
        }
        packets::status::PacketState::Status => packets::client::status::pool::id_to_packet(id),
        packets::status::PacketState::Login => packets::client::login::pool::id_to_packet(id),
    };
    decoded.deserialize(&mut reader);
    Ok(decoded)
}

fn read_packet_server(
    data: Bytes,
    status: packets::status::PacketState,
    protocol_version: i32,
) -> Result<Box<dyn Packet>> {
    let mut buf = BytesMut::new();
    buf.extend_from_slice(&data);
    let mut reader = PacketReader::new(Box::new(&mut buf));
    let id = reader.varint().0 as u32;
    
    let mut decoded = match status {
        packets::status::PacketState::Handshaking => Box::new(nullpacket::NullPacket::new()),
        packets::status::PacketState::Status => packets::server::status::pool::id_to_packet(id),
        packets::status::PacketState::Login => {
            packets::server::login::pool::id_to_packet(id, protocol_version)
        }
    };
    decoded.deserialize(&mut reader);
    Ok(decoded)
}