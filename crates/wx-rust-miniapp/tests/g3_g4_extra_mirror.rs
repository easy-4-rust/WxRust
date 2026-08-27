#![allow(clippy::field_reassign_with_default)]
//! miniapp 镜像补足测试——扩展到 miniapp 整个 src/test 目录的非 bean 测试类。
//!
//! 当前 g3/g4 深度补测已覆盖 26 个 Java 测试类，本文件新增 8 个 Java 测试类镜像，
//! 使总镜像数达到 34（>= 30 目标）。
//!
//! 新增镜像：
//! - WxMaAnalysisServiceImplTest（数据分析）
//! - WxMaJsapiServiceImplTest（JSAPI 签名）
//! - WxMaOpenApiServiceImplTest（OpenAPI 配额管理）
//! - WxMaSchemeServiceImplTest（Scheme 码）
//! - WxMaLinkServiceImplTest（URL Link / Short Link）
//! - WxMaSettingServiceImplTest（小程序设置）
//! - WxMaPluginServiceImplTest（插件管理）
//! - WxMaShopRegisterServiceImplTest（交易组件-申请接入）

use std::sync::Arc;
use std::sync::atomic::Ordering;

use wx_rust_miniapp::api::WxMaService;
use wx_rust_miniapp::api::r#impl::WxMaServiceImpl;
use wx_rust_miniapp::config::WxMaConfig;
use wx_rust_miniapp::config::r#impl::WxMaDefaultConfig;

struct MockServer {
    addr: std::net::SocketAddr,
    last_body: Arc<std::sync::Mutex<String>>,
    last_request_line: Arc<std::sync::Mutex<String>>,
    stop: Arc<std::sync::atomic::AtomicBool>,
}

impl MockServer {
    async fn start<F>(handler: F) -> Self
    where
        F: Fn(&str) -> String + Send + Sync + 'static,
    {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let last_body = Arc::new(std::sync::Mutex::new(String::new()));
        let last_request_line = Arc::new(std::sync::Mutex::new(String::new()));
        let stop = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let handler = Arc::new(handler);
        let last_body_c = last_body.clone();
        let last_request_line_c = last_request_line.clone();
        let stop_c = stop.clone();
        tokio::spawn(async move {
            loop {
                if stop_c.load(Ordering::SeqCst) {
                    break;
                }
                let Ok((mut socket, _)) = listener.accept().await else {
                    continue;
                };
                let handler = handler.clone();
                let last_body_c = last_body_c.clone();
                let last_request_line_c = last_request_line_c.clone();
                tokio::spawn(async move {
                    use tokio::io::{AsyncReadExt, AsyncWriteExt};
                    let mut buf = [0u8; 65536];
                    let n = socket.read(&mut buf).await.unwrap_or(0);
                    let request = String::from_utf8_lossy(&buf[..n]).to_string();
                    if let Some(line) = request.lines().next() {
                        *last_request_line_c.lock().unwrap() = line.to_string();
                    }
                    if let Some(idx) = request.find("\r\n\r\n") {
                        *last_body_c.lock().unwrap() = request[idx + 4..].to_string();
                    }
                    let path = request
                        .lines()
                        .next()
                        .map(|l| l.split_whitespace().nth(1).unwrap_or("/").to_string())
                        .unwrap_or_default();
                    let body = handler(&path);
                    let resp = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                        body.len(),
                        body
                    );
                    let _ = socket.write_all(resp.as_bytes()).await;
                });
            }
        });
        Self {
            addr,
            last_body,
            last_request_line,
            stop,
        }
    }
    fn url(&self, p: &str) -> String {
        format!("http://{}{}", self.addr, p)
    }
    fn last_body(&self) -> String {
        self.last_body.lock().unwrap().clone()
    }
    fn last_body_json(&self) -> serde_json::Value {
        serde_json::from_str(&self.last_body()).unwrap()
    }
    fn last_request_line(&self) -> String {
        self.last_request_line.lock().unwrap().clone()
    }
}

impl Drop for MockServer {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
    }
}

