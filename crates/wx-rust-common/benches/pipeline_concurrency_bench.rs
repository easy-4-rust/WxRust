//! 并发验收基准（G7）：统一管线 [`execute_pipeline`] 与
//! [`CircuitBreaker`] 在 1000 并发下的正确性证明（criterion `--test`
//! 模式 = 每个基准跑一次迭代做断言；正式 bench 模式同样先断言再计时——
//! 断言失败即 bench 失败，CI 红）。
//!
//! 覆盖两组（计划 Task 10 的组 1/组 3；组 2「token 单飞」依赖真实
//! token 刷新路径，落在 `wx-rust-miniapp/benches/token_single_flight_bench.rs`
//! ——common 不能反向依赖 miniapp）：
//!
//! 1. [`thousand_concurrent_share_fresh_token`]：1000 并发
//!    `execute_pipeline` 共享同一未过期 token + [`MockTransport`]（零网络、
//!    计数应答）——断言 1000 全成功、transport 应答计数 = 1000、所有应答
//!    携带同一 token；
//! 2. [`circuit_breaker_open_close_under_load`]：熔断阈值 5 / 窗口 50ms，
//!    混合应答（前段失败后恢复）并发驱动——断言曾进入 Open（存在
//!    「熔断器开启」拒绝且 Open 期间零 transport 调用）→ 窗口后 HalfOpen
//!    探测成功 → Closed（后续并发全放行）。
//!
//! 运行：
//! - 验收（一次迭代 + 断言）：`cargo bench -p wx-rust-common --bench
//!   pipeline_concurrency_bench -- --test`
//! - 计时：`cargo bench -p wx-rust-common --bench pipeline_concurrency_bench`
//!
//! 零新增依赖：criterion（common 既有 dev-dep）、tokio / futures-util /
//! dashmap（既有依赖）。runtime 用 current_thread（workspace tokio 未启用
//! rt-multi-thread；纯异步 MockTransport 场景下并发度不受单线程影响——
//! 任务在 `.await` 点交错即并发）。计数一律 [`std::sync::atomic::AtomicUsize`]。

use std::collections::HashSet;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::Duration;

use criterion::{Criterion, criterion_group, criterion_main};
use futures_util::future::join_all;

use wx_rust_common::circuit::CircuitBreaker;
use wx_rust_common::enums::WxType;
use wx_rust_common::http::{MockTransport, TransportBody, TransportResponse};
use wx_rust_common::pipeline::{PipelineContext, execute_pipeline};

/// 组 1 并发规模：1000（计划 G7 验收值）。
const THOUSAND: usize = 1000;
/// 组 3 熔断参数：连续失败阈值 5、Open 窗口 50ms（计划 Task 10 指定）。
const BREAKER_THRESHOLD: u32 = 5;
const BREAKER_WINDOW_MS: u64 = 50;

/// current_thread runtime（纯异步 MockTransport 无网络 I/O，并发度由
/// `.await` 交错保证；与 miniapp blocking 门面同一 runtime 选型）。
fn runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("构建 current_thread tokio runtime 失败")
}

