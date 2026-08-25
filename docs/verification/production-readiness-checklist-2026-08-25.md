# Production Readiness Checklist / 生产就绪检查清单

日期：2026-08-25
版本：WxRust 0.1.0
审查人：SRE Agent
方法论：基于证据的门禁审查（Gate Review），每项附实际命令输出

---

## 目录

1. [Build & Test / 构建与测试](#1-build--test--构建与测试)
2. [Security & Supply Chain / 安全与供应链](#2-security--supply-chain--安全与供应链)
3. [Observability / 可观测性](#3-observability--可观测性)
4. [Reliability & Concurrency / 可靠性与并发](#4-reliability--concurrency--可靠性与并发)
5. [Release & Publish / 发布与发布流程](#5-release--publish--发布与发布流程)
6. [Rollback / 回滚能力](#6-rollback--回滚能力)
7. [Documentation / 文档](#7-documentation--文档)
8. [Overall Verdict / 总体判定](#8-overall-verdict--总体判定)

---

## 1. Build & Test / 构建与测试

### G1.1 Workspace 编译

| 字段 | 内容 |
|------|------|
| 证据 | `cargo check --workspace` -- Finished |
| 评分 | **Green** |
| 差距 | 无 |
| 负责人 | -- |
| 行动 | 无需行动 |

### G1.2 单元/集成测试

| 字段 | 内容 |
|------|------|
| 证据 | `cargo test --workspace` -- 1977 passed, 0 failed, 1 ignored; 121 个测试目标全绿 |
| 评分 | **Green** |
| 差距 | 1 个 ignored 测试（需确认是否为有意跳过） |
| 负责人 | Backend |
| 行动 | 确认 ignored 测试的原因；如为环境依赖则补充条件编译说明 |

### G1.3 Clippy 静态分析

| 字段 | 内容 |
|------|------|
| 证据 | `cargo clippy --workspace --all-targets -- -D warnings` -- Finished，0 warnings |
| 评分 | **Green** |
| 差距 | 无 |
| 负责人 | -- |
| 行动 | 无需行动 |

### G1.4 格式化

| 字段 | 内容 |
|------|------|
| 证据 | `cargo fmt --all -- --check` -- exit 0（V1 报告确认） |
| 评分 | **Green** |
| 差距 | 无 |
| 负责人 | -- |
| 行动 | 无需行动 |

### G1.5 代码覆盖率

| 字段 | 内容 |
|------|------|
| 证据 | `cargo llvm-cov --workspace --fail-under-lines 60` -- 61.57% 行覆盖率，exit 0（V3 报告） |
| 评分 | **Yellow** |
| 差距 | 61.57% 刚过 60% 门禁，剩余 25,523 行未覆盖；函数覆盖率仅 41.80% |
| 负责人 | Backend / QA |
| 行动 | 逐步提升至 70%+；优先覆盖 pay 固定域名路径、并发锁竞争分支 |

### G1.6 迁移完整性（对象覆盖率）

| 字段 | 内容 |
|------|------|
| 证据 | `python3 scripts/audit_migration_layout.py` -- 3287/3287 对象已处置（100.0%），0 MISSING |
| 评分 | **Green** |
| 差距 | 无 |
| 负责人 | -- |
| 行动 | 无需行动 |

---

## 2. Security & Supply Chain / 安全与供应链

### G2.1 漏洞扫描

| 字段 | 内容 |
|------|------|
| 证据 | `cargo audit` -- 1 vulnerability: rsa 0.9.10 (RUSTSEC-2023-0071, medium, Marvin Attack) |
| 评分 | **Yellow** |
| 差距 | rsa 0.9.10 存在计时侧信道漏洞（severity 5.9 medium），无修复版本可用（0.10 尚为 RC） |
| 负责人 | Security / Backend |
| 行动 | 已在 deny.toml 配置已知风险例外；RSA-OAEP 盲化 + 固定消息加密作为缓解；**待 rsa 0.10 稳定后立即升级** |

### G2.2 依赖策略合规

| 字段 | 内容 |
|------|------|
| 证据 | `cargo deny check` -- advisories ok, bans ok, licenses ok, sources ok |
| 评分 | **Green** |
| 差距 | 无（deny.toml 已配置许可证白名单、来源限制、已知漏洞例外） |
| 负责人 | -- |
| 行动 | 无需行动 |

### G2.3 unsafe 代码策略

| 字段 | 内容 |
|------|------|
| 证据 | workspace.lints.rust: `unsafe_code = "forbid"`; 全部 10 个 crate 的 lib.rs 均有 `#![forbid(unsafe_code)]`; grep 未发现任何实际 unsafe 使用 |
| 评分 | **Green** |
| 差距 | 无 |
| 负责人 | -- |
| 行动 | 无需行动 |

### G2.4 CI 安全门禁

| 字段 | 内容 |
|------|------|
| 证据 | `.github/workflows/ci.yml` 包含独立 `audit` job：cargo audit + cargo deny check |
| 评分 | **Green** |
| 差距 | 无 |
| 负责人 | -- |
| 行动 | 无需行动 |

---

## 3. Observability / 可观测性

### G3.1 结构化日志

| 字段 | 内容 |
|------|------|
| 证据 | workspace 依赖 `tracing = "0.1.44"`；各 crate 使用 `tracing` 记录关键路径 |
| 评分 | **Yellow** |
| 差距 | 作为 SDK（非服务），tracing 依赖存在但**无默认 subscriber 配置**；调用方需自行接入 tracing-subscriber；无 tracing-opentelemetry 集成 |
| 负责人 | Backend |
| 行动 | 在 README 中补充 tracing 接入示例；考虑提供 `env_logger` / `tracing-subscriber` feature flag |

### G3.2 指标（Metrics）

| 字段 | 内容 |
|------|------|
| 证据 | 无 metrics crate 依赖；无 Prometheus/StatsD 集成 |
| 评分 | **Yellow** |
| 差距 | SDK 不导出请求延迟、错误率、token 刷新次数等关键指标；调用方无法量化 SDK 行为 |
| 负责人 | Backend |
| 行动 | 评估是否在 SDK 层提供可选 metrics feature（reqwest 中间件 + prometheus crate）；如不提供则在文档中说明调用方自行埋点方案 |

### G3.3 分布式追踪

| 字段 | 内容 |
|------|------|
| 证据 | 无 OpenTelemetry / tracing-opentelemetry 依赖 |
| 评分 | **Yellow** |
| 差距 | 作为 SDK，无法生成 span；调用方需在调用 SDK 方法前后手动创建 span |
| 负责人 | Backend |
| 行动 | 评估 tracing span 注入（可选 feature）；当前可接受为 SDK 的已知限制 |

### G3.4 错误分类与可观测性

| 字段 | 内容 |
|------|------|
| 证据 | `thiserror = "2.0.18"` 驱动的结构化错误类型（WxErrorException、WxPayException 等）；错误枚举覆盖 API 错误、网络错误、加密错误 |
| 评分 | **Green** |
| 差距 | 无 |
| 负责人 | -- |
| 行动 | 无需行动 |

---

## 4. Reliability & Concurrency / 可靠性与并发

### G4.1 block_on 门禁

| 字段 | 内容 |
|------|------|
| 证据 | `scripts/check_block_on.sh` -- "block_on 门禁通过"；block_on 仅出现在 `blocking.rs`（sync 门面） |
| 评分 | **Green** |
| 差距 | 无 |
| 负责人 | -- |
| 行动 | 无需行动 |

### G4.2 并发验收测试

| 字段 | 内容 |
|------|------|
| 证据 | CI 包含并发 benchmark step：`wx-rust-common` pipeline + circuit breaker（1000 并发共享未过期 token + 熔断开合）、`wx-rust-miniapp` token single-flight（1000 并发 token 刷新，断言端点应答数 == 1） |
| 评分 | **Green** |
| 差距 | 无 |
| 负责人 | -- |
| 行动 | 无需行动 |

### G4.3 async 运行时安全

| 字段 | 内容 |
|------|------|
| 证据 | async 路径使用 tokio；sync 门面通过 `tokio::runtime::Runtime::new()` 创建独立 runtime，不依赖外部 runtime；workspace 依赖 tokio features = ["io-util", "macros", "net", "rt", "sync", "time"] |
| 评分 | **Green** |
| 差距 | 无 |
| 负责人 | -- |
| 行动 | 无需行动 |

### G4.4 HTTP 客户端可靠性

| 字段 | 内容 |
|------|------|
| 证据 | reqwest + rustls（非 openssl）；retry/pipeline/circuit-breaker 在 wx-rust-common 中实现 |
| 评分 | **Green** |
| 差距 | 无 |
| 负责人 | -- |
| 行动 | 无需行动 |

---

## 5. Release & Publish / 发布与发布流程

### G5.1 crates.io 发布配置

| 字段 | 内容 |
|------|------|
| 证据 | workspace.dependencies 全部 9 个内部 crate 声明 `{ version = "0.1.0", path = "crates/X" }`；各 crate 使用 `X.workspace = true`；V6 报告确认 `cargo publish --dry-run` 对 common 和 facade 打包成功 |
| 评分 | **Green** |
| 差距 | 无 |
| 负责人 | -- |
| 行动 | 无需行动 |

### G5.2 发布顺序

| 字段 | 内容 |
|------|------|
| 证据 | 发布计划文档化（Wave 1: common; Wave 2: 7 个业务 crate; Wave 3: open + facade）；V6 报告记录依赖拓扑约束 |
| 评分 | **Green** |
| 差距 | 无 |
| 负责人 | Release |
| 行动 | 按计划执行 Wave 1/2/3 发布序列 |

### G5.3 版本号一致性

| 字段 | 内容 |
|------|------|
| 证据 | workspace.package.version = "0.1.0"；全部 crate 统一版本 |
| 评分 | **Green** |
| 差距 | 无 |
| 负责人 | -- |
| 行动 | 无需行动 |

### G5.4 MSRV 声明

| 字段 | 内容 |
|------|------|
| 证据 | workspace.package.rust-version = "1.85"；README 标注 MSRV 1.85；本地 rustc 1.97.1 |
| 评分 | **Green** |
| 差距 | 无 |
| 负责人 | -- |
| 行动 | 无需行动 |

### G5.5 发布前冒烟测试

| 字段 | 内容 |
|------|------|
| 证据 | CI 流水线：check -> test（含 block_on gate + 并发 bench）-> clippy -> fmt -> coverage -> audit；全部 gate 串联 |
| 评分 | **Green** |
| 差距 | 无 |
| 负责人 | -- |
| 行动 | 无需行动 |

---

## 6. Rollback / 回滚能力

### G6.1 crates.io 回滚机制

| 字段 | 内容 |
|------|------|
| 证据 | crates.io 发布不可变；`cargo yank` 可标记版本为不推荐（不删除已下载的 crate） |
| 评分 | **Yellow** |
| 差距 | 无真正的"回滚"——yank 仅阻止新项目依赖，已依赖的项目不受影响；首个版本 0.1.0 发布后如有严重缺陷需发布 0.1.1 修复 |
| 负责人 | Release |
| 行动 | 发布前确认所有 gate 全绿；准备 0.1.1 hotfix 分支模板；yank 操作流程写入 Runbook |

### G6.2 版本升级策略

| 字段 | 内容 |
|------|------|
| 证据 | 无 CHANGELOG.md；无 semver 兼容性策略文档 |
| 评分 | **Yellow** |
| 差距 | 作为 0.x 版本，semver 允许任意 breaking change，但缺乏升级指南和变更日志 |
| 负责人 | Backend / Docs |
| 行动 | 创建 CHANGELOG.md；在 README 中补充版本升级说明 |

---

## 7. Documentation / 文档

### G7.1 README

| 字段 | 内容 |
|------|------|
| 证据 | README.md（英文）+ README.zh-CN.md（中文）双语；包含模块图、架构、Quick Start、Features、Quality Gates、Compatibility |
| 评分 | **Green** |
| 差距 | 无 |
| 负责人 | -- |
| 行动 | 无需行动 |

### G7.2 API 文档

| 字段 | 内容 |
|------|------|
| 证据 | Rust doc comments 覆盖主要 trait/struct/enum；`cargo doc --workspace` 可生成 |
| 评分 | **Yellow** |
| 差距 | 未验证 `cargo doc --workspace --no-deps` 是否有 warning；部分生成代码的 doc 可能缺失 |
| 负责人 | Backend |
| 行动 | 运行 `cargo doc --workspace --no-deps 2>&1 | grep warning` 验证并修复 |

### G7.3 验证文档

| 字段 | 内容 |
|------|------|
| 证据 | 7 份验证报告：V0（缺口清零）、V1（工程验证）、V3（覆盖率）、V4（安全审计）、V6（发布验证）、aispeech 接线审计、生产发布计划 |
| 评分 | **Green** |
| 差距 | 无 |
| 负责人 | -- |
| 行动 | 无需行动 |

### G7.4 Runbook / 故障响应

| 字段 | 内容 |
|------|------|
| 证据 | 无 Runbook 文档；无故障响应流程 |
| 评分 | **Yellow** |
| 差距 | 作为 SDK（非服务），Runbook 需求较低；但应有"已知问题与规避"文档（如 rsa 漏洞缓解、block_on 使用限制） |
| 负责人 | Docs |
| 行动 | 创建 docs/known-issues.md，记录 rsa 漏洞、block_on 限制、sync 门面使用注意事项 |

---

## 8. Overall Verdict / 总体判定

### 评分汇总

| Gate | Green | Yellow | Red |
|------|-------|--------|-----|
| Build & Test | 5 | 1 | 0 |
| Security & Supply Chain | 3 | 1 | 0 |
| Observability | 1 | 3 | 0 |
| Reliability & Concurrency | 4 | 0 | 0 |
| Release & Publish | 5 | 0 | 0 |
| Rollback | 0 | 2 | 0 |
| Documentation | 2 | 2 | 0 |
| **合计** | **20** | **9** | **0** |

### Yellow 项清单（需决策层确认风险接受）

1. **G1.5 代码覆盖率** -- 61.57% 刚过门禁，函数覆盖率 41.80% 偏低
2. **G2.1 漏洞扫描** -- rsa 0.9.10 medium 漏洞无修复版本（已缓解，已知风险）
3. **G3.1 结构化日志** -- SDK 依赖 tracing 但无默认 subscriber 配置
4. **G3.2 指标** -- 无内置 metrics 导出
5. **G3.3 分布式追踪** -- 无 OpenTelemetry 集成
6. **G6.1 回滚机制** -- crates.io 发布不可变，仅 yank
7. **G6.2 版本升级策略** -- 无 CHANGELOG、无 semver 兼容性策略
8. **G7.2 API 文档** -- 未验证 cargo doc 无 warning
9. **G7.4 Runbook** -- 无已知问题文档

### 判定

> **Conditionally Ready / 有条件就绪**
>
> 理由：
> - 构建、测试、安全（除 rsa 已知风险）、并发、发布流程全部 Green -- 工程基础扎实
> - 0 个 Red 项 -- 无阻断性缺陷
> - 9 个 Yellow 项中，**G2.1（rsa 漏洞）** 和 **G1.5（覆盖率）** 为核心风险点，其余为 SDK 类型的已知限制
> - 作为 0.1.0 首发 SDK，可观测性/回滚的 Yellow 项属于"可接受的已知限制"而非阻断项
>
> **发布前必须完成**：
> 1. 确认 rsa 漏洞缓解措施已验证（RSA-OAEP 盲化 + 固定消息加密）
> 2. 创建 CHANGELOG.md 和 docs/known-issues.md
> 3. 确认 ignored 测试的原因
>
> **发布后 30 天内完成**：
> 1. 覆盖率提升至 70%+
> 2. 补充 tracing 接入示例文档
> 3. 验证 `cargo doc` 无 warning
