# Qsniffer 量子嗅探器 / Quantum Sniffer
Qsniffer 是 Qexed 项目的基础设施组件，主要用于验证和测试数据包解析函数的正确性。这是一个开发辅助工具，帮助开发者检查网络数据包的解析逻辑。

Qsniffer is an infrastructure component of the Qexed project, primarily used to validate and test the correctness of packet parsing functions. It's a development assistance tool that helps developers inspect network packet parsing logic.
# ⚠️ 重要说明 / Important Notes
## 代码状态说明 / Code Status
Qexed 项目由于尚未开发完成，暂时无法开源，因此独立引用代码使用。开发完成后将使用 MIT 协议开源。

The Qexed project is not yet open source as it's still under active development, hence the code is used as an independent reference. It will be released under MIT license once completed.
## 贡献指南 / Contribution Guidelines
在此期间请不要对 Qexed 内部 crate 的内容进行任何 PR 修改，避免后期开发完成后的适配工作问题。

Please do not submit any PR modifications for the internal Qexed crates during this period to avoid adaptation issues after the development is completed.
# 🚀 快速开始 / Quick Start
## 添加依赖 / Adding Dependencies
```toml
[dependencies]
qsniffer = "0.1.0"
anyhow = "1.0"
tokio = { version = "1.0", features = ["full"] }
```
## 基本用法 / Basic Usage
```rust
use anyhow::Result;
use std::sync::Arc;
use qsniffer::run_proxy;

#[tokio::main]
async fn main() -> Result<()> {
    // 客户端数据包验证器 / Client packet validator
    let client_validator = Arc::new(|data: &bytes::Bytes, version| {
        println!("客户端数据包验证 - 协议版本: {}, 数据长度: {}", version, data.len());
        Ok(())
    });

    // 服务端数据包验证器 / Server packet validator
    let server_validator = Arc::new(|data: &bytes::Bytes, version| {
        println!("服务端数据包验证 - 协议版本: {}, 数据长度: {}", version, data.len());
        Ok(())
    });

    // 运行代理服务器 / Run proxy server
    run_proxy(
        "0.0.0.0:25565",      // 代理绑定地址 / Proxy binding address
        "127.0.0.1:25566",    // 目标服务器地址 / Target server address
        Some(client_validator),
        Some(server_validator),
    ).await?;

    Ok(())
}
```
# 📋 注意事项 / Notes
重要: 量子嗅探器不维护数据包状态机。请自行处理登录后的数据包状态管理。

Important: The quantum sniffer does not maintain packet state machines. Please handle post-login packet state management yourself.
# 💬 交流与支持 / Communication & Support
QQ群: 627495509

QQ Group: 627495509

欢迎提交 Issue 和 Pull Request / Welcome to submit Issues and Pull Requests
# 📄 许可证 / License
本项目采用 MIT 许可证 - 详见 LICENSE文件。

This project is licensed under the MIT License - see the LICENSEfile for details.