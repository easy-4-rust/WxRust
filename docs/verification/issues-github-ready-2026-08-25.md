# GitHub Issues Ready-to-Paste / GitHub 直接粘贴版

日期：2026-08-25
用法：每 Issue 用 `---` 分隔，直接复制标题和正文到 GitHub Issue 创建页面

---

## Issue 1

**Title**: `[security] Add RSA 0.9.10 Marvin Attack mitigation test evidence (RUSTSEC-2023-0071)`

**Labels**: `security`, `supply-chain`, `P0`, `blocked-upstream`

**Body**:

### 背景

rsa 0.9.10 存在 Marvin Attack 计时侧信道漏洞（RUSTSEC-2023-0071，severity 5.9 medium）。上游尚无稳定修复版本（rsa 0.10 仍为 RC）。当前 `deny.toml` 已配置 `ignore = ["RUSTSEC-2023-0071"]`，但缺少缓解措施的测试证据。

影响面：`wx-rust-common`、`wx-rust-cp`、`wx-rust-pay`（RSA-OAEP 加解密、SHA256-RSA 签名）。

### 目标

补充 RSA-OAEP 盲化与固定消息加密的 mitigation 专项测试，证明缓解措施有效，并输出证据文档。

### Scope

- 为 `rsa_oaep_encrypt`/`rsa_oaep_decrypt`/`sign_sha256_rsa` 补充 mitigation 专项测试
- 测试覆盖：固定消息加密一致性、OAEP 填充随机化验证、签名验证往返
- 在 `deny.toml` 中补充缓解措施追踪注释（引用测试文件路径）
- 输出 mitigation 证据文档至 `docs/verification/`

### Non-scope

- 不涉及 rsa 版本升级（待 0.10 稳定后单独 Issue）
- 不修改 RSA 使用逻辑的业务代码

### 验收标准（Definition of Done）

- [ ] `crates/wx-rust-pay/tests/` 下存在 `rsa_mitigation_test.rs`，包含至少 3 个 mitigation 场景测试
- [ ] `cargo test --workspace` 全绿（0 failed）
- [ ] `deny.toml` 中 `RUSTSEC-2023-0071` 条目注释引用测试文件路径
- [ ] `docs/verification/` 下有 mitigation 证据文档
- [ ] `cargo deny check` 继续 PASS

### 实施建议

1. 在 `crates/wx-rust-pay/tests/` 新建 `rsa_mitigation_test.rs`
2. 测试场景：固定消息加密一致性（同一明文多次加密产出不同密文）、OAEP 解密正确性、签名验证往返
3. 复用现有 `platform_private_key()` 测试辅助函数（`coverage_boost_pay_service_mock.rs:267`）
4. 证据文档包含：漏洞描述、缓解策略、测试设计、测试结果、剩余风险、升级触发条件

### 参考路径

- `deny.toml` -- 漏洞例外配置
- `crates/wx-rust-pay/src/util/crypto/wx_pay_v3_crypto_utils.rs` -- RSA 实现
- `crates/wx-rust-pay/tests/wx_pay_v3_crypto_test.rs` -- 现有 RSA 测试
- `docs/verification/V4-security-audit.md` -- 安全审计报告

### 工作量

0.5 天 | **P0 -- 发布阻断级**

---

## Issue 2

**Title**: `[docs] Create CHANGELOG.md with initial 0.1.0 release notes`

**Labels**: `documentation`, `release`, `P1`

**Body**:

### 背景

项目当前无 CHANGELOG.md。作为首个 0.1.0 版本发布，需建立变更日志文件，记录初始版本的功能范围、已知限制、依赖约束。

### 目标

在仓库根目录创建 CHANGELOG.md，遵循 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/) 格式。

### Scope

- 仓库根目录创建 `CHANGELOG.md`
- 记录 0.1.0 版本：功能概览（10 个 crate）、依赖约束（rsa =0.9.10 锁定）、已知限制
- 格式遵循 Keep a Changelog（`Added`/`Security`/`Known Limitations` 分类）

### Non-scope

- 不追溯历史 commit（首次发布）
- 不建立自动 changelog 生成流水线

### 验收标准（Definition of Done）

- [ ] 仓库根目录存在 `CHANGELOG.md`
- [ ] 包含 `[0.1.0] - 2026-08-XX` 条目
- [ ] `Added` 部分列出 10 个 crate 的核心能力
- [ ] `Security` 部分记录 rsa 0.9.10 漏洞及缓解状态
- [ ] `Known Limitations` 引用 `docs/known-issues.md`
- [ ] README.md 中无与 CHANGELOG 矛盾的版本描述

### 实施建议

1. 参考 Keep a Changelog 1.1.0 格式
2. `Added` 列出 mp/miniapp/pay/cp/open/channel/aispeech/qidian/common + facade
3. `Security` 记录 rsa 0.9.10 Marvin Attack 缓解状态
4. `Known Limitations` 引用 `docs/known-issues.md`（与 Issue #3 联动）
5. 版本号与 `Cargo.toml`（`workspace.package.version = "0.1.0"`）一致

### 参考路径

- `Cargo.toml` -- 版本号与 crate 清单
- `deny.toml` -- 安全配置
- `docs/verification/production-readiness-checklist-2026-08-25.md` -- 生产就绪评估
- `docs/verification/production-release-plan-2026-08-25.md` -- 发布计划

