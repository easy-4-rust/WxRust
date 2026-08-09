//! 视频号小店门面装配测试（Wave 3 H3：门面装配 + 注册收尾）。
//!
//! 覆盖：
//! - 25 个子服务 getter 装配（每个返回 `Some` 且恒为同一实例，对应 Java
//!   `BaseWxChannelServiceImpl` 构造器的 12 个 eager 子服务字段 + 13 个
//!   `synchronized` 懒加载 getter）；
//! - 子服务经 `Weak<dyn WxChannelService>` 回调门面执行引擎（请求路径、
//!   body、access_token 注入，对应 Java `new WxChannelFundServiceImpl(this)`）；
//! - access_token 双检锁：缓存命中零网络请求、强制刷新走网络并更新配置；
//! - get/post 基础执行引擎（token 拼接、query 拼接、errcode 上抛）。

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use wx_rust_channel::api::r#impl::WxChannelServiceImpl;
// 注：子服务方法经 `dyn` trait 对象调用（vtable 解析），无需导入子域 trait
use wx_rust_channel::api::WxChannelService;
use wx_rust_channel::config::WxChannelConfig;
use wx_rust_channel::config::r#impl::WxChannelDefaultConfig;
use wx_rust_common::config::WxConfigStorage;

/// 极简 mock HTTP 服务器：按请求路径返回固定响应，记录请求次数与最近一次
/// 请求体、请求路径（含 query）。
struct MockServer {
    addr: std::net::SocketAddr,
    requests: Arc<AtomicUsize>,
    last_body: Arc<std::sync::Mutex<String>>,
    last_path: Arc<std::sync::Mutex<String>>,
    stop: Arc<AtomicBool>,
}

impl MockServer {
    /// 启动服务器（`handler(path) -> body`）。
    async fn start<F>(handler: F) -> Self
    where
        F: Fn(&str) -> String + Send + Sync + 'static,
    {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("绑定端口");
        let addr = listener.local_addr().expect("获取地址");
        let requests = Arc::new(AtomicUsize::new(0));
        let last_body = Arc::new(std::sync::Mutex::new(String::new()));
        let last_path = Arc::new(std::sync::Mutex::new(String::new()));
        let stop = Arc::new(AtomicBool::new(false));
        let handler = Arc::new(handler);

        let requests_clone = requests.clone();
        let last_body_clone = last_body.clone();
        let last_path_clone = last_path.clone();
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
                let last_body_clone = last_body_clone.clone();
                let last_path_clone = last_path_clone.clone();
                tokio::spawn(async move {
                    use tokio::io::{AsyncReadExt, AsyncWriteExt};
                    let mut buf = [0u8; 16384];
                    let n = socket.read(&mut buf).await.unwrap_or(0);
                    let request = String::from_utf8_lossy(&buf[..n]).to_string();
                    if let Some(idx) = request.find("\r\n\r\n") {
                        let body = request[idx + 4..].to_string();
                        *last_body_clone.lock().unwrap() = body;
                    }
                    let path = request
                        .lines()
                        .next()
                        .map(|l| l.split_whitespace().nth(1).unwrap_or("/").to_string())
                        .unwrap_or_else(|| "/".to_string());
                    *last_path_clone.lock().unwrap() = path.clone();
                    let body = handler(&path);
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
            last_body,
            last_path,
            stop,
        }
    }

    fn url(&self, path: &str) -> String {
        format!("http://{}{}", self.addr, path)
    }

    fn request_count(&self) -> usize {
        self.requests.load(Ordering::SeqCst)
    }

    fn last_body(&self) -> String {
        self.last_body.lock().unwrap().clone()
    }

    fn last_path(&self) -> String {
        self.last_path.lock().unwrap().clone()
    }
}

impl Drop for MockServer {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
    }
}

/// 构建指向 mock 服务器的门面服务配置：预置 access_token（免 token 请求）+
/// `api_host_url` 指向 mock 服务器（对应 Java `setApiHostUrl`，执行引擎替换
/// `https://api.weixin.qq.com` 前缀）。
fn config_with_host(host: &str) -> Arc<dyn WxChannelConfig> {
    let mut config = WxChannelDefaultConfig::new("wxappid", "secret");
    config.set_token("token123");
    config.update_access_token("MOCK_TOKEN", 7200);
    config.set_api_host_url(host);
    Arc::new(config)
}

