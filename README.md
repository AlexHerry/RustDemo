# Rust 学习项目集合

这是我的个人Rust学习项目仓库，用于存放和管理我在学习Rust过程中编写的各种小项目。通过这些项目，我将逐步掌握Rust的核心概念、语法特性和最佳实践。

## 项目结构

```
├── axum_web_demo/   # Web服务器项目（已完成）
│   ├── .cargo/
│   │   └── config.toml
│   ├── .gitignore
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── README.md
│   └── src/
│       └── main.rs
├── calculator_cli/  # 命令行计算器项目（已完成）
│   ├── .gitignore
│   ├── Cargo.lock
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
└── jsonfmt/         # JSON格式化工具项目（开发中）
    ├── Cargo.toml
    └── src/
        └── main.rs
```

## 项目列表

### 1. axum_web_demo - Web服务器项目

一个基于axum框架的简单Web服务器项目，展示了如何使用Rust构建HTTP服务。

#### 功能特性
- 基本的HTTP路由处理
- 响应"Hello, World!"请求
- 日志记录和追踪功能
- 支持构建Linux可执行文件

#### 使用方法

```bash
# 构建项目
cd axum_web_demo
cargo build --release

# 运行服务器
cargo run

# 访问服务
curl http://localhost:3000
# 输出: Hello, World!
```

#### 构建Linux可执行文件

```bash
# 确保已安装musl-cross工具链
# 构建Linux可执行文件
cd axum_web_demo
cargo build --target x86_64-unknown-linux-musl --release

# 可执行文件位置
# target/x86_64-unknown-linux-musl/release/axum_web_demo
```

#### 实现细节
- 使用axum 0.8.4作为Web框架
- 使用hyper 1.0作为HTTP库
- 使用tracing和tracing-subscriber进行日志记录
- 配置了交叉编译支持

### 2. calculator_cli - 命令行计算器

一个简单的命令行计算器，可以执行基本的算术运算。

#### 功能特性
- 支持加（+）、减（-）、乘（*）、除（/）四种基本运算
- 参数验证和错误处理
- 友好的使用说明
- 防止除零错误

#### 使用方法

```bash
# 构建项目
cd calculator_cli
cargo build --release

# 运行计算器
./target/release/calculator_cli <num1> <op> <num2>

# 示例
./target/release/calculator_cli 10 + 5
# 输出: 10 + 5 = 15

# 查看帮助
./target/release/calculator_cli --help
```

#### 实现细节
- 使用`std::env::args()`获取命令行参数
- 通过模式匹配实现不同的运算操作
- 使用结构体和方法组织代码
- 包含完整的错误处理逻辑

### 3. jsonfmt - JSON格式化工具（开发中）

一个用于格式化JSON文件的命令行工具，目前正在开发中。

#### 计划功能
- 从文件或标准输入读取JSON数据
- 格式化JSON输出（支持缩进、换行等）
- 支持自定义缩进空格数
- 错误处理和JSON验证

#### 开发状态
目前项目仅包含基础框架，计划逐步实现上述功能。

## 如何构建和运行

每个子项目都是独立的Rust项目，可以单独构建和运行：

```bash
# 构建特定项目
cd <项目目录>
cargo build

# 运行项目
cargo run [参数...]

# 构建优化版本
cargo build --release
```

## 学习目标

通过这些项目，我希望能够掌握：
- Rust的基本语法和所有权模型
- 命令行程序开发
- 错误处理模式
- 文件I/O操作
- 数据结构和算法的Rust实现

## 未来计划

1. 完成jsonfmt项目的开发
2. 添加新的Rust项目，包括但不限于：
   - 文件操作工具
   - 数据结构和算法的实现
   - 数据库交互应用
   - 异步编程示例

## 关于作者

这是我个人的Rust学习仓库，用于记录和展示我的学习进度。如果你对这些项目有任何建议或问题，欢迎提出。