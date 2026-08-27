# WxRust 整体迁移计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**From:** `docs/PLAN.md`
**创建日期：** 2026-07-31
**状态：** 已完成（核对日期：2026-08-27，依据：10/10 crate 已发布 crates.io v0.1.0；V0 审计 3287/3287 0 MISSING；workspace 3588 tests 全绿）

**Goal:** 基于 WxJava（4.8.4.B）创建 Rust 实现 WxRust，提供微信后端开发全场景 SDK（公众号、小程序、微信支付、企业微信、开放平台、视频号/小店、AI 语音），托管于 easy-4-rust/WxRust。

**Architecture:** 四层架构——facade（wx-rust feature 门控重导出）→ 业务层（wx-rust-mp/miniapp/pay/cp/open/channel/aispeech/qidian）→ 基础层（wx-rust-common error/config/http/bean）→ 集成层（wx-rust-vernal 对标 Spring Boot starter）。所有 crate 使用 workspace 统一版本管理，`#![forbid(unsafe_code)]`，纯 Rust + 主流生态依赖，无 FFI。

**Tech Stack:** Rust 1.85 / Edition 2024 / resolver 3、tokio（async 运行时）、reqwest（HTTP）、serde_json + quick-xml（序列化）、thiserror（错误）、RustCrypto（加解密）、tracing（日志）、chrono（时间）。

## Global Constraints

- MSRV: Rust 1.85，Edition 2024，Workspace Resolver 3。
- License: Apache-2.0。
- `unsafe` 策略：所有 crate 强制 `#![forbid(unsafe_code)]`。
- WxRust 承诺与 WxJava 的 API 语义/行为对齐，不承诺二进制兼容。
- 业务 crate 之间不互相依赖（open 复用 mp/ma 能力通过组合/trait）。
- facade 只做重导出，不含实现。
- 每个 task 采用 TDD：先写测试，再写最小实现。

---

## 1. 已确认需求与非目标

### 1.1 已确认需求

1. 移植 WxJava 全部 9 个业务模块到 Rust。
2. 提供统一 facade crate（wx-rust），通过 feature 门控重导出。
3. 定义 common 基础 trait（WxService、WxConfigStorage、RequestExecutor、WxError）。
4. 规划集成层（wx-rust-vernal 对标 Spring Boot starter）。
5. 全 async + tokio 运行时模型。
6. 统一 HTTP 客户端（reqwest），删除 Java 多后端设计。

### 1.2 非目标

- 不移植 solon-plugins（JVM 框架，无 Rust 对应）。
- 不实现具体微信 API（属后续骨架实现阶段）。
- 不保证与 Java 实现的二进制兼容。

---

## 2. 目标调用方式

### 2.1 Cargo.toml workspace

```toml
[workspace]
resolver = "3"
members = ["crates/*"]

[workspace.package]
version = "0.1.0"
edition = "2024"
rust-version = "1.85"
license = "Apache-2.0"
```

### 2.2 业务 crate 使用

```rust
use wx_rust_mp::api::WxMpService;
use wx_rust_common::error::WxErrorException;

let service = WxMpService::new(config);
let token = service.get_access_token(false).await?;
```

### 2.3 Feature 门控

```toml
[dependencies]
wx-rust = { version = "0.1", features = ["mp", "pay"] }
```

---

### Task 1: 项目目录与 workspace 配置

**Files:**
- Create: `Cargo.toml`（workspace root）
- Create: `crates/wx-rust/Cargo.toml`
- Create: `crates/wx-rust/src/lib.rs`
- Create: `crates/wx-rust-common/Cargo.toml`
- Create: `crates/wx-rust-mp/Cargo.toml`
- Create: `crates/wx-rust-miniapp/Cargo.toml`
- Create: `crates/wx-rust-pay/Cargo.toml`
- Create: `crates/wx-rust-cp/Cargo.toml`
- Create: `crates/wx-rust-open/Cargo.toml`
- Create: `crates/wx-rust-channel/Cargo.toml`
- Create: `crates/wx-rust-aispeech/Cargo.toml`
- Create: `crates/wx-rust-qidian/Cargo.toml`

- [x] **Step 1: 创建 workspace root Cargo.toml**

- [x] **Step 2: 创建各 crate Cargo.toml 与 lib.rs 占位**

- [x] **Step 3: cargo build --workspace 验证**

### Task 2: wx-rust-common 基础层实现

**Files:**
- Create/Modify: `crates/wx-rust-common/src/error/*.rs`
- Create/Modify: `crates/wx-rust-common/src/bean/*.rs`
- Create/Modify: `crates/wx-rust-common/src/config.rs`
- Create/Modify: `crates/wx-rust-common/src/http.rs`
- Create/Modify: `crates/wx-rust-common/src/session/*.rs`
- Create/Modify: `crates/wx-rust-common/src/service/*.rs`

- [x] **Step 1: 实现错误体系（WxError / WxErrorException / WxRuntimeException）**

- [x] **Step 2: 实现 Bean 类型（WxAccessToken / WxJsapiSignature）**

- [x] **Step 3: 定义 WxConfigStorage async trait**

- [x] **Step 4: 定义 WxService / RequestExecutor async trait**

- [x] **Step 5: 实现 Session 管理**

- [x] **Step 6: 编写基础测试**

### Task 3: 业务 crate 占位与骨架实现

**Files:**
- Create/Modify: `crates/wx-rust-*/src/lib.rs`
- Create/Modify: `crates/wx-rust-*/src/api/*.rs`
- Create/Modify: `crates/wx-rust-*/src/bean/*.rs`
- Create/Modify: `crates/wx-rust-*/src/config/*.rs`
- Create/Modify: `crates/wx-rust-*/src/enums/*.rs`

- [x] **Step 1: wx-rust-mp 骨架（386 src files）**

- [x] **Step 2: wx-rust-miniapp 骨架（635 src files）**

- [x] **Step 3: wx-rust-pay 骨架（589 src files）**

- [x] **Step 4: wx-rust-cp 骨架（654 src files）**

- [x] **Step 5: wx-rust-open 骨架（239 src files）**

- [x] **Step 6: wx-rust-channel 骨架（696 src files）**

- [x] **Step 7: wx-rust-aispeech 骨架（32 src files）**

- [x] **Step 8: wx-rust-qidian 骨架（31 src files）**

### Task 4: 文档与 facade

**Files:**
- Create: `README.md` / `README.zh-CN.md`
- Create: `AGENTS.md`
- Modify: `crates/wx-rust/src/lib.rs`（feature 门控重导出）

- [x] **Step 1: 编写 README**

- [x] **Step 2: 编写 AGENTS.md**

- [x] **Step 3: 实现 facade feature 门控**

---

## 3. 验收矩阵

| 验收项 | 证明方式 |
|---|---|
| workspace 编译 | `cargo build --workspace` 通过 |
| 代码规范 | `cargo fmt --check` 无 diff |
| 静态检查 | `cargo clippy --workspace` 无 error |
| 目录结构 | 与计划一致 |
| crate 数量 | 10 个 crate（含 facade） |
