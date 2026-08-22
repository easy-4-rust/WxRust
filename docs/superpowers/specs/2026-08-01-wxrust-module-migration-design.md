# WxRust 模块迁移设计

日期：2026-08-01
状态：进行中
来源：`docs/migration/weixin-java-*/`（9 模块迁移路线图 + 对象级对照表 + 语义迁移对照表 + 对象名称一致性检查）

## 1. 背景

WxJava 9 个业务模块（common/mp/miniapp/pay/cp/open/channel/aispeech/qidian）共 3287 个 main 对象需要迁移到 Rust。每个模块有独立的迁移四件套（路线图/对象级对照表/语义迁移对照表/名称一致性检查）。迁移按三批推进：第一批（common/mp/miniapp）、第二批（pay/cp/open）、第三批（channel/aispeech/qidian）。

## 2. 目标与非目标

### 2.1 目标

- 9 个模块全部完成 Java→Rust 迁移。
- 每个模块的四件套文档保持与代码同步。
- 迁移遵循 ARCHITECTURE.md 全部锁定决策。
- 迁移按 B2 批次定义推进。

### 2.2 非目标

- 不涉及集成层（wx-rust-vernal，属后续阶段）。
- 不涉及测试补齐（见测试迁移设计规格）。

## 3. 方案比较

### 3.1 按 Java 对象逐个翻译

逐个 Java 文件翻译为 Rust，保持 1:1 对应。优点是可追溯性强；缺点是 Rust 惯例与 Java 不同（无继承、无 getter 呍名），机械翻译会产生不符合 Rust 惯例的代码。

本方案不采用。

### 3.2 按模块批实现，语义对齐 + 惯例适配

以模块为最小迁移单元，先冻结模块设计（四件套），再按 Rust 惯例实现。Java 的继承链坍缩为 trait + 结构体，getter 去 `get_` 前缀，手写适配器改为 derive 宏。

本方案作为最终方案。

## 4. 模块与依赖

### 4.1 模块规模

| 模块 | Java 对象 | Java 方法 | Rust src 文件 | Rust test 文件 | 状态 |
|---|---|---|---|---|---|
| common | 174 | 958 | 105 | 7 | 进行中 |
| mp | 428 | 3748 | 386 | 9 | 进行中 |
| miniapp | 611 | 4942 | 635 | 9 | 进行中 |
| pay | 570 | 6788 | 589 | 7 | 进行中 |
| cp | 594 | 6099 | 654 | 8 | 进行中 |
| open | 240 | 2077 | 239 | 5 | 进行中 |
| channel | 618 | 4308 | 696 | 5 | 进行中 |
| aispeech | 25 | 256 | 32 | 3 | 已完成 |
| qidian | 27 | 285 | 31 | 6 | 已完成 |
| **合计** | **3287** | **29461** | **3367** | **59** | |

### 4.2 依赖方向

```text
wx-rust-mp → wx-rust-common
wx-rust-miniapp → wx-rust-common
wx-rust-pay → wx-rust-common
wx-rust-cp → wx-rust-common
wx-rust-open → wx-rust-common
wx-rust-channel → wx-rust-common
wx-rust-aispeech → wx-rust-common
wx-rust-qidian → wx-rust-common
```

业务 crate 之间不互相依赖。

### 4.3 每个模块的标准结构

```
crates/wx-rust-<module>/
├── Cargo.toml
├── src/
│   ├── lib.rs              # #![forbid(unsafe_code)] + 模块组织
│   ├── api/                # 门面 Service trait + 子域 Service
│   │   ├── mod.rs
│   │   ├── wx_<module>_service.rs
│   │   └── ...
│   ├── bean/               # Bean/DTO 类型
│   ├── config/             # ConfigStorage trait + 实现
│   ├── enums/              # 枚举类型
│   ├── constant/           # 常量
│   ├── util/               # 工具函数
│   └── router/             # 消息路由（mp/cp/channel/open）
└── tests/                  # 集成测试
```

### 4.4 模块迁移设计要点

#### 4.4.1 common（基础层）

- 定义公共 trait：`WxService` / `WxConfigStorage` / `RequestExecutor` / `WxHttpClient`。
- 实现错误体系：`WxError` / `WxErrorException` / `WxRuntimeException`。
- 实现 Bean 类型：`WxAccessToken` / `WxJsapiSignature` / `CommonUploadParam`。
- 实现 Session 管理：`WxSessionManager` / `StandardSession`。
- 实现加解密工具：SHA1 / AES / RSA / Base64。

#### 4.4.2 mp（公众号）

- 门面 Service：`WxMpService` trait + 核心方法（get_access_token / execute / execute_internal）。
- 子域 Service：Kefu / Menu / User / UserTag / Material / MassMessage / TemplateMsg / Qrcode / DataCube / Card / Store / SubscribeMsg / Draft / FreePublish / OAuth2 等 30+。
- 消息路由：`WxMpMessageRouter` + `WxMpMessageRouterRule`（builder 模式）。
- ConfigStorage：`WxMpConfigStorage` trait + 内存实现。

#### 4.4.3 miniapp（小程序）

- 门面 Service：`WxMaService` trait。
- 子域 Service：User / Msg / Media / Kefu / Analysis / Cloud / Code / Express / Link / Scheme / Live / Security / Setting / Subscribe / XPay / Jsapi 等。
- ConfigStorage：`WxMaConfig` trait + 内存实现。

#### 4.4.4 pay（微信支付）

- 门面 Service：`WxPayService` trait（v2 + v3）。
- 子域 Service：PayScore / ProfitSharing / Redpack / Transfer / EntPay / Ecommerce / Complaint 等。
- 特殊依赖：quick-xml（XML 报文）、RustCrypto（v3 签名/证书）。
- ConfigStorage：`WxPayConfig`（含证书管理）。

#### 4.4.5 cp（企业微信）

- 门面 Service：`WxCpService` trait。
- 子域 Service：Agent / Department / User / Media / Message / OAuth2 / ExternalContact / Tag / Menu / Chat / Kf / Meeting / Hr / School 等。
- 第三方代开发：`WxCpTpConfigStorage` / `WxCpTpService`。
- 消息路由：`WxCpMessageRouter` / `WxCpTpMessageRouter`。

#### 4.4.6 open（开放平台）

- 门面 Service：`WxOpenService` trait。
- 子域 Service：ComponentService（第三方平台核心）/ MaService（小程序代管理）。
- 复用 mp/ma 能力通过组合，不建依赖。

#### 4.4.7 channel（视频号/小店）

- 门面 Service：`WxChannelService` trait。
- 子域 Service：Basic / Order / Product / AfterSale / Brand / Category / Coupon / Fund / Vip / Sharer / Warehouse / Freight / Address / Compass / Live 等。
- 消息路由：`WxChannelMessageRouter`。
- 规模最大（618 对象），按子域分批实现。

#### 4.4.8 aispeech（AI 语音）

- 门面 Service：`WxAiSpeechService` trait。
- 子域 Service：KnowledgeService / DialogService。
- 规模最小（25 对象），可一次性完成。已完成。

#### 4.4.9 qidian（企点）

- 门面 Service：`WxQidianService` trait。
- 子域 Service：DialService / CallDataService。
- 复用 mp 配置模式。已完成。

## 5. 验收标准

- 9 模块 src 文件总数 >= 3287（Java 对象数）。
- 每个模块的四件套文档与代码同步。
- `cargo build --workspace` 通过。
- `cargo clippy --workspace` 无 error。
- aispeech 和 qidian 模块测试全覆盖。
