# WxRust 高并发原生架构设计（并发模型收敛与传输抽象）

日期：2026-08-24
状态：已评审（基于双图谱证据校准）
来源：本会话架构设计讨论 + WxJava/WxRust 双知识图谱分析 + 源码核查

## 1. 背景

WxRust 已完成 WxJava 的语义迁移（对象台账 97.8% 已处置、61.57% 行覆盖、1905 测试、
clippy/fmt 干净）。本设计回答下一阶段问题：**在保持与 WxJava 语义逐字节一致的前提下，
按 Rust 特性把并发模型从「直译 Java」收敛为「原生异步」，支撑高并发场景。**

### 1.1 现状校准（源码核查结论，修正设计初稿的两处误判）

| 事实 | 证据 |
|---|---|
| 全库 **零 block_on**，服务层全 async（pay trait 99 个 async fn） | `grep -rn block_on crates/*/src` 为空 |
| token 刷新已是**正确单飞**：新鲜时无锁快读 → 过期时 try_lock(100ms 轮询) + 双检 + 3s 超时 | `wx_ma_service.rs:384-420` |
| `runtime()` 是**错误构造函数**（构造 `WxRuntimeError`），非 sync-over-async 桥；度数 537 来自全部 util 的错误构造 | `wx_pay_service_impl_utils.rs:42-45` |
| `execute_with_retry/execute_internal` 管线在 **6 份 base impl 中重复**（132~270 行/份） | cp/miniapp/open/mp/qidian/channel 六个 base impl |
| 锁纪律基本正确：24 处 tokio 锁 vs 6 处 std 锁（均为短临界区、不跨 await） | 全库扫描 |
| mock 依赖 `api_host_url` 改 host 重定向，传输层无抽象 | 现有全部 coverage_boost 测试 |

### 1.2 真实差距（按高并发影响排序）

1. 传输层无 trait 抽象：executor 直接持 `reqwest::Client`，测试只能改 host 走真网络栈
2. 管线 6 份重复：重试/token 失效重放语义散落，改一处需改六处
3. 无熔断/限流：微信侧 5xx 洪峰会穿透到调用方
4. 大响应全量入内存（对账单/媒体下载）
5. token try_lock 100ms 轮询为忙等风格（语义对，形态可优雅化）
6. 时间不可注入：过期测试依赖 sleep
7. Java 同步用户无迁移门面（异步是唯一形态）
8. 并发行为无基准：单飞正确性只靠单测，无并发压测证明

## 2. 目标与非目标

### 2.1 目标

- **G1 传输抽象**：`HttpTransport` trait + reqwest 默认实现 + `MockTransport`，测试脱离真网络栈
- **G2 管线收敛**：6 份 `execute_with_retry` 收敛到 wx-rust-common 单一实现，重试/token 失效单次重放语义保持
- **G3 过载防护**：per-host 熔断器（零新依赖自写），可选接入管线
- **G4 流式下载**：`execute_stream` 返回字节流，对账单/媒体下载不全量入内存
- **G5 时间注入**：`WxClock` trait，token 过期测试去 sleep
- **G6 同步门面**：`feature = "sync"` 下的 Blocking 包装（专用 runtime，杜绝 block_on 陷阱）
- **G7 并发验收**：criterion 并发基准（1000 并发下 token 刷新次数 = 1、零 block_on 门禁）入 CI
- **G8 完整性收口**：V0 剩余 59 缺口（pay 39 / open 8 等）、aispeech 孤立方法接线复核、4 份未覆盖错误枚举补测

### 2.2 非目标

- 不改变任何对外 trait 方法签名与行为（URL/参数/签名/错误码/golden 逐字节不变）
- 不引入 tower/tower-http 生态（reqwest 0.13 中间件生态未稳定公开，自写轻量件零依赖）
- 不引入 arc-swap（token 快路径已无锁，读侧 std Mutex 无竞争时 ~20ns，YAGNI）
- 不做分布式限流/多进程 token 共享（Redis 存储已由 config storage trait 覆盖）
- 不在本设计内处理 rsa 0.10 升级与 crates.io 发布顺序（外部依赖项，另行跟踪）

## 3. 方案比较

### 3.1 保持现状（直译 Java + host 重定向 mock）

