//! 共享测试基础设施（镜像 Java `ApiTestModule` + `TestConfigStorage` 的
//! 职责：Guice 注入配置 → Rust 直接构造服务与 MockServer）。
//!
//! 该目录被各测试文件以 `mod common;` 引入，不作为独立测试二进制运行。

#![allow(dead_code)]

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use wx_rust_qidian::api::WxQidianService;
use wx_rust_qidian::api::r#impl::WxQidianServiceImpl;
use wx_rust_qidian::config::WxQidianConfigStorage;
use wx_rust_qidian::config::r#impl::WxQidianDefaultConfig;

/// 极简 mock HTTP 服务器：按请求路径返回 body，记录最近一次请求的
/// 方法/路径（含 query）/请求体与请求计数（照抄 miniapp tests/ 模式）。
pub struct MockServer {
    addr: std::net::SocketAddr,
    requests: Arc<AtomicUsize>,
    last_method: Arc<std::sync::Mutex<String>>,
    last_path: Arc<std::sync::Mutex<String>>,
    last_body: Arc<std::sync::Mutex<String>>,
    token_hits: Arc<AtomicUsize>,
    path_hits: Arc<std::sync::Mutex<std::collections::HashMap<String, usize>>>,
    stop: Arc<std::sync::atomic::AtomicBool>,
}

impl MockServer {
    /// 启动服务器（`handler(method, path) -> (content_type, body)`）。
    pub async fn start<F>(handler: F) -> Self
    where
        F: Fn(&str, &str) -> (String, String) + Send + Sync + 'static,
    {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("绑定端口");
        let addr = listener.local_addr().expect("获取地址");
        let requests = Arc::new(AtomicUsize::new(0));
        let last_method = Arc::new(std::sync::Mutex::new(String::new()));
        let last_path = Arc::new(std::sync::Mutex::new(String::new()));
        let last_body = Arc::new(std::sync::Mutex::new(String::new()));
        let token_hits = Arc::new(AtomicUsize::new(0));
        let path_hits = Arc::new(std::sync::Mutex::new(std::collections::HashMap::new()));
        let stop = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let handler = Arc::new(handler);

        let requests_clone = requests.clone();
        let last_method_clone = last_method.clone();
        let last_path_clone = last_path.clone();
        let last_body_clone = last_body.clone();
        let token_hits_clone = token_hits.clone();
        let path_hits_clone = path_hits.clone();
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
                let last_method_clone = last_method_clone.clone();
                let last_path_clone = last_path_clone.clone();
                let last_body_clone = last_body_clone.clone();
                let token_hits_clone = token_hits_clone.clone();
                let path_hits_clone = path_hits_clone.clone();
                tokio::spawn(async move {
                    use tokio::io::{AsyncReadExt, AsyncWriteExt};
                    let mut buf = [0u8; 65536];
                    let n = socket.read(&mut buf).await.unwrap_or(0);
                    let request = String::from_utf8_lossy(&buf[..n]).to_string();
                    // 记录请求方法、路径（含 query）与请求体
                    let mut lines = request.lines();
                    let mut path = String::new();
                    if let Some(request_line) = lines.next() {
                        let mut parts = request_line.split_whitespace();
                        if let Some(method) = parts.next() {
                            *last_method_clone.lock().unwrap() = method.to_string();
                        }
                        if let Some(p) = parts.next() {
                            path = p.to_string();
                            *last_path_clone.lock().unwrap() = p.to_string();
                        }
                    }
                    if let Some(idx) = request.find("\r\n\r\n") {
                        let body = request[idx + 4..].to_string();
                        *last_body_clone.lock().unwrap() = body;
                    }
                    // token 接口命中计数（并发刷新去重断言用）
                    if path.contains("/cgi-bin/token") {
                        token_hits_clone.fetch_add(1, Ordering::SeqCst);
                    }
                    {
                        let mut hits = path_hits_clone.lock().unwrap();
                        *hits.entry(path.clone()).or_insert(0) += 1;
                    }
                    let method = request
                        .lines()
                        .next()
                        .and_then(|l| l.split_whitespace().next())
                        .unwrap_or("GET")
                        .to_string();
                    let (content_type, body) = handler(&method, &path);
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
            last_method,
            last_path,
            last_body,
            token_hits,
            path_hits,
            stop,
        }
    }

    pub fn url(&self, path: &str) -> String {
        format!("http://{}{}", self.addr, path)
    }

    pub fn request_count(&self) -> usize {
        self.requests.load(Ordering::SeqCst)
    }

    pub fn token_hits(&self) -> usize {
        self.token_hits.load(Ordering::SeqCst)
    }

    pub fn last_method(&self) -> String {
        self.last_method.lock().unwrap().clone()
    }

    pub fn last_path(&self) -> String {
        self.last_path.lock().unwrap().clone()
    }

    pub fn last_body(&self) -> String {
        self.last_body.lock().unwrap().clone()
    }

    /// 统计包含指定子串的请求路径命中次数。
    pub fn path_hits(&self, path_contains: &str) -> usize {
        self.path_hits
            .lock()
            .unwrap()
            .iter()
            .filter(|(p, _)| p.contains(path_contains))
            .map(|(_, n)| n)
            .sum()
    }
}

impl Drop for MockServer {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
    }
}

