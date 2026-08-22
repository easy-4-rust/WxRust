# WxRust 整体架构设计

日期：2026-08-01
状态：LOCKED（B1 冻结，变更需评审）
来源：`docs/ARCHITECTURE.md`

## 1. 背景

WxJava（4.8.4.B）是微信后端开发全场景 Java SDK，包含 9 个业务模块（mp/miniapp/pay/cp/open/channel/aispeech/qidian）和 1 个公共基础模块（common），main 合计 3288 文件、36010 javap 公共方法。WxRust 是其 Rust 移植版，需在保持 API 语义/行为对齐的前提下，用纯 Rust 生态重新实现。

Java 侧存在 7 大设计模式（门面 Service + 子域 Service、三层继承链、泛型 RequestHttp、Token 双重检查锁、请求执行引擎 + 指数退避重试、RequestExecutor 策略、消息路由），这些模式在 Rust 中需要重新映射。

## 2. 目标与非目标

### 2.1 目标

- 建立四层架构：facade（wx-rust）→ 业务层（9 个 crate）→ 基础层（wx-rust-common）→ 集成层（wx-rust-vernal）。
- 锁定全部组件替换决策（HTTP/JSON/XML/加解密/日志/时间/集合/错误/异步）。
- 定义 Java→Rust 机制映射（继承链消解、Token 双检锁、请求执行引擎、错误体系、消息路由、ConfigStorage 多租户）。
- 定义命名规则与注释迁移规范。
- 定义 B2 实现批次（9 批次，按模块为最小单元）。

### 2.2 非目标

- 不实现具体业务代码（属 B2 批次）。
- 不修改已锁定的组件替换决策（除非经评审）。
- 不移植 solon-plugins（JVM 框架，无 Rust 对应）。

## 3. 方案比较

### 3.1 保留 Java 多 HTTP 后端设计

Java 侧有 Apache HttpClient / OkHttp / Jodd HTTP 三种后端，通过泛型 `RequestHttp<H,P>` 和 `HttpClientType` 枚举抽象。Rust 侧若保留多后端，需为每种后端实现 trait object，增加维护成本且无实际收益（Rust 生态中 reqwest 已是事实标准）。

本方案不采用。

### 3.2 统一 reqwest 单一后端

reqwest 支持 rustls（无 OpenSSL 依赖）、HTTP/2、streaming、代理，满足微信 SDK 全部 HTTP 需求。Java 的 `HttpClientType` 枚举和三层继承链（ServiceImpl → HttpComponentsImpl → BaseImpl）在 Rust 中坍缩为单一实现结构体。

本方案作为最终方案。

### 3.3 保留 Gson 手写 TypeAdapter 模式

Java 侧各模块有独立的 `*GsonBuilder.create`（连接度最高：WxCpGsonBuilder 774 度），是架构瓶颈。Rust 若保留手写适配器，需为每个 Bean 实现序列化/反序列化，工作量巨大且易出错。

本方案不采用。

### 3.4 使用 serde 派生宏替代

serde 提供编译期零成本序列化，`#[derive(Serialize, Deserialize)]` 自动处理字段映射。微信报文的驼峰/下划线/可选字段通过 `#[serde(rename_all)]` 和 `#[serde(default)]` 表达。

本方案作为最终方案。

## 4. 模块与依赖

### 4.1 总体架构

```
┌──────────────────────────────────────────────────────────┐
│  集成层（后续）  wx-rust-vernal（对标 Spring Boot starter） │
├──────────────────────────────────────────────────────────┤
│  业务层  wx-rust-mp / miniapp / pay / cp / open / channel  │
│          / aispeech / qidian                              │
│          门面 Service(trait) + 子域 Service + bean         │
├──────────────────────────────────────────────────────────┤
│  基础层  wx-rust-common                                   │
│          error / config / http / bean / session / util    │
├──────────────────────────────────────────────────────────┤
│  facade  wx-rust（feature 门控重导出）                     │
└──────────────────────────────────────────────────────────┘
```

### 4.2 依赖方向

```text
wx-rust（facade）→ wx-rust-{mp,miniapp,pay,cp,open,channel,aispeech,qidian}
wx-rust-{mp,miniapp,...} → wx-rust-common
wx-rust-common → tokio / reqwest / serde / thiserror / tracing / chrono / RustCrypto
```

业务 crate 之间不互相依赖。open 复用 mp/ma 能力通过组合/trait，不建依赖。