/// 构建门面服务。
fn new_service(config: Arc<dyn WxChannelConfig>) -> Arc<WxChannelServiceImpl> {
    WxChannelServiceImpl::new_arc(config)
}

// ---- 1. 门面装配：25 个 getter 全部返回装配实例（对应 Java
// `BaseWxChannelServiceImpl` 的 25 个 `getXxxService()`）----

#[tokio::test]
async fn all_getters_assembled_and_singleton() {
    let server = MockServer::start(|_| r#"{"errcode":0,"errmsg":"ok"}"#.to_string()).await;
    let service = new_service(config_with_host(&server.url("")));

    // 25 个 getter 全量断言（顺序与 Java `WxChannelService` 接口声明一致）
    assert!(service.basic_service().is_some());
    assert!(service.category_service().is_some());
    assert!(service.brand_service().is_some());
    assert!(service.product_service().is_some());
    assert!(service.warehouse_service().is_some());
    assert!(service.order_service().is_some());
    assert!(service.after_sale_service().is_some());
    assert!(service.freight_template_service().is_some());
    assert!(service.address_service().is_some());
    assert!(service.coupon_service().is_some());
    assert!(service.sharer_service().is_some());
    assert!(service.fund_service().is_some());
    assert!(service.home_page_service().is_some());
    assert!(service.cooperation_service().is_some());
    assert!(service.compass_shop_service().is_some());
    assert!(service.league_window_service().is_some());
    assert!(service.league_supplier_service().is_some());
    assert!(service.league_promoter_service().is_some());
    assert!(service.league_product_service().is_some());
    assert!(service.lead_component_service().is_some());
    assert!(service.finder_live_service().is_some());
    assert!(service.assistant_service().is_some());
    assert!(service.vip_service().is_some());
    assert!(service.compass_finder_service().is_some());
    assert!(service.live_dashboard_service().is_some());

    // 单例语义：重复调用返回同一实例（Java eager 字段与 synchronized
    // 懒加载 getter 均为同一实例；Rust OnceLock 装配后恒同）
    let basic1 = service.basic_service().unwrap();
    let basic2 = service.basic_service().unwrap();
    assert!(Arc::ptr_eq(&basic1, &basic2));
    let live_dash1 = service.live_dashboard_service().unwrap();
    let live_dash2 = service.live_dashboard_service().unwrap();
    assert!(Arc::ptr_eq(&live_dash1, &live_dash2));
}

// ---- 2. 子服务经弱引用回调门面执行引擎（对应 Java
// `new WxChannelFundServiceImpl(this)` 的循环引用）----

#[tokio::test]
async fn sub_service_calls_back_through_facade() {
    let server = MockServer::start(|path| {
        if path.contains("/channels/ec/funds/getbalance") {
            r#"{"errcode":0,"errmsg":"ok","available_amount":100,"pending_amount":50}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    })
    .await;
    let service = new_service(config_with_host(&server.url("")));

    // 从门面 getter 取资金子服务，经其内部 Weak<dyn WxChannelService>
    // 回调门面的 post 执行引擎
    let fund = service.fund_service().expect("资金服务已装配");
    let resp = fund.get_balance().await.expect("获取余额成功");
    assert_eq!(resp.err_code, 0);
    assert_eq!(resp.available_amount, 100);
    assert_eq!(resp.pending_amount, 50);

    // 请求路径：执行引擎替换 api 域名 + 注入 access_token（Java
    // `executeInternal` 的 uriWithAccessToken 语义）
    let path = server.last_path();
    assert!(
        path.starts_with("/channels/ec/funds/getbalance?"),
        "实际路径：{path}"
    );
    assert!(path.contains("access_token=MOCK_TOKEN"), "实际路径：{path}");
    // Java `getBalance` 请求体为空对象 `{}`
    assert_eq!(server.last_body(), "{}");
}

// ---- 3. access_token 双检锁：缓存命中零网络请求 ----

#[tokio::test]
async fn access_token_cached_without_network() {
    let server = MockServer::start(|_| r#"{"errcode":0}"#.to_string()).await;
    let config = WxChannelDefaultConfig::new("wxappid", "secret");
    config.update_access_token("MOCK_TOKEN", 7200);
    // access_token_url 指向 mock 服务器：若发生网络请求可被计数捕获
    config.set_access_token_url(&server.url("/cgi-bin/token?appid=%s&secret=%s"));
    let service = new_service(Arc::new(config));

    // Java getAccessToken：未过期直接返回缓存，不发网络请求
    let token = service.get_access_token().await.expect("获取 token 成功");
    assert_eq!(token, "MOCK_TOKEN");
    assert_eq!(server.request_count(), 0, "缓存命中不应产生网络请求");
}

// ---- 4. access_token 双检锁：强制刷新走网络并更新配置缓存 ----

#[tokio::test]
async fn access_token_force_refresh_updates_config() {
    let server = MockServer::start(|path| {
        if path.contains("/cgi-bin/token") {
            r#"{"access_token":"NEW_TOKEN","expires_in":7200}"#.to_string()
        } else {
            r#"{"errcode":0}"#.to_string()
        }
    })
    .await;
    let config = WxChannelDefaultConfig::new("wxappid", "secret");
    config.update_access_token("OLD_TOKEN", 7200);
    config.set_access_token_url(&server.url("/cgi-bin/token?appid=%s&secret=%s"));
    let service = new_service(Arc::new(config));

    // Java getAccessToken(true)：强制刷新 → 走网络 → extractAccessToken 更新配置
    let token = service
        .get_access_token_with_force(true)
        .await
        .expect("强制刷新成功");
    assert_eq!(token, "NEW_TOKEN");
    assert_eq!(server.request_count(), 1);
    assert!(server.last_path().contains("appid=wxappid&secret=secret"));
    assert_eq!(
        service.wx_channel_config().access_token().as_deref(),
        Some("NEW_TOKEN"),
        "刷新后配置缓存已更新"
    );

    // 随后非强制获取命中缓存（Java 双检锁第二次检查），不再发网络请求
    let token = service.get_access_token().await.expect("缓存获取成功");
    assert_eq!(token, "NEW_TOKEN");
    assert_eq!(server.request_count(), 1, "缓存命中不再请求");
}

// ---- 5. get 基础执行引擎（对应 Java `get(String, String)`）----

#[tokio::test]
async fn get_appends_token_and_query() {
    let server = MockServer::start(|path| {
        if path.contains("/channels/ec/funds/foo") {
            r#"{"errcode":0,"errmsg":"ok","data":"ok"}"#.to_string()
        } else {
            r#"{"errcode":40003,"errmsg":"invalid"}"#.to_string()
        }
    })
    .await;
    let service = new_service(config_with_host(&server.url("")));

    let body = service
        .get("https://api.weixin.qq.com/channels/ec/funds/foo", "a=1&b=2")
        .await
        .expect("GET 成功");
    assert!(body.contains("\"data\":\"ok\""));

    // Java：uriWithAccessToken 先拼 token，SimpleGetRequestExecutor 再拼 query
    let path = server.last_path();
    assert!(
        path.starts_with("/channels/ec/funds/foo?access_token=MOCK_TOKEN&a=1&b=2"),
        "实际路径：{path}"
    );

    // errcode != 0 时执行器上抛（对应 Java SimpleGetRequestExecutor）
    let err = service
        .get("https://api.weixin.qq.com/channels/ec/funds/err", "a=1")
        .await
        .expect_err("errcode 非 0 应上抛");
    assert_eq!(err.error_code(), Some(40003));
}

// ---- 6. post 基础执行引擎（对应 Java `post(String, String)`）----

#[tokio::test]
async fn post_sends_body_and_throws_on_errcode() {
    let server = MockServer::start(|path| {
        if path.contains("/channels/ec/funds/bar") {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        } else {
            r#"{"errcode":40003,"errmsg":"invalid"}"#.to_string()
        }
    })
    .await;
    let service = new_service(config_with_host(&server.url("")));

    let body = service
        .post(
            "https://api.weixin.qq.com/channels/ec/funds/bar",
            r#"{"amount":100}"#,
        )
        .await
        .expect("POST 成功");
    assert!(body.contains("\"errmsg\":\"ok\""));

    let path = server.last_path();
    assert!(
        path.starts_with("/channels/ec/funds/bar?access_token=MOCK_TOKEN"),
        "实际路径：{path}"
    );
    assert_eq!(server.last_body(), r#"{"amount":100}"#);

    // errcode != 0 时执行器上抛（对应 Java SimplePostRequestExecutor）
    let err = service
        .post("https://api.weixin.qq.com/channels/ec/funds/bad", "{}")
        .await
        .expect_err("errcode 非 0 应上抛");
    assert_eq!(err.error_code(), Some(40003));
}
