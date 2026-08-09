# WxRust Java→Rust 迁移文档索引

> 依据 [rust-java-migration 技能](/Users/wandl/.zcode/skills/rust-java-migration/SKILL.md) 的文档模板，
> 对 WxJava 每个源模块生成一套《迁移路线图 / 对象级对照表 / 语义迁移对照表 / 对象名称一致性检查》，
> 批次交付时按 [rust-java-migration-testing 技能](/Users/wandl/.zcode/skills/rust-java-migration-testing/SKILL.md) 增补《迁移测试对照表 / 覆盖审计报告》。
> 每套文档构成该模块的当前权威迁移契约。

- 生成日期：2026-08-01
- Java 基线：`a49d6e1461752c06b752d2afd8aeeb7e6e78cefe`（WxJava 4.8.4.B）
- Rust 基线：`2026-08-01 working-tree`（weixin-java-common 已实现并测试，其余模块规划阶段）
- 路径算法：从各模块 Java 包根起保留末 `2` 层包目录，转 `snake_case`
- 状态口径：`IMPLEMENTED`/`DEPENDENCY_REUSED`/`PLATFORM_NA` 均需严格证据；`MISSING`/`MISPLACED`/`STUB`/`PARTIAL`/`UNVERIFIED` 为迁移阻断项
- 文档状态：weixin-java-common = `B2-PART1-TESTED`（✅ 全量交付，174 对象 0 MISSING）；weixin-java-mp = `B2-BATCH2-COMPLETE`（✅ 全量交付，428 对象 0 MISSING）；weixin-java-miniapp = `WAVE5-COMPLETE`（✅ 全量交付，611 对象 0 MISSING）；weixin-java-pay/cp/open/channel/aispeech/qidian = `WAVE5-COMPLETE`（✅ 全量交付，各模块 0 MISSING）

## 模块总览

| Java 模块 | Rust crate | 对象数 | 方法数(javap) | 测试对象 | 迁移文档目录 | 核心能力 |
|---|---:|---:|---:|---|---|---|
| weixin-java-common | `wx-rust-common` | 174 | 1077 | 29 | [docs/migration/weixin-java-common](weixin-java-common/) | 错误模型、token、执行引擎、HTTP、会话、重复检查 |
| weixin-java-mp | `wx-rust-mp` | 428 | 4454 | 71 | [docs/migration/weixin-java-mp](weixin-java-mp/) | 公众号 API、消息路由、XML 消息加解密 |
| weixin-java-miniapp | `wx-rust-miniapp` | 611 | 6192 | 68 | [docs/migration/weixin-java-miniapp](weixin-java-miniapp/) | 小程序 code2Session、50+ 子服务 |
| weixin-java-pay | `wx-rust-pay` | 570 | 8088 | 71 | [docs/migration/weixin-java-pay](weixin-java-pay/) | 支付 v2/v3、证书、签名、XML |
| weixin-java-cp | `wx-rust-cp` | 594 | 7229 | 88 | [docs/migration/weixin-java-cp](weixin-java-cp/) | 企业微信、会话存档、OA |
| weixin-java-open | `wx-rust-open` | 240 | 2530 | 13 | [docs/migration/weixin-java-open](weixin-java-open/) | 第三方平台、代 mp/ma |
| weixin-java-channel | `wx-rust-channel` | 618 | 5821 | 31 | [docs/migration/weixin-java-channel](weixin-java-channel/) | 视频号小店电商 |
| weixin-java-aispeech | `wx-rust-aispeech` | 25 | 298 | 2 | [docs/migration/weixin-java-aispeech](weixin-java-aispeech/) | AI 语音 |
| weixin-java-qidian | `wx-rust-qidian` | 27 | 321 | 6 | [docs/migration/weixin-java-qidian](weixin-java-qidian/) | 企点呼叫中心 |
| **合计** | — | **3287** | **36010** | **379** | — | — |

> 对象数来自 `scripts/inventory_java_objects.py`（含包私有类；排除 `package-info`）。
> 测试对象数为 `find <module>/src/test/java -name '*.java' ! -name 'package-info*'` 统计。

## 每模块文档

| 文档 | 职责 | 关键章节 |
|---|---|---|
| [迁移路线图.md](weixin-java-common/迁移路线图.md) | 范围、基线、阶段（B0-B2/V0-V4）、依赖、风险、证据门 | 组件替换决策、阶段任务、风险对策、质量门禁执行结果 |
| [对象级对照表.md](weixin-java-common/对象级对照表.md) | 每个 Java 对象 → 预期 Rust 路径/当前状态 | 对象映射（全量）、统计汇总、合并重命名、依赖复用证据 |
| [语义迁移对照表.md](weixin-java-common/语义迁移对照表.md) | 每个行为族的 Rust 原生实现与可观察行为 | 核心调用链、错误体系、序列化、并发、验证基线 |
| [对象名称一致性检查.md](weixin-java-common/对象名称一致性检查.md) | 名称/文件/目录/方法/参数一致性 | 统计汇总、结构红线、四文档一致性 |
| [迁移测试对照表.md](weixin-java-common/迁移测试对照表.md) | Java 测试逐用例处置 + Rust 义务 + 增值测试台账 | `SOURCE_PARITY`/`RUST_OBLIGATION`/`VALUE_ADD`、验收结果 |
| [覆盖审计报告.md](weixin-java-common/覆盖审计报告.md) | 覆盖率与行为证据审计（V0–V6） | 处置覆盖、证据层级、覆盖率信号与缺口 |