### 4.3 已锁定的组件替换

| Java 职责 | Rust 组件 | 版本基线 | 状态 |
|---|---|---|---|
| HTTP（Apache/OkHttp/Jodd） | `reqwest`（rustls） | 0.13.x | LOCKED |
| JSON（Gson） | `serde` + `serde_json` | 1.0.x | LOCKED |
| XML（支付/消息） | `quick-xml` + `serde` | 0.41.x | LOCKED |
| 加解密/签名 | RustCrypto 家族 | 锁定版本 | LOCKED |
| 日志（SLF4J） | `tracing` | 0.1.x | LOCKED |
| 时间（Joda-time） | `chrono` | 0.4.x | LOCKED |
| 集合（Guava） | std + `DashMap` | — | LOCKED |
| Redis（Jedis/Redisson） | `redis` crate | 0.29.x | CANDIDATE |
| 错误 | `thiserror` | 2.0.x | LOCKED |
| 异步 | `tokio` + `async-trait` | 1.52.x / 0.1.x | LOCKED |

### 4.4 Java→Rust 机制映射

| Java 模式 | Rust 映射 |
|---|---|
| 三层继承链（ServiceImpl → HttpComponentsImpl → BaseImpl） | 单一实现结构体（reqwest 统一） |
| Token 双重检查锁 + ReentrantLock | `tokio::sync::Mutex` + timeout + 双检 |
| 请求执行引擎 + 指数退避重试 | async loop（不用递归）+ `tokio::time::sleep` |
| RequestExecutor<T,E> 策略 | `#[async_trait] trait RequestExecutor` |
| WxErrorException checked 异常 | `#[derive(Error)] enum WxErrorException` |
| WxMpMessageRouter builder | struct + `rule().msg_type().handler().end()` |
| ConfigStorage 多租户 | `trait WxConfigStorage` + `DashMap<String, Arc<dyn WxConfigStorage>>` |
| GsonBuilder 手写适配器 | serde 派生宏（零手写） |

### 4.5 命名规则

| Java | Rust 规则 | 示例 |
|---|---|---|
| 类/接口/枚举 | PascalCase 类型 + snake_case 文件 | `WxMpService` → `wx_mp_service.rs` |
| getter | `name()`（去 get_ 前缀） | `getAccessToken()` → `access_token()` |
| 布尔 | 语义谓词 | `isAccessTokenExpired()` → `is_access_token_expired()` |
| 重载 | canonical + `_with_*` 后缀 | `fromJson(String)` → `from_json()` |
| 禁止 | `get_*` 机械镜像；`Deref` 模拟继承 | — |

### 4.6 B2 实现批次

| 批次 | 模块 | 对象数 | 方法数 |
|---|---|---|---|
| 1 | wx-rust-common | 174 | 958 |
| 2 | wx-rust-mp | 428 | 3748 |
| 3 | wx-rust-miniapp | 611 | 4942 |
| 4 | wx-rust-pay | 570 | 6788 |
| 5 | wx-rust-cp | 594 | 6099 |
| 6 | wx-rust-open | 240 | 2077 |
| 7 | wx-rust-channel | 618 | 4308 |
| 8 | wx-rust-aispeech | 25 | 256 |
| 9 | wx-rust-qidian | 27 | 285 |

## 5. 并发与生命周期契约

| Java | Rust | 注意 |
|---|---|---|
| `ReentrantLock`（token 锁） | `tokio::sync::Mutex<()>` | 超时抢锁语义（tryLock 100ms x 30） |
| `synchronized` | `tokio::sync::Mutex` / `RwLock` | 锁范围最小化，无跨 await 持锁 |
| `ExecutorService`（Router） | `tokio::task::spawn` | JoinHandle 管理 |
| `ThreadLocal`（ConfigStorageHolder） | 显式参数传递 / `tokio::task_local!` | Java ThreadLocal 在 async 中不可用 |
| 资源释放 | RAII（Drop） | reqwest::Client、文件句柄自动释放 |

全 workspace `#![forbid(unsafe_code)]`。

## 6. 验收标准

- 架构决策文档 LOCKED 状态。
- 9 个 B2 批次定义完整（含对象数与方法数）。
- 组件替换表 11 项均有 LOCKED/CANDIDATE 状态。
- 机制映射表 8 项均有明确 Rust 对应。
- 命名规则表可执行（附示例路径映射）。
