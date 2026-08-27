# WxRust 生产就绪收口实施计划（Coverage→Alpha→GA）

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 把 WxRust 从"Conditionally Ready"推到"Ready"：① 覆盖率 69.82%→≥90%（门禁 ≥90%）② Alpha Day-5 Step 5/6 在凭证到位后真实流量闭环 ③ RSA 例外解除路径固化 ④ GA 终态判定与发版。

**Architecture:** 纯测试增量（零 src 改动）为主；Alpha Step 5/6 是运营动作（需用户凭证）；RSA 是跟踪项。镜像率已达 100.8% 不再是缺口。所有任务遵循强制证据链路（git status 空前置 → ls/wc-l 自报 → git show --stat → grep 实跑）。

**Tech Stack:** cargo llvm-cov（≥90 门禁）、httpmock（pay/cp dev-deps）、tokio、zero new deps。

## Global Constraints

- 零 src 改动（纯测试/文档/脚本）；若确需 src 修复须独立 commit 并说明
- `cargo test --workspace` ≥ 3301 且 0 failed（每任务不回归）
- `cargo clippy --workspace --all-targets -- -D warnings` clean / fmt clean
- 强制证据链路：前置空状态证明 → 每文件 ls/wc-l → git show --stat → 实跑数字
- 智能体虚报防线：报告数字必须可复算，禁止凭心智模型估值
- 凭证相关任务（Phase C）在用户提供 test appid+appsecret 前标记 PENDING，不得伪造

---

## 1. 当前基线（2026-08-27 实测）

| 指标 | 值 | 来源 |
|---|---|---|
| 镜像率 | 100.8%（383/380 unique）✅ | 复测 #7（b45feaa） |
| 行覆盖率 | **69.82%**（21399/70911 missed） | 本轮实测 |
| workspace tests | 3301 / 0 failed | 本轮实测 |
| 10/10 crate crates.io | LIVE v0.1.0 | tag v0.1.0 |
| Alpha Day-5 | HOLD（Step 5/6 待凭证） | Day-3 报告 |
| RSA RUSTSEC-2023-0071 | known exception | deny.toml |

### 覆盖率缺口分布（llvm-cov 实测 Top）

| crate | 行覆盖 | 主要未覆盖域 |
|---|---|---|
| wx-rust-pay | ~52% | api/impl 大方法体、v2 XML 解析分支、entpay/redpack 长尾 |
| wx-rust-cp | ~72% | OA weDoc/approval 分支、TP service 边界 |
| wx-rust-miniapp | ~98% | 剩余少量分支 |
| wx-rust-channel | ~110%* | *跨模块声明，实际约 90% |
| wx-rust-common | ~65% | util 边界、config 过期分支 |

---

## Phase A：覆盖率 69.82% → ≥90%（4 任务，无外部依赖）

### Task A1: pay 覆盖率提升（52% → ≥75%）

**Files:**
- Test: `crates/wx-rust-pay/tests/cov_pay_deep.rs`（新建）
- 参考：`crates/wx-rust-pay/src/api/impl/base_wx_pay_service_impl.rs`、`src/util/wx_pay_service_impl_utils.rs`

**Interfaces:**
- Consumes: 现有 httpmock 模式（`phase1_batch4_pay_core.rs`）、`WxPayServiceImpl::new_arc(config)`
- Produces: ≥40 个新测试，每个含 `/// 对应 Java:` 注释与 ≥1 真断言

- [x] **Step 1: 基线采集**

Run: `cargo llvm-cov -p wx-rust-pay --summary-only` 记录 per-file 未覆盖行清单到 `/tmp/pay-gaps.txt`

- [x] **Step 2: 写深度测试（TDD）**

覆盖目标（逐方法读 Java impl 后写）：
- `query_order` 各 trade_state 分支（SUCCESS/REFUND/NOTPAY/CLOSED/REVOKED/USERPAYING/PAYERROR）
- `close_order` 返回码 == OK 与 != OK 两路
- `refund` v1 XML 字段序（out_refund_no/total_fee/refund_fee 断言）
- `download_bill` tencent 头剥离逻辑（`"""
总收款单"` 头三行剥离）
- `trans_xml_map_to_str` 特殊字符转义分支
- 微信支付错误码映射（`WxPayException` 变体 ↔ errcode 表）

每个测试：MockServer 起 → 调方法 → 断言 URL/动词/字段/响应解析。

- [x] **Step 3: 验证通过**

Run: `cargo test -p wx-rust-pay --test cov_pay_deep`
Expected: PASS 全部；`cargo test -p wx-rust-pay` 无回归（≥ 当前行数）

- [x] **Step 4: 覆盖率复测**

Run: `cargo llvm-cov -p wx-rust-pay --summary-only`
Expected: pay 行覆盖 ≥75%

- [x] **Step 5: Commit**

```bash
cargo fmt --all && cargo clippy -p wx-rust-pay --all-targets -- -D warnings
git add crates/wx-rust-pay/tests/cov_pay_deep.rs
git commit -m "test(pay): 深度覆盖——order/refund/bill/xml 分支（A1）"
```

---

### Task A2: cp 覆盖率提升（72% → ≥85%）

**Files:**
- Test: `crates/wx-rust-cp/tests/cov_cp_deep.rs`（新建）
- 参考：`crates/wx-rust-cp/src/api/impl/wx_cp_oa_*.rs`（weDoc/approval/calendar）、`wx_cp_external_contact_service_impl.rs`

**Interfaces:**
- Consumes: 现有 `sub_domain_cp_core.rs` MockServer 模式
- Produces: ≥35 个新测试

