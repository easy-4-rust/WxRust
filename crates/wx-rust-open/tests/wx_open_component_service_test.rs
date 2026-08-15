//! WxOpenComponentService 集成测试（自含 MockServer 模式）。
//!
//! 覆盖：authorizer info 解析（O1 golden）、query auth token 回写、route
//! 分发（verify_ticket 存储链 / authorized / notify_third_fasteregister /
//! 未知类型 / 加密链路端到端）、预授权码与链接、authorizer token 刷新链、
//! component token 40001 自动刷新链、模板列表、multipart 上传、minishop
//! 类目手工解析、oauth2、authorizer list 回写、优惠券创建。
//!
//! MockServer 为 tokio TCP 自实现（HTTP/1.1：读请求头 + Content-Length 读
//! 体 → 按 (method, path 前缀) 路由返回配置的响应序列，末条重复），不依赖
//! 外部 mock 库。

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use wx_rust_open::api::r#impl::WxOpenServiceImpl;
use wx_rust_open::api::{WxOpenComponentService, WxOpenService};
use wx_rust_open::bean::message::WxOpenXmlMessage;
use wx_rust_open::config::WxOpenConfigStorage;
use wx_rust_open::config::r#impl::WxOpenDefaultConfig;
use wx_rust_open::util::crypto::WxOpenCryptUtils;

// ---------------------------------------------------------------------------
// 自含 MockServer
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
        // 读请求头；头与体可能同批到达，保留头结束后的字节作为体前缀
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

        // 块作用域结束即释放锁（guard 不跨 await，保证 spawn 的 future 为 Send）
        let response = {
            let mut st = state.lock().unwrap();
            st.requests.push(RecordedRequest {
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

    /// 指定 path 前缀的请求数。
    fn call_count(&self, path_prefix: &str) -> usize {
        let st = self.state.lock().unwrap();
        st.routes
            .iter()
            .position(|r| r.path_prefix == path_prefix)
            .map(|i| st.calls[i])
            .unwrap_or(0)
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
    /// 构建夹具；`extra_routes` 追加到标准路由（api_component_token）之后；
    /// 测试自带 api_component_token 路由（如刷新链）时不预置默认路由。
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
}

/// 解析 JSON 请求体。
fn req_body_json(req: &RecordedRequest) -> serde_json::Value {
    serde_json::from_str(&req.body).unwrap_or(serde_json::Value::Null)
}

// ---------------------------------------------------------------------------
// 测试
// ---------------------------------------------------------------------------

/// 授权方信息解析（O1 golden 线格式：snake_case 键 + func_info 扁平化 +
/// MiniProgramInfo 大驼峰）。
#[tokio::test]
async fn authorizer_info_parses_java_golden() {
    let golden = r#"{
  "authorizer_info": {
    "nick_name": "美妆饰品",
    "service_type_info": {"id": 0},
    "verify_type_info": {"id": -1},
    "user_name": "gh_c43395cb652e",
    "principal_name": "个人",
    "business_info": {"open_pay": 0, "open_shake": 0},
    "MiniProgramInfo": {
      "network": {"RequestDomain": ["https://weixin.qq.com"], "BizDomain": []},
      "categories": [{"first": "生活服务", "second": "丽人服务"}],
      "visit_status": 0
    },
    "register_type": 0,
    "account_status": 1,
    "basic_config": {"is_phone_configured": true, "is_email_configured": true}
  },
  "authorization_info": {
    "authorizer_appid": "wx326eecacf7370d4e",
    "authorizer_refresh_token": "refreshtoken@@@RU0Sgi7bD6apS7frS9gj8Sbws7OoDejK9Z-cm0EnCzg",
    "func_info": [
      {"funcscope_category": {"id": 3}},
      {"funcscope_category": {"id": 7}, "confirm_info": {"need_confirm": 0}}
    ]
  }
}"#;
    let h = Harness::new(vec![MockRoute::post(
        "/cgi-bin/component/api_get_authorizer_info",
        &[golden],
    )])
    .await;
    let res = h
        .component()
        .get_authorizer_info("wx326eecacf7370d4e")
        .await
        .unwrap();
    assert!(res.is_mini_program());
    let info = res.authorizer_info.as_ref().unwrap();
    assert_eq!(info.nick_name.as_deref(), Some("美妆饰品"));
    assert_eq!(info.account_status, Some(1));
    let auth = res.authorization_info.as_ref().unwrap();
    assert_eq!(auth.func_info, vec![3, 7]);
    assert_eq!(
        auth.authorizer_refresh_token.as_deref(),
        Some("refreshtoken@@@RU0Sgi7bD6apS7frS9gj8Sbws7OoDejK9Z-cm0EnCzg")
    );
    // 请求体镜像 Java：component_appid + authorizer_appid
    let req = h.server.requests().pop().unwrap();
    let body = req_body_json(&req);
    assert_eq!(body["component_appid"], "component_appid_01");
    assert_eq!(body["authorizer_appid"], "wx326eecacf7370d4e");
}

/// getQueryAuth：授权码换授权信息并回写 authorizer token 到配置存储。
#[tokio::test]
async fn get_query_auth_writes_authorizer_tokens() {
    let resp = r#"{"authorization_info":{
      "authorizer_appid":"wx_authorizer_01",
      "authorizer_access_token":"auth_tok_01",
      "expires_in":7200,
      "authorizer_refresh_token":"refresh_tok_01",
      "func_info":[{"funcscope_category":{"id":1}}]
    }}"#;
    let h = Harness::new(vec![MockRoute::post(
        "/cgi-bin/component/api_query_auth",
        &[resp],
    )])
    .await;
    let result = h
        .component()
        .get_query_auth("queryauthcode@@@123")
        .await
        .unwrap();
    let info = result.authorization_info.as_ref().unwrap();
    assert_eq!(info.authorizer_appid.as_deref(), Some("wx_authorizer_01"));
    // 存储链：access_token + refresh_token 已回写
    assert_eq!(
        h.config
            .authorizer_access_token("wx_authorizer_01")
            .as_deref(),
        Some("auth_tok_01")
    );
    assert_eq!(
        h.config
            .authorizer_refresh_token("wx_authorizer_01")
            .as_deref(),
        Some("refresh_tok_01")
    );
    // 请求体镜像 Java
    let req = h.server.requests().pop().unwrap();
    let body = req_body_json(&req);
    assert_eq!(body["authorization_code"], "queryauthcode@@@123");
}