fn config_with_host(host: &str) -> Arc<dyn WxMaConfig> {
    let mut config = WxMaDefaultConfig::new("wxappid", "secret");
    config
        .set_token("token123")
        .set_aes_key("abcdefghijklmnopqrstuvwxyz0123456789ABCDEFG");
    let mut hc = wx_rust_miniapp::config::WxMaHostConfig::new();
    hc.api_host = host.to_string();
    config.set_host_config(hc);
    Arc::new(config)
}

fn dispatch(
    h: impl Fn(&str) -> String + Send + Sync + 'static,
) -> impl Fn(&str) -> String + Send + Sync + 'static {
    move |path: &str| {
        if path.contains("/cgi-bin/token") || path.contains("/stable_token") {
            return r#"{"access_token":"MOCK_TOKEN","expires_in":7200}"#.to_string();
        }
        h(path)
    }
}

// ═══ WxMaAnalysisServiceImplTest（数据分析服务） ═══

/// 对应 Java: WxMaAnalysisServiceImplTest.testGetDailySummaryTrend
#[tokio::test]
async fn analysis_test_get_daily_summary_trend() {
    let s = MockServer::start(dispatch(|path| {
        if path.contains("getweanalysisappiddailysummarytrend") {
            r#"{"list":[{"refDate":"20240101","visitTotal":100,"sharePv":10,"shareUv":5}]}"#.into()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.into()
        }
    }))
    .await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let r = svc
        .analysis_service()
        .unwrap()
        .get_daily_summary_trend("20240101", "20240101")
        .await
        .unwrap();
    let list = r.expect("应返回 list");
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].ref_date, "20240101");
    assert_eq!(list[0].visit_total, 100);
    assert_eq!(list[0].share_pv, 10);
    assert_eq!(list[0].share_uv, 5);
    let b = s.last_body_json();
    assert_eq!(b["begin_date"], "20240101");
    assert_eq!(b["end_date"], "20240101");
}

/// 对应 Java: WxMaAnalysisServiceImplTest.testGetDailyVisitTrend
#[tokio::test]
async fn analysis_test_get_daily_visit_trend() {
    let s = MockServer::start(dispatch(|path| {
        if path.contains("getweanalysisappiddailyvisittrend") {
            r#"{"list":[{"refDate":"20240101","sessionCnt":50,"visitPv":80,"visitUv":60,"visitUvNew":20}]}"#.into()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.into()
        }
    }))
    .await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let r = svc
        .analysis_service()
        .unwrap()
        .get_daily_visit_trend("20240101", "20240101")
        .await
        .unwrap();
    let list = r.expect("应返回 list");
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].ref_date, "20240101");
    assert_eq!(list[0].session_cnt, 50);
    assert_eq!(list[0].visit_pv, 80);
    assert_eq!(list[0].visit_uv, 60);
    assert_eq!(list[0].visit_uv_new, 20);
}

/// 对应 Java: WxMaAnalysisServiceImplTest.testGetVisitPage
#[tokio::test]
async fn analysis_test_get_visit_page() {
    let s = MockServer::start(dispatch(|path| {
        if path.contains("getweanalysisappidvisitpage") {
            r#"{"list":[{"pagePath":"pages/index/index","pageVisitPv":1000,"pageVisitUv":500,"pageStayTimePv":12.5,"entryPagePv":800,"exitPagePv":200,"pageSharePv":30,"pageShareUv":20}]}"#.into()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.into()
        }
    }))
    .await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let r = svc
        .analysis_service()
        .unwrap()
        .get_visit_page("20240101", "20240101")
        .await
        .unwrap();
    let list = r.expect("应返回 list");
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].page_path, "pages/index/index");
    assert_eq!(list[0].page_visit_pv, 1000);
    assert_eq!(list[0].page_visit_uv, 500);
    assert_eq!(list[0].entry_page_pv, 800);
    assert_eq!(list[0].exit_page_pv, 200);
}

/// 对应 Java: WxMaAnalysisServiceImplTest.testGetDailyRetainInfo
#[tokio::test]
async fn analysis_test_get_daily_retain_info() {
    let s = MockServer::start(dispatch(|path| {
        if path.contains("getweanalysisappiddailyretaininfo") {
            r#"{"ref_date":"20240101","visit_uv_new":{"0":100,"1":50},"visit_uv":{"0":200,"1":150}}"#.into()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.into()
        }
    }))
    .await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let r = svc
        .analysis_service()
        .unwrap()
        .get_daily_retain_info("20240101", "20240101")
        .await
        .unwrap();
    assert_eq!(r.ref_date, "20240101");
    assert_eq!(r.visit_uv_new.get(&0), Some(&100));
    assert_eq!(r.visit_uv_new.get(&1), Some(&50));
    assert_eq!(r.visit_uv.get(&0), Some(&200));
}

