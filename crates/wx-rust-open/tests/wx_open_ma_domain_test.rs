//! Ma*/Minishop 子域服务与 oauth2 集成测试（自含 MockServer 模式，与
//! wx_open_component_service_test.rs / wx_open_mp_ma_bridge_test.rs 同一
//! MockServer 实现）。
//!
//! 覆盖（Wave 5）：
//! - 代 ma 桥接七个子服务 getter（对应 Java `WxOpenMaServiceImpl` 构造器
//!   装配的 `@Getter final` 子服务字段）；
//! - MaAuth：`submit`（POST `/wxa/sec/wxaauth` 请求体 + 响应解析）；
//! - MaBasic：`getAccountBasicInfo`（GET `/cgi-bin/account/getaccountbasicinfo`）
//!   与 `getComponentRebindAdminUrl`（URLEncoder 语义编码 + 格式化串）；
//! - MaEmbedded：`getOwnListWith` 分页参数（null 默认值 / num 截断 1000）；
//! - MaIcp：`queryIcpVerifyTask`（POST `/wxa/icp/query_icp_verifytask`）；
//! - MaPrivacy：`getPrivacySetting`（POST `/cgi-bin/component/getprivacysetting`，
//!   privacy_ver 可空）；
//! - MaShoppingOrders：`uploadShoppingInfo`（POST `/user-order/orders`）；
//! - Minishop：`getWxMinishopServiceByAppid` 装配 + `submitMerchantInfo`
//!   （snake_case 请求体 + Java 桩 `Ok(None)` 镜像）；
//! - WxOpenOAuth2ServiceImpl：裸 GET `/sns/oauth2/access_token`（无
//!   component_access_token 注入，镜像 Java `WxOpenServiceAbstractImpl.get`）；
//! - WxOpenMpOAuth2ServiceImpl：component 链路 GET
//!   `/sns/oauth2/component/access_token`（注入 component_access_token，
//!   镜像 Java `wxOpenComponentService.get(url)`）与授权链接格式化；
//! - PrivacyKeyEnum：key/desc/serde rename 对照 Java 常量。

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use wx_rust_common::service::WxOAuth2Service;
use wx_rust_open::api::r#impl::{
    WxOpenMaService, WxOpenMpService, WxOpenOAuth2ServiceImpl, WxOpenServiceImpl,
    downcast_ma_service, downcast_mp_service,
};
use wx_rust_open::api::{
    WxOpenComponentService, WxOpenMaAuthAndIcpService, WxOpenMaAuthService, WxOpenMaBasicService,
    WxOpenMaEmbeddedService, WxOpenMaIcpService, WxOpenMaPrivacyService,
    WxOpenMaShoppingOrdersService, WxOpenMinishopService, WxOpenService,
};
use wx_rust_open::bean::ma::privacy::PrivacyKeyEnum;
use wx_rust_open::bean::{
    MaAuthSubmitParam, MaAuthSubmitParamAuthData, MaAuthSubmitParamContactInfo,
    MaAuthSubmitParamInvoiceInfo, ShoppingInfo,
};
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
    service: Arc<WxOpenServiceImpl>,
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
        let service = WxOpenServiceImpl::new_arc(config.clone());
        Self {
            server,
            config,
            service,
        }
    }

    fn component(&self) -> Arc<dyn WxOpenComponentService> {
        self.service.wx_open_component_service().unwrap()
    }

    fn set_authorizer_token(&self, app_id: &str, token: &str) {
        self.config
            .update_authorizer_access_token_with_expiry(app_id, token, 7200);
    }

    /// 代 ma 桥接服务（按具体类型下转）。
    fn ma_bridge(&self, app_id: &str) -> Arc<WxOpenMaService> {
        let any = self.component().get_wx_ma_service_by_appid(app_id).unwrap();
        any.downcast::<WxOpenMaService>().unwrap()
    }

    /// 代 mp 桥接服务（按具体类型下转）。
    fn mp_bridge(&self, app_id: &str) -> Arc<WxOpenMpService> {
        let any = self.component().get_wx_mp_service_by_appid(app_id).unwrap();
        any.downcast::<WxOpenMpService>().unwrap()
    }
}

