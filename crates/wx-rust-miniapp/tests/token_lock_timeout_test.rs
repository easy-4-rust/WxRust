//! token 锁等待语义测试（并发原生架构 Task 5：try_lock 100ms 轮询 →
//! `tokio::time::timeout` 等待，语义保真微调）。
//!
//! 锁定两条对外语义（改造前后均应绿——等价性回归）：
//! 1. 锁等待总超时 3 秒，超时返回业务错误，错误文案逐字为
//!    「获取accessToken超时：获取时间超时」（对应 Java
//!    `WxRuntimeException("获取accessToken超时：获取时间超时")`）；
//! 2. 等待者在他人刷新完成后能提前返回新 token，且不发起自己的
//!    token HTTP 请求（双检锁「等待者提前返回」语义）。
//!
//! 真实时钟（tokio 未暂停）：两个测试各自限时 < 4s。

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use wx_rust_miniapp::api::WxMaService;
use wx_rust_miniapp::api::r#impl::WxMaServiceImpl;
use wx_rust_miniapp::config::WxMaConfig;
use wx_rust_miniapp::config::WxMaHostConfig;
use wx_rust_miniapp::config::r#impl::WxMaDefaultConfig;

/// 极简计数 mock HTTP 服务器：对任意请求应答合法 token JSON。
///
/// 两个测试都断言零请求——若实现错误地发起了 token HTTP 请求，
/// 计数器会大于 0 使断言失败。
struct CountingServer {
    addr: std::net::SocketAddr,
    requests: Arc<AtomicUsize>,
    stop: Arc<std::sync::atomic::AtomicBool>,
}

impl CountingServer {
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
                    let body = r#"{"access_token":"UNEXPECTED_HTTP_TOKEN","expires_in":7200}"#;
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

/// 构建指向 mock 服务器的小程序配置（初始无 token → 已过期 → 慢路径）。
fn config_with_host(host: &str) -> Arc<dyn WxMaConfig> {
    let config = WxMaDefaultConfig::new("wxappid", "secret");
    let mut host_config = WxMaHostConfig::new();
    host_config.api_host = host.to_string();
    config.set_host_config(host_config);
    Arc::new(config)
}

/// 锁被他人持有超过 3s 时，`get_access_token` 应在 ~3s 返回错误，
/// 错误文案逐字为「获取accessToken超时：获取时间超时」（语义锁定：
/// 旧 try_lock 轮询实现同样 3s 超时，本测试证明该语义在改造后不变）。
#[tokio::test]
async fn token_lock_times_out_with_same_message() {
    let server = CountingServer::start().await;
    let config = config_with_host(&server.url());
    let service = WxMaServiceImpl::new_arc(config.clone());
    assert!(
        config.is_access_token_expired(),
        "初始无 token → 已过期 → 慢路径"
    );

    // 他方持有 token 锁 3.2s（覆盖完整 3s 超时窗口）；lock_owned 产生
    // 'static OwnedMutexGuard，可移入 spawned 任务。
    let lock = config.access_token_lock();
    let guard = lock.lock_owned().await;
    let releaser = tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(3200)).await;
        drop(guard);
    });

    let started = std::time::Instant::now();
    let result = service.get_access_token().await;
    let elapsed = started.elapsed();

    let err = result.expect_err("锁等待 3s 超时应返回错误");
    let wx_error = err.wx_error().expect("应为业务错误变体");
    assert_eq!(wx_error.error_code, -99);
    assert_eq!(
        wx_error.error_msg.as_deref(),
        Some("获取accessToken超时：获取时间超时"),
        "超时错误文案逐字不变"
    );
    assert!(
        elapsed < Duration::from_secs(4),
        "应在 ~3s 超时返回，实际耗时 {elapsed:?}"
    );
    assert!(
        elapsed >= Duration::from_millis(2900),
        "应等满 3s 超时窗口才报错，实际耗时 {elapsed:?}"
    );
    assert_eq!(server.request_count(), 0, "超时路径不应发起 token 请求");

    releaser.await.unwrap();
}

/// 等待者提前返回语义：他人持锁刷新 token 后释放，等待者应拿到新 token
/// 且不发起自己的 token HTTP 请求（双检锁核心语义，改造前后均应绿）。
#[tokio::test]
async fn waiter_returns_early_after_other_refresh() {
    let server = CountingServer::start().await;
    let config = config_with_host(&server.url());
    let service = WxMaServiceImpl::new_arc(config.clone());
    assert!(config.is_access_token_expired());

    // 他人先持锁（lock_owned 产生 'static guard 可移入任务），200ms 后
    // 更新 token 缓存再释放（刷新完成顺序：先写缓存后放锁）
    let lock = config.access_token_lock();
    let guard = lock.lock_owned().await;
    let refresher_config = config.clone();
    let refresher = tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(200)).await;
        refresher_config.update_access_token("REFRESHED_BY_OTHER", 7200);
        drop(guard);
    });

    let token = service
        .get_access_token()
        .await
        .expect("等待者应在他人刷新后提前返回");
    assert_eq!(token, "REFRESHED_BY_OTHER");
    assert_eq!(
        server.request_count(),
        0,
        "等待者不应发起自己的 token HTTP 请求"
    );

    refresher.await.unwrap();
}
