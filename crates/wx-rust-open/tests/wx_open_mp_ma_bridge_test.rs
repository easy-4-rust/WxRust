//! 代 mp/ma 服务桥接集成测试（自含 MockServer 模式，与
//! wx_open_component_service_test.rs 同一 MockServer 实现）。
//!
//! 覆盖（Wave 4）：
//! - `getWxMpServiceByAppid`/`getWxMaServiceByAppid`/
//!   `getWxFastMaServiceByAppid` 返回 Some 且可下转（mp → `dyn WxMpService`，
//!   ma/fast_ma → `dyn WxMaService`）；
//! - authorizer access_token 从 open 配置存储注入桥接服务
//!   （`getAccessToken` 委托 `getAuthorizerAccessToken`，镜像 Java
//!   `WxOpenMpServiceImpl.getAccessToken`）；
//! - 代 mp/ma 的 get/post 走各自执行引擎并注入 authorizer access_token；
//! - open_account 4 方法（create/bind/unbind/get）经
//!   `openAccountServicePost` 语义：请求路径 /cgi-bin/open/create|bind|
//!   unbind|get + 请求体断言；未知 appIdType 抛「appIdType类型异常」；
//! - 双检锁缓存：同 appid 返回同一实例，不同 appid 不同实例，mp/ma
//!   缓存桶相互独立；
//! - 代 ma 的 `jsCode2SessionInfo` 委托组件服务
//!   `miniappJscode2Session`（/sns/component/jscode2session + component
//!   access_token，镜像 Java `WxOpenMaServiceImpl.jsCode2SessionInfo`）。

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use wx_rust_miniapp::api::WxMaService;
use wx_rust_mp::api::WxMpService;
use wx_rust_open::api::r#impl::{WxOpenServiceImpl, downcast_ma_service, downcast_mp_service};
use wx_rust_open::api::{WxOpenComponentService, WxOpenService};
use wx_rust_open::config::WxOpenConfigStorage;
use wx_rust_open::config::r#impl::WxOpenDefaultConfig;

// ---------------------------------------------------------------------------
// 自含 MockServer（与 wx_open_component_service_test.rs 同一实现）
// ---------------------------------------------------------------------------

/// 路由：method + path 前缀匹配，按序消费响应列表（末条重复）。
#[derive(Clone)]
struct MockRoute {
    method: &'static str,
    path_prefix: String,
    responses: Vec<String>,
}

impl MockRoute {
    fn post(path_prefix: &str, responses: &[&str]) -> Self {
        Self {
            method: "POST",
            path_prefix: path_prefix.to_string(),
            responses: responses.iter().map(|s| s.to_string()).collect(),
        }
    }

    fn get(path_prefix: &str, responses: &[&str]) -> Self {
        Self {
            method: "GET",
            path_prefix: path_prefix.to_string(),
            responses: responses.iter().map(|s| s.to_string()).collect(),
        }
    }
}

/// 已收到的请求记录。
#[derive(Debug, Clone)]
struct RecordedRequest {
    method: String,
    path: String,
    body: String,
}

struct MockState {
    routes: Vec<MockRoute>,
    calls: Vec<usize>,
    requests: Vec<RecordedRequest>,
}

/// 自含 HTTP/1.1 MockServer。
struct MockServer {
    addr: SocketAddr,
    state: Arc<Mutex<MockState>>,
}

