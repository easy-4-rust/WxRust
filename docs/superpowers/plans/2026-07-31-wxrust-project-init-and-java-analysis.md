# WxRust 项目启动与 WxJava 源码分析计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**From:** `docs/WXJAVA_ANALYSIS.md` + `docs/PLAN.md`（7.1 节）
**创建日期：** 2026-07-31
**状态：** 已完成（核对日期：2026-08-12，依据：WXJAVA_ANALYSIS.md 存在且内容完整，知识图谱已构建）

**Goal:** 对 WxJava（4.8.4.B）进行全量符号级源码分析，建立知识图谱，提炼移植所需的全部架构要素（7 大设计模式、模块规模、架构热点、关键抽象对照表），为 WxRust 迁移计划提供技术基线。

**Architecture:** 基于 code-review-graph MCP 工具对 WxJava 仓库进行全量图谱构建（3941 文件 / 23094 节点 / 132669 边 / 13 社区 / 2778 执行流），结合源码精读，输出 WXJAVA_ANALYSIS.md 作为后续所有迁移决策的技术补充与移植指南。

**Tech Stack:** code-review-graph MCP（build_or_update_graph / run_postprocess / list_communities / get_hub_nodes / get_bridge_nodes / query_graph / semantic_search_nodes）、源码精读。

## Global Constraints

- 分析对象为 WxJava commit `a49d6e14`（4.8.4.B），main 合计 3288 文件、test 合计 379 文件。
- 分析结论必须可重放：每个发现附带工具命令与图谱数据。
- 不修改 WxJava 源码，纯只读分析。
- 输出文档为中文正文 + 英文结构标题。

---

## 1. 已确认需求与非目标

### 1.1 已确认需求

1. 建立 WxJava 全量知识图谱，支持符号级依赖追溯。
2. 提炼核心设计模式（门面 Service、三层继承链、泛型 RequestHttp、Token 双重检查锁、请求执行引擎、RequestExecutor 策略、消息路由）。
3. 识别架构热点（Hub 节点、Bridge 节点、未测试热点）。
4. 输出 Java→Rust 概念映射总表。
5. 识别高风险区与移植优先级建议。

### 1.2 非目标

- 不实现任何 Rust 代码。
- 不创建 Cargo.toml 或任何 crate。
- 不做 git commit。

---

### Task 1: 构建 WxJava 知识图谱

**Files:**
- 无文件产出（图谱为 MCP 内部数据）

**Interfaces:**
- Produces: 3941 文件 / 23094 节点 / 132669 边的知识图谱

- [x] **Step 1: 全量图谱构建**

调用 `build_or_update_graph`（full 模式）对 WxJava 仓库进行全量扫描。

- [x] **Step 2: 后处理**

调用 `run_postprocess` 生成社区（13 个）、执行流（2778 个）、bare 边解析（3038 个）。

- [x] **Step 3: 提交分析基础设施**

图谱构建完成，后续 Task 基于此进行查询。

### Task 2: 架构热点与设计模式分析

**Files:**
- 产出：`docs/WXJAVA_ANALYSIS.md` 第 2-4 节

- [x] **Step 1: Hub 节点分析**

调用 `get_hub_nodes` 识别连接度最高节点（GsonBuilder.create 774 度为架构瓶颈）。

- [x] **Step 2: Bridge 节点分析**

调用 `get_bridge_nodes` 确认 JSON 序列化是结构关键路径。

- [x] **Step 3: 未测试热点识别**

调用 `get_knowledge_gaps` 发现 50 孤立节点 + 20 未测试热点。

- [x] **Step 4: 社区内聚分析**

调用 `list_communities` 和 `get_community`（token 社区，内聚 0.45）。

- [x] **Step 5: 源码精读关键类**

精读 `WxMpServiceHttpComponentsImpl`、`BaseWxMpServiceImpl`、`RequestHttp`、`WxMpMessageRouter`。

### Task 3: 输出分析文档

**Files:**
- Create: `docs/WXJAVA_ANALYSIS.md`

- [x] **Step 1: 编写仓库总体结构（第 1 节）**

4 层 SDK 平台架构 + 模块规模表。

- [x] **Step 2: 编写核心设计模式（第 2 节）**

7 大模式（A-G）含代码示例与 Rust 映射说明。

- [x] **Step 3: 编写架构热点（第 3 节）**

Hub/Bridge/Untested/Community 四维度。

- [x] **Step 4: 编写关键抽象对照表（第 4 节）**

common 层、ConfigStorage、消息路由家族三张对照表。

- [x] **Step 5: 编写概念映射总表（第 5 节）**

16 项 Java→Rust 概念映射。

- [x] **Step 6: 编写移植风险与优先级（第 6 节）**

高风险区 + 四阶段优先级 + 可删除概念清单。

---

## 2. 验收矩阵

| 验收项 | 证明方式 |
|---|---|
| 知识图谱完整 | 3941 文件 / 23094 节点 / 132669 边 |
| 设计模式全覆盖 | 7 大模式（A-G）均有代码示例与 Rust 映射 |
| 架构热点识别 | Hub/Bridge/Untested 节点表 |
| 文档可追溯 | 每个发现附带工具命令 |