/// 组 1：1000 并发共享未过期 token。
///
/// 断言（单飞之外的第二条并发正确性：并发读共享 token 零刷新、零错发）：
/// - 1000 个 `execute_pipeline` 全部成功且解析值正确；
/// - transport 应答计数 = 1000（每个调用恰好一次，无多余重试）；
/// - 所有请求携带同一 token（`access_token=` 查询参数唯一）。
fn thousand_concurrent_share_fresh_token(c: &mut Criterion) {
    let rt = runtime();
    let mut group = c.benchmark_group("pipeline_concurrency");
    group.sample_size(10);

    group.bench_function("thousand_concurrent_share_fresh_token", |b| {
        b.iter(|| {
            const FRESH_TOKEN: &str = "FRESH_TOKEN_BENCH";
            let calls = Arc::new(AtomicUsize::new(0));
            let seen_tokens: Arc<Mutex<HashSet<String>>> = Arc::new(Mutex::new(HashSet::new()));

            let calls_handler = calls.clone();
            let tokens_handler = seen_tokens.clone();
            let transport = MockTransport::new(move |req| {
                calls_handler.fetch_add(1, Ordering::SeqCst);
                // 提取注入的 access_token 查询参数（管线保证 uri 不自带、
                // 管线拼接 `?access_token=` 或 `&access_token=`）
                let token = req
                    .url
                    .split("access_token=")
                    .nth(1)
                    .unwrap_or_default()
                    .to_string();
                tokens_handler.lock().unwrap().insert(token);
                Ok(TransportResponse {
                    status: 200,
                    headers: vec![],
                    body: br#"{"errcode":0,"errmsg":"ok","data":42}"#.to_vec(),
                })
            });

            let futs = (0..THOUSAND).map(|_| {
                execute_pipeline(
                    PipelineContext {
                        transport: &transport,
                        access_token: FRESH_TOKEN.to_string(),
                        uri: "https://bench.local/cgi-bin/info".to_string(),
                        body: TransportBody::None,
                        replay_on_token_invalid: true,
                        breaker: None,
                    },
                    WxType::MiniApp,
                    |resp: TransportResponse| {
                        let v: serde_json::Value = serde_json::from_slice(&resp.body)
                            .map_err(|e| wx_rust_common::error::WxErrorException::Serde(
                                e.to_string(),
                            ))?;
                        Ok(v["data"].as_i64().unwrap_or_default())
                    },
                    None,
                )
            });
            let results = rt.block_on(join_all(futs));

            // 断言 1：1000 全成功且值正确
            assert_eq!(results.len(), THOUSAND);
            for (i, r) in results.iter().enumerate() {
                assert_eq!(*r.as_ref().expect("并发执行应全部成功"), 42, "第 {i} 个调用");
            }
            // 断言 2：transport 应答计数 = 1000（零多余请求）
            let answered = calls.load(Ordering::SeqCst);
            assert_eq!(answered, THOUSAND, "应答计数应恰为 1000");
            // 断言 3：所有应答携带同一 token
            let tokens = seen_tokens.lock().unwrap();
            assert_eq!(tokens.len(), 1, "全部请求应携带同一未过期 token");
            assert!(tokens.contains(FRESH_TOKEN));
            eprintln!(
                "[share-fresh-token] {THOUSAND} 并发全成功，transport 应答 = {answered}，唯一 token 数 = {}",
                tokens.len()
            );
            results.len()
        })
    });

    group.finish();
}

