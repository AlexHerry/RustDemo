# Axum Web Demo

一个使用Rust和Axum框架构建的简单Web服务器示例，返回"Hello, World!"。

## 项目结构
```
axum_web_demo/
├── Cargo.toml      # 项目配置和依赖
├── src/
│   └── main.rs     # 主程序文件
└── README.md       # 项目说明文档
```

## 实现说明

### Cargo.toml
已添加以下依赖：
- `axum = "0.8.4"` - Web框架
- `tokio = { version = "1", features = ["full"] }` - 异步运行时
- `tracing` 和 `tracing-subscriber` - 日志记录组件

### main.rs
实现了一个简单的Web服务器，包含：
1. 日志配置
2. 路由设置（根路径"/"）
3. 处理函数（返回"Hello, World!"）
4. 服务器启动代码

## 如何运行

### 前提条件
- 安装Rust和Cargo（推荐使用[rustup](https://rustup.rs/)）
- 确保网络连接正常（能够访问crates.io）

### 步骤

1. 进入项目目录
```bash
cd axum_web_demo
```

2. 运行服务器
```bash
cargo run
```

3. 访问服务器
打开浏览器，访问 `http://localhost:3000/`，您将看到"Hello, World!"响应。

## 常见问题

### 网络连接问题
如果遇到类似以下的错误：
```
SSL connect error (LibreSSL SSL_connect: SSL_ERROR_SYSCALL in connection to index.crates.io:443)
```

可能是由于网络问题导致无法连接到crates.io。请尝试：
1. 检查网络连接
2. 等待一段时间后重试
3. 如果使用代理，请确保代理配置正确

## 代码解释

### 主函数
```rust
#[tokio::main]
async fn main() {
    // 设置日志记录
    // 创建路由
    // 启动服务器
}
```
- `#[tokio::main]` 属性将main函数转换为异步函数，并在运行时启动Tokio运行时
- 使用tracing库设置日志记录，方便调试和监控
- 创建Router并配置路由
- 绑定地址并启动服务器

### 处理函数
```rust
async fn hello_world() -> &'static str {
    "Hello, World!"
}
```
- 异步函数，返回静态字符串"Hello, World!"
- Axum会自动将返回值转换为HTTP响应

## 构建Linux可执行文件

项目支持构建适用于Linux系统的可执行文件，可以通过以下步骤进行操作：

### 前提条件
- 确保已安装musl-cross工具链（如示例中的`x86_64-linux-musl-gcc`）
- 配置文件`.cargo/config.toml`已设置正确的链接器路径

### 构建命令

1. 构建x86_64 Linux musl静态链接可执行文件：
```bash
cargo build --target x86_64-unknown-linux-musl --release
```

2. 构建输出位置：
   - 可执行文件将位于`target/x86_64-unknown-linux-musl/release/axum_web_demo`

## 扩展建议
- 添加更多路由和处理函数
- 实现RESTful API
- 添加中间件处理认证、日志等
- 集成数据库

希望这个示例能帮助您开始使用Axum框架构建Web应用！