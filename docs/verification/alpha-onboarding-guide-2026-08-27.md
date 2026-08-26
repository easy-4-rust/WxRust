# WxRust v0.1.0 Alpha 灰度接入指南（第一个内部项目）

日期：2026-08-27
适用：`wx-rust-*` v0.1.0（已上线 crates.io，tag `v0.1.0`）

## 一、Alpha 目标（摘自 production-release-plan Phase 1）

- 接入 **1-2 个内部项目**，观察 **1-2 周**
- 验证目标：token 生命周期、核心 API 往返、错误路径、无 panic/内存问题
- 准出：P0/P1 功能零阻断性缺陷；观察指标全绿

## 二、接入方式（已发布 crate）

```toml
# 接入项目 Cargo.toml——按业务域选 crate
wx-rust-mp       = "0.1.0"   # 公众号
wx-rust-miniapp  = "0.1.0"   # 小程序
wx-rust-pay      = "0.1.0"   # 微信支付
wx-rust-cp       = "0.1.0"   # 企业微信
wx-rust-open     = "0.1.0"   # 第三方平台
wx-rust-channel  = "0.1.0"   # 视频号小店
# 或伞形引入
wx-rust          = "0.1.0"
```

```rust
// 异步用法（推荐）
use wx_rust_mp::api::impl::WxMpServiceImpl;
use std::sync::Arc;

let service = WxMpServiceImpl::new_arc(config);
let user_info = service.user_service().unwrap()
    .get_user_info(openid).await?;

// 同步用法（feature = "sync"，门面专用 runtime）
// [dependencies] wx-rust-miniapp = { version = "0.1.0", features = ["sync"] }
let blocking = WxMaServiceBlocking::new(WxMaServiceImpl::new_arc(config));
let session = blocking.js_code_to_session(code)?;
```

## 三、Alpha 接入检查单（接入项目按此执行）

### 3.1 选型与范围（Day 0）
- [ ] 选 1 个非资金主链路的内部服务（建议：消息推送/素材管理类，避开 pay 大额）
- [ ] 圈定调用的 API 清单（≤10 个方法，Alpha 首批）
- [ ] 配置 fallback：与现网 WxJava 双跑或可一键回切

### 3.2 接入实现（Day 1-2）
- [ ] 引入对应 crate，实现 ConfigStorage（token 持久化接现有存储）
- [ ] 按 `docs.rs/wx-rust-*` 文档对接（docs.rs 已自动构建）
- [ ] 错误处理统一走 `WxErrorException`（thiserror 枚举）
- [ ] 开启 tracing 观测（每次请求自动带 span）

### 3.3 观察期指标（Week 1-2，每日巡检）
| 指标 | 采集方式 | 告警阈值 |
|---|---|---|
| 请求成功率 | 接入方业务日志 | < 99.5% |
| token 刷新次数 | tracing span 统计 | 单 appid 每小时 > 6 次（正常 ~2 次/2h） |
| P99 延迟 | 接入方 APM | > 现网 WxJava 基线 +50% |
| panic/abort | 接入方错误日志 | 任何一次即 No-Go |
| 熔断器状态 | `CircuitBreaker`（如启用） | 非预期 Open |

### 3.4 No-Go 条件（任一命中即暂停 Alpha）
- 资金类接口金额/签名错误（一分钱都不允许）
- token 单飞失效（并发刷新风暴）
- panic / 数据损坏 / 内存泄漏

## 四、回滚方案

- 接入侧：feature flag 一键切回 WxJava 实现（双跑期间保留旧路径）
- 仓库侧：`cargo update --precise` 锁版本；问题修复后发 0.1.1
- crates.io 侧：如 0.1.0 存在严重缺陷且未及修复，`cargo yank wx-rust-xxx@0.1.0`（保留但新项目不可依赖）

## 五、反馈通道

- 缺陷：GitHub Issues（仓库 easy-4-rust/WxRust，已有 bug_report 模板）
- 已知限制：仓库根 `known-issues.md`（RSA RUSTSEC-2023-0071 例外、miniapp g3/g4 审计深度说明等）
- 每周 Alpha 复盘：对照本检查单 3.3 指标出周报

## 六、Alpha → Beta 准出标准

1. 1-2 个项目 × 1-2 周观察期完成，指标全绿
2. 镜像率补测报告（v2-mirror-rerate）中 Top 未镜像类补齐 P0 部分
3. 无 No-Go 事件
4. 覆盖率维持 ≥ 60% 门禁（当前 69.05%）
