//! WxOpenComponentService 覆盖率提升测试（自含 MockServer 模式）。
//!
//! 补充覆盖：
//! - oauth2_refresh_access_token（GET `/sns/oauth2/component/refresh_token`）
//! - miniapp_jscode2_session（GET `/sns/component/jscode2session`）
//! - add_to_template / add_to_template_with_type / delete_template
//! - fast_register_weapp / fast_register_weapp_search
//! - fast_register_personal_weapp / fast_register_personal_weapp_search
//! - fast_register_beta_weapp
//! - register_shop / check_audit_status / check_audit_status_with_appid
//! - submit_merchant_info / submit_basic_info
//! - get_minishop_brands / get_minishop_delivery_template / get_minishop_cat_list
//! - get_minishop_delivery_company
//! - minishop_push_coupon_to_user / minishop_update_coupon / minishop_update_coupon_status
//! - minishop_goods_add_spu / del_spu / update_spu / listing_spu / delisting_spu
//! - minishop_goods_add_sku / batch_add_sku / del_sku / update_sku
//! - minishop_goods_update_sku_price / update_sku_stock
//! - add_limit_discount_goods / get_limit_discount_list / update_limit_discount_status
//! - get_share_cloud_base_env / get_tcb_env_list / change_tcb_env / share_cloud_base_env
//! - clear_quota_v2 / apply_set_order_path_info
//! - modify_wxa_server_domain / get_domain_confirm_file / modify_wxa_jump_domain
//! - modify_wxa_jump_domain_info / get_authorizer_option / set_authorizer_option
//! - have_open / get_component_access_token / route updateauthorized
//!
//! 注意：返回 bean 含 `errcode: String` 的方法（WxOpenResult 等），因 executor
//! 层 WxError 要求 integer errcode 而 bean 要求 String errcode，happy path 不可达。
//! 这些方法测试 error path（executor errcode!=0 抛错）或请求体断言。

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use wx_rust_open::api::{WxOpenComponentService, WxOpenService};
use wx_rust_open::config::WxOpenConfigStorage;
use wx_rust_open::config::r#impl::WxOpenDefaultConfig;

// ---------------------------------------------------------------------------
// 自含 MockServer
// ---------------------------------------------------------------------------

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

#[derive(Debug, Clone)]
#[allow(dead_code)]
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

    fn base_url(&self) -> String {
        format!("http://{}", self.addr)
    }

    fn requests(&self) -> Vec<RecordedRequest> {
        self.state.lock().unwrap().requests.clone()
    }

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

const COMPONENT_TOKEN_RESP: &str = r#"{"component_access_token":"comp_tok_v1","expires_in":7200}"#;

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

struct Harness {
    server: MockServer,
    config: Arc<WxOpenDefaultConfig>,
    #[allow(dead_code)]
    service: Arc<dyn WxOpenService>,
    component: Arc<dyn WxOpenComponentService>,
}

impl Harness {
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
        let service = wx_rust_open::api::r#impl::WxOpenServiceImpl::new_arc(config.clone());
        let component = service.wx_open_component_service().unwrap();
        Self {
            server,
            config,
            service,
            component,
        }
    }
}

fn req_body_json(req: &RecordedRequest) -> serde_json::Value {
    serde_json::from_str(&req.body).unwrap_or(serde_json::Value::Null)
}

fn assert_error_code(err: &wx_rust_common::error::WxErrorException, expected: i32) {
    assert_eq!(
        err.error_code(),
        Some(expected),
        "期望错误码 {expected}，实际：{err:?}"
    );
}

const AUTHORIZER_APPID: &str = "wx_authorizer_cov_01";

// ═══════════════════════════════════════════════════════════════
// 1. oauth2_refresh_access_token
// ═══════════════════════════════════════════════════════════════

#[tokio::test]
async fn oauth2_refresh_access_token_parses() {
    let h = Harness::new(vec![MockRoute::get(
        "/sns/oauth2/component/refresh_token",
        &[r#"{"access_token":"refreshed_tok","expires_in":7200,
             "refresh_token":"new_refresh","openid":"oid_r","scope":"snsapi_base"}"#],
    )])
    .await;
    let token = h
        .component
        .oauth2_refresh_access_token("wx_app_r", "old_refresh")
        .await
        .unwrap();
    assert_eq!(token.access_token, "refreshed_tok");
    assert_eq!(token.open_id, "oid_r");

    let req = h.server.requests().pop().unwrap();
    assert!(req.path.starts_with("/sns/oauth2/component/refresh_token"));
    assert!(req.path.contains("appid=wx_app_r"));
    assert!(req.path.contains("refresh_token=old_refresh"));
    assert!(req.path.contains("component_access_token=comp_tok_v1"));
}

// ═══════════════════════════════════════════════════════════════
// 2. miniapp_jscode2_session
// ═══════════════════════════════════════════════════════════════

