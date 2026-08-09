# WxRust 项目创建计划

> 基于 [WxJava](https://github.com/binarywang/WxJava)（当前版本 **4.8.4.B**）创建 Rust 实现 **WxRust**，
> 托管于 [easy-4-rust/WxRust](https://github.com/easy-4-rust)。
> 本文档定义项目结构、技术决策、模块映射、交付范围与后续路线。

- 创建日期：2026-07-31
- 当前阶段：**规划与源码分析完成**（骨架与代码实现见后续阶段；WxJava 深度分析见 `WXJAVA_ANALYSIS.md`）
- 源码参考：`/Users/wandl/workspaces/workspace-github/WxJava`（4.8.4.B）
- 目标路径：`/Users/wandl/workspaces/workspace-github-easy-4-rust/WxRust/`

---

## 1. 项目定位

`WxRust` 是 WxJava 的 Rust 移植版，提供微信后端开发全场景 SDK：
公众号（MP）、小程序（MiniApp）、微信支付（Pay）、企业微信（CP）、开放平台（Open）、视频号/微信小店（Channel）、AI 语音。

- 仓库：`easy-4-rust/WxRust`
- 版本：`0.1.0`
- MSRV / Edition：Rust `1.85` / `2024`
- Workspace Resolver：`3`
- License：Apache-2.0
- `unsafe` 策略：所有 crate 强制 `#![forbid(unsafe_code)]`
- 成熟度：Experimental（骨架阶段）

> WxRust 承诺与 WxJava 的 **API 语义/行为对齐**，不承诺与 Java 实现的二进制兼容。
> 全部使用纯 Rust + 主流生态依赖，无 FFI，无 `unsafe`。

---

## 2. 已确认的架构决策

| 维度 | 选择 | 说明 |
|---|---|---|
| **项目目录** | `WxRust` | 仓库根目录名，与 GitHub 仓库名一致 |
| **crate 命名** | `wx-rust-*`（根 crate `wx-rust`） | 与 `wx-java-*` 命名习惯对应 |
| **运行时模型** | 全 async + tokio | Service 方法返回 `Future`；WxJava 同步语义用 async 自然映射 |
| **HTTP** | `reqwest`（rustls，禁 default tls） | 与 hutool-rust / sa-token-rust 依赖基线一致 |
| **JSON** | `serde_json`（preserve_order） | 对应 WxJava 的 gson |
| **XML** | `quick-xml` | 微信大量使用 XML 报文（支付、回调） |
| **加解密/签名** | `aes`、`rsa`、`sha2`、`hmac`、`base64` | 消息加解密、支付签名、access_token 等 |
| **并发原语** | `tokio::sync::{RwLock, Mutex}` + `Arc` | 替代 Java 的 `Lock` / `synchronized` |
| **错误** | `thiserror` | 替代 Java checked `WxErrorException` |
| **Trait 异步** | `async-trait` | async fn in trait（兼容 MSRV 1.85） |

---

## 3. 模块映射（WxJava → WxRust）

| Java 模块 | Rust crate | Java 文件数 | 业务范围 | 本次交付 |
|---|---|---|---|---|
| weixin-java-common | `wx-rust-common` | 174 | 基础设施：error/config 抽象/bean/http 抽象 trait/redis 锁 | 后续：基础 trait 实现 |
| weixin-java-mp | `wx-rust-mp` | 428 | 公众号 | 占位 |
| weixin-java-miniapp | `wx-rust-miniapp` | 611 | 小程序 | 占位 |
| weixin-java-pay | `wx-rust-pay` | 570 | 微信支付（含 v3） | 占位 |
| weixin-java-cp | `wx-rust-cp` | 595 | 企业微信 | 占位 |
| weixin-java-open | `wx-rust-open` | 240 | 开放平台（第三方平台） | 占位 |
| weixin-java-channel | `wx-rust-channel` | 618 | 视频号 / 微信小店 | 占位 |
| weixin-java-aispeech | `wx-rust-aispeech` | 25 | AI 语音 | 占位 |
| weixin-java-qidian | （暂不移植，27 文件） | 27 | 企点（已并入企业微信） | — |
| wx-java-bom | （workspace 统一版本管理） | — | — | 不建独立 crate |
| spring-boot-starters | `integration/wx-rust-vernal` (+ `wx-rust-vernal-multi`) | 148 | 框架集成：配置绑定 + Service Bean 装配（对标 Spring Boot starter） | 见 5.6 节 |
| solon-plugins | （不移植） | 100 | Solon 框架插件 | 不建（JVM 框架，无 Rust 对应） |
| —（新增） | `integration/wx-rust-axum` / `wx-rust-actix` | — | 纯 web 框架轻集成（无 IoC 场景） | 见 5.6 节 |
| —（新增） | `wx-rust` | — | feature 门控 facade | facade 重导出 |

> **文件数统计来源**：WxJava 源码 `find <module>/src/main/java -name '*.java' | wc -l`，
> main 合计 3288、test 合计 379；集成层 spring-boot-starters 148 main + solon-plugins 100 main。

---

## 4. 目录结构

```
WxRust/
├── Cargo.toml              # workspace（resolver=3）+ workspace.package/deps/lints
├── Cargo.lock              # 生成
├── README.md               # 英文（对齐 hutool-rust 模板）
├── README.zh-CN.md         # 中文
├── AGENTS.md               # review 规范（移植自 WxJava + Rust 适配）
├── LICENSE                 # Apache-2.0
├── .gitignore
├── docs/
│   ├── PLAN.md             # 本文档
│   ├── ARCHITECTURE.md     # 架构与 Java→Rust 概念映射
│   └── MIGRATION_STATUS.md # 模块移植进度跟踪
└── crates/
    ├── wx-rust/            # facade：feature 门控重导出
    ├── wx-rust-common/     # 基础 trait：error/config/http/bean
    ├── wx-rust-mp/         # 公众号
    ├── wx-rust-miniapp/    # 小程序
    ├── wx-rust-pay/        # 微信支付
    ├── wx-rust-cp/         # 企业微信
    ├── wx-rust-open/       # 开放平台
    ├── wx-rust-channel/    # 视频号 / 小店
    └── wx-rust-aispeech/   # AI 语音
└── integration/            # 框架集成层（对应 spring-boot-starters / solon-plugins）
    ├── wx-rust-vernal/              # Vernal 桥（主推，对标 Spring Boot）
    ├── wx-rust-vernal-multi/        # Vernal 多租户（multi）桥
    ├── wx-rust-axum/                # Axum 直接集成（无 IoC 场景）
    └── wx-rust-actix/               # Actix-web 直接集成（可选）
```

> **集成层说明**：WxJava 的 `spring-boot-starters`（148 文件，单租户 + `multi` 多租户变体）
> 与 `solon-plugins`（100 文件）是配置绑定 + Service Bean 装配层。
> Rust 侧的主推底座是同 workspace 的 [vernal-framework](https://github.com/easy-4-rust/vernal)
> （自研 IoC/AOP/应用上下文，对标 Spring，已提供 `ConfigurationProperties` 派生、条件装配、
> application builder，且 sa-token-rust 已有 `sa-token-vernal` 范本）。
> 因此集成层**首期规划 `wx-rust-vernal`（+ multi）**，对标 spring-boot-starter；
> 对无 IoC 的纯 web 框架另提供 `wx-rust-axum` 等轻集成。
> Rust 侧不移植 solon（Solon 是 JVM 框架，无 Rust 对应）。

---

## 5. 详细实现清单（后续阶段）

### 5.1 Workspace 根 `Cargo.toml`

- `[workspace] resolver = "3"`，`members = ["crates/*"]`
- `[workspace.package]`：
  `version = "0.1.0"`, `edition = "2024"`, `rust-version = "1.85"`,
  `license = "Apache-2.0"`, `authors = ["wandl <hiwepy@gmail.com>"]`,
  `repository = "https://github.com/easy-4-rust/WxRust"`,
  `homepage = "https://github.com/easy-4-rust/WxRust"`, `readme = "README.md"`
- `[workspace.dependencies]`（版本与 hutool-rust 对齐）：
  - `async-trait = "0.1"`
  - `tokio = { version = "1.52", features = ["sync", "macros", "rt", "time"] }`
  - `reqwest = { version = "0.13", default-features = false, features = ["json", "rustls", "stream"] }`
  - `serde = { version = "1.0", features = ["derive"] }`
  - `serde_json = { version = "1.0", features = ["preserve_order"] }`
  - `quick-xml = "0.41"`
  - `thiserror = "2.0"`
  - `tracing = "0.1"`
  - `chrono = { version = "0.4", default-features = false, features = ["clock", "std"] }`
  - `url = "2.5"`
  - `base64 = "0.23"`
  - `aes = "0.9"`、`sha2 = "0.11"`、`hmac = "0.13"`、`rsa = "=0.9.10"`
  - `hex = "0.4"`、`percent-encoding = "2.3"`
- `[workspace.lints.rust]`：`unsafe_code = "forbid"`
- `[workspace.lints.clippy]`：`all = "warn"`，`pedantic`/`nursery` 可选

### 5.2 `wx-rust-common`（首个有实质实现的 crate）

移植 WxJava 的抽象骨架，定义 trait/类型，具体逻辑后补：

| 文件 | 内容 | 对应 Java |
|---|---|---|
| `src/error.rs` | `WxError { error_code, error_msg, error_msg_en, json }`；`WxErrorException`（thiserror）；`WxType` 枚举（MP/CP/PAY/MA/OPEN/CHANNEL）；`WxError::from_json/from_response` | `common.error.*` |
| `src/bean/mod.rs` | `WxAccessToken { access_token, expires_in }`、`WxJsapiSignature`、`CommonUploadParam` | `common.bean.*` |
| `src/config.rs` | `WxConfigStorage` async trait：`get_access_token/is_access_token_expired/expire_access_token/update_access_token/get_ticket/...` | `mp.config.WxMpConfigStorage`（公共部分上提） |
| `src/http.rs` | `WxHttpClient` async trait：`get/post/post_json/upload/download`，通用 escape-hatch 接口 | `common.service.WxService` |
| `src/lib.rs` | 模块组织 + `#![forbid(unsafe_code)]` + crate 文档 | — |

> 并发模型：`WxConfigStorage` 的 `getAccessTokenLock()` 用 `Arc<tokio::sync::RwLock<()>>` 表达，
> Java 中“线程安全 + 只刷新一次”语义由 async trait + 锁在实现层保证。

### 5.3 业务 crate（lib.rs 占位）

每个 crate 统一模板：

- `Cargo.toml`：`[package]` 用 workspace 继承；依赖 `wx-rust-common`（workspace path）+ 相关 workspace deps；`[lints] workspace = true`
- `src/lib.rs`：
  - `#![forbid(unsafe_code)]`
  - crate 文档注释：说明对应 Java 模块、源码路径、规划实现的 Service 列表
  - 模块占位（`api`、`bean`、`config`、`enums`），子模块 `mod x; // TODO` 占位
  - 列出待实现的 Service trait 名，全部 `// TODO` 占位

#### 待实现 Service 清单（参考 WxJava）

- **wx-rust-mp**：`WxMpService` + Kefu / Menu / User / UserTag / Material / MassMessage / TemplateMsg / Qrcode / DataCube / Card / Store / SubscribeMsg / Shake / Wifi / Comment / Device / Draft / FreePublish / Guide* / Marketing / MerchantInvoice / ReimburseInvoice / AiOpen / MemberCard
- **wx-rust-miniapp**：`WxMaService` + User / Msg / Media / Kefu / Analysis / Cloud / Code / Express / Link / Scheme / Live* / Marketing / Plugin / Product* / Promotion / Qrcode* / Run / Security / Setting / Share / Shop* / Subscribe / Vod / XPay / OpenApi / DeviceSubscribe / EmployeeRelation / OrderManagement / OrderShipping / ImmediateDelivery / Intracity / Internet / Jsapi / Complaint / Face / ReimburseInvoice / QrcodeJump / CustomserviceWork
- **wx-rust-pay**：`WxPayService`（v2 + v3）+ PayScore / PartnerPayScore / ProfitSharing / Redpack / Transfer / MerchantTransfer / BrandMerchantTransfer / EntPay / Ecommerce / Complaint / MarketingFavor / MarketingBusiFavor / MarketingMedia / MerchantMedia / MerchantLimitation / Payroll / RealName / SubscriptionBilling / BusinessCircle / BusinessOperationTransfer / Bank / CustomDeclaration / MiPay / WxDeposit / WxEntrustPap / Apply4SubjectConfirm / Applyment4Sub / PartnerTransfer / PartnerPayScoreSignPlan
- **wx-rust-cp**：`WxCpService` + Agent / AgentWorkBench / Chat / CorpGroup / Department / Export / ExternalContact / GroupRobot / Hr / IntelligentRobot / Kf / Living / Media / Meeting / Menu / Message / MsgAudit / OAuth2 / Oa*(Agent/Calendar/Mail/MeetingRoom/Schedule/Service/WeDoc/WeDrive) / School*(Health/User/Service) / Tag / TaskCard / User
- **wx-rust-open**：`WxOpenService` + 第三方平台、代公众号/小程序管理
- **wx-rust-channel**：`WxChannelService`（视频号/小店）
- **wx-rust-aispeech**：`WxAiSpeechService`（AI 语音）

### 5.4 `wx-rust` facade

- `Cargo.toml` `[features]`：`default = []`；`mp = ["dep:wx-rust-mp"]`，`pay`、`miniapp`、`cp`、`open`、`channel`、`aispeech` 同理
- `src/lib.rs`：`#![forbid(unsafe_code)]` + 按 feature 重导出子 crate（参考 hutool 根 crate 写法）

### 5.5 文档

| 文件 | 内容 |
|---|---|
| `README.md` / `README.zh-CN.md` | 对齐 hutool-rust 模板：badges、项目定位、模块表（业务场景→crate）、Java→Rust 映射表、快速开始占位、Cargo feature、质量门、贡献指南；中英对照 |
| `AGENTS.md` | 移植 WxJava review 规范，Rust 适配：空指针→Rust 编译期规避、并发（`Arc`/`Send`/`Sync`）、资源释放（`Drop`）、兼容性；**中文回复 review** |
| `docs/ARCHITECTURE.md` | workspace 结构图、模块映射、async 模型说明、Java→Rust 概念映射（Interface→`async trait`、Lombok→派生宏、ConfigStorage→`Arc<RwLock>`、`Lock`→`Mutex`/`RwLock`、checked exception→`Result`/`thiserror`） |
| `docs/MIGRATION_STATUS.md` | 模块进度表（当前全 0%），标注 Java 文件数与建议优先级 |

### 5.6 框架集成层（对应 spring-boot-starters / solon-plugins）

WxJava 的 starter/plugin 做三件事：**①配置绑定**（`@ConfigurationProperties("wx.mp")`）
→ **②按存储枚举装配 ConfigStorage**（Memory / Jedis / Redisson / RedisTemplate）
→ **③按 HTTP 枚举装配 Service 并注入 config**，另有 `multi` 变体支持多租户。
Rust 侧必须规划对应物，不能留白。分两类：

#### 5.6.1 主推：`wx-rust-vernal`（对标 spring-boot-starter，单租户）

底座用同 workspace 的 **vernal-framework**（自研 IoC/AOP/应用上下文，对标 Spring），
范本参考 `sa-token-rust/sa-token-vernal`。

- `integration/wx-rust-vernal/Cargo.toml`：
  依赖 `wx-rust-{mp,pay,miniapp,cp,open,channel,aispeech}`（feature 门控）
  + `vernal-context` / `vernal-aop` / `vernal-ioc` / `vernal-macros`
  + `serde` / `figment`(或 vernal 配置源)
- `src/properties.rs`：每个业务一份 `WxMpProperties` 等，用 vernal 的
  `#[derive(ConfigurationProperties)]`（`prefix = "wx.mp"`）对标 Java `@ConfigurationProperties`
  - 字段：`app_id / secret / token / aes_key / use_stable_access_token / hosts / config_storage{type, key_prefix, redis, http_proxy_*, timeouts}`
- `src/storage/`：按 `StorageType` 枚举（`Memory`/`Redis`）条件装配 ConfigStorage
  - 对标 Java 的 `*InMemoryConfigStorageConfiguration` / `*InJedisConfigStorageConfiguration`
  - 用 vernal 条件装配（`conditional_component_module`）替代 `@ConditionalOnXxx`
  - **简化**：Rust 用 `moka`/`dashmap` 内存版 + `redis` crate Redis 版，删除 Jedis/Redisson/RedisTemplate 三选一
- `src/service.rs`：注册 `WxMpService` 等为 Vernal 组件，注入 ConfigStorage
  - 对标 Java `WxMpServiceAutoConfiguration` 的 Bean 方法
  - **简化**：删除 `HttpClientType` 多后端选择（reqwest 统一），只剩选 ConfigStorage
- `src/lib.rs`：`#[Component]` 装配入口 + `#![forbid(unsafe_code)]`

#### 5.6.2 `wx-rust-vernal-multi`（对标 `*-multi-spring-boot-starter`，多租户）

- 维护 `Map<tenantId, WxMpService>` 注册表，对标 Java `WxMpMultiServices`（get/remove/switchover）
- `WxMpMultiProperties` 支持多组配置（`Map<String, WxMpProperties>`）
- 参考 WxJava `spring-boot-starters/wx-java-mp-multi-spring-boot-ster/`（结构与单租户基本一致 + 容器管理）

#### 5.6.3 轻集成：`wx-rust-axum` / `wx-rust-actix`（无 IoC 场景，可选）

针对不用 Vernal、直接用 web 框架的用户，提供**纯函数式构造助手**：
- `WxMpService::from_config(config)` 构造函数（读环境变量 / toml）
- web 回调处理（消息接收、验签）的 axum/actix handler 封装
- 不做 IoC，对标"无 Spring 直接 new Service"的用法

#### 5.6.4 不移植：solon-plugins

Solon 是 JVM 微框架，Rust 无对应；其能力由 `wx-rust-vernal` + `wx-rust-axum` 覆盖。

#### 集成层 Java→Rust 对照

| Java（spring-boot-starter） | Rust（wx-rust-vernal） |
|---|---|
| `@ConfigurationProperties("wx.mp")` | `#[derive(ConfigurationProperties)] prefix="wx.mp"` |
| `@Configuration` + `@Bean` | `#[Component]` + Vernal 组件注册 |
| `@ConditionalOnMissingBean` / `@ConditionalOnProperty` | vernal 条件装配模块 |
| `StorageType` 枚举（Memory/Jedis/Redisson/RedisTemplate） | `StorageType` 枚举（Memory/Redis），实现精简 |
| `HttpClientType` 枚举（4 种 HTTP 后端） | **删除**（reqwest 统一） |
| `WxMpMultiServices`（多租户容器） | `WxMpMultiServices` 结构体 + `DashMap` |
| `@AutoConfiguration` / spring.factories | vernal application builder 自动注册 |

---

## 6. 验证标准（后续骨架阶段）

- [ ] `cargo build --workspace` 通过（占位 crate 编译成功，无未解析依赖）
- [ ] `cargo fmt --check` 无 diff
- [ ] `cargo clippy --workspace` 无 error（warn 可接受）
- [ ] 目录结构与本计划一致

---

## 7. 当前进度与本次范围

### 7.1 已完成
- ✅ `docs/PLAN.md`：本计划文档（含集成层规划，已修订）
- ✅ `docs/WXJAVA_ANALYSIS.md`：基于 CodeGraph 的 WxJava 深度分析（7 大设计模式、架构热点、Java→Rust 映射）
- ✅ WxJava 知识图谱构建：3933 文件 / 22364 节点 / 131518 边（可用于后续移植时的依赖追溯）

### 7.2 本阶段不做（属后续骨架实现阶段）
- ❌ 不实现任何具体微信 API（getAccessToken / code2Session / 下单 / 签名验签 等）
- ❌ 不创建 Cargo.toml / 任何 crate / lib.rs
- ❌ 不写 README / AGENTS / ARCHITECTURE / MIGRATION_STATUS
- ❌ 不写单元测试
- ❌ 不做 `git init` / `git commit`，不发布到 crates.io
- ❌ 不移植 solon-plugins（JVM 框架，无 Rust 对应，能力由 vernal/axum 集成覆盖）

---

## 8. 后续路线

1. **Phase 1 — common 完整**：完整实现 `wx-rust-common`（access_token 缓存策略、redis 存储、http 执行器、消息加解密、token 获取+执行引擎，见 WXJAVA_ANALYSIS 模式 D/E）
2. **Phase 2 — mp 样板**：`wx-rust-mp` 作为参考样板（含 `WxMpService` + 消息路由），打通端到端一条链路
3. **Phase 3 — 集成层起步**：`integration/wx-rust-vernal`（对标 spring-boot-starter，配置绑定 + Service 装配，见 5.6 节）—— 可与 Phase 2 并行，用 mp 模块验证装配链路
4. **Phase 4 — 逐模块移植**：按优先级 `miniapp → pay → cp → open → channel → aispeech`，每个模块同步补对应 vernal 集成
5. **Phase 5 — 多租户与轻集成**：`wx-rust-vernal-multi`（多租户）、`wx-rust-axum`/`wx-rust-actix`（无 IoC 轻集成）

每阶段独立规划、独立交付。集成层（Phase 3/5）**不再推迟到最后**，而是在 Phase 2 mp 打通后即起步，确保装配链路与 SDK 同步演进。
