# WxRust Java→Rust 迁移路线图（总览）

- Java 基线：WxJava `4.8.4.B`
- Rust 基线：`easy-4-rust/WxRust`（workspace）
- 分母基线：main 对象 `3287`、test 对象 `379`、javap 公共方法 `36010`
- 文档口径：按 `rust-java-migration` 技能要求，以模块为最小迁移单元，形成《路线图/对象级对照表/语义迁移对照表/对象名称一致性检查》四件套。

## 1. 目标

1. 以 WxJava 源码为唯一语义权威，完成功能语义 100% 迁移。
2. 先完成规划与基线冻结（B0/B1），再执行模块批实现（B2），最后统一做验证门禁（V0-V6）。
3. 所有结论必须有可重放证据（文件路径、命令、测试结果、脚本审计）。

## 2. 已定架构决策（节选）

| 决策 | 结论 | 依据 |
|---|---|---|
| HTTP | reqwest（rustls）统一 | ARCHITECTURE.md |
| JSON/XML | serde_json + quick-xml | ARCHITECTURE.md |
| 错误 | thiserror + Result | ARCHITECTURE.md |
| 并发 | tokio + async-trait | ARCHITECTURE.md |
| 集成层 | vernal（对标 Spring Boot starter） | PLAN.md 5.6 |

## 3. 模块批次规划（主计划）

| 波次 | 范围 | 目标 | 门禁 |
|---|---|---|---|
| B0 | 全量清点 | 对象分母与方法分母冻结 | inventory 可重算 |
| B1 | 架构锁定 | 组件替换决策与映射规则固化 | ARCHITECTURE LOCKED |
| B2-Batch1 | wx-rust-common | 基础层对象全处置 + 基础测试 | workspace tests green |
| B2-Batch2 | wx-rust-mp | 公众号模块全处置 | 模块测试 green |
| B2-Batch3 | wx-rust-miniapp | 小程序模块全处置 | 模块测试 green |
| B2-Batch4 | pay/cp/open/channel/aispeech/qidian | 剩余业务模块全处置 | 模块测试 green |
| V0-V6 | 全量验证 | 静态结构/镜像/黄金差分/live/宿主/非功能 | cargo-llvm-cov 与审计脚本 |

## 4. 当前状态（基于 docs/migration/README.md）

- 全量迁移文档（9 模块 × 4 文档 = 36 份）已生成。
- 对象分母 `3287` 与方法分母 `36010` 已冻结。
- weixin-java-common / mp / miniapp / pay / cp / open / channel / aispeech / qidian 的台账均已完成，且迁移 README 中记录各模块 `0 MISSING`。

## 5. 验收门禁

1. 模块对象完成状态唯一权威台账：`docs/migration/<module>/对象级对照表.md`
2. 语义迁移唯一权威台账：`docs/migration/<module>/语义迁移对照表.md`
3. 名称一致性唯一权威台账：`docs/migration/<module>/对象名称一致性检查.md`
4. 测试与覆盖率证据：模块测试通过 + `cargo-llvm-cov` 覆盖率信号
5. 审计脚本：`scripts/audit_<module>_ledger.py` 输出 0 MISSING

## 6. 风险与缓解

| 风险 | 影响 | 缓解 |
|---|---|---|
| 文档漂移 | 结论不可靠 | 以当前 Rust 工作树重算状态 |
| 隐式简化 | 语义不完整 | 严格按 WxJava 实现，不简化 |
| 覆盖率误读 | 虚假完成 | 覆盖率只作信号，台账状态为权威 |

---

> 本文是路线总览，详细模块进度与证据入口见 `docs/migration/README.md` 及各模块子目录。
