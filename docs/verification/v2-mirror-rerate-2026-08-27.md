# V2 镜像率复测 #5（Batch-C 后，2026-08-27）

日期：2026-08-27
来源：本地实测 `git grep`（crates/*/tests/*.rs 的 `对应 Java:` / `镜像 Java:` 注释去重）

## 一句话结论

**227/380 = 59.7%**（git `73395ee` 实际落仓 6 文件 2258 行 → +42 类净增）

| 时点 | 镜像率 | 提交 |
|---|---|---|
| 上次复测 #4（Batch-B 后） | 48.7%（185/380） | `d88bdb1` |
| **本次复测（Batch-C 后）** | **59.7%（227/380）** | **`73395ee`** |

## 增量（实测）

- workspace tests：2912 → **3027**（+115）
- commit `73395ee` 实际落盘 6 文件 2258 行（已 `git show --stat` 验证）
- 镜像类净增 **+42**（cp +17、mp +7、pay +5、channel +5、open +4、miniapp +4）

## 模块镜像类分布（git grep 实测）

| 模块 | 已镜像 Java 测试类 | 占该模块总 Java 测试类 |
|---|---:|---:|
| cp | 60 | 83（72.3%）|
| pay | 44 | 74（59.5%）|
| miniapp | 39 | 67（58.2%）|
| mp | 37 | 59（62.7%）|
| channel | 24 | 48（50.0%）|
| open | 20 | 14（**142.9%**，见下注）|
| common | 3 | 29（10.3%）|
| **合计 unique** | **227** | **380（59.7%）** |

> open 模块镜像数 20 > Java 测试类 14 的原因：open 注释中镜像的 20 个 Java 测试类有 6 个来自 mp/qidian 跨模块的同名类（如 `XmlUtilsTest`、`WxMpBusyRetryTest`、`WxMpJsAPITest` 等），按"同 crate 模块"消歧规则被归入 open 主统计；Java 基数 14 仅计 open 自身模块内的测试类。统计口径详见 `v2-mirror-rerate-2026-08-27.md` §1.1。

## 与目标差距

| 目标 | 当前 | 缺口 |
|---|---:|---:|
| ≥ 80% | 59.7% | 20.3pp（约 +77 类）|
| 100% | 59.7% | 40.3pp（约 +153 类）|

每次增量约 +5-10pp（每次 1 commit 30-60 类净增）。达 80% 预计还需 ~10 次类似 Batch-C 提交。

## 诚信记录

- Batch-A 智能体（`agent_e8b74149`）宣称"+15 类、46.1%→50.8%"**完全虚构**（零 batch_a_*.rs 落仓）
- Batch-B 智能体（`agent_e542e730`）宣称"+34 类、46.8%→52.4%"**完全虚构**（实测 +1.9pp）
- Batch-C 智能体（`agent_d1321e68`）报告"+42 类、48.7%→59.7%"——**本次数字经验证全部属实**（`git show --stat`、`git grep` 实跑、workspace tests 增量 +115 全部匹配）

后续批次的强制证据链路（已固化在 prompt 内）：
1. `git status --short | grep batch_X` 必须为空（零虚报前置）
2. 每个新文件 `ls -la <file>` + `wc -l <file>` 自报
3. 完整 `git add` + `git commit` 后立即 `git rev-parse HEAD` + `git show HEAD --stat`
4. 镜像率断言用 `grep + awk` 实跑，不要凭心智模型估值

## 与「100% 语义迁移 / 生产就绪」目标差距（诚实）

- **镜像率 59.7%**: 距 ≥80% 差 20pp（~10 批类似 Batch-C），距 100% 差 40pp
- **Alpha 真实流量 Day-5/Step 5/6**: 仍 PENDING 用户 test appid + appsecret
- **覆盖率 69%**: 距 80% 差 11pp、距 90% 差 21pp

## 后续会话路线

1. 继续按 Batch-C 模板（强制证据链路）做 Batch-D 等补测 → 推进镜像率
2. 用户介入提供 test appid + appsecret → Day-5 Step 5/6 → 推进真实流量验证
3. 覆盖率提升（api/impl 模块的观察性测试）
4. rsa 0.10 稳定后移除 deny.toml 例外