impl MockServer {
    async fn start(routes: Vec<MockRoute>) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let state = Arc::new(Mutex::new(MockState {
            calls: vec![0; routes.len()],
            requests: Vec::new(),
            routes,
        }));
        let serve_state = state.clone();
        tokio::spawn(async move {
            loop {
                let (socket, _) = match listener.accept().await {
                    Ok(v) => v,
                    Err(_) => break,
                };
                let state = serve_state.clone();
                tokio::spawn(async move {
                    Self::handle(socket, state).await;
                });
            }
        });
        MockServer { addr, state }
    }

    /// 处理单个连接：读请求头直到 `\r\n\r\n`，按 Content-Length 读请求体，
    /// 记录后按路由返回响应（`Connection: close`）。
    async fn handle(mut socket: TcpStream, state: Arc<Mutex<MockState>>) {
        let mut buffer = Vec::new();
        let mut chunk = [0u8; 8192];
        let mut header_end = None;
        while header_end.is_none() {
            let n = match socket.read(&mut chunk).await {
                Ok(0) | Err(_) => return,
                Ok(n) => n,
            };
            buffer.extend_from_slice(&chunk[..n]);
            header_end = buffer
                .windows(4)
                .position(|w| w == b"\r\n\r\n")
                .map(|p| p + 4);
        }
        let header_end = header_end.unwrap();
        let head = buffer[..header_end].to_vec();
        let mut body = buffer[header_end..].to_vec();

        let head_str = String::from_utf8_lossy(&head);
        let mut lines = head_str.split("\r\n");
        let mut parts = lines.next().unwrap_or_default().split_whitespace();
        let method = parts.next().unwrap_or("").to_string();
        let path = parts.next().unwrap_or("").to_string();
        let mut content_length = 0usize;
        for line in lines {
            if let Some(v) = line.to_ascii_lowercase().strip_prefix("content-length:") {
                content_length = v.trim().parse().unwrap_or(0);
            }
        }
        while body.len() < content_length {
            let n = match socket.read(&mut chunk).await {
                Ok(0) | Err(_) => break,
                Ok(n) => n,
            };
            body.extend_from_slice(&chunk[..n]);
        }
        body.truncate(content_length);
        let body_str = String::from_utf8_lossy(&body).to_string();

        let response = {
            let mut st = state.lock().unwrap();
            st.requests.push(RecordedRequest {
                method: method.clone(),
                path: path.clone(),
                body: body_str,
            });
            let mut match_idx: Option<usize> = None;
            for (i, route) in st.routes.iter().enumerate() {
                if route.method == method && path.starts_with(&route.path_prefix) {
                    match_idx = Some(i);
                    break;
                }
            }
            let mut response =
                "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
                    .to_string();
            if let Some(i) = match_idx {
                let call = st.calls[i];
                st.calls[i] += 1;
                let idx = call.min(st.routes[i].responses.len() - 1);
                let resp_body = st.routes[i].responses[idx].clone();
                response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    resp_body.len(),
                    resp_body
                );
            }
            response
        };
        let _ = socket.write_all(response.as_bytes()).await;
    }

    /// API 前缀（`http://127.0.0.1:PORT`），喂给 `set_api_host_url`。
    fn base_url(&self) -> String {
        format!("http://{}", self.addr)
    }

    /// 已收到的请求（按到达顺序）。
    fn requests(&self) -> Vec<RecordedRequest> {
        self.state.lock().unwrap().requests.clone()
    }
}

// ---------------------------------------------------------------------------
// 测试夹具
// ---------------------------------------------------------------------------

/// 默认组件 token 响应（api_component_token）。
const COMPONENT_TOKEN_RESP: &str = r#"{"component_access_token":"comp_tok_v1","expires_in":7200}"#;

/// 构建默认配置（独立模块：不引入 `WxOpenConfigStorage` trait，避免
/// trait 的 `&self` setter 遮蔽固有 `&mut self` 链式 builder）。
mod build {
    use wx_rust_open::config::r#impl::WxOpenDefaultConfig;

    pub fn default_config(base_url: &str) -> WxOpenDefaultConfig {
        let mut config = WxOpenDefaultConfig::new();
        config.set_component_app_id("component_appid_01");
        config.set_component_app_secret("component_secret_01");
        config.set_component_token("component_token_01");
        config.set_component_aes_key("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=");
        config.set_component_verify_ticket("ticket@@@init");
        config.set_api_host_url(base_url.to_string());
        config
    }
}

