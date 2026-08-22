# WxRust 架构设计计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**From:** `docs/ARCHITECTURE.md`
**创建日期：** 2026-08-01
**状态：** 已完成（核对日期：2026-08-12，依据：ARCHITECTURE.md LOCKED 状态，组件替换决策已冻结）

**Goal:** 建立 WxRust 迁移的架构宪法——B2 整批实现必须遵循的全部决策，包括总体架构、组件替换、Java→Rust 机制映射、命名规则、注释迁移规范、并发与生命周期契约、依赖管理。

**Architecture:** 四层架构（facade → 业务层 → 基础层 → 集成层）。Java 三层继承链（ServiceImpl → HttpComponentsImpl → BaseImpl）坍缩为 Rust 单一实现结构体（reqwest 统一 HTTP 后端）。Token 双重检查锁映射为 async tokio::sync::Mutex。请求执行引擎映射为 async loop（不用递归）。

**Tech Stack:** 见 ARCHITECTURE.md 第 2 节已锁定组件替换表（reqwest/serde_json/quick-xml/RustCrypto/tracing/chrono/thiserror/tokio/async-trait）。

## Global Constraints

- 架构决策 LOCKED（2026-08-01 B1 冻结），变更需评审。
- Java 基线：commit `a49d6e14`（4.8.4.B）。
- 对象分母：3287 main + 379 test；方法分母：36010 javap 公共方法。
- 全 workspace `#![forbid(unsafe_code)]`。
- 业务 crate 之间不互相依赖。
- MSRV ≤ 1.85，无 `unsafe` 优先、有维护、有测试。
- `redis` 为可选 feature，默认关闭。

---

## 1. 已确认需求与非目标

### 1.1 已确认需求

1. 锁定全部组件替换决策（HTTP/JSON/XML/加解密/日志/时间/集合/错误/异步）。
2. 定义继承链消解规则（Java 三层 → Rust 单一结构体）。
3. 定义 Token 双重检查锁的 async 映射。
4. 定义请求执行引擎的 async loop 映射。
5. 定义错误体系（WxErrorException typed enum）。
6. 定义消息路由的 builder 模式映射。
7. 定义 ConfigStorage 多租户映射。
8. 定义命名规则（对象名称一致性基准）。
9. 定义注释迁移规范（语义 100%）。
10. 定义并发与生命周期契约。

### 1.2 非目标

- 不实现具体业务代码（属 B2 批次）。
- 不修改已锁定的组件替换决策（除非经评审）。

---

### Task 1: 锁定组件替换决策

**Files:**
- Create: `docs/ARCHITECTURE.md` 第 2 节

- [x] **Step 1: 定义 HTTP 客户端替换（reqwest 统一）**

- [x] **Step 2: 定义 JSON/XML 序列化替换（serde + quick-xml）**

- [x] **Step 3: 定义加解密替换（RustCrypto 家族）**

- [x] **Step 4: 定义错误/异步/日志/时间/集合替换**

### Task 2: 定义 Java→Rust 机制映射

**Files:**
- Create: `docs/ARCHITECTURE.md` 第 3 节

- [x] **Step 1: 继承链消解规则**

- [x] **Step 2: Token 双重检查锁 async 映射**

- [x] **Step 3: 请求执行引擎 async loop 映射**

- [x] **Step 4: HTTP 执行器 / 错误体系 / 消息路由 / ConfigStorage 映射**

### Task 3: 定义命名与注释规范

**Files:**
- Create: `docs/ARCHITECTURE.md` 第 4-5 节

- [x] **Step 1: 命名规则表（PascalCase 类型 / snake_case 文件 / getter 去 get_ 前缀）**

- [x] **Step 2: 目录/文件映射规则（retain_segments=2）**

- [x] **Step 3: 注释迁移规范（语义 100%，禁止写"对应 Java"）**

### Task 4: 定义并发契约与批次定义

**Files:**
- Create: `docs/ARCHITECTURE.md` 第 6-8 节

- [x] **Step 1: 并发与生命周期契约表**

- [x] **Step 2: 依赖管理规则**

- [x] **Step 3: B2 实现批次定义（9 批次）**

---

## 2. 验收矩阵

| 验收项 | 证明方式 |
|---|---|
| 组件替换全覆盖 | 11 项替换决策均有 LOCKED/CANDIDATE 状态 |
| 机制映射完整 | 7 个映射（继承/Token/执行器/HTTP/错误/路由/Config） |
| 命名规则可执行 | 示例路径映射 3 个 |
| 批次定义完整 | 9 批次含对象数与方法数 |