/// route：verify_ticket 存储链（component_verify_ticket → 配置存储）。
#[tokio::test]
async fn route_verify_ticket_stores_ticket() {
    let h = Harness::new(vec![]).await;
    let message = WxOpenXmlMessage {
        info_type: Some("component_verify_ticket".to_string()),
        component_verify_ticket: Some("ticket@@@6iJtQHC1".to_string()),
        create_time: Some(1413192605),
        ..Default::default()
    };
    let ret = h.component().route(&message).await.unwrap();
    assert_eq!(ret, "success");
    assert_eq!(
        h.config.component_verify_ticket().as_deref(),
        Some("ticket@@@6iJtQHC1")
    );
}

/// route：authorized 事件 → 授权码换授权信息 → success。
#[tokio::test]
async fn route_authorized_dispatch_exchanges_code() {
    let resp = r#"{"authorization_info":{
      "authorizer_appid":"wx_authorizer_02",
      "authorizer_access_token":"auth_tok_02",
      "expires_in":7200,
      "authorizer_refresh_token":"refresh_tok_02",
      "func_info":[]
    }}"#;
    let h = Harness::new(vec![MockRoute::post(
        "/cgi-bin/component/api_query_auth",
        &[resp],
    )])
    .await;
    let message = WxOpenXmlMessage {
        info_type: Some("authorized".to_string()),
        authorization_code: Some("queryauthcode@@@AUTH02".to_string()),
        ..Default::default()
    };
    let ret = h.component().route(&message).await.unwrap();
    assert_eq!(ret, "success");
    // 分发链：query_auth 被调用且授权方 token 已存储
    assert_eq!(h.server.call_count("/cgi-bin/component/api_query_auth"), 1);
    let req = h.server.requests().pop().unwrap();
    assert_eq!(
        req_body_json(&req)["authorization_code"],
        "queryauthcode@@@AUTH02"
    );
    assert_eq!(
        h.config
            .authorizer_access_token("wx_authorizer_02")
            .as_deref(),
        Some("auth_tok_02")
    );
}

