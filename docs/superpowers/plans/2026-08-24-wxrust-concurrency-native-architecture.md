# WxRust 高并发原生架构实施计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**状态：** 已完成（核对日期：2026-08-27，依据：T1-T13 全部落库，checkbox 收口 commit 202c2c3）

**Goal:** 在不改变任何对外行为的前提下，把 WxRust 的并发模型从「直译 Java」收敛为 Rust 原生：传输可注入、管线单实现、过载有熔断、大文件可流式、时间可注入、同步用户有门面，并以并发基准与 block_on 门禁证明高并发正确性；同时清零 V0 迁移缺口。

**Architecture:** 新增件全部落在 wx-rust-common（HttpTransport trait + execute_pipeline + CircuitBreaker + WxClock），六个模块的 base impl 改为调用统一管线；`feature = "sync"` 提供专用 runtime 的 Blocking 门面。星型拓扑与全部对外 trait 签名不变。

**Tech Stack:** tokio（rt/sync/time，已在 workspace）、reqwest 0.13（已在）、dashmap（已在）、criterion 0.5（已在 common dev-deps）、零新增第三方依赖。

**Spec:** `docs/superpowers/specs/2026-08-24-wxrust-concurrency-native-architecture-design.md`

## Global Constraints

- 语义保真：URL/参数/签名/错误码/golden 逐字节不变；现有 1905 个测试零回归。
- 对外 trait 方法签名一律不改；新增能力只走新 trait、新模块或 feature。
- 全库禁止 `block_on`，唯一例外：Task 9 的 sync 门面内部（专用 current_thread runtime）。CI 以 grep 门禁固化。
- 不跨 `.await` 持有 std 锁；新增锁一律 `tokio::sync`。
- 不新增第三方依赖（熔断器自写；不引 tower/arc-swap）。
- 覆盖率门禁 `--fail-under-lines 60` 保持通过。
- 每任务 TDD：先写失败测试并确认失败，再最小实现；提交前跑 `cargo test -p <涉及 crate>`、`cargo clippy --workspace --all-targets -- -D warnings`、`cargo fmt --all`。
- 所有命令需 `export PATH="$HOME/.cargo/bin:$PATH"`；磁盘 < 5Gi 时清理 `target/llvm-cov-target`。

---

## 1. 已确认需求与非目标

### 1.1 已确认需求

1. HttpTransport trait + reqwest 默认实现 + MockTransport（测试零网络）。
2. 6 份 execute_with_retry 管线收敛到 common 单实现，token 失效单次重放语义保持。
3. per-host 熔断器，默认关闭、可选注入。
4. pay 对账单下载流式化（execute_stream）。
5. WxClock 注入，token 过期测试去 sleep。
6. feature="sync" 同步门面（WxMaServiceBlocking 首例）。
7. 并发基准（1000 并发 token 刷新=1）+ block_on CI 门禁。
8. V0 剩余 59 缺口清零（pay 39、open 8 等）+ aispeech 孤立方法接线复核 + 4 份错误枚举补测。

### 1.2 非目标

- 不改对外签名与行为；不引 tower/arc-swap；不做分布式限流；不处理 rsa 升级与发布顺序（外部依赖项）。

---

### Task 1: HttpTransport trait 与 ReqwestTransport / MockTransport

**Files:**
- Create: `crates/wx-rust-common/src/http/mod.rs`、`crates/wx-rust-common/src/http/transport.rs`
- Modify: `crates/wx-rust-common/src/lib.rs`（加 `pub mod http;`）
- Test: `crates/wx-rust-common/tests/http_transport_test.rs`

**Interfaces:**
- Produces:
  - `pub struct TransportRequest { pub method: TransportMethod, pub url: String, pub headers: Vec<(String, String)>, pub body: TransportBody }`
  - `pub enum TransportMethod { Get, Post, PostJson(String), PostXml(String), PostForm(Vec<(String, String)>) }`
  - `pub enum TransportBody { None, Text(String), Bytes(Vec<u8>) }`
  - `pub struct TransportResponse { pub status: u16, pub headers: Vec<(String, String)>, pub body: Vec<u8> }`
  - `#[async_trait] pub trait HttpTransport: Send + Sync { async fn send(&self, req: TransportRequest) -> Result<TransportResponse, WxErrorException>; }`
  - `pub struct ReqwestTransport { client: reqwest::Client }`（`ReqwestTransport::new(client: reqwest::Client) -> Self`）
  - `pub struct MockTransport { handler: Arc<dyn Fn(&TransportRequest) -> Result<TransportResponse, WxErrorException> + Send + Sync> }`（`MockTransport::new<F>(f: F) -> Self where F: Fn(&TransportRequest) -> Result<TransportResponse, WxErrorException> + Send + Sync + 'static`，另提供 `MockTransport::ok_json(body: &str)` 便捷构造）