零改动成本；但 6 份管线重复持续累积，传输不可注入使测试走真网络栈，无过载防护，并发正确性无证明。不采用。

### 3.2 全量 tower 生态重构

引入 tower Service 栈统一超时/重试/熔断/限流/指标。标准化程度最高；但 reqwest 0.13 未公开暴露 tower 组装点，需自建 HTTP 层，改动面大且引入重依赖，违背「语义保真优先」。不采用。

### 3.3 渐进收敛（最终方案）

传输抽象为 trait（mock 注入点）；管线收敛到 common 单实现并在内部结构化重试/熔断挂点；其余目标（流式/时钟/门面/基准）逐项独立落地。每步保持全量测试绿、可独立回滚。**采用。**

## 4. 模块与依赖

```
L4 门面  feature="sync" → Wx*ServiceBlocking（专用 current_thread runtime）
L3 服务  各 crate 子服务 trait（不动签名）
L2 管线  common::pipeline::execute_pipeline（重试环 + token 重放 + 熔断挂点）★新
         common::circuit::CircuitBreaker（per-host，自写 ~100 行）★新
L1 基础  common::http::HttpTransport trait ★新
         ├─ ReqwestTransport（默认实现，包装现有 Client 路径）
         └─ MockTransport（测试，进程内直接应答，零网络）
         common::clock::WxClock ★新
L0 传输  reqwest::Client（连接池复用，保持现状）
```

依赖方向不变（星型拓扑保持）；新增件全部落在 wx-rust-common，各 crate 仅改为调用管线。

## 5. 关键设计决策

### D1 HttpTransport trait（G1）

```rust
#[async_trait]
pub trait HttpTransport: Send + Sync {
    async fn send(&self, req: TransportRequest) -> Result<TransportResponse, WxErrorException>;
}
```
- `TransportRequest`/`TransportResponse` 为纯数据（method/url/headers/body/bytes），与 reqwest 类型解耦
- `ReqwestTransport` 包装现有执行路径；`MockTransport` 以闭包应答，测试零网络零端口
- 现有 `api_host_url` 重定向机制**保留**（向后兼容既有 1905 测试）

### D2 管线收敛（G2）

`common::pipeline::execute_pipeline` 承载现 6 份 `execute_with_retry` 的共同语义：
token 注入 URL、执行、errcode ∈ ACCESS_TOKEN_ERROR_CODES 时加锁比对 token → 过期 → 单次重放（防无限递归 flag）。各 crate 的 base impl 保留差异点（host 替换、稳定 token 双通道）作为参数/回调。

### D3 熔断器（G3，零新依赖）

Closed→Open（连续 N 次失败/时间窗）→HalfOpen（放行 1 探测）→Closed。per-host 实例放
`DashMap<String, Arc<CircuitBreaker>>`。默认不启用，`PipelineConfig::breaker` 可选注入——
不改变既有调用方行为。

### D4 流式下载（G4）

`execute_stream` 返回 `impl Stream<Item = Result<Bytes, WxErrorException>>`；pay
`download_bill/download_raw_bill` 先行改造，语义（gzip/解压/对账单格式）不变。

### D5 时钟注入（G5）

`WxClock`（`now_ms()`）；默认 `SystemClock`；测试注入 `FakeClock`。落地范围收敛在
common 的过期判断与 token 测试。

### D6 同步门面（G6）

`WxMaServiceBlocking`（首例，模式验证后推广）：内部 `Arc<tokio::runtime::Runtime>`
（current_thread，惰性单例），`block_on` 仅存在于门面内；门面**不实现** async trait，
类型上杜绝异步上下文误用。CI 以 grep 门禁禁止门面外 block_on。

### D7 并发基准（G7）

`benches/concurrency_bench.rs`：MockTransport + FakeClock 下 1000 并发调用，
断言 token 刷新次数 = 1、P99 延迟、熔断开合行为；criterion `--test` 模式进 CI。

## 6. 验收标准

- 现有 1905 测试零回归；覆盖率门禁 ≥60% 保持
- golden 差分（Java↔Rust）全绿；对象台账处置率 100%（V0 剩余 59 清零）
- `grep -rn block_on crates/*/src` 仅 sync 门面命中
- 并发基准：1000 并发 token 刷新次数 = 1；熔断器开/合行为有测试证明
- aispeech 孤立方法接线复核结论写入验证报告