/// JSON 响应快捷构造。
pub fn json(body: &str) -> (String, String) {
    ("application/json".to_string(), body.to_string())
}

/// 通用路由 handler：token 请求先应答，业务路径按 contains 分派。
pub fn dispatch(
    handler: impl Fn(&str) -> (String, String) + Send + Sync + 'static,
) -> impl Fn(&str, &str) -> (String, String) + Send + Sync + 'static {
    move |_method: &str, path: &str| {
        if path.contains("/cgi-bin/token") {
            return json(r#"{"access_token":"MOCK_TOKEN","expires_in":7200}"#);
        }
        handler(path)
    }
}

/// 构建指向 mock 服务器的企点服务（appid=wxqidian_default, secret=secret；
/// api 与 qidian 域名均指向 mock，对应 Java ApiTestModule 注入的配置）。
pub fn service_with_host(host: &str) -> Arc<WxQidianServiceImpl> {
    let mut config = WxQidianDefaultConfig::new("wxqidian_default", "secret");
    config.set_token("token123");
    let mut host_config = wx_rust_qidian::bean::WxQidianHostConfig::new();
    host_config.api_host = Some(host.to_string());
    host_config.qidian_host = Some(host.to_string());
    config.set_host_config(host_config);
    WxQidianServiceImpl::new_arc(Arc::new(config))
}

/// 构建多企点配置服务（默认 wxqidian_default + 附加 mp_id，对应 Java
/// `setMultiConfigStorages` 语义）。
pub fn service_with_multi(host: &str, extra_mp_id: &str) -> Arc<WxQidianServiceImpl> {
    let mut config = WxQidianDefaultConfig::new("wxqidian_default", "secret");
    config.set_token("token123");
    let mut host_config = wx_rust_qidian::bean::WxQidianHostConfig::new();
    host_config.api_host = Some(host.to_string());
    host_config.qidian_host = Some(host.to_string());
    config.set_host_config(host_config);

    let mut extra = WxQidianDefaultConfig::new(extra_mp_id, "secret2");
    extra.set_token("token456");
    let mut host_config2 = wx_rust_qidian::bean::WxQidianHostConfig::new();
    host_config2.api_host = Some(host.to_string());
    host_config2.qidian_host = Some(host.to_string());
    extra.set_host_config(host_config2);

    let service = WxQidianServiceImpl::new_arc(Arc::new(config));
    service.add_config_storage(extra_mp_id, Arc::new(extra));
    service
}