- [x] **Step 1: 写失败测试**

```rust
//! HttpTransport 抽象测试。RUST_OBLIGATION：trait 对象可用性 + MockTransport 零网络。
use wx_rust_common::http::{HttpTransport, MockTransport, TransportMethod, TransportRequest, TransportBody};

#[tokio::test]
async fn mock_transport_answers_without_network() {
    let t = MockTransport::new(|req| {
        assert_eq!(req.method, TransportMethod::Get);
        let body = format!("{{\"echo\":\"{}\"}}", req.url);
        Ok(wx_rust_common::http::TransportResponse {
            status: 200, headers: vec![], body: body.into_bytes(),
        })
    });
    let resp = t.send(TransportRequest { method: TransportMethod::Get, url: "https://mock.local/x".into(), headers: vec![], body: TransportBody::None }).await.unwrap();
    assert_eq!(resp.status, 200);
    assert!(String::from_utf8(resp.body).unwrap().contains("mock.local/x"));
}
```

- [x] **Step 2: 运行确认失败**

Run: `cargo test -p wx-rust-common --test http_transport_test`
Expected: FAIL（`unresolved module http`）

- [x] **Step 3: 最小实现**（`transport.rs`：上述类型与两个实现；`ReqwestTransport::send` 把请求映射到 `self.client` 执行并读取 status/headers/body；错误统一 `WxErrorException::Runtime`。`mod.rs` 写模块文档 + `pub use`。`lib.rs` 加 `pub mod http;`）

- [x] **Step 4: 运行测试通过**

Run: `cargo test -p wx-rust-common --test http_transport_test`
Expected: PASS

- [x] **Step 5: Commit**

```bash
git add crates/wx-rust-common/src/http crates/wx-rust-common/src/lib.rs crates/wx-rust-common/tests/http_transport_test.rs
git commit -m "feat(common): HttpTransport trait + ReqwestTransport/MockTransport"
```

---

### Task 2: 统一执行管线 execute_pipeline（重试 + token 失效单次重放）

**Files:**
- Create: `crates/wx-rust-common/src/pipeline/mod.rs`
- Modify: `crates/wx-rust-common/src/lib.rs`（加 `pub mod pipeline;`）
- Test: `crates/wx-rust-common/tests/execute_pipeline_test.rs`

**Interfaces:**
- Consumes: Task 1 的 `HttpTransport`/`TransportRequest`/`MockTransport`；现有 `crate::api::wx_consts::ACCESS_TOKEN_ERROR_CODES`
- Produces:

```rust
pub struct PipelineContext<'a> {
    pub transport: &'a dyn HttpTransport,
    pub access_token: String,
    /// 与 Java「uri 不允许带 access_token」一致的前置校验
    pub uri: String,
    pub body: TransportBody,
}
/// 执行并处理 token 失效重放。语义与 miniapp base impl 的 execute_with_retry 一致：
/// 首次执行；errcode ∈ ACCESS_TOKEN_ERROR_CODES 且 on_token_invalid（加锁比对→置过期，
/// 内部含 .await，故返回 BoxFuture）执行后，自动刷新开启且未重放过时重放一次（防无限递归 flag）。
pub async fn execute_pipeline<T, F>(
    ctx: PipelineContext<'_>,
    wx_type: crate::error::WxType,
    parse: F,
    on_token_invalid: Option<&dyn Fn() -> futures_util::future::BoxFuture<'static, ()>>,
) -> Result<T, WxErrorException>
where F: Fn(TransportResponse) -> Result<T, WxErrorException>;
```

