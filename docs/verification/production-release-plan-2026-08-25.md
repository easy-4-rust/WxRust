# WxRust 生产发布计划

日期：2026-08-25
版本：0.1.0（首次发布）
状态：待评审

---

## 1. 发布目标与范围

### 1.1 发布目标

将 WxRust 0.1.0 以 crates.io 公开 crate 形式发布，为 Rust 生态提供微信 API（公众号/小程序/支付/企业微信/开放平台/视频号/智语/企点）的 SDK。

### 1.2 发布范围

**首批发布（本计划覆盖）：** 全部 10 个 crate，按依赖拓扑分三波。

| 波次 | crate | 内部依赖 | 发布理由 |
|------|-------|----------|----------|
| Wave 1 | `wx-rust-common` | 无 | 基础层，所有其他 crate 的公共依赖；必须最先发布 |
| Wave 2 | `wx-rust-mp` | common | 公众号 SDK |
| Wave 2 | `wx-rust-miniapp` | common | 小程序 SDK（含 sync 门面） |
| Wave 2 | `wx-rust-pay` | common | 支付 SDK（含 v3 认证体系） |
| Wave 2 | `wx-rust-cp` | common | 企业微信 SDK |
| Wave 2 | `wx-rust-channel` | common | 视频号 SDK |
| Wave 2 | `wx-rust-aispeech` | common | 智语 SDK |
| Wave 2 | `wx-rust-qidian` | common | 企点 SDK |
| Wave 3 | `wx-rust-open` | common + mp + miniapp | 开放平台 SDK（代调 mp/ma，需 Wave 2 先就绪） |
| Wave 3 | `wx-rust` | 无内部 crate 依赖 | 门面 crate（待确认后发布） |

### 1.3 发布顺序依据

V6 发布验证报告（2026-08-23）已确认：`cargo publish --dry-run` 对 `wx-rust-common` 和 `wx-rust`（facade）打包成功；其余 8 个 crate 受发布顺序约束（依赖 common 未发布时无法解析版本号）。修复已在 commit 6728cc7 完成：workspace.dependencies 统一声明 `version + path`，各 crate 用 `X.workspace = true`。

---

## 2. 发布顺序与依赖策略

### 2.1 依赖拓扑图

```
wx-rust-common (Wave 1)
    |
    +-- wx-rust-mp ----------+
    +-- wx-rust-miniapp -----+
    +-- wx-rust-pay ---------+
    +-- wx-rust-cp ----------+---> wx-rust-open (Wave 3)
    +-- wx-rust-channel -----+
    +-- wx-rust-aispeech ----+
    +-- wx-rust-qidian ------+

wx-rust (facade, Wave 3, 无内部 crate 依赖)
```

### 2.2 发布命令序列

```bash
# Wave 1: 基础层
cargo publish -p wx-rust-common

# 等待 crates.io 索引同步（通常 30-60 秒）
sleep 60

# Wave 2: 业务 crate（可并行，但建议逐个执行便于定位问题）
cargo publish -p wx-rust-mp
cargo publish -p wx-rust-miniapp
cargo publish -p wx-rust-pay
cargo publish -p wx-rust-cp
cargo publish -p wx-rust-channel
cargo publish -p wx-rust-aispeech
cargo publish -p wx-rust-qidian

# 等待索引同步
sleep 60

# Wave 3: 依赖 Wave 2 的 crate
cargo publish -p wx-rust-open
cargo publish -p wx-rust
```

### 2.3 版本策略

- 全部 crate 统一版本号 `0.1.0`（workspace.package.version）
- 内部依赖声明：`{ version = "0.1.0", path = "crates/X" }`（已在 workspace.dependencies 配置）
- crates.io 发布后 path 被自动剥离，仅保留 version

### 2.4 发布前检查清单

每波发布前执行：

```bash
# 1. 确认 CI 全绿（main 分支最新 commit）
# 2. 本地全量回归
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo fmt --all -- --check

# 3. 确认版本号一致
grep 'version' crates/*/Cargo.toml | grep -v '#'

# 4. dry-run 验证（逐 crate）
cargo publish -p <crate> --dry-run

# 5. 确认 CHANGELOG / README 就绪
```

---

## 3. 灰度策略

### 3.1 首发模式：公开 crate + 文档引导

由于是 SDK 库（非服务端部署），灰度策略以消费者 opt-in 为主：

| 阶段 | 时间 | 范围 | 目标 |
|------|------|------|------|
| Alpha | 发布后 1 周 | 内部团队 + 邀请制 early adopter | 验证 API 易用性、文档完整性 |
| Beta | 发布后 2-4 周 | 公开但标注 "beta" | 收集真实使用反馈、发现文档缺口 |
| Stable | 发布后 4+ 周 | 正式推荐 | 去除 beta 标注 |

### 3.2 Feature Flags

