use anyhow::Result;
use std::sync::Arc;
use qsniffer::run_proxy;

#[tokio::main]
async fn main() -> Result<()> {
    // 示例验证器函数
    let client_validator = Arc::new(|data: &bytes::Bytes, version| {
        println!("客户端数据包验证 - 协议版本: {}, 数据长度: {}",  version, data.len());
        Ok(())
    });

    let server_validator = Arc::new(|data: &bytes::Bytes, version| {
        println!("服务端数据包验证 -  协议版本: {}, 数据长度: {}",  version, data.len());
        Ok(())
    });
    // 请注意:请自行完成数据包状态机的维护!!!
    // 量子嗅探器并不打算对外维护登录后的任何数据包
    // 运行代理服务器
    run_proxy(
        "0.0.0.0:25565",      // 代理绑定地址
        "127.0.0.1:25566",    // 目标服务器地址
        Some(client_validator),
        Some(server_validator),
    ).await?;

    Ok(())
}