- [x] **Step 1: 写失败测试**（三例：正常返回；首次 errcode=40001 + on_token_invalid 触发 + 第二次应答成功 → 重放一次成功；重放仍失败 → 返回错误且仅执行两次。用 `MockTransport` 计数器 `Arc<AtomicUsize>` 断言调用次数）

```rust
#[tokio::test]
async fn token_invalid_replays_exactly_once() {
    let calls = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let c = calls.clone();
    let t = MockTransport::new(move |_| {
        let n = c.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let body = if n == 0 { r#"{"errcode":40001,"errmsg":"invalid credential"}"#.to_string() }
                   else { r#"{"errcode":0,"errmsg":"ok","data":42}"#.to_string() };
        Ok(wx_rust_common::http::TransportResponse { status: 200, headers: vec![], body: body.into_bytes() })
    });
    let expired = std::sync::atomic::AtomicBool::new(false);
    let ctx = PipelineContext { transport: &t, access_token: "T1".into(), uri: "https://mock.local/get".into(), body: TransportBody::None };
    let r: i32 = wx_rust_common::pipeline::execute_pipeline(ctx, wx_rust_common::error::WxType::MiniApp,
        |resp| { let v: serde_json::Value = serde_json::from_slice(&resp.body).map_err(|e| wx_rust_common::error::WxErrorException::Runtime(e.to_string().into()))?; Ok(v["data"].as_i64().unwrap_or(0) as i32) },
        Some(&|| { let expired = &expired; async move { expired.store(true, std::sync::atomic::Ordering::SeqCst) }.boxed() })).await.unwrap();
    assert_eq!(r, 42);
    assert_eq!(calls.load(std::sync::atomic::Ordering::SeqCst), 2);
    assert!(expired.load(std::sync::atomic::Ordering::SeqCst));
}
```

（注：`WxErrorException::Runtime` 现为 `WxRuntimeError::new(impl Into<String>)` 构造，测试按现有错误变体名对齐，实现时以 `crates/wx-rust-common/src/error` 实际定义为准。）

- [x] **Step 2: 运行确认失败**（`cargo test -p wx-rust-common --test execute_pipeline_test` → FAIL 模块不存在）
- [x] **Step 3: 实现 execute_pipeline**（内部：`uri.contains("access_token=")` 时直接报错；组装 URL 注入 token；解析 errcode；命中 ACCESS_TOKEN_ERROR_CODES → 调 on_token_invalid → do_not_auto_refresh flag 循环重放一次；其余错误码直接 Err）
- [x] **Step 4: 测试通过 + 全量回归**（`cargo test -p wx-rust-common` 全绿）
- [x] **Step 5: Commit**（`git commit -m "feat(common): execute_pipeline 统一执行管线（token 失效单次重放）"`）

---

### Task 3: miniapp 接入统一管线（首个 crate，验证等价性）

**Files:**
- Modify: `crates/wx-rust-miniapp/src/api/impl/base_wx_ma_service_impl.rs`（`execute_with_retry` 改为薄封装：保留 host 替换 + 稳定 token 双通道差异点，主体委托 `execute_pipeline`，`on_token_invalid` 承接现有「加锁比对→expire」闭包）
- Test: 复用既有 `crates/wx-rust-miniapp/tests/`（零改动即应全绿——等价性证明）

**Interfaces:**
- Consumes: Task 2 `execute_pipeline/PipelineContext`；Task 1 `ReqwestTransport`（包装 base impl 现有 `http_client()`）

- [x] **Step 1: 基线记录**：`cargo test -p wx-rust-miniapp 2>&1 | tail -1` 记录通过数（当前 253）
- [x] **Step 2: 改造 execute_with_retry 为委托封装**（差异点：`effective_api_host_url` 替换、`is_stable_access_token` 通道选择留在 miniapp 侧；token 比对过期闭包原样传入）
- [x] **Step 3: 运行 miniapp 全量测试**：通过数不变（等价性证明）
- [x] **Step 4: clippy/fmt + Commit**（`git commit -m "refactor(miniapp): execute_with_retry 委托统一管线"`）

---

### Task 4: 其余五 crate 接入统一管线

