# WxRust 技术栈选型设计

日期：2026-08-10
状态：已完成
来源：`docs/TECH_STACK_SELECTION.md`

## 1. 背景

WxRust 需要在 Rust 生态中选择与 WxJava 各组件语义等价的替代品。选型原则：语义等价 + Rust 生态成熟 + 避免多后端并存 + 可观测性与测试能力纳入硬约束。参考 `ddd4j-ddd4r-依赖映射对照表` 进行跨项目选型对齐。

## 2. 目标与非目标

### 2.1 目标

- 锁定核心技术栈（10 项）。
- 明确可选/后续引入候选（3 项）。
- 与 ddd4r 项目选型对齐。

### 2.2 非目标

- 不重复 ARCHITECTURE.md 已锁定的组件替换细节。
- 不涉及具体实现代码。

## 3. 方案比较

### 3.1 HTTP 客户端：reqwest vs ureq vs hyper

- `ureq`：同步阻塞，不满足 async 需求。
- `hyper`：底层 HTTP 库，需大量封装。
- `reqwest`：高层 async HTTP 客户端，内置 rustls/json/streaming/proxy，生态最成熟。

本方案选择 reqwest。

### 3.2 序列化：serde vs nanoserde vs manual

- `nanoserde`：no_std 友好，但功能有限，不支持 rename_all 等微信报文需求。
- `manual`：手写序列化，工作量巨大且易出错。
- `serde`：编译期零成本，derive 宏自动处理字段映射，生态最成熟。

本方案选择 serde + serde_json + quick-xml。

### 3.3 错误处理：thiserror vs anyhow vs 手写

- `anyhow`：运行时动态错误，适合应用层，不适合库（需 typed error）。
- `手写`：实现 Display/Error/From，样板代码多。
- `thiserror`：derive 宏自动生成 Display/Error/From，库级 typed error 最佳选择。

本方案选择 thiserror。

## 4. 模块与依赖

### 4.1 核心选型

| 领域 | 选型 | crate | 状态 | 说明 |
|---|---|---|---|---|
| 异步运行时 | tokio | tokio | 已选定 | async/await 基础设施 |
| HTTP 客户端 | reqwest（rustls） | reqwest | 已选定 | 统一替代 apache/okhttp/jodd |
| JSON | serde + serde_json | serde, serde_json | 已选定 | 编译期零成本序列化 |
| XML | quick-xml | quick-xml | 已选定 | 微信支付/回调报文兼容 |
| 错误处理 | thiserror | thiserror | 已选定 | Result + typed error |
| 日志与追踪 | tracing | tracing, tracing-subscriber | 已选定 | 结构化日志与 span |
| 加解密/签名 | RustCrypto 家族 | aes/sha2/hmac/rsa/base64 | 已选定 | 签名与加解密一致性 |
| 时间处理 | chrono | chrono | 已选定 | 与微信时间语义对齐 |
| 测试与 mock | tokio-test + wiremock | tokio, wiremock | 已选定 | 异步单测与 HTTP mock |
| 覆盖率 | cargo-llvm-cov | — | 已选定 | 迁移测试覆盖率信号 |

### 4.2 可选/后续引入

| 领域 | 候选 | 状态 | 引入条件 |
|---|---|---|---|
| 分布式缓存 | redis crate | 候选 | 真实 Redis 集成阶段（V5） |
| 性能基准 | criterion | 候选 | 需要回归性能基线时 |
| 属性测试 | proptest | 候选 | 关键算法抗变异验证 |

### 4.3 与 ddd4r 映射对照

| ddd4r 领域 | WxRust 映射 | 说明 |
|---|---|---|
| Web/HTTP client | reqwest | 统一 HTTP 客户端 |
| JSON/XML 序列化 | serde_json + quick-xml | 微信报文兼容 |
| Observability | tracing + cargo-llvm-cov | 日志/测试/覆盖率闭环 |
| Error Handling | thiserror + Result | 错误语义可迁移 |

## 5. 验收标准

- 10 项核心选型均有明确 crate 与版本。
- 3 项候选均有引入条件说明。
- 与 ddd4r 映射对照表完整。
