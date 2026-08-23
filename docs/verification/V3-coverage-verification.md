# V3 覆盖率验证报告

日期：2026-08-23（首测）/ 2026-08-24（覆盖率提升波次后终测）
计划：`docs/superpowers/plans/2026-08-10-wxrust-migration-roadmap-and-execution.md` Task 4 Step 4

## 验证结果：✅ 达标（61.57% >= 60% 门禁）

| 指标 | 首测 | Phase 2/3 后 | 提升波次后（终测） | 目标 | 状态 |
|---|---|---|---|---|---|
| 行覆盖率（line） | 40.20% | 40.55% | **61.57%** | >= 60% | ✅ |
| 函数覆盖率（function） | 25.99% | 26.19% | 41.80% | — | 信号 |
| 分支覆盖率（branch） | 38.45% | 38.75% | 59.64% | — | 信号 |
| 未覆盖行数 / 总行数 | 39,714 / 66,409 | 39,482 / 66,409 | **25,523 / 66,409** | — | — |

门禁验证：`cargo llvm-cov --workspace --fail-under-lines 60 --summary-only` → **exit 0**（cargo-llvm-cov 0.8.7）

> 注：llvm-cov TOTAL 行第二列为 Missed Lines（未覆盖行数）；行覆盖率 = (总-未覆盖)/总。

## 提升路径复盘（40.55% → 61.57%，+21.02pp）

三个波次、全部离线可跑（HTTP mock / 纯逻辑测试）：

| 波次 | 手段 | 关键成果 |
|---|---|---|
| 一：api/impl HTTP mock | config `api_host_url` 重定向到本地 mock server（httpmock / 手写 MockServer） | pay +10.09pp、open +12.83pp、channel 全链路 |
| 二：api/impl 续（mp/cp/miniapp） | 同上 + 修复 mock↔bean 字段名/类型匹配 | mp/cp/miniapp 三 crate 全绿 |
| 三：巨型枚举/消息 Bean/trait 默认方法 | 脚本生成全量断言遍历 | common 错误枚举 0.38%→100%（+1,829 行）；miniapp 44.54%→70.12%；cp +9.84pp；mp +6.52pp；url 枚举群 100%；pay/channel trait 默认方法大面积覆盖 |

## 关键技术发现

1. **可配置 base URL 是 mock 化的前提**：`WxPayConfig::api_host_url()`、`WxChannelConfig`、`WxOpenHostConfig`、mp/miniapp/cp 的 config storage 均支持域名重定向——这是 api/impl 层可测性的架构基础。
2. **URL 常量必须对照 enums 源码逐一核对**（曾因臆测路径导致 mock fallthrough）。
3. **部分方法有数字参数校验**（如 category 的 cat_id 必须可解析为 i64），传参错误直接返回 -99 内部错误而非发起请求。
4. **巨型 match/枚举用脚本生成全量测试**：读取源码字面量生成断言数组，一次覆盖上千行。
5. **llvm-cov 并发限制**：共享 `target/llvm-cov-target` 的并发测量会互相破坏（偶发 "never executed"），必须串行运行。

## 结论

- V3 判定由「未通过」改为 **通过**：61.57% ≥ 60%，CI 门禁（ci.yml `--fail-under-lines 60`）本地验证 exit 0
- 覆盖率与台账双达标：workspace 1905 个测试全绿 + clippy `-D warnings` 干净 + fmt 干净
- 剩余未覆盖（25,523 行）主要为：pay 固定域名成功路径（真实微信域名，mock 需改 src）、并发锁竞争分支、不可达防御分支
