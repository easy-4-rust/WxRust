# WxRust 模块迁移计划——第二批（pay / cp / open）

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**From:** `docs/migration/weixin-java-pay/`、`docs/migration/weixin-java-cp/`、`docs/migration/weixin-java-open/`
**创建日期：** 2026-08-01
**状态：** 已完成（核对日期：2026-08-27，依据：pay/cp/open 三模块 0 MISSING；V0 审计 3287/3287 100% 处置）

**Goal:** 完成第二批三个模块（pay / cp / open）的 Java→Rust 迁移。

**Architecture:** pay 实现 v2+v3 支付（含证书、XML 报文）；cp 实现企业微信全场景（含第三方代开发）；open 实现第三方平台（桥接 MP/MA）。

**Tech Stack:** 同 ARCHITECTURE.md。pay 特别依赖 quick-xml（XML 报文）和 RustCrypto（v3 签名/证书）。

## Global Constraints

- 遵循 ARCHITECTURE.md 全部锁定决策。
- pay 模块的 XML 报文必须与微信接口 100% 兼容。
- cp 模块含第三方代开发（TpConfigStorage / TpService）。
- open 模块复用 mp/ma 能力通过组合，不建依赖。

---

## 1. 已确认需求与非目标

### 1.1 已确认需求

1. pay 微信支付全量实现（570 对象 / 6788 方法），含 v2+v3。
2. cp 企业微信全量实现（594 对象 / 6099 方法），含第三方代开发。
3. open 开放平台全量实现（240 对象 / 2077 方法）。

### 1.2 非目标

- 不涉及 channel/aispeech/qidian（属第三批）。

---

### Task 1: wx-rust-pay 微信支付模块

**Files:**
- Create/Modify: `crates/wx-rust-pay/src/api/*.rs`（WxPayService + PayScore / ProfitSharing / Redpack / Transfer / EntPay / Ecommerce 等）
- Create/Modify: `crates/wx-rust-pay/src/bean/*.rs`（支付结果/通知/XML 报文等）
- Create/Modify: `crates/wx-rust-pay/src/config/*.rs`（WxPayConfig 含证书）
- Create/Modify: `crates/wx-rust-pay/src/util/*.rs`（签名/通知/验签工具）
- Create/Modify: `crates/wx-rust-pay/src/enums/*.rs`（交易类型/签名类型等）
- Create/Modify: `crates/wx-rust-pay/src/constant/*.rs`（错误码/常量）

- [x] **Step 1: 实现门面 Service（WxPayService v2+v3）**

- [x] **Step 2: 实现子域 Service（PayScore / ProfitSharing / Redpack / Transfer / EntPay / Ecommerce）**

- [x] **Step 3: 实现 Bean 类型（XML 报文 / 支付结果 / 通知）**

- [x] **Step 4: 实现 ConfigStorage（含证书管理）**

- [x] **Step 5: 实现签名/验签/通知工具**

- [x] **Step 6: 编写测试（7 个测试文件）**

### Task 2: wx-rust-cp 企业微信模块

**Files:**
- Create/Modify: `crates/wx-rust-cp/src/api/*.rs`（WxCpService + Agent / Department / User / Media / Message / OAuth2 / ExternalContact / Tag / Menu / Chat / Kf / Meeting 等）
- Create/Modify: `crates/wx-rust-cp/src/bean/*.rs`
- Create/Modify: `crates/wx-rust-cp/src/config/*.rs`（WxCpConfigStorage / WxCpTpConfigStorage）
- Create/Modify: `crates/wx-rust-cp/src/enums/*.rs`
- Create/Modify: `crates/wx-rust-cp/src/router/*.rs`（WxCpMessageRouter）

- [x] **Step 1: 实现门面 Service（WxCpService）**

- [x] **Step 2: 实现子域 Service（Agent / Department / User / Media / Message / OAuth2 / ExternalContact）**

- [x] **Step 3: 实现第三方代开发（TpConfigStorage / TpService）**

- [x] **Step 4: 实现消息路由（WxCpMessageRouter / WxCpTpMessageRouter）**

- [x] **Step 5: 编写测试（8 个测试文件）**

### Task 3: wx-rust-open 开放平台模块

**Files:**
- Create/Modify: `crates/wx-rust-open/src/api/*.rs`（WxOpenService + ComponentService / MaService）
- Create/Modify: `crates/wx-rust-open/src/bean/*.rs`
- Create/Modify: `crates/wx-rust-open/src/config/*.rs`（WxOpenConfigStorage）
- Create/Modify: `crates/wx-rust-open/src/enums/*.rs`
- Create/Modify: `crates/wx-rust-open/src/constant/*.rs`

- [x] **Step 1: 实现门面 Service（WxOpenService）**

- [x] **Step 2: 实现 ComponentService（第三方平台核心）**

- [x] **Step 3: 实现 MaService（小程序代管理）**

- [x] **Step 4: 编写测试（5 个测试文件）**

---

## 2. 验收矩阵

| 模块 | src 文件数 | test 文件数 | 状态 |
|---|---|---|---|
| wx-rust-pay | 589 | 7 | 进行中 |
| wx-rust-cp | 654 | 8 | 进行中 |
| wx-rust-open | 239 | 5 | 进行中 |