/// 测试夹具：MockServer + 默认配置 + 门面服务。
struct Harness {
    server: MockServer,
    config: Arc<WxOpenDefaultConfig>,
    service: Arc<WxOpenServiceImpl>,
}

impl Harness {
    /// 构建夹具；`extra_routes` 追加到标准路由（api_component_token）之后。
    async fn new(extra_routes: Vec<MockRoute>) -> Self {
        let has_component_token = extra_routes
            .iter()
            .any(|r| r.path_prefix == "/cgi-bin/component/api_component_token");
        let mut routes = Vec::new();
        if !has_component_token {
            routes.push(MockRoute::post(
                "/cgi-bin/component/api_component_token",
                &[COMPONENT_TOKEN_RESP],
            ));
        }
        routes.extend(extra_routes);
        let server = MockServer::start(routes).await;
        let config = Arc::new(build::default_config(&server.base_url()));
        let service = WxOpenServiceImpl::new_arc(config.clone());
        Self {
            server,
            config,
            service,
        }
    }

    /// 组件子服务。
    fn component(&self) -> Arc<dyn WxOpenComponentService> {
        self.service.wx_open_component_service().unwrap()
    }

    /// 注入 authorizer access_token 到 open 配置存储（免刷新链）。
    fn set_authorizer_token(&self, app_id: &str, token: &str) {
        self.config
            .update_authorizer_access_token_with_expiry(app_id, token, 7200);
    }
}

/// 解析 JSON 请求体。
fn req_body_json(req: &RecordedRequest) -> serde_json::Value {
    serde_json::from_str(&req.body).unwrap_or(serde_json::Value::Null)
}

/// 授权方 appid（测试常量）。
const APP_ID_MP: &str = "authorizer_appid_mp_01";
const APP_ID_MA: &str = "authorizer_appid_ma_01";

// ---------------------------------------------------------------------------
// 测试
// ---------------------------------------------------------------------------

/// getWxMpServiceByAppid 返回 Some 且可下转为 Arc<dyn WxMpService>；
/// 桥接配置 appid 映射正确（镜像 Java 持有 authorizer appid 配置）。
#[tokio::test]
async fn get_mp_service_returns_some_and_downcasts() {
    let h = Harness::new(vec![]).await;
    let any = h
        .component()
        .get_wx_mp_service_by_appid(APP_ID_MP)
        .expect("get_wx_mp_service_by_appid 返回 None");
    let mp = downcast_mp_service(any).expect("downcast 到 Arc<dyn WxMpService> 失败");
    assert_eq!(mp.wx_mp_config_storage().app_id(), APP_ID_MP);
    // 同 appid 双检锁缓存：第二次调用返回同一实例
    let any2 = h.component().get_wx_mp_service_by_appid(APP_ID_MP).unwrap();
    let mp2 = downcast_mp_service(any2).unwrap();
    assert!(Arc::ptr_eq(&mp, &mp2));
}

/// getWxMaServiceByAppid / getWxFastMaServiceByAppid 返回 Some 且均可
/// 下转为 Arc<dyn WxMaService>（Java fast_ma 语义等价，Rust 统一承载）。
#[tokio::test]
async fn get_ma_and_fast_ma_services_return_some_and_downcast() {
    let h = Harness::new(vec![]).await;
    let any = h
        .component()
        .get_wx_ma_service_by_appid(APP_ID_MA)
        .expect("get_wx_ma_service_by_appid 返回 None");
    let ma = downcast_ma_service(any).expect("downcast 到 Arc<dyn WxMaService> 失败");
    assert_eq!(ma.wx_ma_config().app_id(), APP_ID_MA);

    let any_fast = h
        .component()
        .get_wx_fast_ma_service_by_appid(APP_ID_MA)
        .expect("get_wx_fast_ma_service_by_appid 返回 None");
    let fast_ma = downcast_ma_service(any_fast).expect("fast_ma downcast 失败");
    assert_eq!(fast_ma.wx_ma_config().app_id(), APP_ID_MA);
    // 独立缓存桶（镜像 Java 独立 map）：fast_ma 与 ma 不是同一实例
    assert!(!Arc::ptr_eq(&ma, &fast_ma));
}

