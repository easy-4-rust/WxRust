# WxRust 技术栈选型与映射规范计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**From:** `docs/TECH_STACK_SELECTION.md`、`docs/OBJECT_MAPPING_TABLE.md`、`docs/SEMANTIC_MAPPING_TABLE.md`、`docs/NAME_CONSISTENCY_CHECK.md`
**创建日期：** 2026-08-10
**状态：** 已完成（核对日期：2026-08-12，依据：四份源文档均存在且内容完整，ARCHITECTURE.md 已锁定选型）

**Goal:** 固化 WxRust 的技术栈选型（以"语义等价 + Rust 生态成熟"为首选）和对象/语义/命名三套映射规范，为 B2 批量实现提供不可争议的技术基线。

**Architecture:** 技术栈选型遵循 ARCHITECTURE.md 第 2 节已锁定决策。映射规范遵循 ARCHITECTURE.md 第 4 节命名规则。

**Tech Stack:** tokio / reqwest / serde_json / quick-xml / thiserror / tracing / RustCrypto / chrono / async-trait / wiremock / cargo-llvm-cov。

## Global Constraints

- 选型原则：语义等价 + Rust 生态成熟 + 避免多后端并存 + 可观测性与测试能力纳入硬约束。
- 映射规范以 ARCHITECTURE.md 为权威，本文档为操作指南。
- 组件替换的详细硬约束以 ARCHITECTURE.md 为准。

---

## 1. 已确认需求与非目标

### 1.1 已确认需求

1. 核心选型表（10 项：异步运行时/HTTP/JSON/XML/错误/日志/加解密/时间/测试/覆盖率）。
2. 可选/后续引入候选表（redis / criterion / proptest）。
3. 对象级对照表总览（9 模块 / 3287 对象 / 0 MISSING）。
4. 语义迁移对照表总览（8 维度覆盖）。
5. 名称一致性检查总览（4 检查维度）。

### 1.2 非目标

- 不重复 ARCHITECTURE.md 已锁定的决策细节。
- 不涉及具体实现代码。

---

### Task 1: 技术栈选型文档化

**Files:**
- Create: `docs/TECH_STACK_SELECTION.md`

- [x] **Step 1: 核心选型表（10 项）**

- [x] **Step 2: 可选/后续引入候选表**

- [x] **Step 3: 与 ddd4r 映射对照**

### Task 2: 对象映射规范文档化

**Files:**
- Create: `docs/OBJECT_MAPPING_TABLE.md`

- [x] **Step 1: 模块对象分布表（9 模块）**

- [x] **Step 2: 状态口径定义**

- [x] **Step 3: 当前结论（0 MISSING）**

### Task 3: 语义映射规范文档化

**Files:**
- Create: `docs/SEMANTIC_MAPPING_TABLE.md`

- [x] **Step 1: 语义覆盖范围（8 维度）**

- [x] **Step 2: 当前结论**

### Task 4: 命名一致性规范文档化

**Files:**
- Create: `docs/NAME_CONSISTENCY_CHECK.md`

- [x] **Step 1: 检查口径（保留末 2 层 / snake_case）**

- [x] **Step 2: 检查维度（4 维度）**

- [x] **Step 3: 当前结论（0 MISSING）**

---

## 2. 验收矩阵

| 验收项 | 证明方式 |
|---|---|
| 选型完整 | 10 项核心 + 3 项候选 |
| 映射覆盖 | 9 模块 / 3287 对象 / 0 MISSING |
| 语义覆盖 | 8 维度全覆盖 |
| 命名一致 | 4 维度检查通过 |
