# Rust 学习项目集合

这是我的个人Rust学习项目仓库，用于存放和管理我在学习Rust过程中编写的各种小项目。通过这些项目，我将逐步掌握Rust的核心概念、语法特性和最佳实践。

## 项目结构

```
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

### 1. calculator_cli - 命令行计算器

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

### 2. jsonfmt - JSON格式化工具（开发中）

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
   - 简单的Web服务
   - 数据结构和算法的实现

## 关于作者

这是我个人的Rust学习仓库，用于记录和展示我的学习进度。如果你对这些项目有任何建议或问题，欢迎提出。