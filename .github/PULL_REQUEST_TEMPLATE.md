## 关联

- Issue: <!-- 关联 issue（修复类必填） #XX -->
- 相关 PR: <!-- 如有 -->

## 改动类型

<!-- 勾选其一 -->

- [ ] 新功能
- [ ] Bug 修复
- [ ] 重构（无功能变更）
- [ ] 文档
- [ ] 依赖升级（依赖 Cargo.lock 变更）
- [ ] CI / 工作流
- [ ] 发布（版本变更）

## 改动说明

<!-- 描述改了什么、为什么改 -->

## 测试

<!-- 勾选已执行的验证 -->

- [ ] `cargo check --workspace --all-features`
- [ ] `cargo test --workspace`（新/相关 crate）
- [ ] `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- [ ] `cargo fmt --all`（已格式化）
- [ ] 涉及 XML 报文 → `crates/wx-rust-pay/tests/wx_pay_v3_crypto_test.rs` golden 对照
- [ ] 涉及真实 API → `docs/operations/alpha-2026-q3/` 报告更新

## 发布影响

<!-- 仅在涉及版本变更时填写 -->

- [ ] 涉及 `Cargo.toml` `version` 字段
- [ ] 涉及 `Cargo.lock`（依赖升级）
- [ ] 需同步更新 `CHANGELOG.md`
- [ ] 需重新触发 `release.yml` 工作流（手工发布）

## 风险评估

<!-- 仅在触及关键路径（pay 真实凭证/RSA 缓解/CI 工作流）时填写 -->

- 影响的语义边界：
- 回滚方案：