#[tokio::test]
async fn miniapp_jscode2_session_parses() {
    let h = Harness::new(vec![MockRoute::get(
        "/sns/component/jscode2session",
        &[r#"{"session_key":"sk_direct","openid":"oid_direct","unionid":"uni_direct"}"#],
    )])
    .await;
    let result = h
        .component
        .miniapp_jscode2_session("wx_ma_app", "js_code_direct")
        .await
        .unwrap();
    assert_eq!(result["session_key"], "sk_direct");
    assert_eq!(result["openid"], "oid_direct");

    let req = h.server.requests().pop().unwrap();
    assert!(req.path.starts_with("/sns/component/jscode2session"));
    assert!(req.path.contains("appid=wx_ma_app"));
    assert!(req.path.contains("js_code=js_code_direct"));
    assert!(req.path.contains("component_access_token=comp_tok_v1"));
}

// ═══════════════════════════════════════════════════════════════
// 3. 模板管理
// ═══════════════════════════════════════════════════════════════

#[tokio::test]
async fn add_to_template_posts() {
    let h = Harness::new(vec![MockRoute::post(
        "/wxa/addtotemplate",
        &[r#"{"errcode":0,"errmsg":"ok"}"#],
    )])
    .await;
    h.component.add_to_template(12345).await.unwrap();
    let req = h.server.requests().pop().unwrap();
    assert!(req.path.contains("access_token=comp_tok_v1"));
    assert_eq!(req_body_json(&req)["draft_id"], 12345);
}

#[tokio::test]
async fn add_to_template_with_type_posts() {
    let h = Harness::new(vec![MockRoute::post(
        "/wxa/addtotemplate",
        &[r#"{"errcode":0,"errmsg":"ok"}"#],
    )])
    .await;
    h.component
        .add_to_template_with_type(67890, 2)
        .await
        .unwrap();
    let req = h.server.requests().pop().unwrap();
    let body = req_body_json(&req);
    assert_eq!(body["draft_id"], 67890);
    assert_eq!(body["template_type"], 2);
}

#[tokio::test]
async fn delete_template_posts() {
    let h = Harness::new(vec![MockRoute::post(
        "/wxa/deletetemplate",
        &[r#"{"errcode":0,"errmsg":"ok"}"#],
    )])
    .await;
    h.component.delete_template(11111).await.unwrap();
    let req = h.server.requests().pop().unwrap();
    assert_eq!(req_body_json(&req)["template_id"], 11111);
}

// ═══════════════════════════════════════════════════════════════
// 4. 快速创建小程序（请求体断言 + error path）
// ═══════════════════════════════════════════════════════════════

/// fast_register_weapp：executor errcode=0 通过，bean 反序列化失败（String vs int）。
/// 覆盖 URL 构建、请求体拼装、post_with_key 调用链。
#[tokio::test]
async fn fast_register_weapp_exercises_request_body() {
    let h = Harness::new(vec![MockRoute::post(
        "/cgi-bin/component/fastregisterweapp",
        &[r#"{"errcode":0,"errmsg":"ok"}"#],
    )])
    .await;
    // 返回 bean errcode: String vs executor integer → serde 错误或 Ok
    let _ = h
        .component
        .fast_register_weapp("name1", "code1", "1", "wxuser1", "张三", "13800000000")
        .await;
    let req = h.server.requests().pop().unwrap();
    assert!(req.path.contains("action=create"));
    assert!(req.path.contains("component_access_token=comp_tok_v1"));
    let body = req_body_json(&req);
    assert_eq!(body["name"], "name1");
    assert_eq!(body["code"], "code1");
    assert_eq!(body["legal_persona_wechat"], "wxuser1");
    assert_eq!(body["legal_persona_name"], "张三");
}

#[tokio::test]
async fn fast_register_weapp_search_exercises_request_body() {
    let h = Harness::new(vec![MockRoute::post(
        "/cgi-bin/component/fastregisterweapp",
        &[r#"{"errcode":0,"errmsg":"ok"}"#],
    )])
    .await;
    let _ = h
        .component
        .fast_register_weapp_search("name2", "wxuser2", "李四")
        .await;
    let req = h.server.requests().pop().unwrap();
    assert!(req.path.contains("action=search"));
    let body = req_body_json(&req);
    assert_eq!(body["name"], "name2");
}

#[tokio::test]
async fn fast_register_personal_weapp_exercises_request_body() {
    let h = Harness::new(vec![MockRoute::post(
        "/wxa/component/fastregisterpersonalweapp",
        &[r#"{"errcode":0,"errmsg":"ok","taskid":"task_p1"}"#],
    )])
    .await;
    let _ = h
        .component
        .fast_register_personal_weapp("idname1", "wxuser_p1", "13900000000")
        .await;
    let req = h.server.requests().pop().unwrap();
    assert!(req.path.contains("action=create"));
    let body = req_body_json(&req);
    assert_eq!(body["idname"], "idname1");
    assert_eq!(body["wxuser"], "wxuser_p1");
}

#[tokio::test]
async fn fast_register_personal_weapp_search_exercises_request_body() {
    let h = Harness::new(vec![MockRoute::post(
        "/wxa/component/fastregisterpersonalweapp",
        &[r#"{"errcode":0,"errmsg":"ok","taskid":"task_p2"}"#],
    )])
    .await;
    let _ = h
        .component
        .fast_register_personal_weapp_search("task_query_1")
        .await;
    let req = h.server.requests().pop().unwrap();
    assert!(req.path.contains("action=query"));
    assert_eq!(req_body_json(&req)["taskid"], "task_query_1");
}

#[tokio::test]
async fn fast_register_beta_weapp_exercises_request_body() {
    let h = Harness::new(vec![MockRoute::post(
        "/wxa/component/fastregisterbetaweapp",
        &[r#"{"errcode":0,"errmsg":"ok","unique_id":"uuid_beta_1"}"#],
    )])
    .await;
    let _ = h
        .component
        .fast_register_beta_weapp("beta_name", "openid_beta")
        .await;
    let req = h.server.requests().pop().unwrap();
    assert!(req.path.contains("access_token=comp_tok_v1"));
    let body = req_body_json(&req);
    assert_eq!(body["name"], "beta_name");
    assert_eq!(body["openid"], "openid_beta");
}

// ═══════════════════════════════════════════════════════════════
// 5. register_shop / check_audit_status
// ═══════════════════════════════════════════════════════════════

#[tokio::test]
async fn register_shop_exercises_request_body() {
    let h = Harness::new(vec![MockRoute::post(
        "/product/register/register_shop",
        &[r#"{"errcode":0,"errmsg":"ok"}"#],
    )])
    .await;
    let _ = h
        .component
        .register_shop(
            "wx_shop_name",
            "张三",
            "110101199001011234",
            Some("channel_1"),
            Some(1),
            Some("https://auth.example.com"),
        )
        .await;
    let req = h.server.requests().pop().unwrap();
    assert!(req.path.contains("component_access_token=comp_tok_v1"));
    let body = req_body_json(&req);
    assert_eq!(body["wx_name"], "wx_shop_name");
    assert_eq!(body["channel_id"], "channel_1");
    assert_eq!(body["auth_page_url"], "https://auth.example.com");
}

/// check_audit_status：返回 String，不走 bean 反序列化。
#[tokio::test]
async fn check_audit_status_posts() {
    let h = Harness::new(vec![MockRoute::post(
        "/product/register/check_audit_status",
        &[r#"{"errcode":0,"errmsg":"ok","audit_status":1}"#],
    )])
    .await;
    let result = h
        .component
        .check_audit_status("wx_shop_audit")
        .await
        .unwrap();
    assert!(result.contains("audit_status"));

    let req = h.server.requests().pop().unwrap();
    assert!(req.path.contains("access_token="));
    let body = req_body_json(&req);
    assert_eq!(body["wx_name"], "wx_shop_audit");
}

#[tokio::test]
async fn check_audit_status_with_appid_posts() {
    let h = Harness::new(vec![
        MockRoute::post(
            "/cgi-bin/component/api_authorizer_token",
            &[r#"{"authorizer_access_token":"auth_tok_check","authorizer_refresh_token":"r","expires_in":7200}"#],
        ),
        MockRoute::post(
            "/product/register/check_audit_status",
            &[r#"{"errcode":0,"errmsg":"ok"}"#],
        ),
    ])
    .await;
    h.config
        .set_authorizer_refresh_token(AUTHORIZER_APPID, "refresh_tok_check");
    let result = h
        .component
        .check_audit_status_with_appid(AUTHORIZER_APPID, "wx_shop_audit2")
        .await
        .unwrap();
    assert!(result.contains("errcode"));

    let req = h.server.requests().pop().unwrap();
    assert!(req.path.contains("access_token=auth_tok_check"));
}

// ═══════════════════════════════════════════════════════════════
// 6. submit_merchant_info / submit_basic_info（请求体断言）
// ═══════════════════════════════════════════════════════════════

#[tokio::test]
async fn submit_merchant_info_exercises_request_body() {
    let h = Harness::new(vec![
        MockRoute::post(
            "/cgi-bin/component/api_authorizer_token",
            &[r#"{"authorizer_access_token":"auth_tok_merch","authorizer_refresh_token":"r","expires_in":7200}"#],
        ),
        MockRoute::post(
            "/product/register/submit_merchantinfo",
            &[r#"{"errcode":0,"errmsg":"ok"}"#],
        ),
    ])
    .await;
    h.config
        .set_authorizer_refresh_token(AUTHORIZER_APPID, "refresh_tok_merch");
    let _ = h
        .component
        .submit_merchant_info(
            AUTHORIZER_APPID,
            "1",
            &wx_rust_open::bean::MinishopBusiLicense::default(),
            None,
            None,
            None,
            Some("shop_short"),
        )
        .await;
    let req = h.server.requests().pop().unwrap();
    assert!(req.path.contains("access_token=auth_tok_merch"));
    let body = req_body_json(&req);
    assert_eq!(body["subject_type"], "1");
    assert_eq!(body["merchant_shortname"], "shop_short");
}

#[tokio::test]
async fn submit_basic_info_exercises_request_body() {
    let h = Harness::new(vec![
        MockRoute::post(
            "/cgi-bin/component/api_authorizer_token",
            &[r#"{"authorizer_access_token":"auth_tok_basic","authorizer_refresh_token":"r","expires_in":7200}"#],
        ),
        MockRoute::post(
            "/product/register/submit_basicinfo",
            &[r#"{"errcode":0,"errmsg":"ok"}"#],
        ),
    ])
    .await;
    h.config
        .set_authorizer_refresh_token(AUTHORIZER_APPID, "refresh_tok_basic");
    let name_info = wx_rust_open::bean::MinishopNameInfo {
        nick_name: "测试昵称".to_string(),
        abbr: "abbr".to_string(),
        introduction: "介绍".to_string(),
        naming_other_stuff: vec![],
    };
    let return_info = wx_rust_open::bean::MinishopReturnInfo::default();
    let _ = h
        .component
        .submit_basic_info(AUTHORIZER_APPID, &name_info, &return_info)
        .await;
    let req = h.server.requests().pop().unwrap();
    assert!(req.path.contains("access_token=auth_tok_basic"));
    let body = req_body_json(&req);
    assert_eq!(body["name_info"]["nickname"], "测试昵称");
}

// ═══════════════════════════════════════════════════════════════
// 7. get_minishop_brands / get_minishop_delivery_template
//    （手动解析 serde_json::Value，happy path 可达）
// ═══════════════════════════════════════════════════════════════

#[tokio::test]
async fn get_minishop_brands_parses() {
    let resp = r#"{"errcode":0,"errmsg":"ok","brands":[
      {"first_cat_id":1,"second_cat_id":2,"third_cat_id":3,
       "brand_info":{"brand_id":100,"brand_name":"品牌A"}}
    ]}"#;
    let h = Harness::new(vec![
        MockRoute::post(
            "/cgi-bin/component/api_authorizer_token",
            &[r#"{"authorizer_access_token":"auth_tok_brand","authorizer_refresh_token":"r","expires_in":7200}"#],
        ),
        MockRoute::post("/product/brand/get", &[resp]),
    ])
    .await;
    h.config
        .set_authorizer_refresh_token(AUTHORIZER_APPID, "refresh_tok_brand");
    let result = h
        .component
        .get_minishop_brands(AUTHORIZER_APPID)
        .await
        .unwrap();
    assert_eq!(result.errcode, 0);
    assert_eq!(result.brands.len(), 1);
    assert_eq!(result.brands[0].first_cat_id, 1);
    assert_eq!(result.brands[0].brand_info.brand_id, 100);
    assert_eq!(result.brands[0].brand_info.brand_name, "品牌A");
}

#[tokio::test]
async fn get_minishop_delivery_template_parses() {
    let resp = r#"{"errcode":0,"errmsg":"ok","template_list":[
      {"template_id":10,"name":"模板A","valuation_type":1},
      {"template_id":20,"name":"模板B","valuation_type":2}
    ]}"#;
    let h = Harness::new(vec![
        MockRoute::post(
            "/cgi-bin/component/api_authorizer_token",
            &[r#"{"authorizer_access_token":"auth_tok_tmpl","authorizer_refresh_token":"r","expires_in":7200}"#],
        ),
        MockRoute::post("/product/delivery/get_freight_template", &[resp]),
    ])
    .await;
    h.config
        .set_authorizer_refresh_token(AUTHORIZER_APPID, "refresh_tok_tmpl");
    let result = h
        .component
        .get_minishop_delivery_template(AUTHORIZER_APPID)
        .await
        .unwrap();
    assert_eq!(result.err_code, 0);
    assert_eq!(result.template_list.len(), 2);
    assert_eq!(result.template_list[0].template_id, 10);
    assert_eq!(
        result.template_list[0].valuation_type,
        wx_rust_open::bean::ValuationType::Weight
    );
    assert_eq!(
        result.template_list[1].valuation_type,
        wx_rust_open::bean::ValuationType::Package
    );
}

// ═══════════════════════════════════════════════════════════════
// 8. get_minishop_cat_list / get_minishop_delivery_company
// ═══════════════════════════════════════════════════════════════

#[tokio::test]
async fn get_minishop_cat_list_parses() {
    let resp = r#"{"errcode":0,"errmsg":"ok","shopcat_list":[
      {"shopcat_id":501,"shopcat_name":"服饰","f_shopcat_id":0,"cat_level":1}
    ]}"#;
    let h = Harness::new(vec![
        MockRoute::post(
            "/cgi-bin/component/api_authorizer_token",
            &[r#"{"authorizer_access_token":"auth_tok_cat","authorizer_refresh_token":"r","expires_in":7200}"#],
        ),
        MockRoute::post("/product/store/get_shopcat", &[resp]),
    ])
    .await;
    h.config
        .set_authorizer_refresh_token(AUTHORIZER_APPID, "refresh_tok_cat");
    let result = h
        .component
        .get_minishop_cat_list(AUTHORIZER_APPID)
        .await
        .unwrap();
    assert_eq!(result.errcode, 0);
    assert_eq!(result.shop_cat_list.len(), 1);
    assert_eq!(result.shop_cat_list[0].shop_cat_id, 501);
    assert_eq!(result.shop_cat_list[0].shop_cat_name, "服饰");
    assert_eq!(result.shop_cat_list[0].cat_level, 1);
}

#[tokio::test]
async fn get_minishop_delivery_company_parses() {
    let resp = r#"{"errcode":0,"errmsg":"ok","company_list":[{"id":1,"name":"顺丰"}]}"#;
    let h = Harness::new(vec![
        MockRoute::post(
            "/cgi-bin/component/api_authorizer_token",
            &[r#"{"authorizer_access_token":"auth_tok_deliv","authorizer_refresh_token":"r","expires_in":7200}"#],
        ),
        MockRoute::post("/product/delivery/get_company_list", &[resp]),
    ])
    .await;
    h.config
        .set_authorizer_refresh_token(AUTHORIZER_APPID, "refresh_tok_deliv");
    let result = h
        .component
        .get_minishop_delivery_company(AUTHORIZER_APPID)
        .await
        .unwrap();
    assert_eq!(result.errcode, 0);
    assert_eq!(result.data[0]["name"], "顺丰");
}

// ═══════════════════════════════════════════════════════════════
// 9. 优惠券操作（请求体断言 + error path）
// ═══════════════════════════════════════════════════════════════

#[tokio::test]
async fn minishop_push_coupon_to_user_exercises_request_body() {
    let h = Harness::new(vec![
        MockRoute::post(
            "/cgi-bin/component/api_authorizer_token",
            &[r#"{"authorizer_access_token":"auth_tok_push","authorizer_refresh_token":"r","expires_in":7200}"#],
        ),
        MockRoute::post("/product/coupon/push", &[r#"{"errcode":0,"errmsg":"ok"}"#]),
    ])
    .await;
    h.config
        .set_authorizer_refresh_token(AUTHORIZER_APPID, "refresh_tok_push");
    let _ = h
        .component
        .minishop_push_coupon_to_user(AUTHORIZER_APPID, "openid_push", 999)
        .await;
    let req = h.server.requests().pop().unwrap();
    let body = req_body_json(&req);
    assert_eq!(body["openid"], "openid_push");
    assert_eq!(body["coupon_id"], 999);
}

/// minishop_update_coupon：返回 i32，手动解析，happy path 可达。
#[tokio::test]
async fn minishop_update_coupon_posts() {
    let h = Harness::new(vec![
        MockRoute::post(
            "/cgi-bin/component/api_authorizer_token",
            &[r#"{"authorizer_access_token":"auth_tok_uc","authorizer_refresh_token":"r","expires_in":7200}"#],
        ),
        MockRoute::post(
            "/product/coupon/update",
            &[r#"{"errcode":0,"errmsg":"ok","data":{"coupon_id":777}}"#],
        ),
    ])
    .await;
    h.config
        .set_authorizer_refresh_token(AUTHORIZER_APPID, "refresh_tok_uc");
    let coupon = make_test_coupon();
    let coupon_id = h
        .component
        .minishop_update_coupon(AUTHORIZER_APPID, &coupon)
        .await
        .unwrap();
    assert_eq!(coupon_id, 777);
}

#[tokio::test]
async fn minishop_update_coupon_status_exercises_request_body() {
    let h = Harness::new(vec![
        MockRoute::post(
            "/cgi-bin/component/api_authorizer_token",
            &[r#"{"authorizer_access_token":"auth_tok_ucs","authorizer_refresh_token":"r","expires_in":7200}"#],
        ),
        MockRoute::post(
            "/product/coupon/update_status",
            &[r#"{"errcode":0,"errmsg":"ok"}"#],
        ),
    ])
    .await;
    h.config
        .set_authorizer_refresh_token(AUTHORIZER_APPID, "refresh_tok_ucs");
    let _ = h
        .component
        .minishop_update_coupon_status(AUTHORIZER_APPID, 555, 1)
        .await;
    let req = h.server.requests().pop().unwrap();
    let body = req_body_json(&req);
    assert_eq!(body["coupon_id"], 555);
    assert_eq!(body["status"], 1);
}

// ═══════════════════════════════════════════════════════════════
// 10. SPU 操作（返回 WxMinishopAddGoodsSpuResult errcode: i32）
// ═══════════════════════════════════════════════════════════════

#[tokio::test]
async fn minishop_goods_add_spu_posts() {
    let resp = r#"{"errcode":0,"errmsg":"ok","data":{"product_id":1001,"out_product_id":"out_1","create_time":1700000000}}"#;
    let h = Harness::new(vec![
        MockRoute::post(
            "/cgi-bin/component/api_authorizer_token",
            &[r#"{"authorizer_access_token":"auth_tok_spu","authorizer_refresh_token":"r","expires_in":7200}"#],
        ),
        MockRoute::post("/product/spu/add", &[resp]),
    ])
    .await;
    h.config
        .set_authorizer_refresh_token(AUTHORIZER_APPID, "refresh_tok_spu");
    let spu = make_test_spu();
    let result = h
        .component
        .minishop_goods_add_spu(AUTHORIZER_APPID, &spu)
        .await
        .unwrap();
    assert_eq!(result.errcode, 0);
    assert_eq!(result.data["product_id"], 1001);
    assert_eq!(result.data["create_time"], 1700000000);
}

/// minishop_goods_add_spu 错误路径：executor 对 errcode!=0 抛错。
#[tokio::test]
async fn minishop_goods_add_spu_error_path() {
    let h = Harness::new(vec![
        MockRoute::post(
            "/cgi-bin/component/api_authorizer_token",
            &[r#"{"authorizer_access_token":"auth_tok_spu_err","authorizer_refresh_token":"r","expires_in":7200}"#],
        ),
        MockRoute::post(
            "/product/spu/add",
            &[r#"{"errcode":1001,"errmsg":"product already exists"}"#],
        ),
    ])
    .await;
    h.config
        .set_authorizer_refresh_token(AUTHORIZER_APPID, "refresh_tok_spu_err");
    let spu = make_test_spu();
    let err = h
        .component
        .minishop_goods_add_spu(AUTHORIZER_APPID, &spu)
        .await
        .unwrap_err();
    assert_error_code(&err, 1001);
}

#[tokio::test]
async fn minishop_goods_del_spu_exercises_request_body() {
    let h = Harness::new(vec![
        MockRoute::post(
            "/cgi-bin/component/api_authorizer_token",
            &[r#"{"authorizer_access_token":"auth_tok_dspu","authorizer_refresh_token":"r","expires_in":7200}"#],
        ),
        MockRoute::post("/product/spu/del", &[r#"{"errcode":0,"errmsg":"ok"}"#]),
    ])
    .await;
    h.config
        .set_authorizer_refresh_token(AUTHORIZER_APPID, "refresh_tok_dspu");
    let _ = h
        .component
        .minishop_goods_del_spu(AUTHORIZER_APPID, 1001, 1002)
        .await;
    let req = h.server.requests().pop().unwrap();
    let body = req_body_json(&req);
    assert_eq!(body["product_id"], 1001);
    assert_eq!(body["out_product_id"], "1002");
}

#[tokio::test]
async fn minishop_goods_update_spu_posts() {
    let resp = r#"{"errcode":0,"errmsg":"ok","data":{"product_id":1003,"update_time":1700000001}}"#;
    let h = Harness::new(vec![
        MockRoute::post(
            "/cgi-bin/component/api_authorizer_token",
            &[r#"{"authorizer_access_token":"auth_tok_uspu","authorizer_refresh_token":"r","expires_in":7200}"#],
        ),
        MockRoute::post("/product/spu/update", &[resp]),
    ])
    .await;
    h.config
        .set_authorizer_refresh_token(AUTHORIZER_APPID, "refresh_tok_uspu");
    let spu = make_test_spu();
    let result = h
        .component
        .minishop_goods_update_spu(AUTHORIZER_APPID, &spu)
        .await
        .unwrap();
    assert_eq!(result.errcode, 0);
    assert_eq!(result.data["product_id"], 1003);
    assert_eq!(result.data["update_time"], 1700000001);
}

#[tokio::test]
async fn minishop_goods_listing_spu_exercises_request_body() {
    let h = Harness::new(vec![
        MockRoute::post(
            "/cgi-bin/component/api_authorizer_token",
            &[r#"{"authorizer_access_token":"auth_tok_lspu","authorizer_refresh_token":"r","expires_in":7200}"#],
        ),
        MockRoute::post("/product/spu/listing", &[r#"{"errcode":0,"errmsg":"ok"}"#]),
    ])
    .await;
    h.config
        .set_authorizer_refresh_token(AUTHORIZER_APPID, "refresh_tok_lspu");
    let _ = h
        .component
        .minishop_goods_listing_spu(AUTHORIZER_APPID, 2001, 2002)
        .await;
    let req = h.server.requests().pop().unwrap();
    let body = req_body_json(&req);
    assert_eq!(body["product_id"], 2001);
}

#[tokio::test]
async fn minishop_goods_delisting_spu_exercises_request_body() {
    let h = Harness::new(vec![
        MockRoute::post(
            "/cgi-bin/component/api_authorizer_token",
            &[r#"{"authorizer_access_token":"auth_tok_dlspu","authorizer_refresh_token":"r","expires_in":7200}"#],
        ),
        MockRoute::post("/product/spu/delisting", &[r#"{"errcode":0,"errmsg":"ok"}"#]),
    ])
    .await;
    h.config
        .set_authorizer_refresh_token(AUTHORIZER_APPID, "refresh_tok_dlspu");
    let _ = h
        .component
        .minishop_goods_delisting_spu(AUTHORIZER_APPID, 3001, 3002)
        .await;
    let req = h.server.requests().pop().unwrap();
    let body = req_body_json(&req);
    assert_eq!(body["product_id"], 3001);
}

// ═══════════════════════════════════════════════════════════════
// 11. SKU 操作
// ═══════════════════════════════════════════════════════════════

#[tokio::test]
async fn minishop_goods_add_sku_posts() {
    let resp = r#"{"errcode":0,"errmsg":"ok","data":{"sku_id":5001,"create_time":1700000002}}"#;
    let h = Harness::new(vec![
        MockRoute::post(
            "/cgi-bin/component/api_authorizer_token",
            &[r#"{"authorizer_access_token":"auth_tok_asku","authorizer_refresh_token":"r","expires_in":7200}"#],
        ),
        MockRoute::post("/product/sku/add", &[resp]),
    ])
    .await;
    h.config
        .set_authorizer_refresh_token(AUTHORIZER_APPID, "refresh_tok_asku");
    let sku = make_test_sku();
    let result = h
        .component
        .minishop_goods_add_sku(AUTHORIZER_APPID, &sku)
        .await
        .unwrap();
    assert_eq!(result.errcode, 0);
    assert_eq!(result.data["sku_id"], 5001);
}

#[tokio::test]
async fn minishop_goods_batch_add_sku_exercises_request_body() {
    let h = Harness::new(vec![
        MockRoute::post(
            "/cgi-bin/component/api_authorizer_token",
            &[r#"{"authorizer_access_token":"auth_tok_bsku","authorizer_refresh_token":"r","expires_in":7200}"#],
        ),
        MockRoute::post("/product/sku/batch_add", &[r#"{"errcode":0,"errmsg":"ok"}"#]),
    ])
    .await;
    h.config
        .set_authorizer_refresh_token(AUTHORIZER_APPID, "refresh_tok_bsku");
    let skus = vec![make_test_sku(), make_test_sku()];
    let _ = h
        .component
        .minishop_goods_batch_add_sku(AUTHORIZER_APPID, &skus)
        .await;
    let req = h.server.requests().pop().unwrap();
    let body = req_body_json(&req);
    assert!(body["skus"].as_array().unwrap().len() >= 2);
}

#[tokio::test]
async fn minishop_goods_del_sku_exercises_request_body() {
    let h = Harness::new(vec![
        MockRoute::post(
            "/cgi-bin/component/api_authorizer_token",
            &[r#"{"authorizer_access_token":"auth_tok_dsku","authorizer_refresh_token":"r","expires_in":7200}"#],
        ),
        MockRoute::post("/product/sku/del", &[r#"{"errcode":0,"errmsg":"ok"}"#]),
    ])
    .await;
    h.config
        .set_authorizer_refresh_token(AUTHORIZER_APPID, "refresh_tok_dsku");
    let _ = h
        .component
        .minishop_goods_del_sku(AUTHORIZER_APPID, 1001, 1002, "out_sku_1", 6001)
        .await;
    let req = h.server.requests().pop().unwrap();
    let body = req_body_json(&req);
    assert_eq!(body["product_id"], 1001);
    assert_eq!(body["sku_id"], 6001);
    assert_eq!(body["out_sku_id"], "out_sku_1");
}

#[tokio::test]
async fn minishop_goods_update_sku_exercises_request_body() {
    let h = Harness::new(vec![
        MockRoute::post(
            "/cgi-bin/component/api_authorizer_token",
            &[r#"{"authorizer_access_token":"auth_tok_usku","authorizer_refresh_token":"r","expires_in":7200}"#],
        ),
        MockRoute::post("/product/sku/update", &[r#"{"errcode":0,"errmsg":"ok"}"#]),
    ])
    .await;
    h.config
        .set_authorizer_refresh_token(AUTHORIZER_APPID, "refresh_tok_usku");
    let sku = make_test_sku();
    let _ = h
        .component
        .minishop_goods_update_sku(AUTHORIZER_APPID, &sku)
        .await;
    let req = h.server.requests().pop().unwrap();
    assert!(req.path.starts_with("/product/sku/update"));
    assert!(req.path.contains("access_token=auth_tok_usku"));
}

#[tokio::test]
async fn minishop_goods_update_sku_price_exercises_request_body() {
    let h = Harness::new(vec![
        MockRoute::post(
            "/cgi-bin/component/api_authorizer_token",
            &[r#"{"authorizer_access_token":"auth_tok_uskup","authorizer_refresh_token":"r","expires_in":7200}"#],
        ),
        MockRoute::post("/product/sku/update_price", &[r#"{"errcode":0,"errmsg":"ok"}"#]),
    ])
    .await;
    h.config
        .set_authorizer_refresh_token(AUTHORIZER_APPID, "refresh_tok_uskup");
    let _ = h
        .component
        .minishop_goods_update_sku_price(
            AUTHORIZER_APPID,
            1001,
            1002,
            "out_sku_2",
            7001,
            9900,
            19900,
        )
        .await;
    let req = h.server.requests().pop().unwrap();
    let body = req_body_json(&req);
    assert_eq!(body["sku_id"], 7001);
    // 镜像 Java 上游 bug：sale_price/market_price 均为 out_sku_id
    assert_eq!(body["sale_price"], "out_sku_2");
}

#[tokio::test]
async fn minishop_goods_update_sku_stock_exercises_request_body() {
    let h = Harness::new(vec![
        MockRoute::post(
            "/cgi-bin/component/api_authorizer_token",
            &[r#"{"authorizer_access_token":"auth_tok_uskus","authorizer_refresh_token":"r","expires_in":7200}"#],
        ),
        MockRoute::post("/product/stock/update", &[r#"{"errcode":0,"errmsg":"ok"}"#]),
    ])
    .await;
    h.config
        .set_authorizer_refresh_token(AUTHORIZER_APPID, "refresh_tok_uskus");
    let _ = h
        .component
        .minishop_goods_update_sku_stock(AUTHORIZER_APPID, 1001, 1002, "out_sku_3", 8001, 1, 500)
        .await;
    let req = h.server.requests().pop().unwrap();
    let body = req_body_json(&req);
    assert_eq!(body["type"], 1);
    assert_eq!(body["stock_num"], 500);
}

// ═══════════════════════════════════════════════════════════════
// 12. 限时折扣（手动解析，happy path 可达）
// ═══════════════════════════════════════════════════════════════

#[tokio::test]
async fn add_limit_discount_goods_posts() {
    let h = Harness::new(vec![
        MockRoute::post(
            "/cgi-bin/component/api_authorizer_token",
            &[r#"{"authorizer_access_token":"auth_tok_ld","authorizer_refresh_token":"r","expires_in":7200}"#],
        ),
        MockRoute::post(
            "/product/limiteddiscount/add/",
            &[r#"{"errcode":0,"errmsg":"ok","task_id":42}"#],
        ),
    ])
    .await;
    h.config
        .set_authorizer_refresh_token(AUTHORIZER_APPID, "refresh_tok_ld");
    let goods = wx_rust_open::bean::LimitDiscountGoods {
        product_id: 1001,
        start_time: "1700000000".to_string(),
        end_time: "1700086400".to_string(),
        limit_discount_sku_list: vec![wx_rust_open::bean::LimitDiscountSku {
            sku_id: 5001,
            sale_price: "99.99".to_string(),
            sale_stock: 100,
        }],
        ..Default::default()
    };
    let task_id = h
        .component
        .add_limit_discount_goods(AUTHORIZER_APPID, &goods)
        .await
        .unwrap();
    assert_eq!(task_id, 42);

    let req = h.server.requests().pop().unwrap();
    assert!(req.path.starts_with("/product/limiteddiscount/add/"));
    assert!(req.path.contains("access_token=auth_tok_ld"));
}

#[tokio::test]
async fn get_limit_discount_list_parses() {
    let resp = r#"{"errcode":0,"errmsg":"ok","limited_discount_list":[
      {"task_id":1,"status":1,"start_time":1700000,"end_time":1700086,
       "limited_discount_sku_list":[{"sku_id":5001,"sale_price":9999,"sale_stock":50}]}
    ]}"#;
    let h = Harness::new(vec![
        MockRoute::post(
            "/cgi-bin/component/api_authorizer_token",
            &[r#"{"authorizer_access_token":"auth_tok_gld","authorizer_refresh_token":"r","expires_in":7200}"#],
        ),
        MockRoute::post("/product/limiteddiscount/get_list/", &[resp]),
    ])
    .await;
    h.config
        .set_authorizer_refresh_token(AUTHORIZER_APPID, "refresh_tok_gld");
    let list = h
        .component
        .get_limit_discount_list(AUTHORIZER_APPID, Some(1))
        .await
        .unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].task_id, 1);
    assert_eq!(list[0].limit_discount_sku_list.len(), 1);

    let req = h.server.requests().pop().unwrap();
    let body = req_body_json(&req);
    assert_eq!(body["status"], 1);
}

#[tokio::test]
async fn get_limit_discount_list_empty() {
    let h = Harness::new(vec![
        MockRoute::post(
            "/cgi-bin/component/api_authorizer_token",
            &[r#"{"authorizer_access_token":"auth_tok_gld_e","authorizer_refresh_token":"r","expires_in":7200}"#],
        ),
        MockRoute::post(
            "/product/limiteddiscount/get_list/",
            &[r#"{"errcode":0,"errmsg":"ok","limited_discount_list":[]}"#],
        ),
    ])
    .await;
    h.config
        .set_authorizer_refresh_token(AUTHORIZER_APPID, "refresh_tok_gld_e");
    let list = h
        .component
        .get_limit_discount_list(AUTHORIZER_APPID, None)
        .await
        .unwrap();
    assert!(list.is_empty());
}

#[tokio::test]
async fn update_limit_discount_status_exercises_request_body() {
    let h = Harness::new(vec![
        MockRoute::post(
            "/cgi-bin/component/api_authorizer_token",
            &[r#"{"authorizer_access_token":"auth_tok_ulds","authorizer_refresh_token":"r","expires_in":7200}"#],
        ),
        MockRoute::post(
            "/product/limiteddiscount/update_status/",
            &[r#"{"errcode":0,"errmsg":"ok"}"#],
        ),
    ])
    .await;
    h.config
        .set_authorizer_refresh_token(AUTHORIZER_APPID, "refresh_tok_ulds");
    let _ = h
        .component
        .update_limit_discount_status(AUTHORIZER_APPID, 100, 2)
        .await;
    let req = h.server.requests().pop().unwrap();
    let body = req_body_json(&req);
    assert_eq!(body["task_id"], 100);
    assert_eq!(body["status"], 2);
}

// ═══════════════════════════════════════════════════════════════
// 13. TCB（云开发）
// ═══════════════════════════════════════════════════════════════

#[tokio::test]
async fn get_share_cloud_base_env_posts() {
    let h = Harness::new(vec![MockRoute::post(
        "/componenttcb/batchgetenvid",
        &[r#"{"relation_data":[{"appid":"wx_app_1","env_list":["env_1"]}],"err_list":[]}"#],
    )])
    .await;
    let result = h
        .component
        .get_share_cloud_base_env(&["wx_app_1".to_string()])
        .await
        .unwrap();
    assert_eq!(result.relation_data.len(), 1);
    assert_eq!(result.relation_data[0].appid, "wx_app_1");
}

#[tokio::test]
async fn get_tcb_env_list_posts() {
    let h = Harness::new(vec![MockRoute::post(
        "/componenttcb/describeenvs",
        &[r#"{"info_list":[{"env":"env_desc_1","alias":"test","status":"normal"}]}"#],
    )])
    .await;
    let result = h.component.get_tcb_env_list().await.unwrap();
    assert_eq!(result.info_list.len(), 1);
    assert_eq!(result.info_list[0].env, "env_desc_1");
}

#[tokio::test]
async fn change_tcb_env_exercises_request_body() {
    let h = Harness::new(vec![MockRoute::post(
        "/tcb/modifyenv",
        &[r#"{"errcode":0,"errmsg":"ok"}"#],
    )])
    .await;
    let _ = h.component.change_tcb_env("env_change_1").await;
    let req = h.server.requests().pop().unwrap();
    assert_eq!(req_body_json(&req)["env"], "env_change_1");
}

/// share_cloud_base_env：executor 需要 integer errcode，bean 需要 String。
/// 测试请求体断言。
#[tokio::test]
async fn share_cloud_base_env_exercises_request_body() {
    let h = Harness::new(vec![MockRoute::post(
        "/componenttcb/batchshareenv",
        &[r#"{"errcode":0,"errmsg":"ok","err_list":[]}"#],
    )])
    .await;
    let request = wx_rust_open::bean::ShareCloudBaseEnvRequest {
        data: vec![wx_rust_open::bean::DataDTO {
            env: "env_share_1".to_string(),
            appids: vec!["wx_share_1".to_string()],
        }],
        source_type: 1,
    };
    let _ = h.component.share_cloud_base_env(&request).await;
    let req = h.server.requests().pop().unwrap();
    let body = req_body_json(&req);
    assert_eq!(body["data"][0]["env"], "env_share_1");
}

// ═══════════════════════════════════════════════════════════════
// 14. clear_quota_v2
// ═══════════════════════════════════════════════════════════════

#[tokio::test]
async fn clear_quota_v2_exercises_request_body() {
    let h = Harness::new(vec![MockRoute::post(
        "/cgi-bin/component/clear_quota/v2",
        &[r#"{"errcode":0,"errmsg":"ok"}"#],
    )])
    .await;
    let _ = h.component.clear_quota_v2("wx_clear_01").await;
    let req = h.server.requests().pop().unwrap();
    let body = req_body_json(&req);
    assert_eq!(body["appid"], "wx_clear_01");
    assert_eq!(body["component_appid"], "component_appid_01");
}

// ═══════════════════════════════════════════════════════════════
// 15. apply_set_order_path_info
// ═══════════════════════════════════════════════════════════════

#[tokio::test]
async fn apply_set_order_path_info_exercises_request_body() {
    let h = Harness::new(vec![MockRoute::post(
        "/wxa/security/applysetorderpathinfo",
        &[r#"{"errcode":0,"errmsg":"ok"}"#],
    )])
    .await;
    let info = wx_rust_open::bean::WxOpenMaApplyOrderPathInfo {
        batch_req: wx_rust_open::bean::BatchReqBean {
            path: "/pages/order/list".to_string(),
            app_id_list: vec!["wx_order_app".to_string()],
            ..Default::default()
        },
    };
    let _ = h.component.apply_set_order_path_info(&info).await;
    let req = h.server.requests().pop().unwrap();
    let body = req_body_json(&req);
    assert_eq!(body["batch_req"]["path"], "/pages/order/list");
}

// ═══════════════════════════════════════════════════════════════
// 16. 域名管理
// ═══════════════════════════════════════════════════════════════

#[tokio::test]
async fn modify_wxa_server_domain_add_exercises_request_body() {
    let h = Harness::new(vec![MockRoute::post(
        "/cgi-bin/component/modify_wxa_server_domain",
        &[r#"{"errcode":0,"errmsg":"ok"}"#],
    )])
    .await;
    let _ = h
        .component
        .modify_wxa_server_domain(
            "add",
            &["https://a.example.com".to_string()],
            &["wss://b.example.com".to_string()],
            &["https://c.example.com".to_string()],
            &["https://d.example.com".to_string()],
            &[],
            &[],
        )
        .await;
    let req = h.server.requests().pop().unwrap();
    let body = req_body_json(&req);
    assert_eq!(body["action"], "add");
    assert_eq!(body["requestdomain"][0], "https://a.example.com");
}

#[tokio::test]
async fn modify_wxa_server_domain_get_exercises_request_body() {
    let h = Harness::new(vec![MockRoute::post(
        "/cgi-bin/component/modify_wxa_server_domain",
        &[r#"{"errcode":0,"errmsg":"ok","requestdomain":["https://e.example.com"]}"#],
    )])
    .await;
    let _ = h
        .component
        .modify_wxa_server_domain("get", &[], &[], &[], &[], &[], &[])
        .await;
    let req = h.server.requests().pop().unwrap();
    let body = req_body_json(&req);
    assert_eq!(body["action"], "get");
    assert!(body.get("requestdomain").is_none());
}

/// get_domain_confirm_file：executor 需要 integer errcode，bean 需要 String errcode。
/// 测试请求体断言 + 忽略反序列化结果。
#[tokio::test]
async fn get_domain_confirm_file_exercises_request_body() {
    let h = Harness::new(vec![MockRoute::post(
        "/cgi-bin/component/get_domain_confirmfile",
        &[r#"{"errcode":0,"errmsg":"ok","file_name":"verify.txt","file_content":"abc123"}"#],
    )])
    .await;
    let _ = h.component.get_domain_confirm_file().await;
    // 请求已发出（覆盖 URL 构建 + post 调用链）
    assert_eq!(
        h.server
            .call_count("/cgi-bin/component/get_domain_confirmfile"),
        1
    );
}

/// modify_wxa_jump_domain：返回 String，happy path 可达。
#[tokio::test]
async fn modify_wxa_jump_domain_add_posts() {
    let h = Harness::new(vec![MockRoute::post(
        "/cgi-bin/component/modify_wxa_jump_domain",
        &[r#"{"errcode":0,"errmsg":"ok"}"#],
    )])
    .await;
    let result = h
        .component
        .modify_wxa_jump_domain("add", &["https://biz.example.com".to_string()])
        .await
        .unwrap();
    assert!(result.contains("errcode"));

    let req = h.server.requests().pop().unwrap();
    let body = req_body_json(&req);
    assert_eq!(body["action"], "add");
    assert_eq!(body["webviewdomain"][0], "https://biz.example.com");
}

#[tokio::test]
async fn modify_wxa_jump_domain_get_posts() {
    let h = Harness::new(vec![MockRoute::post(
        "/cgi-bin/component/modify_wxa_jump_domain",
        &[r#"{"errcode":0,"errmsg":"ok","webviewdomain":["https://x.com"]}"#],
    )])
    .await;
    let result = h
        .component
        .modify_wxa_jump_domain("get", &[])
        .await
        .unwrap();
    assert!(result.contains("webviewdomain"));

    let req = h.server.requests().pop().unwrap();
    let body = req_body_json(&req);
    assert_eq!(body["action"], "get");
    assert!(body.get("webviewdomain").is_none());
}

/// modify_wxa_jump_domain_info：executor 需要 integer errcode，bean 需要 String。
/// 测试请求体断言。
#[tokio::test]
async fn modify_wxa_jump_domain_info_exercises_request_body() {
    let h = Harness::new(vec![MockRoute::post(
        "/cgi-bin/component/modify_wxa_jump_domain",
        &[r#"{"errcode":0,"errmsg":"ok","webviewdomain":["https://y.com"]}"#],
    )])
    .await;
    let _ = h.component.modify_wxa_jump_domain_info("get", &[]).await;
    assert_eq!(
        h.server
            .call_count("/cgi-bin/component/modify_wxa_jump_domain"),
        1
    );
}

// ═══════════════════════════════════════════════════════════════
// 17. get_authorizer_option / set_authorizer_option
// ═══════════════════════════════════════════════════════════════

#[tokio::test]
async fn get_authorizer_option_posts() {
    let h = Harness::new(vec![
        MockRoute::post(
            "/cgi-bin/component/api_authorizer_token",
            &[r#"{"authorizer_access_token":"auth_tok_opt","authorizer_refresh_token":"r","expires_in":7200}"#],
        ),
        MockRoute::post(
            "/cgi-bin/component/get_authorizer_option",
            &[r#"{"authorizer_appid":"wx_opt","option_name":"opt_n","option_value":"option_val_1"}"#],
        ),
    ])
    .await;
    h.config
        .set_authorizer_refresh_token(AUTHORIZER_APPID, "refresh_tok_opt");
    let result = h
        .component
        .get_authorizer_option(AUTHORIZER_APPID, "option_name_1")
        .await
        .unwrap();
    assert_eq!(result.option_value.as_deref(), Some("option_val_1"));

    let req = h.server.requests().pop().unwrap();
    assert!(req.path.contains("access_token=auth_tok_opt"));
    let body = req_body_json(&req);
    assert_eq!(body["option_name"], "option_name_1");
}

#[tokio::test]
async fn set_authorizer_option_posts() {
    let h = Harness::new(vec![
        MockRoute::post(
            "/cgi-bin/component/api_authorizer_token",
            &[r#"{"authorizer_access_token":"auth_tok_setopt","authorizer_refresh_token":"r","expires_in":7200}"#],
        ),
        MockRoute::post(
            "/cgi-bin/component/set_authorizer_option",
            &[r#"{"errcode":0,"errmsg":"ok"}"#],
        ),
    ])
    .await;
    h.config
        .set_authorizer_refresh_token(AUTHORIZER_APPID, "refresh_tok_setopt");
    h.component
        .set_authorizer_option(AUTHORIZER_APPID, "opt_name", "opt_val")
        .await
        .unwrap();

    let req = h.server.requests().pop().unwrap();
    let body = req_body_json(&req);
    assert_eq!(body["option_name"], "opt_name");
    assert_eq!(body["option_value"], "opt_val");
}

// ═══════════════════════════════════════════════════════════════
// 18. have_open
// ═══════════════════════════════════════════════════════════════

/// have_open：executor 需要 integer errcode，bean 需要 String errcode。
/// 测试请求体断言。
#[tokio::test]
async fn have_open_exercises_request_body() {
    let h = Harness::new(vec![MockRoute::get(
        "/cgi-bin/open/have",
        &[r#"{"errcode":0,"errmsg":"ok","have_open":true}"#],
    )])
    .await;
    let _ = h.component.have_open().await;
    assert_eq!(h.server.call_count("/cgi-bin/open/have"), 1);
}

// ═══════════════════════════════════════════════════════════════
// 19. get_component_access_token
// ═══════════════════════════════════════════════════════════════

#[tokio::test]
async fn get_component_access_token_direct() {
    let h = Harness::new(vec![]).await;
    let token = h.component.get_component_access_token(false).await.unwrap();
    assert_eq!(token, "comp_tok_v1");

    let token2 = h.component.get_component_access_token(true).await.unwrap();
    assert_eq!(token2, "comp_tok_v1");
    assert_eq!(
        h.server
            .call_count("/cgi-bin/component/api_component_token"),
        2
    );
}

// ═══════════════════════════════════════════════════════════════
// 20. route: updateauthorized 事件
// ═══════════════════════════════════════════════════════════════

#[tokio::test]
async fn route_updateauthorized_dispatches() {
    use wx_rust_open::bean::message::WxOpenXmlMessage;

    let resp = r#"{"authorization_info":{
      "authorizer_appid":"wx_update_auth_01",
      "authorizer_access_token":"auth_tok_update",
      "expires_in":7200,
      "authorizer_refresh_token":"refresh_tok_update",
      "func_info":[]
    }}"#;
    let h = Harness::new(vec![MockRoute::post(
        "/cgi-bin/component/api_query_auth",
        &[resp],
    )])
    .await;
    let message = WxOpenXmlMessage {
        info_type: Some("updateauthorized".to_string()),
        authorization_code: Some("queryauthcode@@@UPDATE01".to_string()),
        ..Default::default()
    };
    let ret = h.component.route(&message).await.unwrap();
    assert_eq!(ret, "success");
    assert_eq!(
        h.config
            .authorizer_access_token("wx_update_auth_01")
            .as_deref(),
        Some("auth_tok_update")
    );
}

// ═══════════════════════════════════════════════════════════════
// 21. 错误路径覆盖
// ═══════════════════════════════════════════════════════════════

#[tokio::test]
async fn get_minishop_categories_error_path() {
    let h = Harness::new(vec![
        MockRoute::post(
            "/cgi-bin/component/api_authorizer_token",
            &[r#"{"authorizer_access_token":"auth_tok_cat_err","authorizer_refresh_token":"r","expires_in":7200}"#],
        ),
        MockRoute::post(
            "/product/category/get",
            &[r#"{"errcode":40001,"errmsg":"invalid access_token"}"#],
        ),
    ])
    .await;
    h.config
        .set_authorizer_refresh_token(AUTHORIZER_APPID, "refresh_tok_cat_err");
    let err = h
        .component
        .get_minishop_categories(AUTHORIZER_APPID, 0)
        .await
        .unwrap_err();
    assert_error_code(&err, 40001);
}

#[tokio::test]
async fn get_minishop_delivery_template_error_path() {
    let h = Harness::new(vec![
        MockRoute::post(
            "/cgi-bin/component/api_authorizer_token",
            &[r#"{"authorizer_access_token":"auth_tok_dterr","authorizer_refresh_token":"r","expires_in":7200}"#],
        ),
        MockRoute::post(
            "/product/delivery/get_freight_template",
            &[r#"{"errcode":40001,"errmsg":"invalid"}"#],
        ),
    ])
    .await;
    h.config
        .set_authorizer_refresh_token(AUTHORIZER_APPID, "refresh_tok_dterr");
    let err = h
        .component
        .get_minishop_delivery_template(AUTHORIZER_APPID)
        .await
        .unwrap_err();
    assert_error_code(&err, 40001);
}

#[tokio::test]
async fn minishop_create_coupon_error_path() {
    let h = Harness::new(vec![
        MockRoute::post(
            "/cgi-bin/component/api_authorizer_token",
            &[r#"{"authorizer_access_token":"auth_tok_cc_err","authorizer_refresh_token":"r","expires_in":7200}"#],
        ),
        MockRoute::post(
            "/product/coupon/create",
            &[r#"{"errcode":1001,"errmsg":"coupon limit exceeded"}"#],
        ),
    ])
    .await;
    h.config
        .set_authorizer_refresh_token(AUTHORIZER_APPID, "refresh_tok_cc_err");
    let coupon = make_test_coupon();
    let err = h
        .component
        .minishop_create_coupon(AUTHORIZER_APPID, &coupon)
        .await
        .unwrap_err();
    assert_error_code(&err, 1001);
}

// ═══════════════════════════════════════════════════════════════
// 辅助构造函数
// ═══════════════════════════════════════════════════════════════

fn make_test_coupon() -> wx_rust_open::bean::WxMinishopCoupon {
    wx_rust_open::bean::WxMinishopCoupon {
        coupon_id: -1,
        status: -1,
        r#type: 1,
        name: "测试券".to_string(),
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
    }
}

fn make_test_spu() -> wx_rust_open::bean::WxMinishopSpu {
    wx_rust_open::bean::WxMinishopSpu {
        out_product_id: "out_prod_1".to_string(),
        title: "测试商品".to_string(),
        sub_title: "副标题".to_string(),
        head_imgs: vec!["https://img.example.com/1.jpg".to_string()],
        desc_info_imgs: vec!["https://img.example.com/desc.jpg".to_string()],
        brand_id: 0,
        shop_cats: vec![],
        attrs: vec![],
        model: "model_1".to_string(),
        express_template_id: 0,
        skus: vec![make_test_sku()],
    }
}

fn make_test_sku() -> wx_rust_open::bean::WxMinishopSku {
    wx_rust_open::bean::WxMinishopSku {
        product_id: 0,
        out_product_id: "out_prod_1".to_string(),
        out_sku_id: "out_sku_1".to_string(),
        thumb_img: "https://img.example.com/thumb.jpg".to_string(),
        sale_price: 9999,
        market_price: 19999,
        stock_num: 100,
        sku_code: "SKU001".to_string(),
        bar_code: "BAR001".to_string(),
        sku_attrs: vec![wx_rust_open::bean::WxMinishopGoodsSkuAttr {
            attr_key: "颜色".to_string(),
            attr_value: "红色".to_string(),
        }],
    }
}
