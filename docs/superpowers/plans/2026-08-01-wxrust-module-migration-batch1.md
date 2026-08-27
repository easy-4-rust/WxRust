# WxRust 模块迁移计划——第一批（common / mp / miniapp）

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**From:** `docs/migration/weixin-java-common/`、`docs/migration/weixin-java-mp/`、`docs/migration/weixin-java-miniapp/`（迁移路线图.md + 对象级对照表.md + 语义迁移对照表.md + 对象名称一致性检查.md）
**创建日期：** 2026-08-01
**状态：** 已完成（核对日期：2026-08-27，依据：common/mp/miniapp 三模块 0 MISSING；V0 审计 3287/3287 100% 处置）

**Goal:** 完成第一批三个模块（common / mp / miniapp）的 Java→Rust 迁移，包括基础层 trait 定义、公众号全场景 SDK、小程序全场景 SDK。

**Architecture:** common 定义 WxService/WxConfigStorage/RequestExecutor/WxError 等公共 trait；mp 和 miniapp 各自实现门面 Service + 子域 Service + bean + config + enums。继承链坍缩为单一实现结构体。

**Tech Stack:** 同 ARCHITECTURE.md 已锁定组件替换。

## Global Constraints

- 遵循 ARCHITECTURE.md 全部锁定决策。
- 每个模块按四件套（路线图/对象级对照表/语义迁移对照表/名称一致性检查）跟踪。
- 对象完成状态以 `docs/migration/<module>/对象级对照表.md` 为权威。
- 语义覆盖以 `docs/migration/<module>/语义迁移对照表.md` 为权威。
- 测试对标以 `docs/migration/<module>/迁移测试对照表.md` 为权威。

---

## 1. 已确认需求与非目标

### 1.1 已确认需求

1. common 基础层全量实现（174 对象 / 958 方法）。
2. mp 公众号模块全量实现（428 对象 / 3748 方法）。
3. miniapp 小程序模块全量实现（611 对象 / 4942 方法）。
4. 每个模块同步编写迁移测试。

### 1.2 非目标

- 不涉及 pay/cp/open/channel/aispeech/qidian（属第二、三批）。
- 不涉及集成层（属后续阶段）。

---

### Task 1: wx-rust-common 基础层

**Files:**
- Create/Modify: `crates/wx-rust-common/src/error/*.rs`（WxError / WxErrorException / WxRuntimeException / 各模块错误枚举）
- Create/Modify: `crates/wx-rust-common/src/bean/*.rs`（WxAccessToken / WxJsapiSignature / CommonUploadParam）
- Create/Modify: `crates/wx-rust-common/src/config.rs`（WxConfigStorage async trait）
- Create/Modify: `crates/wx-rust-common/src/http.rs`（WxHttpClient async trait）
- Create/Modify: `crates/wx-rust-common/src/service/*.rs`（WxService / WxOAuth2Service / WxOcrService / WxImgProcService）
- Create/Modify: `crates/wx-rust-common/src/session/*.rs`（WxSessionManager / StandardSession）
- Create/Modify: `crates/wx-rust-common/src/crypto/*.rs`（SHA1 / AES / 签名工具）
- Create/Modify: `crates/wx-rust-common/src/util/*.rs`（文件工具 / XML 工具 / 数据工具）

- [x] **Step 1: 实现错误体系**

- [x] **Step 2: 实现 Bean 类型**

- [x] **Step 3: 定义公共 trait（WxService / WxConfigStorage / RequestExecutor）**

- [x] **Step 4: 实现 Session 管理**

- [x] **Step 5: 实现加解密工具**

- [x] **Step 6: 编写测试（5 个测试文件：source_parity_bean_error / source_parity_crypto_util / source_parity_fs_utils / core_utils / coverage_boost_common）**

### Task 2: wx-rust-mp 公众号模块

**Files:**
- Create/Modify: `crates/wx-rust-mp/src/api/*.rs`（WxMpService + 30+ 子域 Service）
- Create/Modify: `crates/wx-rust-mp/src/bean/*.rs`（消息/用户/菜单/素材等 Bean）
- Create/Modify: `crates/wx-rust-mp/src/config/*.rs`（WxMpConfigStorage）
- Create/Modify: `crates/wx-rust-mp/src/enums/*.rs`（API URL / 卡片类型等）
- Create/Modify: `crates/wx-rust-mp/src/router/*.rs`（WxMpMessageRouter）

- [x] **Step 1: 实现门面 Service（WxMpService trait + 核心方法）**

- [x] **Step 2: 实现子域 Service（Kefu / Menu / User / Material / TemplateMsg / Qrcode / OAuth2）**

- [x] **Step 3: 实现 Bean 类型（消息/用户/菜单/素材等）**

- [x] **Step 4: 实现 ConfigStorage**

- [x] **Step 5: 实现消息路由（WxMpMessageRouter）**

- [x] **Step 6: 编写测试（9 个测试文件）**

### Task 3: wx-rust-miniapp 小程序模块

**Files:**
- Create/Modify: `crates/wx-rust-miniapp/src/api/*.rs`（WxMaService + 子域 Service）
- Create/Modify: `crates/wx-rust-miniapp/src/bean/*.rs`
- Create/Modify: `crates/wx-rust-miniapp/src/config/*.rs`（WxMaConfig）
- Create/Modify: `crates/wx-rust-miniapp/src/enums/*.rs`

- [x] **Step 1: 实现门面 Service（WxMaService）**

- [x] **Step 2: 实现子域 Service（User / Msg / Qrcode / Kefu / Analysis / Code / Live / Security）**

- [x] **Step 3: 实现 Bean 类型**

- [x] **Step 4: 实现 ConfigStorage**

- [x] **Step 5: 编写测试（9 个测试文件）**

---

## 2. 验收矩阵

| 模块 | src 文件数 | test 文件数 | 状态 |
|---|---|---|---|
| wx-rust-common | 105 | 7 | 进行中（基础 trait 已定义，部分子域待补） |
| wx-rust-mp | 386 | 9 | 进行中（核心 Service 已有骨架，部分子域待补） |
| wx-rust-miniapp | 635 | 9 | 进行中（核心 Service 已有骨架，部分子域待补） |
