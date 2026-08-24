#![cfg(feature = "sync")]
//! feature="sync" 同步门面（`WxMaServiceBlocking`）集成测试。
//!
//! 全部为同步 `#[test]`（**无 `#[tokio::test]`**）：MockServer 沿用
//! `sub_domain_g1_core.rs` 的 T3 模式（async `start`），起动时经同一全局
//! current_thread runtime（`blocking::block_on`）block_on 起；被测门面方法
//! 全程同步调用——证明门面在纯同步上下文可用（token 获取 + 业务请求的
//! 整条 async 管线在 block_on 内完成，且与 mock 服务器同 runtime 驱动）。

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use wx_rust_miniapp::api::r#impl::WxMaServiceImpl;
use wx_rust_miniapp::bean::WxMaJscode2SessionResult;
use wx_rust_miniapp::blocking::{WxMaServiceBlocking, block_on};
use wx_rust_miniapp::config::WxMaConfig;
use wx_rust_miniapp::config::r#impl::WxMaDefaultConfig;

/// 极简 mock HTTP 服务器：按请求路径返回 (Content-Type, body)，记录
/// 最近一次请求路径（含 query）与请求体、请求计数。
struct MockServer {
    addr: std::net::SocketAddr,
    requests: Arc<AtomicUsize>,
    last_path: Arc<std::sync::Mutex<String>>,
    last_body: Arc<std::sync::Mutex<String>>,
    stop: Arc<std::sync::atomic::AtomicBool>,
}

impl MockServer {
    /// 启动服务器（`handler(path) -> (content_type, body)`）。
    async fn start<F>(handler: F) -> Self
    where
        F: Fn(&str) -> (String, String) + Send + Sync + 'static,
    {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("绑定端口");
        let addr = listener.local_addr().expect("获取地址");
        let requests = Arc::new(AtomicUsize::new(0));
        let last_path = Arc::new(std::sync::Mutex::new(String::new()));
        let last_body = Arc::new(std::sync::Mutex::new(String::new()));
        let stop = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let handler = Arc::new(handler);

        let requests_clone = requests.clone();
        let last_path_clone = last_path.clone();
        let last_body_clone = last_body.clone();
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
                let handler = handler.clone();
                let last_path_clone = last_path_clone.clone();
                let last_body_clone = last_body_clone.clone();
                tokio::spawn(async move {
                    use tokio::io::{AsyncReadExt, AsyncWriteExt};
                    let mut buf = [0u8; 16384];
                    let n = socket.read(&mut buf).await.unwrap_or(0);
                    let request = String::from_utf8_lossy(&buf[..n]).to_string();
                    // 记录请求路径（含 query）与请求体（POST 场景）
                    if let Some(path) = request
                        .lines()
                        .next()
                        .and_then(|l| l.split_whitespace().nth(1))
                    {
                        *last_path_clone.lock().unwrap() = path.to_string();
                    }
                    if let Some(idx) = request.find("\r\n\r\n") {
                        let body = request[idx + 4..].to_string();
                        *last_body_clone.lock().unwrap() = body;
                    }
                    let path = request
                        .lines()
                        .next()
                        .map(|l| l.split_whitespace().nth(1).unwrap_or("/").to_string())
                        .unwrap_or_else(|| "/".to_string());
                    let (content_type, body) = handler(&path);
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
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
            last_path,
            last_body,
            stop,
        }
    }

    fn url(&self, path: &str) -> String {
        format!("http://{}{}", self.addr, path)
    }

    fn request_count(&self) -> usize {
        self.requests.load(Ordering::SeqCst)
    }

    fn last_path(&self) -> String {
        self.last_path.lock().unwrap().clone()
    }

    fn last_body(&self) -> String {
        self.last_body.lock().unwrap().clone()
    }
}

impl Drop for MockServer {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
    }
}

/// JSON 响应快捷构造。
fn json(body: &str) -> (String, String) {
    ("application/json".to_string(), body.to_string())
}

/// 构建指向 mock 服务器的小程序配置（appid=wxappid, secret=secret）。
fn config_with_host(host: &str) -> Arc<dyn WxMaConfig> {
    let mut config = WxMaDefaultConfig::new("wxappid", "secret");
    config.set_token("token123");
    let mut host_config = wx_rust_miniapp::config::WxMaHostConfig::new();
    host_config.api_host = host.to_string();
    config.set_host_config(host_config);
    Arc::new(config)
}