| Feature | 默认状态 | 说明 |
|---------|----------|------|
| `sync` | 关闭 | 同步门面（WxMaServiceBlocking），需显式启用 `features = ["sync"]` |
| `redis` | 关闭 | Redis 集成存储，需显式启用 |

默认 feature 集（无额外依赖）即为异步 API 形态，覆盖绝大多数使用场景。

### 3.3 消费者接入方式

```toml
# 最简接入（仅需某模块）
[dependencies]
wx-rust-miniapp = "0.1.0"

# 门面接入（全部模块）
[dependencies]
wx-rust = "0.1.0"

# 同步用户
[dependencies]
wx-rust-miniapp = { version = "0.1.0", features = ["sync"] }
```

---

## 4. 回滚策略

### 4.1 语义化版本策略

- 遵循 SemVer 2.0：`0.y.z` 阶段允许 breaking change（但尽量避免）
- Patch（0.1.x）：bug 修复、文档更新，不改公共 API
- Minor（0.y.0）：新增功能、可能有 deprecation 但不 breaking
- Major（1.0.0）：首个稳定版本承诺，此后严格遵守 SemVer

### 4.2 Yank 策略

| 场景 | 操作 | 原因 |
|------|------|------|
| 安全漏洞（高/严重） | 立即 yank + 发布修复版 | 用户安全 |
| 数据损坏风险 | 立即 yank + 发布修复版 | 数据完整性 |
| API 语义错误（签名/参数解析） | yank + 发布修复版 | 正确性 |
| 非关键 bug | 不 yank，在下个 patch 修复 | 避免给下游造成构建中断 |

**Yank 原则：** yank 不删除已发布版本，仅阻止新项目依赖它。已依赖的项目不受影响。yank 后必须立即或在 24 小时内发布修复版。

### 4.3 回滚剧本

**场景 A：发布后发现关键 bug**

```
1. 评估影响范围：哪些 crate 受影响？哪些 API？
2. 决策：yank 还是直接发修复版？
   - 如果 bug 影响数据完整性或安全性 → yank
   - 如果 bug 仅影响特定 API 边界情况 → 发修复版
3. 执行 yank（如需）：
   cargo yank --version 0.1.0 -p <crate>
4. 修复并验证：
   cargo test --workspace
5. 发布修复版 0.1.1：
   cargo publish -p <crate>
6. 通知用户（GitHub Release + README 更新）
```

**场景 B：依赖链断裂（某 crate 发布失败）**

```
1. 已成功发布的 crate 无需回滚（它们的依赖在 crates.io 上可用）
2. 失败的 crate 排查原因（常见：版本号冲突、描述过长、license 文件缺失）
3. 修复后重新 cargo publish
4. 如果 Wave 2 中某 crate 失败，不影响其他 Wave 2 crate（它们之间无依赖关系）
```

**场景 C：下游大规模兼容性问题**

```
1. 在 GitHub Issues 收集反馈，确认影响范围
2. 如果影响 >50% 用户且有 workaround → 发布 0.1.1 修复
3. 如果影响面小且有 workaround → 更新 README 文档说明
4. 不轻易 yank（0.x 阶段用户应预期可能的变更）
```

---

## 5. 验收标准

### 5.1 功能验收

| 检查项 | 命令 | 通过标准 | 当前状态 |
|--------|------|----------|----------|
| 全量测试 | `cargo test --workspace` | 0 failures | 1968 passed (V0 gap closure 后) |
| clippy | `cargo clippy --workspace --all-targets --all-features -- -D warnings` | 0 warnings | 干净 |
| fmt | `cargo fmt --all -- --check` | exit 0 | 干净 |
| 覆盖率 | `cargo llvm-cov --workspace --fail-under-lines 60 --summary-only` | >= 60% | 61.57% |
| block_on 门禁 | `scripts/check_block_on.sh` | 仅 blocking.rs 命中 | 通过 |
| 并发基准 | `cargo bench -p wx-rust-common --bench pipeline_concurrency_bench -- --test` | 1000 并发 token 刷新=1 | 通过 |
| 并发基准 | `cargo bench -p wx-rust-miniapp --bench token_single_flight_bench -- --test` | 单飞正确 | 通过 |

### 5.2 并发验收

| 检查项 | 验证方法 | 通过标准 |
|--------|----------|----------|
| token 单飞 | 1000 并发 MockTransport + 计数器 | 刷新应答次数 = 1 |
| 熔断器开合 | 阈值压测 + MockTransport 计数 | Open 期间零 transport 调用 |
| sync 门面隔离 | `#[test]`（非 tokio）调用 blocking 方法 | 同步上下文正确返回 |
| block_on 作用域 | grep 门禁 | 仅 `blocking.rs` 文件内 |

### 5.3 可观测性验收