/// authorizer access_token 从 open 配置存储注入桥接服务：桥接
/// getAccessToken 委托 getAuthorizerAccessToken（Java
/// `WxOpenMpServiceImpl.getAccessToken`），缓存未过期时不发网络请求。
#[tokio::test]
async fn bridge_access_token_reads_open_authorizer_cache() {
    let h = Harness::new(vec![]).await;
    h.set_authorizer_token(APP_ID_MP, "auth_tok_1");
    h.set_authorizer_token(APP_ID_MA, "auth_tok_2");

    let mp =
        downcast_mp_service(h.component().get_wx_mp_service_by_appid(APP_ID_MP).unwrap()).unwrap();
    assert_eq!(mp.get_access_token().await.unwrap(), "auth_tok_1");
    assert_eq!(
        mp.get_access_token_with_force(false).await.unwrap(),
        "auth_tok_1"
    );

    let ma =
        downcast_ma_service(h.component().get_wx_ma_service_by_appid(APP_ID_MA).unwrap()).unwrap();
    assert_eq!(ma.get_access_token().await.unwrap(), "auth_tok_2");

    // 全程无网络请求（token 缓存命中，组件刷新链未触发）
    assert!(h.server.requests().is_empty());
}

/// 代 mp 服务的 post 走 mp 执行引擎并注入 authorizer access_token
/// （Java `wxMpService.post` 语义；open_account 系列依赖此注入）。
#[tokio::test]
async fn bridge_mp_post_injects_authorizer_token() {
    let h = Harness::new(vec![MockRoute::post(
        "/test/echo",
        &[r#"{"errcode":0,"errmsg":"ok"}"#],
    )])
    .await;
    h.set_authorizer_token(APP_ID_MP, "auth_tok_mp");
    let mp =
        downcast_mp_service(h.component().get_wx_mp_service_by_appid(APP_ID_MP).unwrap()).unwrap();
    let url = format!("{}/test/echo", h.server.base_url());
    let resp = mp.post(&url, r#"{"a":1}"#).await.unwrap();
    assert!(resp.contains("errcode"));

    let req = h.server.requests().pop().unwrap();
    assert_eq!(req.method, "POST");
    assert!(req.path.starts_with("/test/echo"));
    assert!(req.path.contains("access_token=auth_tok_mp"));
    assert_eq!(req_body_json(&req)["a"], 1);
}

/// 代 ma 服务的 post 走 ma 执行引擎并注入 authorizer access_token。
#[tokio::test]
async fn bridge_ma_post_injects_authorizer_token() {
    let h = Harness::new(vec![MockRoute::post(
        "/wxa/ma_echo",
        &[r#"{"errcode":0,"errmsg":"ok"}"#],
    )])
    .await;
    h.set_authorizer_token(APP_ID_MA, "auth_tok_ma");
    let ma =
        downcast_ma_service(h.component().get_wx_ma_service_by_appid(APP_ID_MA).unwrap()).unwrap();
    let url = format!("{}/wxa/ma_echo", h.server.base_url());
    let _ = ma.post(&url, r#"{"b":2}"#).await.unwrap();

    let req = h.server.requests().pop().unwrap();
    assert_eq!(req.method, "POST");
    assert!(req.path.starts_with("/wxa/ma_echo"));
    assert!(req.path.contains("access_token=auth_tok_ma"));
    assert_eq!(req_body_json(&req)["b"], 2);
}

/// 创建开放平台帐号（createOpenAccount，appIdType=mp）：经代 mp 服务
/// post `/cgi-bin/open/create`，请求体 {"appid"} + authorizer
/// access_token；结果解析 open_appid（Java `WxOpenCreateResult.fromJson`）。
#[tokio::test]
async fn create_open_account_mp_request_and_result() {
    let h = Harness::new(vec![MockRoute::post(
        "/cgi-bin/open/create",
        &[r#"{"open_appid":"openappid_01","errcode":0,"errmsg":""}"#],
    )])
    .await;
    h.set_authorizer_token(APP_ID_MP, "auth_tok_mp");
    let result = h
        .component()
        .create_open_account(APP_ID_MP, "mp")
        .await
        .unwrap();
    assert_eq!(result.open_appid, "openappid_01");
    assert_eq!(result.errcode, "0");

    let req = h.server.requests().pop().unwrap();
    assert_eq!(req.method, "POST");
    assert!(req.path.starts_with("/cgi-bin/open/create"));
    assert!(req.path.contains("access_token=auth_tok_mp"));
    let body = req_body_json(&req);
    assert_eq!(body["appid"], APP_ID_MP);
}

/// 绑定 open 帐号（bindOpenAccount，appIdType=mini）：经代 ma 服务
/// post `/cgi-bin/open/bind`，请求体 {"appid","open_appid"}；errcode=0
/// 时返回 true（Java `WxOpenResult.fromJson(json).isSuccess()`）。
#[tokio::test]
async fn bind_open_account_mini_returns_true() {
    let h = Harness::new(vec![MockRoute::post(
        "/cgi-bin/open/bind",
        &[r#"{"errcode":0,"errmsg":"ok"}"#],
    )])
    .await;
    h.set_authorizer_token(APP_ID_MA, "auth_tok_ma");
    let ok = h
        .component()
        .bind_open_account(APP_ID_MA, "mini", "openappid_02")
        .await
        .unwrap();
    assert!(ok);

    let req = h.server.requests().pop().unwrap();
    assert_eq!(req.method, "POST");
    assert!(req.path.starts_with("/cgi-bin/open/bind"));
    assert!(req.path.contains("access_token=auth_tok_ma"));
    let body = req_body_json(&req);
    assert_eq!(body["appid"], APP_ID_MA);
    assert_eq!(body["open_appid"], "openappid_02");
}

/// 解绑 open 帐号（unbindOpenAccount）：post `/cgi-bin/open/unbind`；
/// errcode 缺失（Java null errcode）时 isSuccess 为 false。
#[tokio::test]
async fn unbind_open_account_returns_false_without_errcode() {
    let h = Harness::new(vec![MockRoute::post(
        "/cgi-bin/open/unbind",
        &[r#"{"errmsg":"ok"}"#],
    )])
    .await;
    h.set_authorizer_token(APP_ID_MP, "auth_tok_mp");
    let ok = h
        .component()
        .unbind_open_account(APP_ID_MP, "mp", "openappid_03")
        .await
        .unwrap();
    // Java：errcode 为 null → isSuccess()=false（缺失 → "" → false 镜像）
    assert!(!ok);

    let req = h.server.requests().pop().unwrap();
    assert!(req.path.starts_with("/cgi-bin/open/unbind"));
    assert!(req.path.contains("access_token=auth_tok_mp"));
    let body = req_body_json(&req);
    assert_eq!(body["appid"], APP_ID_MP);
    assert_eq!(body["open_appid"], "openappid_03");
}

/// 获取 open 帐号（getOpenAccount）：post `/cgi-bin/open/get`，请求体
/// {"appid"}；结果解析 open_appid（Java `WxOpenGetResult.fromJson`）。
#[tokio::test]
async fn get_open_account_parses_result() {
    let h = Harness::new(vec![MockRoute::post(
        "/cgi-bin/open/get",
        &[r#"{"open_appid":"openappid_04","errcode":0,"errmsg":""}"#],
    )])
    .await;
    h.set_authorizer_token(APP_ID_MA, "auth_tok_ma");
    let result = h
        .component()
        .get_open_account(APP_ID_MA, "mini")
        .await
        .unwrap();
    assert_eq!(result.open_appid, "openappid_04");

    let req = h.server.requests().pop().unwrap();
    assert!(req.path.starts_with("/cgi-bin/open/get"));
    assert!(req.path.contains("access_token=auth_tok_ma"));
    let body = req_body_json(&req);
    assert_eq!(body["appid"], APP_ID_MA);
    assert_eq!(body.get("open_appid"), None);
}

/// 未知 appIdType：镜像 Java `openAccountServicePost` 的 default 分支
/// 抛「appIdType类型异常」。
#[tokio::test]
async fn open_account_rejects_unknown_app_id_type() {
    let h = Harness::new(vec![]).await;
    let err = h
        .component()
        .create_open_account(APP_ID_MP, "unknown_type")
        .await
        .unwrap_err();
    assert!(
        err.to_string().contains("appIdType类型异常"),
        "实际错误：{err}"
    );
}

/// 双检锁缓存：同 appid 缓存命中返回同一实例；不同 appid 装配不同
/// 实例；mp/ma 缓存桶相互独立（镜像 Java 独立 map）。
#[tokio::test]
async fn bridge_services_cached_per_appid_and_independent() {
    let h = Harness::new(vec![]).await;
    // 同 appid 同桶：同一实例
    let a1 =
        downcast_mp_service(h.component().get_wx_mp_service_by_appid(APP_ID_MP).unwrap()).unwrap();
    let a2 =
        downcast_mp_service(h.component().get_wx_mp_service_by_appid(APP_ID_MP).unwrap()).unwrap();
    assert!(Arc::ptr_eq(&a1, &a2));
    // 不同 appid：不同实例
    let b1 = downcast_mp_service(
        h.component()
            .get_wx_mp_service_by_appid("other_appid")
            .unwrap(),
    )
    .unwrap();
    assert!(!Arc::ptr_eq(&a1, &b1));
    // mp/ma 桶独立：同 appid 的 ma 服务与 mp 服务不是同一实例
    // （Any 层指针比较：两个缓存桶的底层 Arc 不同）
    let mp_any = h.component().get_wx_mp_service_by_appid(APP_ID_MP).unwrap();
    let ma_any = h.component().get_wx_ma_service_by_appid(APP_ID_MP).unwrap();
    assert!(!Arc::ptr_eq(&mp_any, &ma_any));
}

/// 代 ma 的 jsCode2SessionInfo 委托组件服务 miniappJscode2Session：
/// GET `/sns/component/jscode2session?appid=&js_code=&component_appid=`
/// 注入 component_access_token（Java `WxOpenMaServiceImpl
/// .jsCode2SessionInfo` 语义，非普通小程序 appid/secret 链路）。
#[tokio::test]
async fn bridge_ma_jscode2session_delegates_component() {
    let h = Harness::new(vec![MockRoute::get(
        "/sns/component/jscode2session",
        &[r#"{"session_key":"sk_01","openid":"oid_01","unionid":"uni_01"}"#],
    )])
    .await;
    let ma =
        downcast_ma_service(h.component().get_wx_ma_service_by_appid(APP_ID_MA).unwrap()).unwrap();
    let session = ma.js_code_to_session("js_code_abc").await.unwrap();
    assert_eq!(session.openid, "oid_01");
    assert_eq!(session.session_key, "sk_01");

    let req = h.server.requests().pop().unwrap();
    assert_eq!(req.method, "GET");
    assert!(req.path.starts_with("/sns/component/jscode2session"));
    assert!(req.path.contains("appid=authorizer_appid_ma_01"));
    assert!(req.path.contains("js_code=js_code_abc"));
    assert!(req.path.contains("component_appid=component_appid_01"));
    // 组件链路注入 component_access_token
    assert!(req.path.contains("access_token=comp_tok_v1"));
}
