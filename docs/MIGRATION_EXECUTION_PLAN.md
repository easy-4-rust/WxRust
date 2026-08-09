# WxRust 多智能体迁移执行计划

- 依据文档：`docs/PLAN.md`、`docs/ARCHITECTURE.md`、`docs/migration/README.md`
- 技能要求：`rust-java-migration`（迁移） + `rust-java-migration-testing`（测试）

## 执行模式

采用“按模块并行智能体 + 统一门禁”的方式推进：
- 每个模块由一个智能体负责端到端迁移与模块测试。
- 设一个集成智能体负责 workspace 测试聚合与 `cargo-llvm-cov` 覆盖率验证。

## 模块分配

| 智能体 | 负责模块 | 任务范围 | 交付门禁 |
|---|---|---|---|
| Agent-Common | wx-rust-common | 对象实现 + 模块测试 + 语义表更新 | cargo test -p wx-rust-common green |
| Agent-MP | wx-rust-mp | 同上 | cargo test -p wx-rust-mp green |
| Agent-MiniApp | wx-rust-miniapp | 同上 | cargo test -p wx-rust-miniapp green |
| Agent-Pay | wx-rust-pay | 同上 | cargo test -p wx-rust-pay green |
| Agent-CP | wx-rust-cp | 同上 | cargo test -p wx-rust-cp green |
| Agent-Open | wx-rust-open | 同上 | cargo test -p wx-rust-open green |
| Agent-Channel | wx-rust-channel | 同上 | cargo test -p wx-rust-channel green |
| Agent-AISpeech | wx-rust-aispeech | 同上 | cargo test -p wx-rust-aispeech green |
| Agent-Qidian | wx-rust-qidian | 同上 | cargo test -p wx-rust-qidian green |
| Agent-Integration | workspace | workspace 聚合测试 + cov + 审计脚本 | cargo test --workspace green + cov 报告 |

## 统一验收流程

1. 模块智能体完成后提交：实现代码 + 模块测试 + 该模块四文档（或四文档增量）。
2. 集成智能体执行：
   - `cargo test --workspace`
   - `cargo llvm-cov` 生成覆盖率信号
   - `scripts/audit_<module>_ledger.py` 输出 0 MISSING
3. 仅当模块文档与集成门禁同时通过，才视为批次完成。

## 覆盖率策略

- 覆盖率用于“发现缺口”，不作为完成权威。
- 完成权威仍为：模块对象表 + 语义表 + 名称一致性表 + 测试台账。

## 并行节奏建议

- 第一轮：common / mp / miniapp（基础与高频）
- 第二轮：pay / cp / open
- 第三轮：channel / aispeech / qidian
- 每轮结束后由集成智能体做 workspace 回归与 cov 审计。