/// 对应 Java: WxMaAnalysisServiceImplTest.testGetVisitDistribution
#[tokio::test]
async fn analysis_test_get_visit_distribution() {
    let s = MockServer::start(dispatch(|path| {
        if path.contains("getweanalysisappidvisitdistribution") {
            r#"{"ref_date":"20240101","list":[{"index":"access_source_session_cnt","item_list":[{"key":1,"value":100},{"key":2,"value":200}]},{"index":"access_staytime_info","item_list":[{"key":1,"value":50}]}]}"#.into()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.into()
        }
    }))
    .await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let r = svc
        .analysis_service()
        .unwrap()
        .get_visit_distribution("20240101", "20240101")
        .await
        .unwrap();
    assert_eq!(r.ref_date, "20240101");
    assert!(r.list.contains_key("access_source_session_cnt"));
    assert!(r.list.contains_key("access_staytime_info"));
    assert_eq!(r.list["access_source_session_cnt"].get(&1), Some(&100));
    assert_eq!(r.list["access_source_session_cnt"].get(&2), Some(&200));
}

// ═══ WxMaJsapiServiceImplTest（JSAPI 签名服务） ═══

/// 对应 Java: WxMaJsapiServiceImplTest.testGetJsapiTicket
#[tokio::test]
async fn jsapi_test_get_jsapi_ticket() {
    let s = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/ticket/getticket") {
            r#"{"errcode":0,"errmsg":"ok","ticket":"sM4AOVdWfPE4DxkXGEs8VMKv9ss_SL5g1SdwfGwjFZdBbVh8Hn0jQ","expires_in":7200}"#.into()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.into()
        }
    }))
    .await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let ticket = svc
        .jsapi_service()
        .unwrap()
        .get_jsapi_ticket()
        .await
        .unwrap();
    assert!(!ticket.is_empty());
    assert_eq!(
        ticket,
        "sM4AOVdWfPE4DxkXGEs8VMKv9ss_SL5g1SdwfGwjFZdBbVh8Hn0jQ"
    );
    let req_line = s.last_request_line();
    assert!(
        req_line.contains("type=jsapi"),
        "应请求 jsapi 类型 ticket：{req_line}"
    );
}

/// 对应 Java: WxMaJsapiServiceImplTest.testCreateJsapiSignature
#[tokio::test]
async fn jsapi_test_create_jsapi_signature() {
    let s = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/ticket/getticket") {
            r#"{"errcode":0,"errmsg":"ok","ticket":"test_ticket_value","expires_in":7200}"#.into()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.into()
        }
    }))
    .await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let sig = svc
        .jsapi_service()
        .unwrap()
        .create_jsapi_signature("http://www.qq.com")
        .await
        .unwrap();
    assert!(!sig.signature.is_empty());
    assert_eq!(sig.url, "http://www.qq.com");
    assert_eq!(sig.app_id, "wxappid");
    assert!(!sig.nonce_str.is_empty());
    assert!(sig.timestamp > 0);
}

// ═══ WxMaOpenApiServiceImplTest（OpenAPI 配额管理） ═══

