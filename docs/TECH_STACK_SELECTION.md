# WxRust 技术选型（参考 ddd4j-ddd4r-依赖映射对照表）

- 参考来源：`/Users/wandl/workspaces/workspace-ddd4r/ddd4r/docs/ddd4j-ddd4r-依赖映射对照表.md`
- WxRust 架构约束来源：`docs/ARCHITECTURE.md`、`docs/PLAN.md`

## 选型原则

1. 以“语义等价 + Rust 生态成熟”为首选。
2. 避免多后端并存；优先统一抽象，减少迁移复杂度。
3. 可观测性与测试能力纳入选型硬约束。

## 核心选型

| 领域 | 选型 | crate | 状态 | 说明 |
|---|---|---|---|---|
| 异步运行时 | tokio | tokio | ✅ | async/await 基础设施 |
| HTTP 客户端 | reqwest（rustls） | reqwest | ✅ | 统一替代 apache/okhttp/jodd |
| JSON | serde + serde_json | serde, serde_json | ✅ | 编译期零成本序列化 |
| XML | quick-xml | quick-xml | ✅ | 微信支付/回调报文兼容 |
| 错误处理 | thiserror | thiserror | ✅ | Result + typed error |
| 日志与追踪 | tracing | tracing, tracing-subscriber | ✅ | 结构化日志与 span |
| 加解密/签名 | RustCrypto 家族 | aes/sha2/hmac/rsa/base64 | ✅ | 签名与加解密一致性 |
| 时间处理 | chrono | chrono | ✅ | 与微信时间语义对齐 |
| 测试与 mock | tokio-test + wiremock | tokio, wiremock | ✅ | 异步单测与 HTTP mock |
| 覆盖率 | cargo-llvm-cov | — | ✅ | 迁移测试覆盖率信号 |

## 可选/后续引入

| 领域 | 候选 | 状态 | 引入条件 |
|---|---|---|---|
| 分布式缓存 | redis crate | 🔧 | 真实 Redis 集成阶段（V5） |
| 性能基准 | criterion | 🔧 | 需要回归性能基线时 |
| 属性测试 | proptest | 🔧 | 关键算法抗变异验证 |

## 与 ddd4r 映射对照（节选）

| ddd4r 领域 | WxRust 映射 | 说明 |
|---|---|---|
| Web/HTTP client | reqwest | 统一 HTTP 客户端 |
| JSON/XML 序列化 | serde_json + quick-xml | 微信报文兼容 |
| Observability | tracing + cargo-llvm-cov | 日志/测试/覆盖率闭环 |
| Error Handling | thiserror + Result | 错误语义可迁移 |

> 本文为项目级技术选型；组件替换的详细硬约束以 `docs/ARCHITECTURE.md` 为准。
