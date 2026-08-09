<a id="readme-top"></a>

<div align="center">

# WxRust

**面向微信生态的 Rust 全栈 SDK，移植自 WxJava**

[![MSRV](https://img.shields.io/badge/MSRV-1.85-orange)](#3-rust-基线)
[![License](https://img.shields.io/badge/license-Apache--2.0-green)](LICENSE)

[English](./README.md) | [简体中文](./README.zh-CN.md)

[定位](#1-项目定位) · [模块](#2-模块总览) · [架构](#3-架构设计) ·
[快速开始](#5-快速开始) · [Features](#6-cargo-features) ·
[质量门禁](#8-质量门禁) · [兼容性](#9-与-wxjava-的兼容性) ·
[贡献](#12-贡献与许可证)

</div>

---

> **版本**：`0.1.0`<br>
> **MSRV**：Rust `1.85`<br>
> **Edition**：`2024`<br>
> **Workspace Resolver**：`3`<br>
> **成熟度**：实验性<br>
> **最后核验**：2026-08-10

## 1. 项目定位

### 1.1 是什么

**WxRust 是一个 Rust workspace，提供微信后端开发全场景 SDK，覆盖公众号（MP）、小程序、微信支付、企业微信（CP）、开放平台、视频号/小店和 AI 语音。**

它是 [WxJava](https://github.com/binarywang/WxJava)（v4.8.4.B）的语义移植版，目标是 100% 功能行为对齐，同时采用惯用 Rust 模式。

| 维度 | 值 |
|---|---|
| 版本 | `0.1.0` |
| MSRV / Edition | `1.85` / `2024` |
| unsafe 策略 | `#![forbid(unsafe_code)]` |
| 异步运行时 | tokio |
| HTTP 客户端 | reqwest（rustls） |
| 许可证 | `Apache-2.0` |

### 1.2 不是什么

- 不是 WxJava 的二进制兼容替代品；API 名称保留但签名使用 Rust 惯用法（`Result`、`Option`、`async fn`）。
- 不是 FFI 薄封装；是纯 Rust 重新实现。
- 尚未达到生产就绪状态；项目处于实验性/迁移阶段。

### 1.3 状态证据

| 声明 | 当前值 | 证据 |
|---|---|---|
| workspace 可编译 | 通过 | `cargo check --workspace` |
| 测试 | 全绿，0 失败 | `cargo test --workspace` |
| 行覆盖率 | 39.81% | `cargo llvm-cov --workspace` |
| Java 对象已映射 | 3287 / 3287（0 MISSING） | `docs/migration/README.md` |
| Rust 源文件数 | 3406 | `find crates -name '*.rs'` |

## 2. 模块总览

| Java 模块 | Rust crate | 对象数 | 业务范围 |
|---|---|---:|---|
| `weixin-java-common` | `wx-rust-common` | 174 | 错误模型、token 执行引擎、HTTP、会话、重复检查 |
| `weixin-java-mp` | `wx-rust-mp` | 428 | 公众号 API、消息路由、XML 加解密 |
| `weixin-java-miniapp` | `wx-rust-miniapp` | 611 | 小程序 code2Session、50+ 子服务 |
| `weixin-java-pay` | `wx-rust-pay` | 570 | 微信支付 v2/v3、证书、签名、XML |
| `weixin-java-cp` | `wx-rust-cp` | 594 | 企业微信、会话存档、OA |
| `weixin-java-open` | `wx-rust-open` | 240 | 第三方平台、代 mp/ma |
| `weixin-java-channel` | `wx-rust-channel` | 618 | 视频号/微信小店电商 |
| `weixin-java-aispeech` | `wx-rust-aispeech` | 25 | AI 语音 |
| `weixin-java-qidian` | `wx-rust-qidian` | 27 | 企点呼叫中心 |
| facade | `wx-rust` | — | Feature 门控重导出 |

## 3. 架构设计

```text
┌─────────────────────────────────────────────────────────┐
│  集成层（后续）  wx-rust-vernal / wx-rust-axum            │
├─────────────────────────────────────────────────────────┤
│  业务层  wx-rust-mp / miniapp / pay / cp / open /        │
│          channel / aispeech / qidian                     │
│          门面 Service trait + 子域 Service                │
├─────────────────────────────────────────────────────────┤
│  基础层  wx-rust-common                                  │
│          error / config / http / bean / session          │
├─────────────────────────────────────────────────────────┤
│  Facade  wx-rust（feature 门控重导出）                     │
└─────────────────────────────────────────────────────────┘
```

### 3.1 依赖规则

- `wx-rust-common` 不依赖任何业务 crate。
- 业务 crate 仅依赖 `common`；业务 crate 之间不互相依赖。
- Facade 只做重导出，不含逻辑。

### 3.2 Java→Rust 关键映射

| Java 机制 | Rust 设计 | 原因 |
|---|---|---|
| 继承链（`ServiceImpl -> HttpComponentsImpl -> Base`） | trait + 组合 | Rust 无继承 |
| `synchronized` / `ReentrantLock` | `tokio::sync::Mutex` / `RwLock` | 原生异步 |
| `Gson` | `serde` + `serde_json` | 编译期零成本 |
| `Apache HttpClient` / `OkHttp` / `Jodd` | `reqwest`（单一后端） | 简化为一个 HTTP 栈 |
| checked exception | `thiserror` + `Result` | 显式错误传播 |
| `null` | `Option<T>` | 空值在类型中可见 |
| `ExecutorService`（Router） | `tokio::task::spawn` | 异步任务调度 |

## 4. Rust 基线

| 项目 | 值 | 来源 |
|---|---|---|
| MSRV | `1.85` | `workspace.package.rust-version` |
| Edition | `2024` | `workspace.package.edition` |
| Resolver | `3` | `[workspace] resolver` |
| Clippy | `-D warnings` | CI |

## 5. 快速开始

### 5.1 添加依赖

```toml
[dependencies]
wx-rust-common = { git = "https://github.com/easy-4-rust/WxRust", branch = "main" }
```

### 5.2 最小示例

```rust
use wx_rust_common::error::WxError;

fn main() {
    let json = r#"{"errcode":0,"errmsg":"ok"}"#;
    let err = WxError::from_json(json);
    println!("error_code={}", err.error_code);
}
```

### 5.3 启用业务模块

```toml
[dependencies]
wx-rust-mp = { git = "https://github.com/easy-4-rust/WxRust", branch = "main" }
```

## 6. Cargo Features

| Feature | 默认 | 说明 | Crate |
|---|:---:|---|---|
| `default` | 是 | 核心类型与 trait | `wx-rust-common` |
| `redis` | 否 | Redis 存储（重复检查/分布式锁/配置存储） | `wx-rust-common` |

```toml
[dependencies]
wx-rust-common = { version = "0.1.0", default-features = false, features = ["redis"] }
```

## 7. 项目结构

```text
WxRust/
├── Cargo.toml              # Workspace
├── crates/
│   ├── wx-rust/            # Facade（feature 门控）
│   ├── wx-rust-common/     # 基础层
│   ├── wx-rust-mp/         # 公众号
│   ├── wx-rust-miniapp/    # 小程序
│   ├── wx-rust-pay/        # 微信支付
│   ├── wx-rust-cp/         # 企业微信
│   ├── wx-rust-open/       # 开放平台
│   ├── wx-rust-channel/    # 视频号/小店
│   ├── wx-rust-aispeech/   # AI 语音
│   └── wx-rust-qidian/     # 企点
├── docs/                   # 架构、迁移、技术选型
├── scripts/                # 审计与生成脚本
├── integration/            # 框架集成（后续）
└── README.md
```

## 8. 质量门禁

### 8.1 基础门禁

```bash
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

### 8.2 覆盖率

```bash
cargo llvm-cov --workspace --summary-only
```

| 指标 | 值 |
|---|---|
| 行覆盖率 | 39.81% |
| 函数覆盖率 | 25.67% |
| 区域覆盖率 | 38.00% |

> 覆盖率是缺口发现的信号指标，不是完成权威。完成权威是模块对象台账 + 语义表 + 名称一致性检查。

## 9. 与 WxJava 的兼容性

### 9.1 上游参考

| 项目 | 值 |
|---|---|
| 上游项目 | [WxJava](https://github.com/binarywang/WxJava) |
| 固定版本 | `4.8.4.B`（commit `a49d6e1`） |
| 权威来源 | WxJava 源码、测试、fixtures |
| Rust 目标 | 行为等价 + 惯用 Rust |
| 非目标 | ABI 兼容、JVM/字节码/平台专属类 |

### 9.2 对象迁移状态

| Java 模块 | 对象数 | 已实现 | PLATFORM_NA | DEPENDENCY_REUSED | MISSING |
|---|---:|---:|---:|---:|---:|
| common | 174 | 90 | 87 | 1 | 0 |
| mp | 428 | 284 | 44 | 39 | 0 |
| miniapp | 611 | 560 | 35 | 16 | 0 |
| pay | 570 | 556 | 12 | 2 | 0 |
| cp | 594 | 565 | 20 | 9 | 0 |
| open | 240 | 213 | 14 | 13 | 0 |
| channel | 618 | 603 | 9 | 6 | 0 |
| aispeech | 25 | 23 | 2 | 0 | 0 |
| qidian | 27 | 20 | 7 | 0 | 0 |
| **合计** | **3287** | **2914** | **230** | **86** | **0** |

### 9.3 关键语义映射

| Java 模式 | Rust 实现 |
|---|---|
| AccessToken 双重检查锁 | `async fn` + `tokio::sync::Mutex` + 3s 超时 |
| 请求执行引擎 + 重试 | `loop` + 指数退避（`1s, 2s, 4s, 8s, 16s`） |
| Token 自动刷新（单次重试） | `execute_internal` + `no_auto_refresh` 标志 |
| 消息路由（builder 模式） | `WxMpMessageRouter` + `Vec<Rule>` + async 分发 |
| ConfigStorage 多后端 | `trait WxConfigStorage` + 内存/Redis 实现 |

### 9.4 明确不迁移项

| Java 组件 | 原因 | Rust 替代 |
|---|---|---|
| Apache/OkHttp/Jodd HTTP 后端 | Java HTTP 客户端适配层 | `reqwest` 统一 |
| Gson TypeAdapter | Gson 专属 | `serde` derive |
| Native-image 配置 | GraalVM 专属 | Rust 原生二进制 |
| Solon 插件 | JVM 框架 | 无 Rust 对应 |

## 10. 文档

| 文档 | 路径 | 用途 |
|---|---|---|
| 迁移路线图 | `docs/MIGRATION_ROADMAP.md` | B0-V6 阶段规划 |
| 对象级对照表 | `docs/OBJECT_MAPPING_TABLE.md` | 3287 对象索引 |
| 语义迁移对照表 | `docs/SEMANTIC_MAPPING_TABLE.md` | 核心调用链/错误/序列化 |
| 名称一致性检查 | `docs/NAME_CONSISTENCY_CHECK.md` | Java→Rust 命名一致性 |
| 技术选型 | `docs/TECH_STACK_SELECTION.md` | 依赖选型决策 |
| 架构设计 | `docs/ARCHITECTURE.md` | 组件替换决策 |
| 模块迁移文档 | `docs/migration/<module>/` | 每模块 4 件套 |

## 11. 故障排查

| 症状 | 常见原因 | 处理 |
|---|---|---|
| `cargo check` 失败 | MSRV 过低 | 使用 Rust >= 1.85 |
| Feature 编译错误 | 可选依赖缺失 | 启用对应 feature |
| 测试超时 | 异步运行时冲突 | 确保 `tokio` features 匹配 |

## 12. 贡献与许可证

欢迎贡献。提交前请运行：

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
```

本项目采用 [Apache-2.0](LICENSE) 许可证。

上游来源：[WxJava](https://github.com/binarywang/WxJava)（Apache-2.0）。

---

<div align="center">

[返回顶部](#readme-top) · [Issues](https://github.com/easy-4-rust/WxRust/issues)

</div>