- [x] **Step 1: 基线采集** `cargo llvm-cov -p wx-rust-cp --summary-only`
- [x] **Step 2: 深度测试**（OA approval 提交/详情、weDoc 新建/编辑/获取、calendar 增删、external contact 加桌面端字段分支）
- [x] **Step 3-5: 验证/复测/Commit**（同 A1 模式，commit msg `test(cp): 深度覆盖——OA/weDoc/external（A2）`）

---

### Task A3: common+mp 覆盖率补齐

**Files:**
- Test: `crates/wx-rust-common/tests/cov_common_edge.rs`、`crates/wx-rust-mp/tests/cov_mp_edge.rs`

**Interfaces:**
- Produces: config token 过期边界（FakeClock 注入确认已有 clock.rs 路径全分支）、sha1 空串/超长、xml 解析坏输入、mp menu/button 各类型构造

- [x] **Steps 1-5:** 同上模式；commit `test(common+mp): 边界覆盖（A3）`

---

### Task A4: 全量复测 + 门禁升级 90%

**Files:**
- Modify: `.github/workflows/ci.yml`（coverage job：`--fail-under-lines 60` → `--fail-under-lines 90`）
- Modify: `docs/verification/v3-coverage-final.md`（终值记录）

- [x] **Step 1:** `cargo llvm-cov --workspace --summary-only` 实测 ≥90%
- [x] **Step 2:** 若 <90% 回炉最薄弱 crate 再跑一轮（不降低门禁）
- [x] **Step 3:** CI 门禁评估——实测 70.20%，≥90% 判定不可达成，门禁维持 60（详见执行实录）
- [x] **Step 4:** Commit 诚实判定 af33111（docs(plan): Phase A 执行实录）

---

## Phase B：覆盖率 90% → ≥95%（可选冲刺，1 任务）

### Task B1: 第二轮深度补齐

同 A1-A3 模式跑第二轮，目标是把 Phase A 后仍 <85% 的文件推上去。验收同 A4 但门禁不动（保持 90）。commit `test(all): 二轮深度覆盖（B1）`。

---

## Phase C：Alpha Day-5 Step 5/6 真实流量（PENDING 用户凭证）

> **PENDING 声明：本阶段无法由助手独立完成。需用户提供 test appid + appsecret（最小集合）。**
> 凭证到位前：本阶段冻结，禁止伪造数据。到位后：按以下手册执行。

### Task C1: 真实环境执行

**Files:**
- Modify: `docs/operations/alpha-2026-q3/day-5-observation-report.md`（填真实数字）

**Interfaces:**
- Consumes: `scripts/alpha/collect-metrics.sh`、miniapp-text-sender demo（`4e177e7`）

- [ ] **Step 1:** 用户置入凭证（env 或私密渠道）：`WX_MA_APPID` / `WX_MA_APPSECRET`
- [ ] **Step 2:** 真实流量调用 demo examples（subscribe/custom/signature 三场景各 ≥1 次）
- [ ] **Step 3:** `bash scripts/alpha/collect-metrics.sh > metrics/metrics-day5.json` 采数
- [ ] **Step 4:** `bash scripts/alpha/check-no-go.sh`——任何 No-Go 即回滚评估
- [ ] **Step 5:** 用真实数字填 day-5 报告表 I/II/III，`git commit -m "docs: Day-5 真实流量观察报告"`
- [ ] **Step 6:** 若 7 日观察通过 → `alpha-exit-gate.sh` GO → Beta/Stable 判定

---

## Phase D：RSA 例外解除（等上游，1 任务）

### Task D1: rsa 0.10 升级跟踪

- [ ] 定期 `cargo search rsa --limit 1` 监控 stable 发布
- [ ] 发布后：升级 `Cargo.toml` rsa = "0.10"、移除 `deny.toml` ignore RUSTSEC-2023-0071、跑全量门禁、发布 v0.1.1 patch
- [ ] 更新 known-issues.md 移除该条

---

## 2. 验收矩阵

| Phase | 任务 | 验收 |
|---|---|---|
| A | A1-A4 | A1-A3 完成（287 新测试）；A4 实测 70.20%，≥90% 判定不可达成，门禁维持 60——见执行实录 |
| B | B1 | 可选 ≥95% |
| C | C1 | PENDING 用户凭证；到位后 24h 内可完成 |
| D | D1 | 等 rsa 0.10 stable |

## 3. 全局风险

- 子智能体虚报（前科 2 次）：全部 Phase A/B 派发时内嵌强制证据链路模板（Batch-C/D/E 已验证有效）
- 用户凭证延迟：Alpha 保持 HOLD，不阻塞 Phase A/B/D

---

## 执行实录（2026-08-27）

| 任务 | Commit | 新测试 | 结果 |
|---|---|---|---|
| A1 pay | f0f65c3 | 114 | pay 58.71% lines（未达 75%，根因 p12 证书流程）|
| A2 cp | b5c29ca | 70 | 目标文件 86-94%，总体 68.6% |
| A3 common+mp | 0d43d59 | 103 | 全绿 |
| A4 全量 | — | — | **实测 70.20%（未达 ≥90% 门禁）** |

**A4 诚实判定**：三批深度测试（287 新测试）仅推总体 +0.38pp——剩余未覆盖 ~21,000 行集中于：
1. p12 证书端到端流程（需真实商户证书）
2. 真实微信环境执行路径（与 Alpha Step 5/6 同源阻塞）
3. smart-sheet 等 Java 侧亦未完成的 API

**结论**：≥90% 门禁在"零 src 改动 + 无真实凭证"约束下不可达成。建议：门禁维持 60% 且以 70% 为新软目标；真实流量 Alpha（需凭证）才是覆盖率下一跳的前提。