**Files:**
- Modify: `crates/wx-rust-mp/src/api/impl/base_wx_mp_service_impl.rs`、`crates/wx-rust-cp/src/api/impl/base_wx_cp_service_impl.rs`、`crates/wx-rust-open/src/api/impl/base_wx_open_service_impl.rs`、`crates/wx-rust-qidian/src/api/impl/base_wx_qidian_service_impl.rs`、`crates/wx-rust-channel/src/api/impl/base_wx_channel_service_impl.rs`（channel 为 execute/execute0 变体，同样委托）

**Interfaces:**
- Consumes: 同 Task 3。各 crate 差异点（cp 的 corpid 域名、open 的 component_access_token 锁、qidian 的 host config）保留在各自封装内。

- [x] **Step 1: 逐 crate 记录基线**（`cargo test -p <crate> 2>&1 | tail -1`）
- [x] **Step 2: mp 接入 + 测试通过数不变 + commit**
- [x] **Step 3: cp 接入 + 测试通过数不变 + commit**
- [x] **Step 4: open 接入 + 测试通过数不变 + commit**（实际未接入：component_access_token 键 + 重放重建 URL 与管线不同构；保持旧路径）
- [x] **Step 5: qidian 接入 + 测试通过数不变 + commit**
- [x] **Step 6: channel 接入（execute/execute0 变体）+ 测试通过数不变 + commit**（实际仅 post 走管线；GET 字节序被现有测试冻结，保持旧路径）
- [x] **Step 7: workspace 全量回归**（`cargo test --workspace`，1905 全绿）

---

### Task 5: token try_lock 轮询改 timeout 等待（保语义微调）

**Files:**
- Modify: `crates/wx-rust-miniapp/src/api/wx_ma_service.rs:398-415`（try_lock 100ms 轮询循环 → `tokio::time::timeout(Duration::from_millis(3000), lock.lock()).await`，超时返回同一错误文案「获取accessToken超时：获取时间超时」；双检（取锁后再查 is_access_token_expired）保持）
- Test: `crates/wx-rust-miniapp/tests/token_lock_timeout_test.rs`

- [x] **Step 1: 写失败测试**：持有锁的一方 sleep 3.2s，调用方应在 ~3s 收到含「获取accessToken超时」的错误（用 `tokio::time::pause()` + FakeClock 不适用于 tokio::time——直接真实时钟，测试限时 <4s）
- [x] **Step 2: 确认失败/确认现状行为**（改造前该路径为轮询，同样 3s 超时——测试先证明语义，改造后仍绿）
- [x] **Step 3: 改造为 timeout(lock().await)**
- [x] **Step 4: `cargo test -p wx-rust-miniapp` 全绿 + Commit**

---

### Task 6: CircuitBreaker（per-host 熔断，零依赖）

**Files:**
- Create: `crates/wx-rust-common/src/circuit/mod.rs`
- Modify: `crates/wx-rust-common/src/lib.rs`、`crates/wx-rust-common/src/pipeline/mod.rs`（`PipelineContext` 增加可选 `breaker: Option<&CircuitBreaker>` 字段——默认 None，行为不变）
- Test: `crates/wx-rust-common/tests/circuit_breaker_test.rs`

**Interfaces:**
- Produces:

```rust
pub struct CircuitBreaker { /* 连续失败计数/窗口/状态机，内部 tokio::sync::Mutex */ }
impl CircuitBreaker {
    pub fn new(failure_threshold: u32, open_duration: std::time::Duration) -> Self;
    /// 请求前调用：Open 且未到 HalfOpen 时返回 Err（错误文案「熔断器开启：<host>」）
    pub async fn before(&self, host: &str) -> Result<(), WxErrorException>;
    /// 成功→复位；失败→计数，达阈值→Open
    pub async fn after(&self, host: &str, ok: bool);
}
```

- [x] **Step 1: 写失败测试**：阈值 2、Open 100ms——两次失败后第三次 `before` 返回 Err；advance 110ms 后放行一次（HalfOpen 探测）成功后复位 Closed
- [x] **Step 2: 确认失败 → 实现 → 通过**
- [x] **Step 3: 管线接入测试**：breaker=Some 时熔断期间零 transport 调用（MockTransport 计数=0）
- [x] **Step 4: 全量回归 + Commit**

---

### Task 7: execute_stream 流式下载（pay 对账单先行）