/// route：notify_third_fasteregister（status=0）→ 分发。
#[tokio::test]
async fn route_notify_third_fasteregister_dispatches() {
    let resp = r#"{"authorization_info":{"authorizer_appid":"wx_fast_ma_01",
      "authorizer_access_token":"t","expires_in":7200,
      "authorizer_refresh_token":"r","func_info":[]}}"#;
    let h = Harness::new(vec![MockRoute::post(
        "/cgi-bin/component/api_query_auth",
        &[resp],
    )])
    .await;
    let message = WxOpenXmlMessage {
        info_type: Some("notify_third_fasteregister".to_string()),
        status: Some(0),
        auth_code: Some("queryauthcode@@@FAST01".to_string()),
        ..Default::default()
    };
    assert_eq!(h.component().route(&message).await.unwrap(), "success");
    // status != 0 时不符合分发条件，镜像 Java 落到末尾 `return ""`
    let message = WxOpenXmlMessage {
        info_type: Some("notify_third_fasteregister".to_string()),
        status: Some(1),
        auth_code: Some("queryauthcode@@@FAST02".to_string()),
        ..Default::default()
    };
    assert_eq!(h.component().route(&message).await.unwrap(), "");
    assert_eq!(h.server.call_count("/cgi-bin/component/api_query_auth"), 1);
}

/// route：未知 InfoType 返回空串（镜像 Java `return ""`）。
#[tokio::test]
async fn route_unknown_info_type_returns_empty() {
    let h = Harness::new(vec![]).await;
    let message = WxOpenXmlMessage {
        info_type: Some("unauthorized".to_string()),
        ..Default::default()
    };
    assert_eq!(h.component().route(&message).await.unwrap(), "");
}

/// 加密回调链路端到端：encrypt_context → from_encrypted_xml → route。
#[tokio::test]
async fn route_encrypted_verify_ticket_end_to_end() {
    let h = Harness::new(vec![]).await;
    // 明文 verify_ticket 推送（InfoType 必须为 component_verify_ticket，
    // 对应 Java route 的 equalsIgnoreCase 匹配）
    let plain = r#"<xml>
  <AppId><![CDATA[wxb1234567890abcdef]]></AppId>
  <CreateTime>1413192605</CreateTime>
  <InfoType><![CDATA[component_verify_ticket]]></InfoType>
  <ComponentVerifyTicket><![CDATA[ticket@@@encrypted01]]></ComponentVerifyTicket>
</xml>"#;
    let crypt = WxOpenCryptUtils::new(h.config.as_ref()).unwrap();
    let ctx = crypt.encrypt_context(plain).unwrap();
    let wrapped = format!(
        "<xml><Encrypt><![CDATA[{}]]></Encrypt></xml>",
        ctx.encrypted_xml
    );
    let message = WxOpenXmlMessage::from_encrypted_xml(
        &wrapped,
        h.config.as_ref(),
        &ctx.timestamp,
        &ctx.nonce,
        &ctx.signature,
    )
    .unwrap();
    assert_eq!(
        message.info_type.as_deref(),
        Some("component_verify_ticket")
    );
    let ret = h.component().route(&message).await.unwrap();
    assert_eq!(ret, "success");
    assert_eq!(
        h.config.component_verify_ticket().as_deref(),
        Some("ticket@@@encrypted01")
    );
}

