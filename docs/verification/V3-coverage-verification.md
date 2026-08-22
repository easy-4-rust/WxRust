# V3 覆盖率验证报告

日期：2026-08-23
计划：`docs/superpowers/plans/2026-08-10-wxrust-migration-roadmap-and-execution.md` Task 4 Step 4

## 验证结果：❌ 未达目标（40.20% < 60%）

| 指标 | 值 | 目标 | 状态 |
|---|---|---|---|
| 行覆盖率（line） | **40.20%** | >= 60% | ❌ |
| 函数覆盖率（function） | 25.99% | — | 信号 |
| 分支覆盖率（branch） | 38.45% | — | 信号 |
| 覆盖行数 / 总行数 | 39,714 / 66,409 | — | — |

命令：`cargo llvm-cov --workspace --summary-only`（cargo-llvm-cov 0.8.7）

## 分模块覆盖率（行）

| 模块 | 行覆盖 | 函数覆盖 | 分支覆盖 |
|---|---|---|---|
| wx-rust-common | 43.53% | 26.23% | 42.55% |
| wx-rust-mp | 47.79% | 27.16% | 46.92% |
| wx-rust-miniapp | 48.16% | 30.18% | 47.02% |
| wx-rust-pay | 32.38% | 20.90% | 29.93% |
| wx-rust-cp | 46.30% | 30.40% | 44.63% |
| wx-rust-open | 38.58% | 24.25% | 36.17% |
| wx-rust-channel | 34.49% | 21.76% | 32.68% |
| wx-rust-aispeech | 高（测试全覆盖） | — | — |
| wx-rust-qidian | 高（测试全覆盖） | — | — |

## 结论与处置

- 覆盖率 **40.20% 未达 60% 门禁**，这是真实缺口信号（与生产就绪计划 Phase 2/3 的 153 个测试文件缺口一致）
- 按计划约束「覆盖率只作信号，台账状态为权威」：V3 记为 **未通过（需 Phase 2/3 测试补齐后复测）**
- 已生成完整明细日志：`/tmp/wxrust-cov.log`（含逐文件覆盖率）
- 复测时机：Phase 2（P1 86 文件）与 Phase 3（P2 67 文件）完成后，重跑 `cargo llvm-cov --workspace --summary-only`