/// 通用路由 handler：token 请求先应答，业务路径按 contains 分派。
fn dispatch(
    handler: impl Fn(&str) -> (String, String) + Send + Sync + 'static,
) -> impl Fn(&str) -> (String, String) + Send + Sync + 'static {
    move |path: &str| {
        if path.contains("/cgi-bin/token") || path.contains("/cgi-bin/stable_token") {
            return json(r#"{"access_token":"MOCK_TOKEN","expires_in":7200}"#);
        }
        handler(path)
    }
}

#[test]
fn blocking_js_code_to_session() {
    // 纯同步上下文：GET /sns/jscode2session（token 自动获取注入），
    // 响应 {openid, session_key, unionid} 解析为 WxMaJscode2SessionResult。
    let server = block_on(MockServer::start(dispatch(|path| {
        if path.contains("/sns/jscode2session") {
            json(r#"{"openid":"o1","session_key":"sk_1","unionid":"u1"}"#)
        } else {
            json("{}")
        }
    })));
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let facade = WxMaServiceBlocking::new(service);

    let session: WxMaJscode2SessionResult =
        facade.js_code_to_session("aaa").expect("同步登录会话成功");
    assert_eq!(session.openid, "o1");
    assert_eq!(session.session_key, "sk_1");
    assert_eq!(session.unionid, "u1");
    // token 请求 + 业务请求共 2 次；路径含完整 query
    assert!(server.request_count() >= 2, "token 请求 + 业务请求");
    let path = server.last_path();
    assert!(path.contains("/sns/jscode2session"), "路径: {path}");
    assert!(path.contains("js_code=aaa"), "路径: {path}");
    assert!(
        path.contains("grant_type=authorization_code"),
        "路径: {path}"
    );
    assert!(path.contains("access_token=MOCK_TOKEN"), "路径: {path}");
}

#[test]
fn blocking_get_phone_number() {
    // 纯同步上下文：POST /wxa/business/getuserphonenumber，请求体
    // {"code": ...}，响应 phone_info 解析为 Option<WxMaPhoneNumberInfo>；
    // 无 phone_info 的响应返回 None（Java null 语义）。
    let server = block_on(MockServer::start(dispatch(|path| {
        if path.contains("/wxa/business/getuserphonenumber") {
            json(
                r#"{"phone_info":{"phoneNumber":"13800138000","purePhoneNumber":"13800138000","countryCode":"86","watermark":{"appid":"wxappid","timestamp":"1700000000"}}}"#,
            )
        } else {
            json("{}")
        }
    })));
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let facade = WxMaServiceBlocking::new(service);

    let phone = facade
        .get_phone_number("code123")
        .expect("同步获取手机号成功")
        .expect("phone_info 存在");
    assert_eq!(phone.phone_number, "13800138000");
    assert_eq!(phone.pure_phone_number, "13800138000");
    assert_eq!(phone.country_code, "86");
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["code"], "code123");
    assert!(
        server
            .last_path()
            .contains("/wxa/business/getuserphonenumber")
    );
}

#[test]
fn blocking_get_access_token_sync_with_cache() {
    // 纯同步上下文：token 端点返回 MOCK_TOKEN；第二次调用命中配置缓存
    // （双检锁路径：不强制刷新且未过期直接返回），不再发起 HTTP。
    let server = block_on(MockServer::start(dispatch(|_path| json("{}"))));
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let facade = WxMaServiceBlocking::new(service);

    let token = facade.get_access_token_sync().expect("同步 token 成功");
    assert_eq!(token, "MOCK_TOKEN");
    let count_after_first = server.request_count();
    assert!(count_after_first >= 1, "至少一次 token 请求");

    let token_cached = facade.get_access_token_sync().expect("同步缓存 token 成功");
    assert_eq!(token_cached, "MOCK_TOKEN");
    assert_eq!(
        server.request_count(),
        count_after_first,
        "缓存命中不发起 HTTP"
    );
    assert!(server.last_path().contains("/cgi-bin/token"));
}
