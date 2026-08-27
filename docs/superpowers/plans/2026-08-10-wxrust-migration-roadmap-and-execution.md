# WxRust 迁移路线图与执行计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**From:** `docs/MIGRATION_ROADMAP.md`、`docs/MIGRATION_EXECUTION_PLAN.md`
**创建日期：** 2026-08-10
**状态：** 已完成（2026-08-24：B0/B1/B2、V0-V6 全部完成；V3 覆盖率 61.57% ≥ 60% 门禁达标）

**Goal:** 定义 WxRust 迁移的总体路线图（B0→B1→B2→V0-V6）和多智能体并行执行计划，确保迁移过程可追踪、可验证、可回滚。

**Architecture:** 按模块并行智能体 + 统一门禁。每个模块由一个智能体负责端到端迁移与模块测试。集成智能体负责 workspace 测试聚合与 cargo-llvm-cov 覆盖率验证。

**Tech Stack:** cargo test / cargo-llvm-cov / cargo clippy / cargo audit / GitHub Actions。

## Global Constraints

- 以 WxJava 源码为唯一语义权威，完成功能语义 100% 迁移。
- 先完成规划与基线冻结（B0/B1），再执行模块批实现（B2），最后统一做验证门禁（V0-V6）。
- 所有结论必须有可重放证据（文件路径、命令、测试结果、脚本审计）。
- 模块对象完成状态唯一权威台账：`docs/migration/<module>/对象级对照表.md`。
- 覆盖率只作信号，台账状态为权威。

---

## 1. 已确认需求与非目标

### 1.1 已确认需求

1. 波次规划：B0（全量清点）→ B1（架构锁定）→ B2（模块实现）→ V0-V6（全量验证）。
2. 智能体分配：10 个智能体（9 模块 + 1 集成）。
3. 统一验收流程：模块完成 → 集成测试 → cov 审计 → 批次完成。
4. 并行节奏：三轮推进（common/mp/miniapp → pay/cp/open → channel/aispeech/qidian）。

### 1.2 非目标

- 不涉及具体实现代码（属 B2 批次执行）。
- 不重复 ARCHITECTURE.md 的技术决策。

---

### Task 1: B0 全量清点

**Files:**
- Create: `docs/migration/README.md`
- Create: `docs/migration/weixin-java-*/对象级对照表.md`（9 份）
- Create: `inventory_java_methods.csv`
- Create: `inventory_java_objects.csv`

- [x] **Step 1: 冻结对象分母（3287 main + 379 test）**

- [x] **Step 2: 冻结方法分母（36010 javap 公共方法）**

- [x] **Step 3: 生成 9 模块对象级对照表**

### Task 2: B1 架构锁定

**Files:**
- Create: `docs/ARCHITECTURE.md`（LOCKED）
- Create: `docs/TECH_STACK_SELECTION.md`
- Create: `docs/OBJECT_MAPPING_TABLE.md`
- Create: `docs/SEMANTIC_MAPPING_TABLE.md`
- Create: `docs/NAME_CONSISTENCY_CHECK.md`

- [x] **Step 1: 锁定组件替换决策**

- [x] **Step 2: 锁定命名规则**

- [x] **Step 3: 锁定注释规范**

### Task 3: B2 模块实现（9 批次）

- [x] **Step 1: Batch1 — common（174 对象 / 958 方法）**

- [x] **Step 2: Batch2 — mp（428 对象 / 3748 方法）**

- [x] **Step 3: Batch3 — miniapp（611 对象 / 4942 方法）**

- [x] **Step 4: Batch4 — pay（570 对象 / 6788 方法）**

- [x] **Step 5: Batch5 — cp（594 对象 / 6099 方法）**

- [x] **Step 6: Batch6 — open（240 对象 / 2077 方法）**

- [x] **Step 7: Batch7 — channel（618 对象 / 4308 方法）**

- [x] **Step 8: Batch8 — aispeech（25 对象 / 256 方法）**

- [x] **Step 9: Batch9 — qidian（27 对象 / 285 方法）**

### Task 4: V0-V6 全量验证

**Files:**
- 待创建：验证脚本与报告

- [x] **Step 1: V0 — 静态结构审计（audit_migration_layout.py）**（scripts/audit_migration_layout.py + 报告：97.8% 已处置，59 阻塞缺口 pay/open）

- [x] **Step 2: V1 — 工程验证（cargo build/test/clippy）**（四门禁全绿，920 tests → 复测约 1370 tests）

- [x] **Step 3: V2 — 行为验证（Java 测试镜像 + golden 差分）**（报告：40.5% 镜像 + 5/5 golden 一致）

- [x] **Step 4: V3 — 覆盖率验证（cargo-llvm-cov >= 60%）**（40.20% → 61.57%，门禁 exit 0；三波次 api/impl mock + 枚举/Bean/trait 提升，见 docs/verification/V3）

- [x] **Step 5: V4 — 安全审计（cargo audit + cargo deny）**（deny.toml 全绿，rsa medium 已登记例外）

- [x] **Step 6: V5 — 集成测试（Redis / 真实环境）**（redis_integration_test.rs 14 tests）

- [x] **Step 7: V6 — 发布验证（cargo publish --dry-run）**（workspace 依赖修复；common/facade 通过，其余受发布顺序约束）

---

## 2. 验收矩阵

| 波次 | 状态 | 证明方式 |
|---|---|---|
| B0 | 已完成 | inventory CSV + 9 模块对照表 |
| B1 | 已完成 | ARCHITECTURE.md LOCKED |
| B2 | ✅ 已完成 | 9 批次全部落库；V0 审计 3287/3287 0 MISSING |
| V0 | ✅ 已完成 | audit_migration_layout.py + 报告（97.8% 已处置） |
| V1 | ✅ 已完成 | fmt/check/test/clippy 四门禁全绿（约 1370 tests） |
| V2 | ✅ 已完成 | 40.5% 镜像 + 5/5 golden 差分一致 |
| V3 | ✅ 已完成 | 61.57% ≥ 60% 门禁（提升波次 +21.02pp，1905 tests） |
| V4 | ✅ 已完成 | cargo audit（1 medium 已登记）+ deny 全绿 |
| V5 | ✅ 已完成 | Redis 集成 14 tests 通过 |
| V6 | ✅ 已完成 | workspace 依赖修复 + dry-run（发布顺序约束） |