/// 对应 Java: WxMaOpenApiServiceImplTest.clearQuota
#[tokio::test]
async fn open_api_test_clear_quota() {
    let s = MockServer::start(dispatch(|_| r#"{"errcode":0,"errmsg":"ok"}"#.into())).await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let r = svc.open_api_service().unwrap().clear_quota().await.unwrap();
    assert!(r);
    let b = s.last_body_json();
    assert_eq!(b["appid"], "wxappid");
}

/// 对应 Java: WxMaOpenApiServiceImplTest.getApiQuota
#[tokio::test]
async fn open_api_test_get_api_quota() {
    let s = MockServer::start(dispatch(|_| {
        r#"{"errcode":0,"errmsg":"ok","quota":{"daily_limit":10000,"used":100,"remain":9900},"rateLimit":{"call_count":1000,"refresh_second":1},"componentRateLimit":{"call_count":500,"refresh_second":1}}"#.into()
    }))
    .await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let r = svc
        .open_api_service()
        .unwrap()
        .get_api_quota("/cgi-bin/openapi/quota/get")
        .await
        .unwrap();
    assert_eq!(r.quota.daily_limit, 10000);
    assert_eq!(r.quota.used, 100);
    assert_eq!(r.quota.remain, 9900);
    assert_eq!(r.rate_limit.call_count, 1000);
    let b = s.last_body_json();
    assert_eq!(b["cgi_path"], "/cgi-bin/openapi/quota/get");
}

/// 对应 Java: WxMaOpenApiServiceImplTest.clearQuotaByAppSecret
#[tokio::test]
async fn open_api_test_clear_quota_by_app_secret() {
    let s = MockServer::start(dispatch(|_| r#"{"errcode":0,"errmsg":"ok"}"#.into())).await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let r = svc
        .open_api_service()
        .unwrap()
        .clear_quota_by_app_secret()
        .await
        .unwrap();
    assert!(r);
    let req_line = s.last_request_line();
    assert!(
        req_line.contains("clear_quota/v2"),
        "应请求 clear_quota/v2：{req_line}"
    );
}

// ═══ WxMaSchemeServiceImplTest（Scheme 码服务） ═══

/// 对应 Java: WxMaSchemeServiceImplTest.testGenerate
#[tokio::test]
async fn scheme_test_generate() {
    let s = MockServer::start(dispatch(|_| {
        r#"{"errcode":0,"errmsg":"ok","openlink":"weixin://dl/business/?t=TOKEN123"}"#.into()
    }))
    .await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let mut req = wx_rust_miniapp::bean::scheme::WxMaGenerateSchemeRequest::default();
    req.jump_wxa.path = "pages/productView/editPhone/editPhone".into();
    req.jump_wxa.query = "".into();
    req.is_expire = true;
    req.expire_time = 1700003600;
    let r = svc.scheme_service().unwrap().generate(&req).await.unwrap();
    assert_eq!(r, "weixin://dl/business/?t=TOKEN123");
    let b = s.last_body_json();
    assert_eq!(
        b["jump_wxa"]["path"],
        "pages/productView/editPhone/editPhone"
    );
    assert_eq!(b["is_expire"], true);
    assert_eq!(b["expire_time"], 1700003600);
}

/// 对应 Java: WxMaSchemeServiceImplTest.testGenerateNfc
#[tokio::test]
async fn scheme_test_generate_nfc() {
    let s = MockServer::start(dispatch(|_| {
        r#"{"errcode":0,"errmsg":"ok","openlink":"weixin://dl/business/?t=NFC_TOKEN"}"#.into()
    }))
    .await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let mut req = wx_rust_miniapp::bean::scheme::WxMaGenerateNfcSchemeRequest::default();
    req.jump_wxa.path = "pages/productView/editPhone/editPhone".into();
    req.jump_wxa.query = "".into();
    req.model_id = "MODEL_001".into();
    req.sn = "SN_001".into();
    let r = svc
        .scheme_service()
        .unwrap()
        .generate_nfc(&req)
        .await
        .unwrap();
    assert_eq!(r, "weixin://dl/business/?t=NFC_TOKEN");
    let b = s.last_body_json();
    assert_eq!(
        b["jump_wxa"]["path"],
        "pages/productView/editPhone/editPhone"
    );
    assert_eq!(b["model_id"], "MODEL_001");
    assert_eq!(b["sn"], "SN_001");
}

// ═══ WxMaLinkServiceImplTest（URL Link / Short Link 服务） ═══

/// 对应 Java: WxMaLinkServiceImplTest.testGenerateUrlLink
#[tokio::test]
async fn link_test_generate_url_link() {
    let s = MockServer::start(dispatch(|_| {
        r#"{"errcode":0,"errmsg":"ok","url_link":"https://wxa.url.cn/xxx"}"#.into()
    }))
    .await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let mut req = wx_rust_miniapp::bean::urllink::GenerateUrlLinkRequest::default();
    req.path = "pages/tabBar/home/home".into();
    req.expire_time = 1700086400;
    let r = svc
        .link_service()
        .unwrap()
        .generate_url_link(&req)
        .await
        .unwrap();
    assert_eq!(r, "https://wxa.url.cn/xxx");
    let b = s.last_body_json();
    assert_eq!(b["path"], "pages/tabBar/home/home");
}

/// 对应 Java: WxMaLinkServiceImplTest.testGenerateShortLink
#[tokio::test]
async fn link_test_generate_short_link() {
    let s = MockServer::start(dispatch(|_| {
        r#"{"errcode":0,"errmsg":"ok","link":"https://s.wxa.url.cn/xxx"}"#.into()
    }))
    .await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let mut req = wx_rust_miniapp::bean::shortlink::GenerateShortLinkRequest::default();
    req.page_url = "pages/productView/editPhone/editPhone?id=31832".into();
    req.page_title = "productView".into();
    req.is_permanent = false;
    let r = svc
        .link_service()
        .unwrap()
        .generate_short_link(&req)
        .await
        .unwrap();
    assert_eq!(r, "https://s.wxa.url.cn/xxx");
    let b = s.last_body_json();
    assert_eq!(
        b["page_url"],
        "pages/productView/editPhone/editPhone?id=31832"
    );
    assert_eq!(b["page_title"], "productView");
    assert_eq!(b["is_permanent"], false);
}

// ═══ WxMaSettingServiceImplTest（小程序设置服务） ═══

/// 对应 Java: WxMaSettingServiceImplTest.testModifyDomain
#[tokio::test]
async fn setting_test_modify_domain() {
    let s = MockServer::start(dispatch(|_| {
        r#"{"errcode":0,"errmsg":"ok","action":"get","requestdomain":["https://api.example.com"],"wsrequestdomain":[],"uploaddomain":[],"downloaddomain":[]}"#.into()
    }))
    .await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let mut action = wx_rust_miniapp::bean::WxMaDomainAction::default();
    action.action = "get".into();
    let r = svc
        .setting_service()
        .unwrap()
        .modify_domain(&action)
        .await
        .unwrap();
    assert_eq!(r.action, "get");
    assert_eq!(r.request_domain.len(), 1);
    assert_eq!(r.request_domain[0], "https://api.example.com");
    let b = s.last_body_json();
    assert_eq!(b["action"], "get");
}

/// 对应 Java: WxMaSettingServiceImplTest.testBindTester
#[tokio::test]
async fn setting_test_bind_tester() {
    let s = MockServer::start(dispatch(|_| r#"{"errcode":0,"errmsg":"ok"}"#.into())).await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    svc.setting_service()
        .unwrap()
        .bind_tester("WeChatId")
        .await
        .unwrap();
    let b = s.last_body_json();
    assert_eq!(b["wechatid"], "WeChatId");
}

/// 对应 Java: WxMaSettingServiceImplTest.testUnbindTester
#[tokio::test]
async fn setting_test_unbind_tester() {
    let s = MockServer::start(dispatch(|_| r#"{"errcode":0,"errmsg":"ok"}"#.into())).await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    svc.setting_service()
        .unwrap()
        .unbind_tester("WeChatId")
        .await
        .unwrap();
    let b = s.last_body_json();
    assert_eq!(b["wechatid"], "WeChatId");
}

// ═══ WxMaPluginServiceImplTest（插件管理服务） ═══

/// 对应 Java: WxMaPluginServiceImplTest.testApplyPlugin
#[tokio::test]
async fn plugin_test_apply_plugin() {
    let s = MockServer::start(dispatch(|_| r#"{"errcode":0,"errmsg":"ok"}"#.into())).await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    svc.plugin_service()
        .unwrap()
        .apply_plugin("wx4418e3e031e551be", "测试申请理由")
        .await
        .unwrap();
    let b = s.last_body_json();
    assert_eq!(b["action"], "apply");
    assert_eq!(b["plugin_appid"], "wx4418e3e031e551be");
    assert_eq!(b["reason"], "测试申请理由");
}

/// 对应 Java: WxMaPluginServiceImplTest.testGetPluginList
#[tokio::test]
async fn plugin_test_get_plugin_list() {
    let s = MockServer::start(dispatch(|_| {
        r#"{"errcode":0,"errmsg":"ok","plugin_list":[{"appid":"wx4418e3e031e551be","status":"1","nickname":"测试插件","headimgurl":"https://img.example.com/plugin.jpg"}]}"#.into()
    }))
    .await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let r = svc
        .plugin_service()
        .unwrap()
        .get_plugin_list()
        .await
        .unwrap();
    assert_eq!(r.plugin_list.len(), 1);
    assert_eq!(r.plugin_list[0].app_id, "wx4418e3e031e551be");
    assert_eq!(r.plugin_list[0].nick_name, "测试插件");
    let b = s.last_body_json();
    assert_eq!(b["action"], "list");
}

/// 对应 Java: WxMaPluginServiceImplTest.testUnbindPlugin
#[tokio::test]
async fn plugin_test_unbind_plugin() {
    let s = MockServer::start(dispatch(|_| r#"{"errcode":0,"errmsg":"ok"}"#.into())).await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    svc.plugin_service()
        .unwrap()
        .unbind_plugin("wx4418e3e031e551be")
        .await
        .unwrap();
    let b = s.last_body_json();
    assert_eq!(b["action"], "unbind");
    assert_eq!(b["plugin_appid"], "wx4418e3e031e551be");
}

/// 对应 Java: WxMaPluginServiceImplTest.testUpdatePlugin
#[tokio::test]
async fn plugin_test_update_plugin() {
    let s = MockServer::start(dispatch(|_| r#"{"errcode":0,"errmsg":"ok"}"#.into())).await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    svc.plugin_service()
        .unwrap()
        .update_plugin("wx4418e3e031e551be", "2.0.2")
        .await
        .unwrap();
    let b = s.last_body_json();
    assert_eq!(b["action"], "update");
    assert_eq!(b["plugin_appid"], "wx4418e3e031e551be");
    assert_eq!(b["user_version"], "2.0.2");
}

// ═══ WxMaShopRegisterServiceImplTest（交易组件-申请接入） ═══

/// 对应 Java: WxMaShopRegisterServiceImplTest.testRegisterApply
#[tokio::test]
async fn shop_register_test_register_apply() {
    let s = MockServer::start(dispatch(|_| r#"{"errcode":0,"errmsg":"ok"}"#.into())).await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let r = svc
        .shop_register_service()
        .unwrap()
        .register_apply()
        .await
        .unwrap();
    assert_eq!(r.err_code, 0);
}

/// 对应 Java: WxMaShopRegisterServiceImplTest.testRegisterCheck
#[tokio::test]
async fn shop_register_test_register_check() {
    let s = MockServer::start(dispatch(|_| {
        r#"{"errcode":0,"errmsg":"ok","data":{"access_status":1}}"#.into()
    }))
    .await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let r = svc
        .shop_register_service()
        .unwrap()
        .register_check()
        .await
        .unwrap();
    assert_eq!(r.err_code, 0);
}

/// 对应 Java: WxMaShopRegisterServiceImplTest.testRegisterFinishAccessInfo
#[tokio::test]
async fn shop_register_test_register_finish_access_info() {
    let s = MockServer::start(dispatch(|_| r#"{"errcode":0,"errmsg":"ok"}"#.into())).await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let mut req =
        wx_rust_miniapp::bean::shop::request::WxMaShopRegisterFinishAccessInfoRequest::default();
    req.access_info_item = 6;
    let r = svc
        .shop_register_service()
        .unwrap()
        .register_finish_access_info(&req)
        .await
        .unwrap();
    assert_eq!(r.err_code, 0);
    let b = s.last_body_json();
    assert_eq!(b["access_info_item"], 6);
}

/// 对应 Java: WxMaShopRegisterServiceImplTest.testRegisterApplyScene
#[tokio::test]
async fn shop_register_test_register_apply_scene() {
    let s = MockServer::start(dispatch(|_| r#"{"errcode":0,"errmsg":"ok"}"#.into())).await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let mut req =
        wx_rust_miniapp::bean::shop::request::WxMaShopRegisterApplySceneRequest::default();
    req.scene_group_id = 1;
    let r = svc
        .shop_register_service()
        .unwrap()
        .register_apply_scene(&req)
        .await
        .unwrap();
    assert_eq!(r.err_code, 0);
    let b = s.last_body_json();
    assert_eq!(b["scene_group_id"], 1);
}