/// 预授权码 + 预授权链接构建（auth_type/biz_appid 占位替换）。
#[tokio::test]
async fn get_pre_auth_code_and_url_build() {
    let h = Harness::new(vec![MockRoute::post(
        "/cgi-bin/component/api_create_preauthcode",
        &[r#"{"pre_auth_code":"preauthcode@@@P01"}"#],
    )])
    .await;
    let url = h
        .component()
        .get_pre_auth_url_with(
            "https://example.com/callback?x=1",
            Some("1"),
            Some("wx_biz_01"),
        )
        .await
        .unwrap();
    assert!(url.contains("component_appid=component_appid_01"));
    assert!(url.contains("pre_auth_code=preauthcode@@@P01"));
    // redirect_uri 经 encodeURIComponent 语义编码（percent_encoding
    // NON_ALPHANUMERIC 对 '.' 亦编码，Wave 0 冻结语义）
    assert!(url.contains("redirect_uri=https%3A%2F%2Fexample%2Ecom%2Fcallback%3Fx%3D1"));
    assert!(url.contains("auth_type=1"));
    assert!(url.contains("biz_appid=wx_biz_01"));
    assert!(!url.contains("auth_type=xxx"));
    // 移动端链接
    let mobile = h
        .component()
        .get_mobile_pre_auth_url("https://example.com/m")
        .await
        .unwrap();
    assert!(mobile.contains("open.weixin.qq.com/wxaopen/safe/bindcomponent"));
}

/// authorizer access_token 刷新链：refresh_token 换新 token + 缓存复用。
#[tokio::test]
async fn authorizer_access_token_refresh_chain() {
    let h = Harness::new(vec![MockRoute::post(
        "/cgi-bin/component/api_authorizer_token",
        &[r#"{"authorizer_access_token":"new_auth_tok",
             "authorizer_refresh_token":"new_refresh_tok",
             "expires_in":7200}"#],
    )])
    .await;
    h.config
        .set_authorizer_refresh_token("wx_authorizer_03", "old_refresh_tok");
    let component = h.component();
    let token = component
        .get_authorizer_access_token("wx_authorizer_03", true)
        .await
        .unwrap();
    assert_eq!(token, "new_auth_tok");
    assert_eq!(
        h.config
            .authorizer_access_token("wx_authorizer_03")
            .as_deref(),
        Some("new_auth_tok")
    );
    assert_eq!(
        h.config
            .authorizer_refresh_token("wx_authorizer_03")
            .as_deref(),
        Some("new_refresh_tok")
    );
    // 请求体镜像 Java：component_appid/authorizer_appid/authorizer_refresh_token
    let req = h.server.requests().pop().unwrap();
    let body = req_body_json(&req);
    assert_eq!(body["authorizer_appid"], "wx_authorizer_03");
    assert_eq!(body["authorizer_refresh_token"], "old_refresh_tok");
    // 未过期 + 非强制刷新 → 缓存直返，不再发请求
    let cached = component
        .get_authorizer_access_token("wx_authorizer_03", false)
        .await
        .unwrap();
    assert_eq!(cached, "new_auth_tok");
    assert_eq!(
        h.server
            .call_count("/cgi-bin/component/api_authorizer_token"),
        1
    );
}

/// component token 40001 自动刷新链：过期 → 强刷 → 重试成功。
#[tokio::test]
async fn component_token_auto_refresh_on_40001() {
    let h = Harness::new(vec![
        MockRoute::post(
            "/cgi-bin/component/api_component_token",
            &[
                r#"{"component_access_token":"comp_tok_v1","expires_in":7200}"#,
                r#"{"component_access_token":"comp_tok_v2","expires_in":7200}"#,
            ],
        ),
        MockRoute::post(
            "/cgi-bin/component/api_start_push_ticket",
            &[
                r#"{"errcode":40001,"errmsg":"invalid credential"}"#,
                r#"{"errcode":0,"errmsg":"ok"}"#,
            ],
        ),
    ])
    .await;
    h.component().start_push_ticket().await.unwrap();
    // 刷新链：第一次 api_component_token 取 v1 → 40001 → 强刷取 v2 → 重试
    assert_eq!(
        h.server
            .call_count("/cgi-bin/component/api_component_token"),
        2
    );
    assert_eq!(
        h.server
            .call_count("/cgi-bin/component/api_start_push_ticket"),
        2
    );
    let requests = h.server.requests();
    // 取最后一次 start_push_ticket 请求（重试应携带刷新后的 v2 token）
    let retry = requests
        .iter()
        .rev()
        .find(|r| {
            r.path
                .starts_with("/cgi-bin/component/api_start_push_ticket")
        })
        .unwrap();
    assert!(retry.path.contains("component_access_token=comp_tok_v2"));
}

/// 模板列表解析（注入键 access_token，非默认 component_access_token）。
#[tokio::test]
async fn get_template_list_parses() {
    let resp = r#"{"template_list":[
      {"template_id":1,"user_version":"v1.0","user_desc":"desc","template_type":1,"create_time":1700000000,"source_miniprogram_appid":"wx_src_01","audit_status":0,"reason":"","developer":"dev"}
    ]}"#;
    let h = Harness::new(vec![MockRoute::get(
        "/wxa/gettemplatelist?template_type=1",
        &[resp],
    )])
    .await;
    let list = h
        .component()
        .get_template_list_with_type(Some(1))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].template_id, 1);
    assert_eq!(list[0].user_version, "v1.0");
    // 注入键为 access_token
    let req = h.server.requests().pop().unwrap();
    assert!(req.path.contains("access_token=comp_tok_v1"));
    assert!(!req.path.contains("component_access_token"));
    // 无 template_type 参数版本
    let h2 = Harness::new(vec![MockRoute::get("/wxa/gettemplatelist", &[resp])]).await;
    let list2 = h2.component().get_template_list().await.unwrap().unwrap();
    assert_eq!(list2.len(), 1);
}