### 工作量

0.5 天 | **P1 -- 发布前必须完成**

---

## Issue 3

**Title**: `[docs] Create docs/known-issues.md documenting accepted risks and limitations`

**Labels**: `documentation`, `risk-management`, `P1`

**Body**:

### 背景

生产就绪评估中多个 Yellow 项属于"已接受的已知风险"。需建立 `docs/known-issues.md`，将风险显式记录，为调用方提供透明度，为后续维护提供升级触发条件。

### 目标

创建已知问题文档，覆盖所有已接受的 Yellow 项风险。

### Scope

- `docs/` 目录创建 `known-issues.md`
- 记录以下已知问题：
  1. rsa 0.9.10 Marvin Attack（缓解状态 + 升级触发条件）
  2. 代码覆盖率 61.57%（函数覆盖率 41.80%）
  3. SDK 无默认 tracing subscriber 配置
  4. 无内置 metrics / OpenTelemetry 集成
  5. crates.io 发布不可变，回滚仅限 yank
  6. sync 门面通过 `tokio::runtime::Runtime::new()` 创建独立 runtime
- 每个问题含：描述、影响范围、缓解措施、升级/修复触发条件、负责人

### Non-scope

- 不修复任何已知问题（仅记录）
- 不建立自动化追踪流水线

### 验收标准（Definition of Done）

- [ ] `docs/known-issues.md` 存在且格式规范
- [ ] 覆盖上述 6 个已知问题，每个含"触发条件"字段
- [ ] CHANGELOG.md 的 `Known Limitations` 部分引用该文件
- [ ] README.md 中无与 known-issues.md 矛盾的风险描述
- [ ] 每个条目有负责人/团队字段

### 实施建议

1. 使用表格格式，字段：ID、标题、严重度、状态、描述、影响、缓解、触发条件、负责人
2. rsa 条目引用 `deny.toml` 和 Issue #1 的 mitigation 测试
3. 覆盖率条目引用 `docs/verification/V3-coverage-verification.md`
4. 可观测性条目说明 SDK 定位（非服务），调用方需自行接入
5. 与 Issue #2 的 CHANGELOG 保持交叉引用

### 参考路径

- `docs/verification/production-readiness-checklist-2026-08-25.md` -- Yellow 清单（第 356-367 行）
- `docs/verification/V3-coverage-verification.md` -- 覆盖率报告
- `docs/verification/V4-security-audit.md` -- 安全审计
- `deny.toml` -- 漏洞例外配置
- `README.md` / `README.zh-CN.md` -- 现有文档

### 工作量

0.5 天 | **P1 -- 发布前必须完成**

---

## Issue 4

**Title**: `[testing] Document the ignored doctest in attachment_builder.rs with rationale`

**Labels**: `testing`, `documentation`, `P2`

**Body**:

### 背景

`cargo test --workspace` 结果：1977 passed, 0 failed, **1 ignored**。该 ignored 测试为 `crates/wx-rust-cp/src/bean/external/msg/attachment_builder.rs` 第 11 行的 doctest，使用 `ignore` 标记。需确认原因并补充注释。

当前 doctest 内容：
```rust
//! ```ignore
//! let attachment = AttachmentBuilder::image_builder()
//!     .media_id("MEDIA_ID").pic_url("URL").build();
//! ```
```

### 目标

确认 ignore 原因，补充行内注释，评估是否可改为 `no_run` 或 `compile_fail`。

### Scope

- 审查 `attachment_builder.rs:11` 的 `ignore` doctest
- 确认 ignore 原因（示例代码依赖未导入类型 / API 未完整实现）
- 补充行内注释说明 ignore 原因
- 评估改为 `no_run` 或 `compile_fail` 的可行性

### Non-scope

- 不大规模审查所有 doctest
- 不修改 `attachment_builder.rs` 的业务逻辑

### 验收标准（Definition of Done）

- [ ] `attachment_builder.rs` 第 11 行的 `ignore` 有行内注释说明原因
- [ ] 注释格式统一：`// DOC-IGNORE: <原因>`
- [ ] 如改为 `no_run` 或 `compile_fail`，`cargo test --workspace` 仍全绿
- [ ] `cargo test --workspace 2>&1 | grep ignored` 结果可追溯到注释

### 实施建议

1. 读取 `crates/wx-rust-cp/src/bean/external/msg/attachment_builder.rs` 第 11-14 行
2. 分析 `ImageBuilder` 的 `media_id()`/`pic_url()`/`build()` 方法是否已实现
3. 如 API 完整：改为 `no_run`（编译验证但不执行）
4. 如 API 不完整：保持 `ignore`，添加 `// DOC-IGNORE: ImageBuilder API 尚未完整实现，待后续版本改为 no_run`
5. 本地验证 `cargo test --doc` 通过后再提交

### 参考路径

- `crates/wx-rust-cp/src/bean/external/msg/attachment_builder.rs:11` -- ignored 测试
- `cargo test --workspace 2>&1 | grep ignored` -- 验证命令
- `docs/verification/production-readiness-checklist-2026-08-25.md` -- G1.2 条目

### 工作量

0.25 天 | **P2 -- 发布前确认，非阻断**