| 检查项 | 验证方法 | 通过标准 |
|--------|----------|----------|
| 错误类型完备 | 4 份错误枚举补测（cp/ma/mp/channel） | 全部错误码有对应消息 |
| 文档完整 | 各 crate README + docs.rs 构建 | docs.rs 无构建错误 |
| 安全审计 | `cargo audit` + `cargo deny check` | 无高/严重漏洞（medium 已登记例外） |

### 5.4 发布后验证（每个 crate 发布后）

```bash
# 从 crates.io 拉取并验证
cargo install --list | grep wx-rust  # 如有 CLI 工具
# 或创建临时项目依赖已发布版本
mkdir /tmp/wxrust-verify && cd /tmp/wxrust-verify
cargo init
# 添加依赖到 Cargo.toml
cargo check
cargo test  # 如有测试
```

---

## 6. 风险清单

### 6.1 已知风险

| # | 风险 | 严重度 | 影响范围 | 缓解措施 | 状态 |
|---|------|--------|----------|----------|------|
| R1 | RSA Marvin Attack（RUSTSEC-2023-0071） | Medium | wx-rust-common / wx-rust-cp / wx-rust-pay（RSA-OAEP 加密/签名） | RSA-OAEP 盲化 + 固定消息加密；deny.toml 已登记例外；待 rsa 0.10 稳定后升级 | 已接受，持续跟踪 |
| R2 | 内部耦合：6 份管线已收敛但 open/channel 保留旧路径 | Low | wx-rust-open / wx-rust-channel | open 的 component_access_token 键不同构、channel 的 GET 字节序被测试冻结；保持旧路径，不影响正确性 | 已记录，后续迭代优化 |
| R3 | async 运行时约束：SDK 绑定 tokio | Medium | 所有异步用户 | 文档明确说明 tokio 依赖；`sync` feature 提供非异步门面 | 已缓解 |
| R4 | rsa =0.9.10 精确版本锁定 | Low | 升级时可能与上游 breaking change 冲突 | 精确锁定避免意外升级；关注 rsa 0.10 稳定版发布 | 已记录 |
| R5 | 首次发布，无真实用户反馈 | Medium | API 设计、文档质量 | Alpha 阶段邀请 early adopter；快速迭代修复 | 待执行 |

### 6.2 No-Go 条件

以下任一条件成立时，**不得发布**：

| 条件 | 检查方法 |
|------|----------|
| `cargo test --workspace` 存在 failure | CI 红灯 |
| `cargo clippy --D warnings` 存在 warning | CI 红灯 |
| 覆盖率 < 60% | `cargo llvm-cov --fail-under-lines 60` |
| cargo audit 报 high/critical 漏洞 | `cargo audit` |
| block_on 出现在 sync 门面以外的文件 | `scripts/check_block_on.sh` |
| 并发基准失败（token 刷新 != 1） | `cargo bench -- --test` |
| crates.io 上同名 crate 已存在 | `cargo search wx-rust` |
| workspace 版本号不一致 | `grep version crates/*/Cargo.toml` |

### 6.3 回滚触发条件

| 条件 | 响应 |
|------|------|
| 发布后 24 小时内收到 >3 个独立报告的 P0 bug | yank + 修复 |
| 安全漏洞被公开披露（任何严重度） | 立即评估 + yank（如需） |
| 下游大规模构建失败（crates.io 索引问题） | 等待 crates.io 恢复 + 通知用户 |

---

## 7. 时间线与责任分工

### 7.1 发布时间线

| 日期 | 里程碑 | 交付物 |
|------|--------|--------|
| 2026-08-25 | 发布计划评审 | 本文档经团队 review 通过 |
| 2026-08-26 | 发布前检查 | 全量 CI 验证 + README 最终确认 + CHANGELOG 编写 |
| 2026-08-27 | Wave 1 发布 | `wx-rust-common` 上线 crates.io |
| 2026-08-27 | Wave 2 发布 | 7 个业务 crate 上线 crates.io |
| 2026-08-27 | Wave 3 发布 | `wx-rust-open` + `wx-rust` 上线 crates.io |
| 2026-08-27 | 发布验证 | 拉取已发布版本 + 验证构建 + GitHub Release |
| 2026-08-28 ~ 09-03 | Alpha 阶段 | 收集 early adopter 反馈 |
| 2026-09-03 ~ 09-24 | Beta 阶段 | 公开 beta，收集更广泛反馈 |
| 2026-09-24+ | Stable 评估 | 根据反馈决定是否去除 beta 标注 |

### 7.2 角色与职责

| 角色 | 职责 | 负责人 |
|------|------|--------|
| 发布执行人 | 执行 cargo publish、监控发布状态 | @wandl |
| 质量门禁 | 确认 CI 全绿、验收标准通过 | @wandl |
| 文档维护 | README、CHANGELOG、docs.rs 配置 | @wandl |
| 安全响应 | 监控 cargo audit、处理安全报告 | @wandl |