/// 组 3：熔断阈值压测开合行为（阈值 5 / 窗口 50ms）。
///
/// 场景（每批迭代全部成立才计时有效）：
/// 1. 顺序驱动 5 个失败请求（errcode -1 系统繁忙）→ 达阈值转 Open；
/// 2. 第 6 个请求被 `before` 拒绝且 **零 transport 调用**（G3 证明），
///    错误文案含「熔断器开启」；
/// 3. Open 期间 20 并发全部被拒绝、transport 计数不变（熔断即过载保护）；
/// 4. 窗口 50ms 过后（sleep 60ms）HalfOpen 放行探测——transport 转为
///    成功应答（`recover` 翻转），探测成功复位 Closed；
/// 5. Closed 后 10 并发全部经 transport 成功放行（无拒绝）。
fn circuit_breaker_open_close_under_load(c: &mut Criterion) {
    let rt = runtime();
    let mut group = c.benchmark_group("pipeline_concurrency");
    // 每次迭代含固定 60ms 冷却等待：缩小采样保持 bench 总时长可控
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(500));
    group.measurement_time(Duration::from_secs(3));

    group.bench_function("circuit_breaker_open_close_under_load", |b| {
        b.iter(|| {
            let calls = Arc::new(AtomicUsize::new(0));
            let breaker_rejections = Arc::new(AtomicUsize::new(0));
            let recover = Arc::new(AtomicBool::new(false));

            let calls_handler = calls.clone();
            let recover_handler = recover.clone();
            let transport = MockTransport::new(move |_req| {
                calls_handler.fetch_add(1, Ordering::SeqCst);
                // 混合应答：恢复前 errcode -1（非 token 失效码，不触发重放），
                // 恢复后 errcode 0
                let body = if recover_handler.load(Ordering::SeqCst) {
                    br#"{"errcode":0,"errmsg":"ok","data":7}"#.to_vec()
                } else {
                    br#"{"errcode":-1,"errmsg":"system busy"}"#.to_vec()
                };
                Ok(TransportResponse {
                    status: 200,
                    headers: vec![],
                    body,
                })
            });
            let breaker = CircuitBreaker::new(
                BREAKER_THRESHOLD,
                Duration::from_millis(BREAKER_WINDOW_MS),
            );

            let run_one = |transport: &MockTransport,
                           breaker: &CircuitBreaker|
             -> Result<i64, wx_rust_common::error::WxErrorException> {
                let fut = execute_pipeline(
                    PipelineContext {
                        transport,
                        access_token: "T".to_string(),
                        uri: "https://breaker.local/cgi-bin/info".to_string(),
                        body: TransportBody::None,
                        replay_on_token_invalid: false,
                        breaker: Some(breaker),
                    },
                    WxType::MiniApp,
                    |resp: TransportResponse| {
                        let v: serde_json::Value = serde_json::from_slice(&resp.body).map_err(
                            |e| wx_rust_common::error::WxErrorException::Serde(e.to_string()),
                        )?;
                        Ok(v["data"].as_i64().unwrap_or_default())
                    },
                    None,
                );
                rt.block_on(fut)
            };

            // —— 1. 顺序失败达阈值：5 个 errcode -1 → Open ——
            for i in 0..BREAKER_THRESHOLD {
                let r = run_one(&transport, &breaker);
                assert!(r.is_err(), "失败应答（errcode -1）应返回错误（第 {} 个）", i + 1);
            }
            assert_eq!(
                calls.load(Ordering::SeqCst),
                BREAKER_THRESHOLD as usize,
                "阈值前 5 个请求都应到达 transport"
            );

            // —— 2. 第 6 个：Open 拒绝 + 零 transport 调用 ——
            let rejected = run_one(&transport, &breaker);
            let err = rejected.expect_err("熔断开启后应拒绝放行");
            assert!(
                err.to_string().contains("熔断器开启"),
                "应为熔断拒绝错误，实际：{err}"
            );
            assert_eq!(
                calls.load(Ordering::SeqCst),
                BREAKER_THRESHOLD as usize,
                "Open 期间零 transport 调用（G3 过载保护证明）"
            );

            // —— 3. Open 期间并发负载：20 并发全拒绝、计数不变 ——
            let futs = (0..20usize).map(|_| {
                execute_pipeline(
                    PipelineContext {
                        transport: &transport,
                        access_token: "T".to_string(),
                        uri: "https://breaker.local/cgi-bin/info".to_string(),
                        body: TransportBody::None,
                        replay_on_token_invalid: false,
                        breaker: Some(&breaker),
                    },
                    WxType::MiniApp,
                    |resp: TransportResponse| {
                        let v: serde_json::Value = serde_json::from_slice(&resp.body).map_err(
                            |e| wx_rust_common::error::WxErrorException::Serde(e.to_string()),
                        )?;
                        Ok(v["data"].as_i64().unwrap_or_default())
                    },
                    None,
                )
            });
            let rejected_counts: Vec<bool> =
                rt.block_on(join_all(futs)).into_iter().map(|r| r.is_err()).collect();
            assert!(
                rejected_counts.iter().all(|&rejected| rejected),
                "Open 期间 20 并发应全部被拒绝"
            );
            assert_eq!(
                calls.load(Ordering::SeqCst),
                BREAKER_THRESHOLD as usize,
                "Open 期间并发负载仍零 transport 调用"
            );
            breaker_rejections.fetch_add(rejected_counts.len() as usize, Ordering::SeqCst);

            // —— 4. 窗口过后 HalfOpen 探测：恢复成功 → Closed ——
            std::thread::sleep(Duration::from_millis(BREAKER_WINDOW_MS + 10));
            recover.store(true, Ordering::SeqCst);
            let probe = run_one(&transport, &breaker);
            assert_eq!(
                probe.expect("HalfOpen 探测应放行且成功"),
                7,
                "探测请求应拿到恢复后的成功应答"
            );

            // —— 5. Closed：10 并发全部经 transport 成功放行 ——
            let futs = (0..10usize).map(|_| {
                execute_pipeline(
                    PipelineContext {
                        transport: &transport,
                        access_token: "T".to_string(),
                        uri: "https://breaker.local/cgi-bin/info".to_string(),
                        body: TransportBody::None,
                        replay_on_token_invalid: false,
                        breaker: Some(&breaker),
                    },
                    WxType::MiniApp,
                    |resp: TransportResponse| {
                        let v: serde_json::Value = serde_json::from_slice(&resp.body).map_err(
                            |e| wx_rust_common::error::WxErrorException::Serde(e.to_string()),
                        )?;
                        Ok(v["data"].as_i64().unwrap_or_default())
                    },
                    None,
                )
            });
            let closed_results = rt.block_on(join_all(futs));
            assert!(
                closed_results
                    .iter()
                    .all(|r| matches!(r, Ok(v) if *v == 7)),
                "Closed 后并发应全部成功"
            );
            assert_eq!(
                calls.load(Ordering::SeqCst),
                BREAKER_THRESHOLD as usize + 1 + 10,
                "探测 1 次 + Closed 后 10 次全部到达 transport"
            );
            eprintln!(
                "[breaker] Open 拒绝（并发）= {}，HalfOpen 探测成功，Closed 后 10 并发全放行，transport 总计 = {}",
                breaker_rejections.load(Ordering::SeqCst),
                calls.load(Ordering::SeqCst)
            );
            calls.load(Ordering::SeqCst)
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    thousand_concurrent_share_fresh_token,
    circuit_breaker_open_close_under_load
);
criterion_main!(benches);
