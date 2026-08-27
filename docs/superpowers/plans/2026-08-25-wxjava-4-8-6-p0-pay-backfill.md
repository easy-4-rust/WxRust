# WxJava 4.8.4→4.8.6 P0 追补：点金计划/转账授权/V3电子发票实施计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**状态：** 已完成（核对日期：2026-08-27，依据：P0 三 commit 7691dbe/fef8c3b/0fbb529 + P1/P2/P3 收口；终态报告 docs/verification/wxjava-4-8-6-p0-p3-backfill-completion-report-2026-08-25.md）

**Goal:** 把 WxJava 4.8.5/4.8.6 中新增的 P0 商业能力（微信支付点金计划 + 商家转账用户授权免确认 + V3 服务商电子发票）补齐至 WxRust，使其与 WxJava 4.8.6 的支付模块保持功能对齐，并通过编译/测试/审计门禁。

**Architecture:** 支付模块的新增以 `WxPayService` trait 新增默认方法 + 子服务 bean（serde derive）为骨架，所有新接口保持 `async fn -> Result<T, WxErrorException>`；bean 字段遵循 Java `@SerializedName` 名称直接映射至 `serde(rename = "...")`。HTTP mock 使用现有 `httpmock` dev-dep；测试遵循三层规范（SOURCE_PARITY / RUST_OBLIGATION / VALUE_ADD）。

**Tech Stack:** reqwest（已存在）、serde/serde_json（已存在）、thiserror（已存在）、httpmock（已在 pay dev-deps）、cargo test/clippy/fmt。

**Source Commit:** WxJava `v4.8.4..v4.8.6`（59 commits；新增主源 199 文件）。
**Target:** WxRust HEAD `8ecbfac`（4.8.4→4.8.6 diff 分析已落盘）。

## Global Constraints

- 语义保真：bean 字段名、方法签名、错误码、URL 常量全部与 Java 4.8.6 逐字节一致。
- `cargo test --workspace`（当前 ~1991 tests）必须保持 ≥1991。
- `cargo clippy --workspace --all-targets -- -D warnings` 干净。
- `cargo fmt --all` 干净。
- 每个新增文件遵循"WxRust Rust 项目规范"：snake_case 文件名 + PascalCase 类型 + `/// 对应 Java:<ClassName>` 注释。
- 覆盖率门禁：`cargo llvm-cov --workspace --fail-under-lines 60`（V3 门禁）。
- 禁止使用 `todo!()` / `unimplemented!()` 充数：每个默认方法必须实现真实 HTTP mock 测试覆盖（或显式标注暂不实现并给出原因）。

## 1. 已确认需求

1. **P0 #1 点金计划**：`GoldPlanService` trait + `GoldPlanServiceImpl` + `GoldPlanResult` bean + 测试
2. **P0 #2 商家转账用户授权**：`TransferService` 新增方法（6 个）+ 25 个 bean + 测试
3. **P0 #3 V3 服务商电子发票**：`PartnerInvoiceService` + `PartnerInvoiceServiceImpl` + 19 bean + 测试

## 2. 已确认非目标

- P1 channel 新功能骨架（后续会话）
- P2/P3 长尾 bean（后续会话）
- 不改现有 API 签名

---

### Task 1: P0 #1 GoldPlanService

**Files:**
- Create: `crates/wx-rust-pay/src/service/gold_plan_service.rs`
- Create: `crates/wx-rust-pay/src/api/impl/gold_plan_service_impl.rs`
- Create: `crates/wx-rust-pay/src/bean/goldplan/gold_plan_result.rs`
- Modify: `crates/wx-rust-pay/src/api/mod.rs` (add `pub mod gold_plan_service`)
- Modify: `crates/wx-rust-pay/src/service/mod.rs` (add `pub mod gold_plan_service`)
- Modify: `crates/wx-rust-pay/src/bean/mod.rs` (add `pub mod goldplan`)
- Test: `crates/wx-rust-pay/tests/gold_plan_test.rs`

**Interfaces (from WxJava):**
- `GoldPlanService`: trait with methods for get/move/query/suspend/resume plus get-bind/get-bind results
- `GoldPlanResult`: bean with `errcode`, `errmsg`, `reason` fields
- `GoldPlanServiceImpl`: async fn implementations using WxPayService base HTTP