fn req_body_json(req: &RecordedRequest) -> serde_json::Value {
    serde_json::from_str(&req.body).unwrap_or(serde_json::Value::Null)
}

/// 授权方 appid（测试常量）。
const APP_ID_MA: &str = "authorizer_appid_ma_01";
const APP_ID_MP: &str = "authorizer_appid_mp_01";

// ---------------------------------------------------------------------------
// 测试
// ---------------------------------------------------------------------------

/// 代 ma 桥接装配七个子服务 getter（对应 Java `WxOpenMaServiceImpl`
/// 构造器 `@Getter final` 字段），且子服务请求经组件按 appid 取回同一
/// 桥接实例（双检锁缓存）。
#[tokio::test]
async fn ma_bridge_exposes_seven_sub_services() {
    let h = Harness::new(vec![]).await;
    let ma = h.ma_bridge(APP_ID_MA);
    // 七个 getter 均可取（Java `getBasicService`/`getAuthService`/
    // `getIcpService`/`getPrivacyService`/`getShoppingOrdersService`/
    // `getEmbeddedService`/`getAuthAndIcpService`）
    assert_eq!(ma.get_basic_service().app_id(), APP_ID_MA);
    assert_eq!(ma.get_auth_service().app_id(), APP_ID_MA);
    assert_eq!(ma.get_icp_service().app_id(), APP_ID_MA);
    assert_eq!(ma.get_privacy_service().app_id(), APP_ID_MA);
    assert_eq!(ma.get_shopping_orders_service().app_id(), APP_ID_MA);
    assert_eq!(ma.get_embedded_service().app_id(), APP_ID_MA);
    assert_eq!(ma.get_auth_and_icp_service().app_id(), APP_ID_MA);
    // 代 mp 桥接经 new_arc 装配 oauth2 服务（对应 Java 构造器
    // setOAuth2Service）
    let mp = h.mp_bridge(APP_ID_MP);
    assert!(mp.oauth2_service().is_some());
    // downcast 辅助仍可用（兼容既有调用方）
    let any = h.component().get_wx_ma_service_by_appid(APP_ID_MA).unwrap();
    assert!(downcast_ma_service(any).is_some());
    let any_mp = h.component().get_wx_mp_service_by_appid(APP_ID_MP).unwrap();
    assert!(downcast_mp_service(any_mp).is_some());
}