（注：当前为单人维护项目，全部角色由同一人承担。后续如有团队扩展，需重新分配。）

### 7.3 检查点

| 检查点 | 时间 | 通过标准 |
|--------|------|----------|
| CP1: 计划评审通过 | 2026-08-25 | 团队 sign-off |
| CP2: 发布前全量验证 | 2026-08-26 | 全部 No-Go 条件不成立 |
| CP3: Wave 1 发布成功 | 2026-08-27 | crates.io 可搜索到 wx-rust-common 0.1.0 |
| CP4: Wave 2 发布成功 | 2026-08-27 | 7 个 crate 均在 crates.io 可搜索 |
| CP5: Wave 3 发布成功 | 2026-08-27 | open + facade 均在 crates.io 可搜索 |
| CP6: 发布后验证通过 | 2026-08-27 | 临时项目依赖已发布版本可编译 |
| CP7: Alpha 反馈收集 | 2026-09-03 | 无 P0 未解决问题 |

---

## 8. 发布后监控

### 8.1 监控维度

| 维度 | 指标 | 数据源 | 告警阈值 |
|------|------|--------|----------|
| 下载量 | 每日下载数 | crates.io API / crates.io 页面 | 信号指标，无告警 |
| 问题反馈 | GitHub Issues 数量/严重度 | GitHub | P0 issue > 0 |
| 构建成功率 | 下游 CI 构建成功 | 用户反馈 / GitHub Issues | 大规模构建失败报告 |
| 安全漏洞 | cargo audit 新增 | cargo audit + RustSec Advisory DB | high/critical 新增 |

### 8.2 关键指标跟踪

**延迟相关（SDK 使用场景）：**
- 本项目为 SDK 库，不直接暴露服务端点
- 下游用户需自行配置监控（reqwest 超时、tokio runtime 指标）
- SDK 内部提供 tracing 日志（`tracing` crate 已集成），下游可通过 subscriber 收集

**错误相关：**
- `WxErrorException` 分类：Runtime / Business（含 errcode）
- Token 刷新失败率：下游需自行监控 token 端点响应
- 熔断器状态：`CircuitBreaker` 默认关闭，启用后下游可通过日志观察 Open/Closed 状态

**Token 刷新指标（SDK 内部可观测）：**
- token 单飞正确性已由并发基准验证（1000 并发刷新次数 = 1）
- token 过期判断使用 `WxClock`（默认 SystemClock），日志级别 tracing::debug
- 下游应配置 tracing subscriber 并关注 `access_token` 相关日志

**熔断器状态：**
- 默认不启用（`PipelineConfig::breaker = None`）
- 启用后行为：Closed（正常）-> Open（连续 N 次失败）-> HalfOpen（探测）-> Closed
- Open 状态返回错误文案「熔断器开启：<host>」，下游可据此告警

### 8.3 发布后值班安排

| 时段 | 值班人 | 响应时间 |
|------|--------|----------|
| 发布后 24 小时 | @wandl | 1 小时内响应 P0 |
| 发布后 1 周 | @wandl | 4 小时内响应 P0 |
| Alpha 阶段 | @wandl | 下个工作日响应 |

### 8.4 发布后 Checklist

发布完成后 24 小时内确认：

- [ ] 全部 10 个 crate 在 crates.io 可搜索且版本正确
- [ ] GitHub Release 已创建（含 CHANGELOG 摘要）
- [ ] README.md 中的安装说明已更新为 crates.io 版本
- [ ] docs.rs 上各 crate 文档构建成功
- [ ] 无 P0/P1 未关闭 issue
- [ ] cargo audit 无新增漏洞

---

## 附录 A: 验证报告索引

| 报告 | 日期 | 结论 |
|------|------|------|
| V0 迁移缺口清零 | 2026-08-24 | MISSING=0，3287 对象 100% 处置 |
| V1 工程验证 | 2026-08-23 | 全绿（920 tests，后续增至 1968） |
| V3 覆盖率验证 | 2026-08-24 | 61.57% >= 60% 门禁 |
| V4 安全审计 | 2026-08-23 | PASS（含 1 项已登记 rsa 例外） |
| V6 发布验证 | 2026-08-23 | PASS（配置修复完成，发布顺序约束已记录） |
| aispeech 接线复核 | 2026-08-25 | 23/23 方法全覆盖，零接线缺口 |

## 附录 B: 已登记的已知风险例外

| Advisory ID | 依赖 | 严重度 | 缓解措施 | 追踪 |
|-------------|------|--------|----------|------|
| RUSTSEC-2023-0071 | rsa 0.9.10 | Medium (5.9) | RSA-OAEP 盲化 + 固定消息加密 | 待 rsa 0.10 稳定后升级 |