**Files:**
- Create: `crates/wx-rust-common/src/pipeline/stream.rs`
- Modify: `crates/wx-rust-pay/src/api/impl/base_wx_pay_service_impl.rs`（`download_bill/download_raw_bill` 增加流式变体 `download_bill_stream`，原方法保留且内部聚合流——签名不动，新增方法）
- Test: `crates/wx-rust-pay/tests/download_stream_test.rs`

**Interfaces:**
- Produces: `pub async fn execute_stream(transport: &dyn HttpTransport, url: String, token: String) -> Result<impl futures_util::Stream<Item = Result<bytes::Bytes, WxErrorException>> + Send, WxErrorException>`（futures-util/bytes 已在 workspace deps）

- [x] **Step 1: 失败测试**：MockTransport 返回 3 个分块，断言流聚合等于全量、逐块顺序正确
- [x] **Step 2: 实现（ReqwestTransport 需补 `send_stream`）→ 通过**
- [x] **Step 3: pay 流式变体 + golden 对照（与既有 download_bill 输出逐字节一致）**
- [x] **Step 4: 全量回归 + Commit**

---

### Task 8: WxClock 注入（token 过期测试去 sleep）

**Files:**
- Create: `crates/wx-rust-common/src/clock.rs`
- Modify: `crates/wx-rust-common/src/lib.rs`；`wx-rust-common/src/config/mod.rs`（`is_access_token_expired` 等过期判断改经 `WxClock`，默认 `SystemClock`——行为不变）
- Test: `crates/wx-rust-common/tests/clock_test.rs`

**Interfaces:**
- Produces: `pub trait WxClock: Send + Sync { fn now_ms(&self) -> i64; }`、`pub struct SystemClock;`、`pub struct FakeClock(pub std::sync::Arc<std::sync::atomic::AtomicI64>)`（`FakeClock::advance_ms(i64)`）

- [x] **Step 1: 失败测试**（FakeClock advance 越过 expires_in → `is_access_token_expired` 翻转，零 sleep）
- [x] **Step 2: 实现 → 通过 → 全量回归（SystemClock 默认下既有测试不变）**
- [x] **Step 3: Commit**

---

### Task 9: feature="sync" 同步门面（WxMaServiceBlocking）

**Files:**
- Create: `crates/wx-rust-miniapp/src/blocking.rs`（`#![cfg(feature="sync")]`）
- Modify: `crates/wx-rust-miniapp/Cargo.toml`（`[features] sync = []`；`tokio` feature 增加 `"rt"` 已具备）
- Test: `crates/wx-rust-miniapp/tests/blocking_facade_test.rs`（`#![cfg(feature="sync")]`）

**Interfaces:**
- Produces:

```rust
pub struct WxMaServiceBlocking { inner: Arc<crate::api::impl::WxMaServiceImpl>, rt: std::sync::Arc<tokio::runtime::Runtime> }
impl WxMaServiceBlocking {
    pub fn new(inner: Arc<crate::api::impl::WxMaServiceImpl>) -> Self; // 惰性全局 current_thread runtime
    pub fn js_code_to_session(&self, /* 与 async 版同参 */) -> Result<.., WxErrorException>; // 首批 3 个高频方法
}
```
（门面不实现 async trait——类型上杜绝异步上下文误用。）

- [x] **Step 1: 失败测试**：同步上下文（非 #[tokio::test] 的 #[test]）调用 `js_code_to_session`（MockTransport 经 host 重定向），返回解析结果
- [x] **Step 2: 实现（OnceLock 全局 current_thread runtime + block_on 仅在此文件）→ 通过**
- [x] **Step 3: block_on 门禁**：`scripts/check_block_on.sh`——`grep -rn "block_on" crates/*/src --include="*.rs" | grep -v "blocking.rs" | grep -v "^.*//"` 非空则 exit 1；接入 ci.yml 新 step
- [x] **Step 4: 全量回归（含 feature 开关两态）+ Commit**

---

### Task 10: 并发基准与 CI 并发验收

**Files:**
- Create: `crates/wx-rust-common/benches/concurrency_bench.rs`
- Modify: `crates/wx-rust-common/Cargo.toml`（`[[bench]] name="concurrency_bench" harness=false`）、`.github/workflows/ci.yml`（test job 后追加 `cargo bench -p wx-rust-common --bench concurrency_bench -- --test`）

