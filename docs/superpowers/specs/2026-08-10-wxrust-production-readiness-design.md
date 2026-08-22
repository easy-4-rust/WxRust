# WxRust 生产就绪设计

日期：2026-08-10
状态：进行中
来源：`docs/PRODUCTION_READINESS_PLAN.md`

## 1. 背景

WxRust 当前处于 Experimental 阶段（骨架阶段），测试覆盖率低，缺少 CI/CD 流水线和安全审计。要达到生产就绪标准，需补齐测试缺口、建立 CI/CD 门禁、完成安全审计和性能基准。

## 2. 目标与非目标

### 2.1 目标

- 测试覆盖率 >= 60% line（cargo-llvm-cov）。
- CI/CD 流水线：cargo test + clippy + llvm-cov + audit。
- 安全审计：cargo audit + cargo deny 无高危漏洞。
- 发布验证：所有 crate cargo publish --dry-run 成功。
- 性能基准：criterion benchmark for token/execute。
- Redis 集成测试：testcontainers + Redis 后端验证。

### 2.2 非目标

- 不追求 100% 行覆盖率。
- 不在 v1 实现分布式事务或跨进程持久化。

## 3. 方案比较

### 3.1 手动测试 + 人工审计

每次发布前手动运行测试和审计。优点是简单；缺点是不可靠、不可重复、容易遗漏。

本方案不采用。

### 3.2 CI/CD 自动化 + 门禁

GitHub Actions 自动运行测试、clippy、覆盖率、审计，设置门禁（不通过则阻断合并）。可重复、可靠、可追溯。

本方案作为最终方案。

## 4. 模块与依赖

### 4.1 CI/CD 流水线设计

```yaml
# .github/workflows/ci.yml
name: CI
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo test --workspace
      - run: cargo clippy --workspace --all-targets --all-features -- -D warnings
      - run: cargo fmt --check
  coverage:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo llvm-cov --workspace --lcov --output-path lcov.info
      - uses: codecov/codecov-action@v4
  audit:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: cargo audit
      - run: cargo deny check
```

### 4.2 覆盖率门禁

- 最低阈值：60% line coverage。
- 工具：cargo-llvm-cov。
- 报告格式：lcov（兼容 Codecov）。
- 门禁规则：PR 合并前覆盖率不得低于阈值。

### 4.3 安全审计

| 工具 | 用途 | 门禁 |
|---|---|---|
| `cargo audit` | 已知漏洞扫描 | 无高危/严重漏洞 |
| `cargo deny` | license 白名单 + 重复依赖 + 来源校验 | Apache-2.0/MIT/BSD/ISC/Unlicense |

### 4.4 发布验证

- 所有 crate `cargo publish --dry-run` 成功。
- docs.rs metadata 配置正确。
- 版本号遵循 semver。

### 4.5 性能基准

使用 criterion 建立性能基线：

| 场景 | 指标 | 目标 |
|---|---|---|
| token 获取 | 延迟 p99 | < 100ms（本地缓存命中） |
| 请求执行 | 吞吐 | > 1000 req/s（单连接） |
| 序列化/反序列化 | 延迟 p99 | < 1ms（单条消息） |

### 4.6 Redis 集成测试

使用 testcontainers 启动 Redis 容器，验证：

1. ConfigStorage Redis 实现的 token 缓存。
2. WxMessageInRedisDuplicateChecker 的去重功能。
3. 多租户 switchover 的正确性。

### 4.7 执行计划

| Phase | 范围 | 工作量 |
|---|---|---|
| Phase 1 (P0) | 核心测试 42 文件 | 9.5 天 |
| Phase 2 (P1) | 重要测试 86 文件 | 15 天 |
| Phase 3 (P2) | 扩展测试 67 文件 | 18 天 |
| Phase 4 (CI/CD) | CI/CD + 安全 + 性能 + Redis | 4.5 天 |
| **合计** | | **47 天** |

### 4.8 验收标准

- [ ] 所有 P0 测试文件编写完成并通过
- [ ] 所有 P1 测试文件编写完成并通过
- [ ] `cargo test --workspace` 全绿
- [ ] `cargo-llvm-cov --workspace` 行覆盖率 >= 60%
- [ ] `cargo clippy --workspace --all-targets --all-features -- -D warnings` 零警告
- [ ] `cargo audit` 无高危漏洞
- [ ] GitHub Actions CI 流水线运行通过
- [ ] 所有 crate `cargo publish --dry-run` 成功

## 5. 验收标准

- CI/CD 流水线设计完整（test + coverage + audit）。
- 覆盖率门禁 >= 60%。
- 安全审计工具选型明确（cargo audit + cargo deny）。
- 性能基准场景定义（token/execute/serialization）。
- Redis 集成测试场景定义。
- 四阶段执行计划有明确时间估算。
