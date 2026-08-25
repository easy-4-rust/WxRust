# Issue Pack: Yellow Items / 黄灯任务包

日期：2026-08-25
版本：WxRust 0.1.0
来源：Production Readiness Checklist（Conditionally Ready 判定，9 项 Yellow 中提取 4 项发布阻断级）
状态：待创建 Issue

---

## 目录

1. [Issue #1: RSA 0.9.10 RUSTSEC-2023-0071 Mitigation 测试与证据](#issue-1)
2. [Issue #2: CHANGELOG.md 建立](#issue-2)
3. [Issue #3: known-issues.md 建立](#issue-3)
4. [Issue #4: 确认并记录 ignored 测试原因](#issue-4)

---

<a id="issue-1"></a>
## Issue #1: RSA 0.9.10 RUSTSEC-2023-0071 Mitigation 测试与证据

- **Title**: `[security] Add RSA 0.9.10 Marvin Attack mitigation test evidence (RUSTSEC-2023-0071)`
- **Labels**: `security`, `supply-chain`, `P0`, `blocked-upstream`
- **Priority**: **P0** -- 发布阻断级，无缓解证据不得发布

### Description

rsa 0.9.10 存在 Marvin Attack 计时侧信道漏洞（RUSTSEC-2023-0071，severity 5.9 medium）。上游尚无稳定修复版本（rsa 0.10 仍为 RC 阶段）。当前 `deny.toml` 已配置 `ignore = ["RUSTSEC-2023-0071"]`，但缺少缓解措施的测试证据。需补充 RSA-OAEP 盲化与固定消息加密的测试，证明在实际攻击场景下密钥恢复难度已显著提高，并将证据文档化。

影响面：`wx-rust-common`、`wx-rust-cp`、`wx-rust-pay`（RSA-OAEP 加解密、SHA256-RSA 签名）。

### Scope

- 为 `rsa_oaep_encrypt`/`rsa_oaep_decrypt`/`sign_sha256_rsa` 补充 mitigation 专项测试
- 测试覆盖：固定消息加密一致性、OAEP 填充随机化验证、不同密钥长度下的行为
- 在 `deny.toml` 中补充缓解措施追踪注释（已有基础注释，需扩展为测试引用）
- 输出 mitigation 证据文档（可放在 `docs/verification/` 下）

### Non-scope

- 不涉及 rsa 版本升级（待 0.10 稳定后单独 Issue）
- 不修改 RSA 使用逻辑的业务代码
- 不替换 RSA 为其他算法

### Definition of Done

1. `crates/wx-rust-pay/tests/` 下存在 `rsa_mitigation_test.rs`（或等效文件），包含至少 3 个 mitigation 场景测试
2. 测试通过 `cargo test --workspace` 全绿
3. `deny.toml` 中 `RUSTSEC-2023-0071` 条目注释引用测试文件路径
4. `docs/verification/` 下有 mitigation 证据文档，记录测试设计、结果、结论
5. CI 中 `cargo deny check` 继续 PASS

### 实施建议

1. 在 `crates/wx-rust-pay/tests/` 新建 `rsa_mitigation_test.rs`
2. 测试场景：
   - **固定消息加密一致性**：同一公钥 + 同一明文，多次加密产出不同密文（验证 OAEP 随机化）
   - **OAEP 解密正确性**：加密后解密还原为原始明文
   - **签名验证往返**：`sign_sha256_rsa` + `verify_sha256_rsa` 往返一致性
3. 复用现有 `platform_private_key()` 测试辅助函数（见 `coverage_boost_pay_service_mock.rs:267`）
4. 证据文档模板：漏洞描述、缓解策略、测试设计、测试结果、剩余风险、升级触发条件

### 风险与缓解

| 风险 | 缓解 |
|------|------|
| rsa 0.10-rc API 变化导致未来迁移成本 | 测试隔离在独立文件，迁移时仅改 crypto utils |
| 测试无法真正证明侧信道不可利用 | 文档明确声明"缓解而非修复"，记录剩余风险 |

### 参考路径

- 漏洞配置：`deny.toml`（`[advisories] ignore`）
- RSA 加解密实现：`crates/wx-rust-pay/src/util/crypto/wx_pay_v3_crypto_utils.rs`（`rsa_oaep_encrypt`/`rsa_oaep_decrypt`/`sign_sha256_rsa`）
- 现有 RSA 测试：`crates/wx-rust-pay/tests/wx_pay_v3_crypto_test.rs`（`rsa_oaep_roundtrip`、`rsa_sign_matches_openssl_golden`）
- 测试辅助：`crates/wx-rust-pay/tests/coverage_boost_pay_service_mock.rs:267`（`platform_private_key()`）
- 安全审计报告：`docs/verification/V4-security-audit.md`
- 验证命令：`cargo test --workspace` / `cargo deny check`

### 工作量估算

0.5 天（测试编写 2h + 证据文档 1h + CI 验证 1h）

---

<a id="issue-2"></a>
## Issue #2: CHANGELOG.md 建立

- **Title**: `[docs] Create CHANGELOG.md with initial 0.1.0 release notes`
- **Labels**: `documentation`, `release`, `P1`
- **Priority**: **P1** -- 发布前必须完成

### Description

项目当前无 CHANGELOG.md。作为首个 0.1.0 版本发布，需建立变更日志文件，记录初始版本的功能范围、已知限制、依赖约束。遵循 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/) 格式，为后续版本迭代提供变更追踪基础。

### Scope

- 在仓库根目录创建 `CHANGELOG.md`
- 记录 0.1.0 版本的：功能概览、模块组成（10 个 crate）、依赖约束（rsa =0.9.10 锁定）、已知限制
- 格式遵循 Keep a Changelog（`Added`/`Changed`/`Deprecated`/`Removed`/`Fixed`/`Security` 分类）
- 中文为主，关键术语保留英文

### Non-scope

- 不追溯历史 commit（首次发布，无历史版本）
- 不修改 README 中的版本说明（但应保持一致）
- 不建立自动 changelog 生成流水线（后续 Issue）

### Definition of Done

1. 仓库根目录存在 `CHANGELOG.md`
2. 包含 `[0.1.0] - 2026-08-XX` 条目（日期与实际发布日一致）
3. 覆盖 `Added`（模块功能）、`Security`（rsa 已知风险）、`Known Limitations` 三个分类
4. `cargo doc --workspace` 构建不因该文件产生 warning
5. README.md 中无与 CHANGELOG 矛盾的版本描述

### 实施建议

1. 参考 Keep a Changelog 1.1.0 格式创建文件
2. `Added` 部分列出 10 个 crate 的核心能力（mp/miniapp/pay/cp/open/channel/aispeech/qidian/common + facade）
3. `Security` 部分记录 rsa 0.9.10 漏洞及缓解状态
4. `Known Limitations` 部分引用 `docs/known-issues.md`（与 Issue #3 联动）
5. 版本号与 `workspace.package.version`（0.1.0）保持一致

### 风险与缓解

| 风险 | 缓解 |
|------|------|
| CHANGELOG 内容与实际代码不一致 | 以 cargo test 全绿 + cargo deny check 为基线 |
| 后续版本忘记更新 CHANGELOG | 在 CI 中添加 CHANGELOG 变更检查（后续 Issue） |

### 参考路径

- 版本号来源：`Cargo.toml`（`workspace.package.version = "0.1.0"`）
- 模块清单：`Cargo.toml`（`workspace.dependencies` 中 9 个内部 crate）
- 安全信息：`deny.toml`（`[advisories]`）、`docs/verification/V4-security-audit.md`
- 生产就绪评估：`docs/verification/production-readiness-checklist-2026-08-25.md`
- 发布计划：`docs/verification/production-release-plan-2026-08-25.md`

### 工作量估算

0.5 天（编写 2h + 团队审查 1h + 与 README 一致性校验 1h）

---

<a id="issue-3"></a>
## Issue #3: known-issues.md 建立

- **Title**: `[docs] Create docs/known-issues.md documenting accepted risks and limitations`
- **Labels**: `documentation`, `risk-management`, `P1`
- **Priority**: **P1** -- 发布前必须完成

### Description

项目当前无已知问题文档。生产就绪评估中多个 Yellow 项（rsa 漏洞、覆盖率边缘、可观测性限制、sync 门面约束）属于"已接受的已知风险"而非阻断缺陷。需建立 `docs/known-issues.md`，将这些风险显式记录，为调用方提供风险透明度，为后续维护提供升级触发条件。

### Scope

- 在 `docs/` 目录创建 `known-issues.md`
- 记录以下已知问题：
  1. rsa 0.9.10 Marvin Attack 漏洞（缓解状态 + 升级触发条件）
  2. 代码覆盖率 61.57%（刚过 60% 门禁，函数覆盖率 41.80%）
  3. SDK 无默认 tracing subscriber 配置
  4. 无内置 metrics / OpenTelemetry 集成
  5. crates.io 发布不可变，回滚仅限 yank
  6. sync 门面通过 `tokio::runtime::Runtime::new()` 创建独立 runtime
- 每个问题包含：描述、影响范围、缓解措施、升级/修复触发条件

### Non-scope

- 不修复任何已知问题（仅记录）
- 不修改代码中的 warning/suppression
- 不建立自动化的已知问题追踪流水线

### Definition of Done

1. `docs/known-issues.md` 存在且格式规范
2. 覆盖上述 6 个已知问题，每个含"触发条件"字段
3. CHANGELOG.md（Issue #2）的 `Known Limitations` 部分引用该文件
4. README.md 中无与 known-issues.md 矛盾的风险描述
5. 每个问题条目有明确的负责人/团队字段

### 实施建议

1. 使用表格或分级列表格式，每个问题包含：ID、标题、严重度、状态、描述、影响、缓解、触发条件、负责人
2. rsa 漏洞条目引用 `deny.toml` 配置和 Issue #1 的 mitigation 测试
3. 覆盖率条目引用 `docs/verification/V3-coverage-verification.md`
4. 可观测性条目说明 SDK 定位（非服务），调用方需自行接入
5. 与 Issue #2 的 CHANGELOG 保持交叉引用

### 风险与缓解

| 风险 | 缓解 |
|------|------|
| 已知问题列表遗漏关键风险 | 以 production-readiness-checklist 的 9 项 Yellow 为基线逐一核对 |
| 文档过时 | 在 README 中添加"已知问题"链接，版本发布时同步审查 |

### 参考路径

- 生产就绪评估（Yellow 清单）：`docs/verification/production-readiness-checklist-2026-08-25.md`（第 356-367 行）
- 覆盖率报告：`docs/verification/V3-coverage-verification.md`
- 安全审计：`docs/verification/V4-security-audit.md`
- deny.toml 配置：`deny.toml`
- 现有 README：`README.md`、`README.zh-CN.md`

### 工作量估算

0.5 天（编写 2h + 与 checklist 交叉核对 1h + 团队审查 1h）

---

<a id="issue-4"></a>
## Issue #4: 确认并记录 ignored 测试原因

- **Title**: `[testing] Document the ignored doctest in attachment_builder.rs with rationale`
- **Labels**: `testing`, `documentation`, `P2`
- **Priority**: **P2** -- 发布前确认，非阻断

### Description

`cargo test --workspace` 结果显示 1977 passed、0 failed、**1 ignored**。该 ignored 测试为 `crates/wx-rust-cp/src/bean/external/msg/attachment_builder.rs` 第 11 行的 doctest，使用 `ignore` 标记而非 `no_run` 或 `compile_fail`。需确认该 doctest 被 ignore 的原因是否合理，补充注释说明，并评估是否可改为 `no_run`（编译但不运行）或 `compile_fail`（预期编译失败）。

### Scope

- 定位并审查 `attachment_builder.rs:11` 的 `ignore` doctest
- 确认 ignore 原因（当前分析：示例代码依赖未导入的类型，无法独立编译）
- 补充行内注释说明 ignore 原因
- 评估是否可改为 `no_run` 或 `compile_fail`
- 如可改进则实施，如不可改进则在注释中记录决策理由

### Non-scope

- 不大规模审查所有 doctest 的 ignore 使用（当前仅 1 个）
- 不修改 `attachment_builder.rs` 的业务逻辑
- 不建立 doctest ignore 审计自动化

### Definition of Done

1. `attachment_builder.rs` 第 11 行的 `ignore` 标记有行内注释说明原因
2. 注释格式：`// SAFETY-IGNORE: <原因>` 或 `// DOC-IGNORE: <原因>`（统一约定）
3. 如改为 `no_run` 或 `compile_fail`，`cargo test --workspace` 仍全绿（0 failed）
4. `cargo test --workspace 2>&1 | grep ignored` 输出中该测试的 ignore 原因可追溯到注释

### 实施建议

1. 读取 `crates/wx-rust-cp/src/bean/external/msg/attachment_builder.rs` 第 11-14 行
2. 当前 doctest 内容：
   ```rust
   //! ```ignore
   //! let attachment = AttachmentBuilder::image_builder()
   //!     .media_id("MEDIA_ID").pic_url("URL").build();
   //! ```
   ```
3. 分析：`image_builder()` 返回 `ImageBuilder`，`.media_id()` 和 `.pic_url()` 方法需确认是否存在；`.build()` 返回 `Attachment`。如果 API 签名正确但 doctest 缺少 `use` 语句，可改为 `no_run`；如果 API 尚未实现，保持 `ignore` 并注释说明
4. 优先尝试改为 `no_run`（编译验证通过但不执行），如编译失败则保持 `ignore` 并添加 `// DOC-IGNORE: ImageBuilder API 尚未完整实现，待 #xxx 完成后改为 no_run`

### 风险与缓解

| 风险 | 缓解 |
|------|------|
| 改为 `no_run` 后编译失败导致 CI 红 | 先本地验证 `cargo test --doc` 再提交 |
| 忘记后续将 `ignore` 改为 `no_run` | 在注释中引用追踪 Issue 编号 |

### 参考路径

- ignored 测试位置：`crates/wx-rust-cp/src/bean/external/msg/attachment_builder.rs:11`
- 验证命令：`cargo test --workspace 2>&1 | grep ignored`
- 当前测试基线：1977 passed, 0 failed, 1 ignored
- 生产就绪评估：`docs/verification/production-readiness-checklist-2026-08-25.md`（G1.2）

### 工作量估算

0.25 天（分析 0.5h + 修改/注释 0.5h + CI 验证 0.5h）

---

## 排期建议

| 优先级 | Issue | 工作量 | 建议排期 |
|--------|-------|--------|----------|
| P0 | #1 RSA mitigation 测试 | 0.5 天 | Day 1（阻断发布） |
| P1 | #2 CHANGELOG.md | 0.5 天 | Day 1（与 #3 并行） |
| P1 | #3 known-issues.md | 0.5 天 | Day 1（与 #2 并行） |
| P2 | #4 ignored 测试确认 | 0.25 天 | Day 2（非阻断） |

**总工作量**：1.75 天（约 2 天，含缓冲）

**关键路径**：Issue #1（P0）-> 确认 mitigation 证据 -> Issue #2 + #3 并行 -> Issue #4 -> 发布前最终 gate check

**依赖关系**：
- Issue #2 的 `Known Limitations` 部分引用 Issue #3 的 `known-issues.md`（#2 应在 #3 完成后定稿）
- Issue #1 的 mitigation 证据文档被 Issue #3 的 rsa 条目引用（#1 应先于 #3 完成）
- Issue #4 独立，无前置依赖
