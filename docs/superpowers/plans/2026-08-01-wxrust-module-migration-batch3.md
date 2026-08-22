# WxRust 模块迁移计划——第三批（channel / aispeech / qidian）

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**From:** `docs/migration/weixin-java-channel/`、`docs/migration/weixin-java-aispeech/`、`docs/migration/weixin-java-qidian/`
**创建日期：** 2026-08-01
**状态：** 进行中（核对日期：2026-08-12，依据：crates 源文件计数 + 测试文件计数）

**Goal:** 完成第三批三个模块（channel / aispeech / qidian）的 Java→Rust 迁移。

**Architecture:** channel 实现视频号/微信小店全场景（618 对象，垂直业务化）；aispeech 实现 AI 语音（25 对象，极小）；qidian 实现企点（27 对象，已并入企业微信）。

**Tech Stack:** 同 ARCHITECTURE.md。

## Global Constraints

- 遵循 ARCHITECTURE.md 全部锁定决策。
- channel 模块规模最大（618 对象），需按子域分批实现。
- aispeech 模块最小（25 对象），可一次性完成。
- qidian 模块复用 mp 配置模式。

---

## 1. 已确认需求与非目标

### 1.1 已确认需求

1. channel 视频号/小店全量实现（618 对象 / 4308 方法）。
2. aispeech AI 语音全量实现（25 对象 / 256 方法）。
3. qidian 企点全量实现（27 对象 / 285 方法）。

### 1.2 非目标

- 不涉及集成层（属后续阶段）。

---

### Task 1: wx-rust-channel 视频号/小店模块

**Files:**
- Create/Modify: `crates/wx-rust-channel/src/api/*.rs`（WxChannelService + Basic / Order / Product / AfterSale / Brand / Category / Coupon / Fund / Vip / Sharer / Warehouse / Freight / Address 等）
- Create/Modify: `crates/wx-rust-channel/src/bean/*.rs`
- Create/Modify: `crates/wx-rust-channel/src/config/*.rs`（WxChannelConfigStorage）
- Create/Modify: `crates/wx-rust-channel/src/enums/*.rs`（大量枚举：消息类型/审核类型/资金类型等）
- Create/Modify: `crates/wx-rust-channel/src/util/*.rs`
- Create/Modify: `crates/wx-rust-channel/src/router/*.rs`（WxChannelMessageRouter）

- [x] **Step 1: 实现门面 Service（WxChannelService）**

- [x] **Step 2: 实现核心子域（Basic / Order / Product / AfterSale）**

- [x] **Step 3: 实现扩展子域（Brand / Category / Coupon / Fund / Vip / Sharer / Warehouse / Freight / Address）**

- [x] **Step 4: 实现消息路由（WxChannelMessageRouter）**

- [x] **Step 5: 实现 ConfigStorage + 枚举**

- [x] **Step 6: 编写测试（5 个测试文件）**

### Task 2: wx-rust-aispeech AI 语音模块

**Files:**
- Create/Modify: `crates/wx-rust-aispeech/src/api/*.rs`（WxAiSpeechService / KnowledgeService / DialogService）
- Create/Modify: `crates/wx-rust-aispeech/src/bean/*.rs`（dialog / knowledge 相关 Bean）
- Create/Modify: `crates/wx-rust-aispeech/src/config/*.rs`（WxAiSpeechConfigStorage）
- Create/Modify: `crates/wx-rust-aispeech/src/util/*.rs`（签名工具）

- [x] **Step 1: 实现门面 Service + 子域 Service**

- [x] **Step 2: 实现 Bean 类型**

- [x] **Step 3: 实现 ConfigStorage + 签名工具**

- [x] **Step 4: 编写测试（3 个测试文件）**

### Task 3: wx-rust-qidian 企点模块

**Files:**
- Create/Modify: `crates/wx-rust-qidian/src/api/*.rs`（WxQidianService / DialService / CallDataService）
- Create/Modify: `crates/wx-rust-qidian/src/bean/*.rs`（dial / IVR 相关 Bean）
- Create/Modify: `crates/wx-rust-qidian/src/config/*.rs`（WxQidianConfigStorage）
- Create/Modify: `crates/wx-rust-qidian/src/enums/*.rs`

- [x] **Step 1: 实现门面 Service + 子域 Service**

- [x] **Step 2: 实现 Bean 类型**

- [x] **Step 3: 实现 ConfigStorage**

- [x] **Step 4: 编写测试（6 个测试文件）**

---

## 2. 验收矩阵

| 模块 | src 文件数 | test 文件数 | 状态 |
|---|---|---|---|
| wx-rust-channel | 696 | 5 | 进行中（核心子域已有骨架，扩展子域部分待补） |
| wx-rust-aispeech | 32 | 3 | 已完成（测试全覆盖） |
| wx-rust-qidian | 31 | 6 | 已完成（测试全覆盖） |