/// 草稿列表：无 draft_list 字段时返回 None（镜像 Java null）。
#[tokio::test]
async fn get_template_draft_list_none_when_absent() {
    let h = Harness::new(vec![MockRoute::get(
        "/wxa/gettemplatedraftlist",
        &[r#"{"errcode":0,"errmsg":"ok"}"#],
    )])
    .await;
    let list = h.component().get_template_draft_list().await.unwrap();
    assert!(list.is_none());
}

/// multipart 上传（MinishopUploadRequestExecutor：字段名 media）。
#[tokio::test]
async fn upload_minishop_media_file_multipart() {
    let resp = r#"{"errcode":"0","errmsg":"","picFile":{"mediaId":"media_01","tempImgUrl":"https://mmbiz.qpic.cn/x"}}"#;
    let h = Harness::new(vec![MockRoute::post("/product/img/upload", &[resp])]).await;
    // 临时文件
    let dir = std::env::temp_dir();
    let file_path = dir.join(format!("wx_open_upload_test_{}.png", std::process::id()));
    std::fs::write(&file_path, b"fake-image-bytes-123").unwrap();
    let url = format!(
        "{}/product/img/upload?access_token=upload_tok&height=100&width=200",
        h.server.base_url()
    );
    let result = h
        .service
        .upload_minishop_media_file(&url, file_path.to_str().unwrap())
        .await
        .unwrap();
    std::fs::remove_file(&file_path).unwrap();
    assert_eq!(result.errcode, "0");
    assert_eq!(result.pic_file.media_id, "media_01");
    // multipart 线格式：字段名 media + 文件名 + 文件内容
    let req = h.server.requests().pop().unwrap();
    assert!(req.body.contains("name=\"media\""));
    assert!(req.body.contains("filename=\"wx_open_upload_test_"));
    assert!(req.body.contains("fake-image-bytes-123"));
}

/// minishop 类目手工解析（Java 手动拼装语义）。
#[tokio::test]
async fn get_minishop_categories_manual_parse() {
    let resp = r#"{"errcode":0,"errmsg":"","cat_list":[
      {"cat_id":1001,"f_cat_id":0,"name":"服饰"},
      {"cat_id":1002,"f_cat_id":1001,"name":"男装"}
    ]}"#;
    let h = Harness::new(vec![
        // 授权方 token 刷新链
        MockRoute::post(
            "/cgi-bin/component/api_authorizer_token",
            &[r#"{"authorizer_access_token":"shop_tok","authorizer_refresh_token":"r","expires_in":7200}"#],
        ),
        MockRoute::post("/product/category/get", &[resp]),
    ])
    .await;
    h.config
        .set_authorizer_refresh_token("wx_shop_01", "refresh_tok_shop");
    let categories = h
        .component()
        .get_minishop_categories("wx_shop_01", 0)
        .await
        .unwrap();
    assert_eq!(categories.errcode, 0);
    assert_eq!(categories.cat_list.len(), 2);
    assert_eq!(categories.cat_list[0].cat_id, 1001);
    assert_eq!(categories.cat_list[0].name, "服饰");
    assert_eq!(categories.cat_list[1].f_cat_id, 1001);
    // 裸 post：URL 已带授权方 access_token
    let req = h.server.requests().pop().unwrap();
    assert!(
        req.path
            .starts_with("/product/category/get?access_token=shop_tok")
    );
    assert_eq!(req_body_json(&req)["f_cat_id"], 0);
}