**Interfaces:**
- Consumes: Task 1 MockTransport、Task 8 FakeClock、Task 6 CircuitBreaker

- [x] **Step 1: 写基准**：三组——(a) 1000 并发走 execute_pipeline 共享未过期 token，断言 MockTransport 收到的 token 全同且请求全成功；(b) token 过期场景 1000 并发刷新，断言「刷新应答」计数 = 1（单飞证明）；(c) 熔断阈值压测开合行为
- [x] **Step 2: `cargo bench -p wx-rust-common --bench concurrency_bench -- --test` 本地通过**
- [x] **Step 3: ci.yml 接入 + YAML 校验（python3 yaml.safe_load）**
- [x] **Step 4: 全量回归 + Commit**

---

### Task 11: 4 份错误枚举补测（cp/ma/mp/channel）

**Files:**
- Test: `crates/wx-rust-common/tests/coverage_boost_error_enum_rest.rs`

- [x] **Step 1: 复用 open 枚举的脚本遍历模式**（读取 `wx_cp/wx_ma/wx_mp/wx_channel_error_msg_enum.rs` 的 match 臂生成 ALL_CODES 循环 + 未知码 None + 各 5 条 Java golden 比对，`// 对应 Java:` 注释；Java 参照 `WxJava/weixin-java-{cp,miniapp,mp,channel}` 对应枚举）
- [x] **Step 2: `cargo test -p wx-rust-common` 全绿（预计 +815 行覆盖）**
- [x] **Step 3: 覆盖率复测 ≥61.5% 保持 + Commit**

---

### Task 12: V0 剩余缺口清零（pay 39 / open 8 等 59 项）

**Files:**
- 参照：`python3 scripts/audit_migration_layout.py --verbose`（MISSING 清单）
- Create: `docs/verification/V0-gap-closure.md`（逐项处置结论）

- [x] **Step 1: 运行审计取最新 MISSING 清单**（pay 的 v3 auth/crypto 类、open 的 8 项等）
- [x] **Step 2: 逐项处置**——能实现则实现（pay v3 的 CertificateDownloader/Verifier 等语义迁移）；确属平台不可达（JNI/云内部）的记 PLATFORM_NA 并写明依据到报告
- [x] **Step 3: 重跑审计至 MISSING=0（或全部转 PLATFORM_NA 且有依据）**
- [x] **Step 4: 全量回归 + 报告 + Commit**

---

### Task 13: aispeech 孤立方法接线复核

**Files:**
- Create: `docs/verification/aispeech-wiring-audit.md`

- [x] **Step 1: 对照图谱 50 个 degree=1 方法清单（WxAispeechDialogService/KnowledgeService trait 方法）逐一核对：facade 是否暴露、impl 是否接线、测试是否覆盖**
- [x] **Step 2: 缺接线的补线、缺测试的补测（沿用三层规范）**
- [x] **Step 3: 结论写入报告（含图谱复核前后对照）+ 全量回归 + Commit**

---

## 2. 验收矩阵

| 目标 | 任务 | 证明 |
|---|---|---|
| G1 传输抽象 | T1 | http_transport_test 绿 |
| G2 管线收敛 | T2-T4 | 6 crate 测试通过数逐项不变 + workspace 1905 全绿 |
| G3 熔断 | T6 | 熔断开合测试 + 零 transport 调用证明 |
| G4 流式 | T7 | 分块顺序测试 + golden 逐字节一致 |
| G5 时钟 | T8 | 零 sleep 过期翻转测试 |
| G6 门面 | T9 | 同步上下文测试 + block_on grep 门禁绿 |
| G7 并发验收 | T10 | 1000 并发刷新=1 基准入 CI |
| G8 完整性 | T11-T13 | 错误枚举覆盖、V0 MISSING=0、aispeech 报告 |

## 3. 风险与回滚

- T3/T4 等价性风险以「每 crate 基线通过数不变」为硬门禁，任一变化即中止该 crate 改造。
- T6/T9 默认关闭/独立 feature，不影响既有行为；任一任务可独立回滚（逐任务提交）。
