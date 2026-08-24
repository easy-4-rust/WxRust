//! token 单飞并发验收基准（G7 核心证明，计划 Task 10 组 2）。
//!
//! 证明目标：1000 个并发 `get_access_token()`（config 起始无 token →
//! 全部走慢路径刷新）在双检锁单飞语义下**只发起 1 次 token 端点 HTTP
//! 请求**，且全部 1000 个调用者拿到同一 token。
//!
//! 为什么在本 crate：单飞路径在 `WxMaService::get_access_token_with_force`
//! 的真实刷新链路（双检锁 + token HTTP 请求 + 缓存更新）；common 不能反向
//! 依赖 miniapp，故共享 token / 熔断两组基准在
//! `wx-rust-common/benches/pipeline_concurrency_bench.rs`，本文件专注
//! 真实 token 刷新单飞（mock 模式沿用
//! `tests/token_lock_timeout_test.rs` 的 CountingServer：本机 TcpListener
//! + `WxMaHostConfig.api_host` 重定向，计数一律 AtomicUsize）。
//!
//! 运行：
//! - 验收（一次迭代 + 断言，输出含「token 端点应答数 == 1」证明）：
//!   `cargo bench -p wx-rust-miniapp --bench token_single_flight_bench -- --test`
//! - 计时：`cargo bench -p wx-rust-miniapp --bench token_single_flight_bench`
//!
//! 零新增第三方依赖：criterion（workspace 既有，见 dev-deps）、
//! tokio / futures-util（既有依赖）。runtime 用 current_thread
//! （与 blocking 门面同一选型）：1000 个调用者在 `.await` 点交错并发，
//! 赢家持锁发 HTTP，999 个等待者在双检锁上提前返回。

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use criterion::{Criterion, criterion_group, criterion_main};
use futures_util::future::join_all;

use wx_rust_miniapp::api::WxMaService;
use wx_rust_miniapp::api::r#impl::WxMaServiceImpl;
use wx_rust_miniapp::config::WxMaConfig;
use wx_rust_miniapp::config::WxMaHostConfig;
use wx_rust_miniapp::config::r#impl::WxMaDefaultConfig;

/// 并发调用者规模：1000（计划 G7 验收值）。
const CONCURRENT_CALLERS: usize = 1000;
/// mock token 端点返回的 token 值（全部调用者应拿到同一值）。
const MOCK_TOKEN: &str = "BENCH_SINGLE_FLIGHT_TOKEN";

/// 极简计数 mock token 端点：对任意请求应答固定 token JSON 并计数。
///
/// 单飞断言的"1"即来自该计数——若实现失去单飞（如锁失效退化为人人
/// 请求），计数会等于并发数使断言失败。
struct CountingServer {
    addr: std::net::SocketAddr,
    requests: Arc<AtomicUsize>,
    stop: Arc<std::sync::atomic::AtomicBool>,
}

impl CountingServer {
    /// 起动服务器（须在目标 runtime 的 `block_on` 内调用，accept 循环
    /// spawn 到该 runtime、由后续 block_on 一并驱动）。
    async fn start() -> Self {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("绑定端口");
        let addr = listener.local_addr().expect("获取地址");
        let requests = Arc::new(AtomicUsize::new(0));
        let stop = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let requests_clone = requests.clone();
        let stop_clone = stop.clone();
        tokio::spawn(async move {
            loop {
                if stop_clone.load(Ordering::SeqCst) {
                    break;
                }
                let Ok((mut socket, _)) = listener.accept().await else {
                    continue;
                };
                requests_clone.fetch_add(1, Ordering::SeqCst);
                tokio::spawn(async move {
                    use tokio::io::{AsyncReadExt, AsyncWriteExt};
                    let mut buf = [0u8; 16384];
                    let _ = socket.read(&mut buf).await.unwrap_or(0);
                    let body = format!(r#"{{"access_token":"{MOCK_TOKEN}","expires_in":7200}}"#);
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                        body.len(),
                        body
                    );
                    let _ = socket.write_all(response.as_bytes()).await;
                });
            }
        });
        Self {
            addr,
            requests,
            stop,
        }
    }

    fn url(&self) -> String {
        format!("http://{}", self.addr)
    }

    fn request_count(&self) -> usize {
        self.requests.load(Ordering::SeqCst)
    }
}

impl Drop for CountingServer {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
    }
}

/// 构建指向 mock 服务器的小程序配置（**无预置 token** → 首次调用即
/// 过期慢路径，1000 个调用者同时竞争刷新）。
fn config_with_host(host: &str) -> Arc<dyn WxMaConfig> {
    let config = WxMaDefaultConfig::new("wxappid", "secret");
    let mut host_config = WxMaHostConfig::new();
    host_config.api_host = host.to_string();
    config.set_host_config(host_config);
    Arc::new(config)
}

/// 组 2：1000 并发单飞 token 刷新（核心证明）。
///
/// 每次迭代（criterion `--test` 模式即本迭代一次，断言失败 = bench 失败）：
/// 1. 新建无 token 的 config + service（上一迭代缓存的新 token 不影响本批）；
/// 2. `join_all` 1000 个并发 `get_access_token()`；
/// 3. 断言全部成功、全部返回同一 token（== mock 端点返回值）；
/// 4. 断言 token 端点应答计数增量 == 1（单飞证明）并以 eprintln 留证。
fn thousand_concurrent_single_flight_refresh(c: &mut Criterion) {
    // current_thread runtime：驱动 1000 个并发调用者 + mock 服务器任务
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("构建 current_thread tokio runtime 失败");
    let server = rt.block_on(CountingServer::start());

    let mut group = c.benchmark_group("token_single_flight");
    group.sample_size(10);
    group.warm_up_time(std::time::Duration::from_millis(500));
    group.measurement_time(std::time::Duration::from_secs(3));

    group.bench_function("thousand_concurrent_single_flight_refresh", |b| {
        b.iter(|| {
            let config = config_with_host(&server.url());
            assert!(
                config.is_access_token_expired(),
                "前置：起始无 token → 已过期 → 全部走慢路径"
            );
            let service = WxMaServiceImpl::new_arc(config);

            let before = server.request_count();
            let futs = (0..CONCURRENT_CALLERS).map(|_| service.get_access_token());
            let tokens = rt.block_on(join_all(futs));
            let responses = server.request_count() - before;

            // 断言 1：1000 全部成功
            assert_eq!(tokens.len(), CONCURRENT_CALLERS);
            let ok_tokens: Vec<&String> = tokens
                .iter()
                .map(|r| r.as_ref().expect("并发 get_access_token 应全部成功"))
                .collect();
            // 断言 2：全部拿到同一 token（== mock 端点值）
            assert!(
                ok_tokens.iter().all(|t| t.as_str() == MOCK_TOKEN),
                "1000 个调用者应全部拿到同一 token"
            );
            // 断言 3（核心单飞证明）：token 端点应答数 == 1
            assert_eq!(
                responses, 1,
                "单飞证明失败：{CONCURRENT_CALLERS} 并发刷新应只发起 1 次 token 请求，实际 {responses}"
            );
            eprintln!(
                "[single-flight] {CONCURRENT_CALLERS} 并发 get_access_token → token 端点应答数 = {responses}（应为 1），全部返回同一 token"
            );
            responses
        })
    });

    group.finish();
}

criterion_group!(benches, thousand_concurrent_single_flight_refresh);
criterion_main!(benches);