/// oauth2 code 换 access_token（GET sns 接口 + component token 注入）。
#[tokio::test]
async fn oauth2_get_access_token_parses() {
    let h = Harness::new(vec![MockRoute::get(
        "/sns/oauth2/component/access_token",
        &[r#"{"access_token":"oauth_tok","expires_in":7200,
             "refresh_token":"oauth_refresh","openid":"openid_01",
             "scope":"snsapi_userinfo","unionid":"unionid_01"}"#],
    )])
    .await;
    let token = h
        .component()
        .oauth2_get_access_token("wx_app_01", "auth_code_01")
        .await
        .unwrap();
    assert_eq!(token.access_token, "oauth_tok");
    assert_eq!(token.open_id, "openid_01");
    // 授权链接构建（纯字符串，无 HTTP）
    let url = h.component().oauth2_build_authorization_url(
        "wx_app_01",
        "https://example.com/cb",
        "snsapi_userinfo",
        "state_01",
    );
    assert!(url.contains("appid=wx_app_01"));
    assert!(url.contains("redirect_uri=https%3A%2F%2Fexample%2Ecom%2Fcb"));
    assert!(url.contains("component_appid=component_appid_01"));
}

/// 授权方列表解析 + refresh_token 回写配置存储。
#[tokio::test]
async fn get_authorizer_list_writes_refresh_tokens() {
    let resp = r#"{"total_count":2,"list":[
      {"authorizer_appid":"wx_auth_list_01","refresh_token":"refresh_list_01","auth_time":"1700000000"},
      {"authorizer_appid":"wx_auth_list_02","refresh_token":"refresh_list_02","auth_time":"1700000001"}
    ]}"#;
    let h = Harness::new(vec![MockRoute::post(
        "/cgi-bin/component/api_get_authorizer_list",
        &[resp],
    )])
    .await;
    let result = h.component().get_authorizer_list(0, 0).await.unwrap();
    assert_eq!(result.total_count, Some(2));
    assert_eq!(result.list.as_ref().unwrap().len(), 2);
    assert_eq!(
        h.config
            .authorizer_refresh_token("wx_auth_list_01")
            .as_deref(),
        Some("refresh_list_01")
    );
    assert_eq!(
        h.config
            .authorizer_refresh_token("wx_auth_list_02")
            .as_deref(),
        Some("refresh_list_02")
    );
    // 请求体镜像 Java：offset/count（len==0 → 10）
    let req = h.server.requests().pop().unwrap();
    let body = req_body_json(&req);
    assert_eq!(body["offset"], 0);
    assert_eq!(body["count"], 10);
}