/// MaAuth.submit：POST `/wxa/sec/wxaauth`，请求体含 auth_data，响应解析
/// taskid/auth_url（对应 Java `WxOpenMaAuthServiceImpl.submit`）。
#[tokio::test]
async fn ma_auth_submit_posts_wxaauth_and_parses() {
    let h = Harness::new(vec![MockRoute::post(
        "/wxa/sec/wxaauth",
        &[r#"{"errcode":0,"errmsg":"ok","taskid":"task_1","auth_url":"https://x/verify"}"#],
    )])
    .await;
    h.set_authorizer_token(APP_ID_MA, "auth_tok_ma");
    let auth = h.ma_bridge(APP_ID_MA).get_auth_service();

    let param = MaAuthSubmitParam {
        auth_data: MaAuthSubmitParamAuthData {
            customer_type: 1,
            contact_info: MaAuthSubmitParamContactInfo::default(),
            invoice_info: MaAuthSubmitParamInvoiceInfo::default(),
            qualification: "qual".to_string(),
            ..Default::default()
        },
    };
    let result = auth.submit(&param).await.expect("submit 应成功");
    assert_eq!(result.task_id, "task_1");
    assert_eq!(result.auth_url, "https://x/verify");

    let requests = h.server.requests();
    let req = requests.last().unwrap();
    assert_eq!(req.method, "POST");
    assert!(req.path.starts_with("/wxa/sec/wxaauth"));
    let body = req_body_json(req);
    assert_eq!(body["auth_data"]["customer_type"], 1);
    assert_eq!(body["auth_data"]["qualification"], "qual");
}

/// MaBasic.getAccountBasicInfo：GET `/cgi-bin/account/getaccountbasicinfo`
/// 并解析基本信息（对应 Java `WxOpenMaBasicServiceImpl.getAccountBasicInfo`）。
#[tokio::test]
async fn ma_basic_get_account_basic_info() {
    let h = Harness::new(vec![MockRoute::get(
        "/cgi-bin/account/getaccountbasicinfo",
        &[r#"{"errcode":0,"errmsg":"ok","appid":"wxa_app","name":"demo"}"#],
    )])
    .await;
    h.set_authorizer_token(APP_ID_MA, "auth_tok_ma");
    let basic = h.ma_bridge(APP_ID_MA).get_basic_service();
    let result = basic.get_account_basic_info().await.expect("应成功");
    assert_eq!(result.app_id, "wxa_app");

    let requests = h.server.requests();
    let req = requests.last().unwrap();
    assert_eq!(req.method, "GET");
    assert!(req.path.starts_with("/cgi-bin/account/getaccountbasicinfo"));
    // authorizer access_token 注入（代 ma 执行引擎）
    assert!(req.path.contains("access_token=auth_tok_ma"));
}

/// MaBasic.getComponentRebindAdminUrl：URLEncoder（空格 → `+`）编码 +
/// `URL_COMPONENT_REBIND_ADMIN` 格式化（对应 Java 同名方法）。
#[tokio::test]
async fn ma_basic_get_component_rebind_admin_url_encodes() {
    let h = Harness::new(vec![]).await;
    let basic = h.ma_bridge(APP_ID_MA).get_basic_service();
    let url =
        basic.get_component_rebind_admin_url("https://example.com/redirect?a=1 2", "mp_appid_01");
    // URLEncoder.encode：空格 → `+`
    assert!(url.contains("redirect_uri=https%3A%2F%2Fexample.com%2Fredirect%3Fa%3D1+2"));
    assert!(url.contains("appid=mp_appid_01"));
    assert!(url.contains("component_appid=component_appid_01"));
    assert!(url.starts_with("https://mp.weixin.qq.com/wxopen/componentrebindadmin?"));
}

/// MaEmbedded.getOwnListWith：分页参数默认值（null → 0/10）与 num 截断
/// 1000（对应 Java `getOwnList(Integer start, Integer num)`）。
#[tokio::test]
async fn ma_embedded_get_own_list_with_pagination() {
    let h = Harness::new(vec![MockRoute::get(
        "/wxaapi/wxaembedded/get_own_list",
        &[r#"{"errcode":0,"errmsg":"ok","wxa_embedded_list":[{"appid":"embed_1"}]}"#],
    )])
    .await;
    h.set_authorizer_token(APP_ID_MA, "auth_tok_ma");
    let embedded = h.ma_bridge(APP_ID_MA).get_embedded_service();

    // 无参：GET ?num=1000（Java `getOwnList()`）
    let r1 = embedded.get_own_list().await.expect("应成功");
    assert_eq!(r1.embedded_list.len(), 1);
    let requests = h.server.requests();
    let req = requests.last().unwrap();
    assert!(
        req.path
            .contains("/wxaapi/wxaembedded/get_own_list?num=1000")
    );

    // 带参：start/num 显式传值
    let _ = embedded
        .get_own_list_with(Some(2), Some(100))
        .await
        .expect("应成功");
    let requests = h.server.requests();
    let req = requests.last().unwrap();
    assert!(req.path.contains("?start=2&num=100"));

    // num > 1000 截断为 1000（Java `if (num > 1000) num = 1000`）
    let _ = embedded
        .get_own_list_with(None, Some(99999))
        .await
        .expect("应成功");
    let requests = h.server.requests();
    let req = requests.last().unwrap();
    assert!(req.path.contains("?start=0&num=1000"));
}

/// MaIcp.queryIcpVerifyTask：POST `/wxa/icp/query_icp_verifytask`，
/// 请求体 task_id、响应解析（对应 Java `WxOpenMaIcpServiceImpl`）。
#[tokio::test]
async fn ma_icp_query_verify_task_posts() {
    let h = Harness::new(vec![MockRoute::post(
        "/wxa/icp/query_icp_verifytask",
        &[r#"{"errcode":0,"errmsg":"ok","task_id":"t1","is_finish":true}"#],
    )])
    .await;
    h.set_authorizer_token(APP_ID_MA, "auth_tok_ma");
    let icp = h.ma_bridge(APP_ID_MA).get_icp_service();
    let result = icp.query_icp_verify_task("t1").await.expect("应成功");
    assert!(result.finish);

    let requests = h.server.requests();
    let req = requests.last().unwrap();
    assert!(req.path.starts_with("/wxa/icp/query_icp_verifytask"));
    let body = req_body_json(req);
    assert_eq!(body["task_id"], "t1");
}

/// MaPrivacy.getPrivacySetting：POST `/cgi-bin/component/getprivacysetting`，
/// privacy_ver 为 null 时不携带该字段（对应 Java
/// `getPrivacySetting(Integer privacyVer)`）。
#[tokio::test]
async fn ma_privacy_get_setting_omits_null_ver() {
    let h = Harness::new(vec![MockRoute::post(
        "/cgi-bin/component/getprivacysetting",
        &[r#"{"errcode":0,"errmsg":"ok","code_exist":1,"setting_list":[]}"#],
    )])
    .await;
    h.set_authorizer_token(APP_ID_MA, "auth_tok_ma");
    let privacy = h.ma_bridge(APP_ID_MA).get_privacy_service();

    let _ = privacy.get_privacy_setting(None).await.expect("应成功");
    let requests = h.server.requests();
    let req = requests.last().unwrap();
    let body = req_body_json(req);
    assert!(body.get("privacy_ver").is_none());

    let _ = privacy.get_privacy_setting(Some(1)).await.expect("应成功");
    let requests = h.server.requests();
    let req = requests.last().unwrap();
    let body = req_body_json(req);
    assert_eq!(body["privacy_ver"], 1);
}

/// MaShoppingOrders.uploadShoppingInfo：POST `/user-order/orders`，请求体
/// 序列化（对应 Java `WxOpenMaShoppingOrdersServiceImpl.upload`）。
#[tokio::test]
async fn ma_shopping_orders_upload_posts() {
    let h = Harness::new(vec![MockRoute::post(
        "/user-order/orders",
        &[r#"{"errcode":0,"errmsg":"ok"}"#],
    )])
    .await;
    h.set_authorizer_token(APP_ID_MA, "auth_tok_ma");
    let orders = h.ma_bridge(APP_ID_MA).get_shopping_orders_service();

    let info = ShoppingInfo {
        order_key: wx_rust_open::bean::OrderKeyBean {
            order_number_type: 1,
            transaction_id: "txn_1".to_string(),
            mch_id: "mch_1".to_string(),
            out_trade_no: "out_1".to_string(),
        },
        order_list: vec![wx_rust_open::bean::OrderListBean {
            merchant_order_no: "m_1".to_string(),
            order_detail_jump_link: wx_rust_open::bean::OrderDetailBean {
                url: "https://x/order".to_string(),
                app_id: "wxa_app".to_string(),
                path: "pages/order".to_string(),
                r#type: 1,
            },
            item_list: vec![wx_rust_open::bean::OrderItemListBean {
                merchant_item_id: "i_1".to_string(),
                name: "item".to_string(),
                description: "d".to_string(),
                unit_price: 100,
                quantity: 1,
                image_url: vec![],
            }],
        }],
        payer: wx_rust_open::bean::PayerBean {
            openid: "openid_1".to_string(),
        },
        logistics_type: 1,
        upload_time: "2026-08-01T00:00:00+08:00".to_string(),
    };
    let result = orders.upload_shopping_info(&info).await.expect("应成功");
    assert_eq!(result.errcode, "0");

    let requests = h.server.requests();
    let req = requests.last().unwrap();
    assert!(req.path.starts_with("/user-order/orders"));
    let body = req_body_json(req);
    assert_eq!(body["order_key"]["out_trade_no"], "out_1");
    assert_eq!(
        body["order_list"][0]["item_list"][0]["merchant_item_id"],
        "i_1"
    );
}

/// Minishop：`getWxMinishopServiceByAppid` 双检锁装配 + 同一 appid 返回
/// 同一实例；`submitMerchantInfo` 按 Java snake_case 线格式 POST 后
/// `Ok(None)` 镜像 Java 桩 `return null`。
#[tokio::test]
async fn minishop_submit_merchant_info_mirrors_null() {
    let h = Harness::new(vec![MockRoute::post(
        "/product/register/submit_merchantinfo",
        &[r#"{"errcode":0,"errmsg":"ok"}"#],
    )])
    .await;
    h.set_authorizer_token(APP_ID_MA, "auth_tok_ma");

    let any1 = h
        .component()
        .get_wx_minishop_service_by_appid(APP_ID_MA)
        .expect("minishop 服务已装配");
    let any2 = h
        .component()
        .get_wx_minishop_service_by_appid(APP_ID_MA)
        .expect("minishop 服务已装配");
    let minishop = any1
        .downcast::<wx_rust_open::api::r#impl::WxOpenMinishopServiceImpl>()
        .unwrap();
    let minishop2 = any2
        .downcast::<wx_rust_open::api::r#impl::WxOpenMinishopServiceImpl>()
        .unwrap();
    // 双检锁缓存：同一 appid 同一实例（镜像 Java 静态 map）
    assert!(Arc::ptr_eq(&minishop, &minishop2));

    let result = minishop
        .submit_merchant_info(
            "shop_appid_1",
            "1",
            &wx_rust_open::bean::MinishopBusiLicense::default(),
            &wx_rust_open::bean::MinishopOrganizationCodeInfo::default(),
            &wx_rust_open::bean::MinishopIdcardInfo::default(),
            &wx_rust_open::bean::MinishopSuperAdministratorInfo::default(),
            "shop",
        )
        .await
        .expect("应成功（Java 桩恒 return null → Ok(None)）");
    assert!(result.is_none());

    let requests = h.server.requests();
    let req = requests.last().unwrap();
    assert!(
        req.path
            .starts_with("/product/register/submit_merchantinfo")
    );
    let body = req_body_json(req);
    assert_eq!(body["app_id"], "shop_appid_1");
    assert_eq!(body["subject_type"], "1");
    // Java `toJsonObject()` snake_case 线格式
    assert!(body["busi_license"].is_object());
    assert!(body["super_administrator_info"].is_object());
}

/// WxOpenOAuth2ServiceImpl：普通链路裸 GET `/sns/oauth2/access_token`
/// （无 component_access_token 注入，镜像 Java
/// `WxOpenServiceAbstractImpl.get`）；buildAuthorizationUrl 按
/// `QRCONNECT_URL` 格式化（encodeURIComponent + trim）。
#[tokio::test]
async fn oauth2_service_bare_get_and_authorize_url() {
    let h = Harness::new(vec![MockRoute::get(
        "/sns/oauth2/access_token",
        &[r#"{"access_token":"oa_tok_1","expires_in":7200,"refresh_token":"r1","openid":"o1","scope":"snsapi_base"}"#],
    )])
    .await;
    let oauth2 = WxOpenOAuth2ServiceImpl::new(
        "mp_appid_01".to_string(),
        "mp_secret_01".to_string(),
        h.service.clone(),
    );
    let token = oauth2.get_access_token("code_1").await.expect("应成功");
    assert_eq!(token.access_token, "oa_tok_1");
    assert_eq!(token.open_id, "o1");

    let requests = h.server.requests();
    let req = requests.last().unwrap();
    assert!(req.path.starts_with("/sns/oauth2/access_token"));
    assert!(req.path.contains("appid=mp_appid_01"));
    assert!(req.path.contains("secret=mp_secret_01"));
    assert!(req.path.contains("code=code_1"));
    // 裸 GET：无 component_access_token（对应 Java 不注入）
    assert!(!req.path.contains("component_access_token"));

    // QRCONNECT_URL 格式化：open.weixin.qq.com/connect/qrconnect +
    // encodeURIComponent（空格 → %20，与 URLEncoder 的 `+` 不同）+ trim
    let url = oauth2.build_authorization_url("https://x/r 1", "snsapi_userinfo", "  st  ");
    assert!(url.starts_with(
        "https://open.weixin.qq.com/connect/qrconnect?appid=mp_appid_01&redirect_uri=https%3A%2F%2Fx%2Fr%201&response_type=code&scope=snsapi_userinfo&state=st"
    ));
    assert!(url.ends_with("#wechat_redirect"));
}

/// WxOpenMpOAuth2ServiceImpl：component 链路 GET
/// `/sns/oauth2/component/access_token`（组件服务 get 注入
/// component_access_token，镜像 Java `wxOpenComponentService.get(url)`）；
/// buildAuthorizationUrl 按 `CONNECT_OAUTH2_AUTHORIZE_URL` 格式化。
#[tokio::test]
async fn mp_oauth2_service_component_link() {
    let h = Harness::new(vec![MockRoute::get(
        "/sns/oauth2/component/access_token",
        &[r#"{"access_token":"comp_oa_tok","expires_in":7200,"refresh_token":"cr","openid":"co1","scope":"snsapi_userinfo"}"#],
    )])
    .await;
    let mp = h.mp_bridge(APP_ID_MP);
    let oauth2 = mp.oauth2_service().expect("new_arc 已装配 oauth2");

    let token = oauth2.get_access_token("code_2").await.expect("应成功");
    assert_eq!(token.access_token, "comp_oa_tok");

    let requests = h.server.requests();
    let req = requests.last().unwrap();
    assert!(req.path.starts_with("/sns/oauth2/component/access_token"));
    assert!(req.path.contains("appid=authorizer_appid_mp_01"));
    assert!(req.path.contains("code=code_2"));
    assert!(req.path.contains("component_appid=component_appid_01"));
    // 组件服务 get 注入 component_access_token
    assert!(req.path.contains("component_access_token=comp_tok_v1"));

    let url = oauth2.build_authorization_url("https://x/au", "snsapi_userinfo", "st");
    assert!(url.starts_with("https://open.weixin.qq.com/connect/oauth2/authorize?appid=authorizer_appid_mp_01&redirect_uri=https%3A%2F%2Fx%2Fau&response_type=code&scope=snsapi_userinfo&state=st&component_appid=component_appid_01"));
    assert!(url.ends_with("#wechat_redirect"));
}

/// PrivacyKeyEnum：key/desc 对照 Java 常量，serde rename 输出 Java 枚举
/// 常量名（Gson 默认枚举名序列化）。
#[test]
fn privacy_key_enum_mirrors_java() {
    assert_eq!(PrivacyKeyEnum::UserInfo.key(), "UserInfo");
    assert_eq!(
        PrivacyKeyEnum::UserInfo.desc(),
        "用户信息（微信昵称、头像）"
    );
    assert_eq!(PrivacyKeyEnum::Location.key(), "Location");
    assert_eq!(PrivacyKeyEnum::ExOrderInfo.key(), "EXOrderInfo");
    assert_eq!(PrivacyKeyEnum::MessageFile.key(), "MessageFile");
    assert_eq!(PrivacyKeyEnum::AlbumWriteOnly.desc(), "相册（仅写入）权限");
    // serde rename：Java Gson 默认输出枚举常量名
    assert_eq!(
        serde_json::to_string(&PrivacyKeyEnum::UserInfo).unwrap(),
        r#""USER_INFO""#
    );
    assert_eq!(
        serde_json::to_string(&PrivacyKeyEnum::PhoneNumber).unwrap(),
        r#""PHONE_NUMBER""#
    );
    assert_eq!(
        serde_json::from_str::<PrivacyKeyEnum>(r#""EX_ORDER_INFO""#).unwrap(),
        PrivacyKeyEnum::ExOrderInfo
    );
}

/// MaAuthAndIcp 子服务（对应 Java `WxOpenMaAuthAndIcpServiceImpl`）：
/// `queryAuthAndIcp` POST `/wxa/sec/query_auth_and_icp` 请求体
/// procedure_id 与响应解析（覆盖最后一个子域服务）。
#[tokio::test]
async fn ma_auth_and_icp_query_posts() {
    let h = Harness::new(vec![MockRoute::post(
        "/wxa/sec/query_auth_and_icp",
        &[r#"{"errcode":0,"errmsg":"ok","procedure_status":1,"orderid":1001}"#],
    )])
    .await;
    h.set_authorizer_token(APP_ID_MA, "auth_tok_ma");
    let svc = h.ma_bridge(APP_ID_MA).get_auth_and_icp_service();
    let result = svc.query_auth_and_icp("procedure_1").await.expect("应成功");
    assert_eq!(result.procedure_status, 1);
    assert_eq!(result.order_id, 1001);

    let requests = h.server.requests();
    let req = requests.last().unwrap();
    assert!(req.path.starts_with("/wxa/sec/query_auth_and_icp"));
    let body = req_body_json(req);
    assert_eq!(body["procedure_id"], "procedure_1");
}