- [x] **Step 1: 写失败测试** — Create `gold_plan_test.rs` with 4 tests (get, move, suspend, resume) using httpmock
- [x] **Step 2: 运行确认失败** — `cargo test -p wx-rust-pay --test gold_plan_test` → FAIL
- [x] **Step 3: 实现 bean + service + impl** — minimum code to pass all 4 tests
- [x] **Step 4: 运行测试通过 + 全量回归** — `cargo test -p wx-rust-pay` AND `cargo test --workspace` 均绿
- [x] **Step 5: clippy/fmt + Commit** — `feat(pay): P0 #1 GoldPlanService 点金计划接口`（commit 7691dbe，实现先于本计划落地，本任务验证 + fmt 收尾 d63c53c）

---

### Task 2: P0 #2 TransferService 新增方法

**Files:**
- Modify: `crates/wx-rust-pay/src/service/transfer_service.rs` (add 6 new methods)
- Modify: `crates/wx-rust-pay/src/api/impl/transfer_service_impl.rs` (implement new methods)
- Create: 25 bean files under `crates/wx-rust-pay/src/bean/transfer/`
- Test: `crates/wx-rust-pay/tests/transfer_authorization_test.rs`

**Interfaces (from WxJava `TransferService.java` new methods):**
- `pre_transfer_with_authorization(params) -> PreTransferWithAuthorizationResult`
- `transfer_bills_after_authorization(params) -> TransferBillsAfterAuthorizationResult`
- `transfer_bills_request(params) -> TransferBillsRequest`
- `user_authorization_notify_result` / `user_confirm_authorization`
- `transfer_bills_request/result` + `user_authorization_notify_result`

- [x] **Step 1: 写失败测试** — 8 tests covering all 6 new methods + error paths
- [x] **Step 2: 确认失败** — `cargo test -p wx-rust-pay --test transfer_authorization_test` → FAIL
- [x] **Step 3: 实现 bean + service methods + impl** — minimum code
- [x] **Step 4: 测试通过 + 全量回归**
- [x] **Step 5: clippy/fmt + Commit** — `feat(pay): P0 #2 TransferService 商家转账用户授权接口`

---

### Task 3: P0 #3 PartnerInvoiceService V3 服务商电子发票

**Files:**
- Create: `crates/wx-rust-pay/src/service/partner_invoice_service.rs`
- Create: `crates/wx-rust-pay/src/api/impl/partner_invoice_service_impl.rs`
- Create: 19 bean files under `crates/wx-rust-pay/src/bean/invoice/`
- Test: `crates/wx-rust-pay/tests/partner_invoice_test.rs`

**Interfaces (from WxJava `PartnerInvoiceService.java`):**
- methods for invoice creation, query, cancellation, PDF generation, etc. — each maps to a bean request/result
- All HTTP calls via `WxPayService::post` (v3 JSON path)

- [x] **Step 1: 写失败测试** — 10+ tests covering core methods
- [x] **Step 2: 确认失败**
- [x] **Step 3: 实现 bean + service + impl**
- [x] **Step 4: 测试通过 + 全量回归**
- [x] **Step 5: clippy/fmt + Commit** — `feat(pay): P0 #3 PartnerInvoiceService V3 服务商电子发票`

---

## 3. 验收矩阵

| 任务 | 交付 | 门禁 |
|---|---|---|
| Task 1 | GoldPlanService 测试全绿 | cargo test -p wx-rust-pay |
| Task 2 | TransferService 新方法测试全绿 | cargo test -p wx-rust-pay |
| Task 3 | PartnerInvoiceService 测试全绿 | cargo test -p wx-rust-pay |
| 全量 | 1991+ tests + clippy/fmt | cargo test/clippy/fmt（实际：**2516 tests 全绿**、clippy 9 crate 严格通过、fmt clean） |

## 实际完成数据（2026-08-25 收口）

- **P0**（3 commit）：7691dbe / fef8c3b / 0fbb529 —— 点金计划 + 转账授权 + V3 电子发票，48 新测试
- **P1**（9a44167）：channel 10 service 骨架 + 68 bean + 20 smoke 测试
- **P2/P3**（bcf8ae2）：pay/cp/common 长尾 bean
- **覆盖率**：69.05% line（较追补前 61.57% +7.5pp，超 60% 门禁）
- **V0 审计**：3287/3287（100%），0 MISSING
- **终态报告**：docs/verification/wxjava-4-8-6-p0-p3-backfill-completion-report-2026-08-25.md
- **遗留**（诚实声明）：P1 为骨架级对齐（非完整语义）；P3 的智能机器人 API 模式 / 待办 API impl 层待后续会话

## 4. 风险与回滚

- 每个 Task 独立可回滚（独立提交）
- 若某个 bean 字段映射不确定，以 Java 源码为准并记录
- 若 httpmock 配置失败，降级为纯单元测试（bean serde roundtrip）