/// minishop 优惠券创建：返回 coupon_id。
#[tokio::test]
async fn minishop_create_coupon_returns_id() {
    let resp = r#"{"errcode":0,"errmsg":"","data":{"coupon_id":888}}"#;
    let h = Harness::new(vec![
        MockRoute::post(
            "/cgi-bin/component/api_authorizer_token",
            &[r#"{"authorizer_access_token":"shop_tok","authorizer_refresh_token":"r","expires_in":7200}"#],
        ),
        MockRoute::post("/product/coupon/create", &[resp]),
    ])
    .await;
    h.config
        .set_authorizer_refresh_token("wx_shop_02", "refresh_tok_shop2");
    let coupon = wx_rust_open::bean::WxMinishopCoupon {
        coupon_id: -1,
        status: -1,
        r#type: 1,
        name: "满减券".to_string(),
        discount_info: wx_rust_open::bean::WxMinishopCouponDiscountInfo {
            discount_fee: 10,
            discount_num: 1,
            discount_condition: wx_rust_open::bean::WxMinishopCouponDiscountCondition {
                product_cnt: 1,
                product_ids: vec![1, 2],
                product_price: 100,
            },
        },
        ext_info: wx_rust_open::bean::WxMinishopCouponExtInfo {
            notes: "note".to_string(),
            valid_time: 0,
            invalid_time: 0,
            jump_product_id: 0,
        },
        promote_info: wx_rust_open::bean::WxMinishopCouponPromoteInfo {
            customize_channel: String::new(),
            promotion_type: 0,
        },
        receive_info: wx_rust_open::bean::WxMinishopCouponReceiveInfo {
            end_time: 0,
            limit_num_one_person: 1,
            start_time: 0,
            total_num: 100,
        },
        valid_info: wx_rust_open::bean::WxMinishopCouponValidInfo {
            end_time: 0,
            start_time: 0,
            valid_day_num: 30,
            valid_type: 1,
        },
    };
    let coupon_id = h
        .component()
        .minishop_create_coupon("wx_shop_02", &coupon)
        .await
        .unwrap();
    assert_eq!(coupon_id, 888);
    // 请求体镜像 Java toJsonObject：snake_case + discount_info 嵌套
    let req = h.server.requests().pop().unwrap();
    let body = req_body_json(&req);
    assert_eq!(body["name"], "满减券");
    assert_eq!(body["discount_info"]["discount_fee"], 10);
    assert_eq!(
        body["discount_info"]["discount_condition"]["product_ids"],
        serde_json::json!([1, 2])
    );
    assert_eq!(body["valid_info"]["valid_day_num"], 30);
}

/// Wave 4 接线后：代 mp/ma 服务返回 Some（可下转）；Wave 5 接线后：
/// 代 minishop 服务亦返回 Some（镜像 Java
/// `WX_OPEN_MINISHOP_SERVICE_MAP` 双检锁装配，完整断言见
/// wx_open_ma_domain_test.rs）；open 帐号方法走桥接后由
/// wx_open_mp_ma_bridge_test.rs 全量覆盖。
#[tokio::test]
async fn mp_ma_bridge_returns_none() {
    let h = Harness::new(vec![]).await;
    // Wave 4 接线：mp/ma 桥接服务已装配（完整断言见 wx_open_mp_ma_bridge_test.rs）
    let mp = h
        .component()
        .get_wx_mp_service_by_appid("wx_app_01")
        .expect("get_wx_mp_service_by_appid 已接线，应返回 Some");
    assert!(wx_rust_open::api::r#impl::downcast_mp_service(mp).is_some());
    let ma = h
        .component()
        .get_wx_ma_service_by_appid("wx_app_01")
        .expect("get_wx_ma_service_by_appid 已接线，应返回 Some");
    assert!(wx_rust_open::api::r#impl::downcast_ma_service(ma).is_some());
    // Wave 5 接线：minishop 服务已装配（Java 静态 map 语义）
    let minishop = h
        .component()
        .get_wx_minishop_service_by_appid("wx_app_01")
        .expect("get_wx_minishop_service_by_appid 已接线，应返回 Some");
    assert!(
        minishop
            .downcast::<wx_rust_open::api::r#impl::WxOpenMinishopServiceImpl>()
            .is_ok()
    );
}

/// 幂等镜像：minishopGetCouponList / minishopCommonPost Java 恒 null。
#[tokio::test]
async fn minishop_null_returns_mirror() {
    let h = Harness::new(vec![MockRoute::post(
        "/cgi-bin/component/api_authorizer_token",
        &[r#"{"authorizer_access_token":"shop_tok","authorizer_refresh_token":"r","expires_in":7200}"#],
    )])
    .await;
    h.config
        .set_authorizer_refresh_token("wx_shop_03", "refresh_tok_shop3");
    // Java 先取授权方 token（forceRefresh）后 return null
    let list = h
        .component()
        .minishop_get_coupon_list("wx_shop_03", "2024-01-01", "2024-12-31", 1, 1, 10)
        .await
        .unwrap();
    assert!(list.is_none());
    assert_eq!(
        h.server
            .call_count("/cgi-bin/component/api_authorizer_token"),
        1
    );
    let common = h
        .component()
        .minishop_common_post("wx_shop_03", "https://x", "{}")
        .await
        .unwrap();
    assert!(common.is_none());
}