> 测试对照表与覆盖审计报告目前仅 weixin-java-common 随 B2 批次 1 交付；其余 8 模块随各自批次增补。

## 当前状态（2026-08-01）

- ✅ 9 模块 × 4 文档 = **36 份迁移文档**全部生成并填充；weixin-java-common 额外交付《迁移测试对照表》《覆盖审计报告》
- ✅ 3287 个 main Java 对象 + 379 个测试对象已清点入库（`inventory_java_objects.csv`）
- ✅ **B0 完成**：36010 个 javap 公共方法分母冻结（29461 源声明 + 6549 继承/lombok），`inventory_java_methods.csv`
- ✅ **B1 完成**：架构与组件替换决策已锁定（`docs/ARCHITECTURE.md`，状态 `LOCKED`）
- ✅ 未迁移 8 模块对象状态：`MISSING`（规划阶段，无实现）——符合事实
- ✅ 各模块文档共用同一 Java/Rust 基线、同一对象/方法分母
- ✅ **B2 批次 1 完成**：wx-rust-common 实现（90 对象 `IMPLEMENTED` + 1 `DEPENDENCY_REUSED`（sha1 0.11.0，黄金向量 POC 闭环）+ 87 `PLATFORM_NA` + 1 `RUST_EXTENSION`，未完成 0）
- ✅ **批次 1 测试门禁通过**：`cargo test -p wx-rust-common --all-features` **83/83**；`cargo clippy` 零警告；`cargo llvm-cov` line 26.07% / fn 50.28% / region 19.46%（信号）；Java 侧 98 个 `@Test` 用例 98/98 处置（42 镜像/适配 + 56 不适用 + 0 缺失）
- ✅ 新增配套文档：《迁移测试对照表》《覆盖审计报告》（weixin-java-common、weixin-java-mp、weixin-java-miniapp 已交付）
- ✅ **B2 批次 2 全量交付**：wx-rust-mp 428 对象全部处置（284 IMPLEMENTED + 44 PLATFORM_NA + 39 DEPENDENCY_REUSED，0 MISSING），153/153 workspace tests 全绿（mp 70）
- ✅ **B2 批次 3 全量交付（Wave 0–5）**：wx-rust-miniapp 635 个 .rs 文件，234/234 workspace tests 全绿（miniapp 81：6 个集成测试文件 75 + 单元 6）；台账 611 对象全部处置（560 IMPLEMENTED + 35 PLATFORM_NA + 16 DEPENDENCY_REUSED，0 MISSING）；Wave 5 审计发现的 7 个真实缺口（消息路由族 5 + ImgProc/Ocr 2）已全部补齐并带镜像测试，`scripts/audit_miniapp_ledger.py` 重跑翻转 0 MISSING，详见《迁移测试对照表》《覆盖审计报告》
- ✅ **B2 批次 4 全量交付（Wave 0–5，多智能体）**：wx-rust-pay 570 / wx-rust-cp 594 / wx-rust-open 240 / wx-rust-channel 618 / wx-rust-aispeech 25 / wx-rust-qidian 27 对象全部处置，**各模块 0 MISSING**（pay 556+12+2、cp 565+20+9、open 213+14+13、channel 603+9+6、aispeech 23+2、qidian 20+7）；workspace **566/566 tests 全绿**（pay 55 / cp 120 / open 47 / channel 84 / aispeech 8 / qidian 18）；Wave 5 审计发现的 164 个真实缺口（pay 90 / cp 48 / open 21 / channel 5）已全部补齐或依语义归类（util/json adaptor → DEPENDENCY_REUSED、executor/HTTP 后端/Redis → PLATFORM_NA、native SDK 依赖 → PLATFORM_NA），各模块审计脚本（scripts/audit_<module>_ledger.py）重跑均 0 MISSING
- ⏳ 后续：common 模块 V5 集成（Redis 后端、真实 HTTP 宿主）与 Java 侧测试执行（JDK 8）留待后续；集成层 wx-rust-vernal（Spring Boot starter 对标）与 wx-rust-axum 按 PLAN.md 5.6 节规划

## 关键架构决策（规划阶段已定）

| 决策 | 结论 | 依据 |
|---|---|---|
| HTTP 客户端 | reqwest（rustls）统一，删除 apache/okhttp/jodd 三后端 | 语义表·组件替换 |
| JSON | serde_json（preserve_order），替代 Gson 手写 TypeAdapter | 语义表·序列化 |
| XML（支付/消息） | quick-xml + serde | 语义表·序列化 |
| 加解密/签名 | RustCrypto（aes/rsa/sha2/hmac） | 语义表·组件替换 |
| 并发 | `tokio::sync::Mutex` + `tokio::task::spawn` + `tokio::time::sleep` | 语义表·并发 |
| 错误 | `thiserror` + `Result`，替代 checked exception | 语义表·错误体系 |
| 继承链（Impl→HttpComponentsImpl→Base） | trait + 组合，Rust 无继承 | 路线图·设计原则 |
| 集成层 | `wx-rust-vernal`（对标 Spring Boot starter）+ `wx-rust-axum` | PLAN.md 5.6 节 |

## 项目级文档

- [PLAN.md](../PLAN.md) — 项目骨架计划（含集成层规划）
- [WXJAVA_ANALYSIS.md](../WXJAVA_ANALYSIS.md) — WxJava 深度分析（7 大设计模式）
- [inventory_java_objects.csv](../inventory_java_objects.csv) — 对象清点机器可读清单
- [scripts/](../../scripts/) — 清点与填充脚本（可重放）
