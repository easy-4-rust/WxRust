#![allow(clippy::field_reassign_with_default, dead_code)]
//! 小程序 G3/G4 分组深度补测——严格按 Java 源码逐方法核对。
//!
//! 每个测试函数顶部 `/// 对应 Java: <ClassName>.<method>` + 中文简述，
//! 断言覆盖：状态码、字段值、错误码（errcode != 0）、请求体关键字段。

use std::sync::Arc;
use std::sync::atomic::Ordering;

use wx_rust_miniapp::api::WxMaService;
use wx_rust_miniapp::api::r#impl::WxMaServiceImpl;
use wx_rust_miniapp::config::WxMaConfig;
use wx_rust_miniapp::config::r#impl::WxMaDefaultConfig;

struct MockServer {
    addr: std::net::SocketAddr,
    last_body: Arc<std::sync::Mutex<String>>,
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
        let stop = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let handler = Arc::new(handler);
        let last_body_c = last_body.clone();
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
                tokio::spawn(async move {
                    use tokio::io::{AsyncReadExt, AsyncWriteExt};
                    let mut buf = [0u8; 65536];
                    let n = socket.read(&mut buf).await.unwrap_or(0);
                    let request = String::from_utf8_lossy(&buf[..n]).to_string();
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

// ═══ G3: 电商服务组 ═══

/// 对应 Java: WxMaShopAccountServiceImplTest.testGetCategoryList
#[tokio::test]
async fn g3_shop_account_test_get_category_list() {
    let s = MockServer::start(dispatch(|_| r#"{"errcode":0,"errmsg":"ok","data":[{"first_cat_id":1,"first_cat_name":"服饰","second_cat_id":2,"second_cat_name":"男装","third_cat_id":3,"third_cat_name":"T恤"}]}"#.into())).await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let r = svc
        .shop_account_service()
        .unwrap()
        .get_category_list()
        .await
        .unwrap();
    assert_eq!(r.err_code, 0);
    assert!(!r.items.is_empty());
    assert_eq!(r.items[0].first_cat_name, "服饰");
    assert_eq!(r.items[0].third_cat_id, 3);
}

/// 对应 Java: WxMaShopAccountServiceImplTest.testGetBrandList
#[tokio::test]
async fn g3_shop_account_test_get_brand_list() {
    let s = MockServer::start(dispatch(|_| r#"{"errcode":0,"errmsg":"ok","data":[{"brand_id":100,"brand_wording":"测试品牌","brand_audit_type":1}]}"#.into())).await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let r = svc
        .shop_account_service()
        .unwrap()
        .get_brand_list()
        .await
        .unwrap();
    assert_eq!(r.err_code, 0);
    assert!(!r.items.is_empty());
    assert_eq!(r.items[0].brand_id, 100);
    assert_eq!(r.items[0].brand_wording, "测试品牌");
}

/// 对应 Java: WxMaShopAccountServiceImplTest.testUpdateInfo
#[tokio::test]
async fn g3_shop_account_test_update_info() {
    let s = MockServer::start(dispatch(|_| r#"{"errcode":0,"errmsg":"ok"}"#.into())).await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let mut req = wx_rust_miniapp::bean::shop::request::WxMaShopAccountUpdateInfoRequest::default();
    req.service_agent_phone = "020-888888".into();
    req.service_agent_path = "https://www.web.com".into();
    let r = svc
        .shop_account_service()
        .unwrap()
        .update_info(&req)
        .await
        .unwrap();
    assert_eq!(r.err_code, 0);
    let b = s.last_body_json();
    assert_eq!(b["service_agent_phone"], "020-888888");
    assert_eq!(b["service_agent_path"], "https://www.web.com");
}

/// 对应 Java: WxMaShopAccountServiceImplTest.testGetInfo
#[tokio::test]
async fn g3_shop_account_test_get_info() {
    let s = MockServer::start(dispatch(|_| {
        r#"{"errcode":0,"errmsg":"ok","data":{"brand_id":200,"brand_wording":"品牌名"}}"#.into()
    }))
    .await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let r = svc
        .shop_account_service()
        .unwrap()
        .get_info()
        .await
        .unwrap();
    assert_eq!(r.err_code, 0);
    assert_eq!(r.data.brand_id, 200);
    assert_eq!(r.data.brand_wording, "品牌名");
}

/// 对应 Java: WxMaShopAuditServiceImplTest.testAuditBrand
#[tokio::test]
async fn g3_shop_audit_test_audit_brand() {
    let s = MockServer::start(dispatch(|_| r#"{"errcode":0,"errmsg":"ok"}"#.into())).await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let mut req = wx_rust_miniapp::bean::shop::request::WxMaShopAuditBrandRequest::default();
    req.audit_req.license = vec!["https://img.example.com/license.jpg".into()];
    req.audit_req.brand_info.brand_audit_type = 1;
    req.audit_req.brand_info.trademark_type = "29".into();
    req.audit_req.brand_info.brand_wording = "346225226351203275".into();
    req.audit_req.brand_info.trademark_registrant = "张三".into();
    req.audit_req.brand_info.trademark_applicant = "张三".into();
    let r = svc
        .shop_audit_service()
        .unwrap()
        .audit_brand(&req)
        .await
        .unwrap();
    assert_eq!(r.err_code, 0);
    let b = s.last_body_json();
    assert_eq!(b["audit_req"]["brand_info"]["brand_audit_type"], 1);
    assert_eq!(b["audit_req"]["brand_info"]["trademark_type"], "29");
    assert_eq!(
        b["audit_req"]["brand_info"]["brand_wording"],
        "346225226351203275"
    );
    assert_eq!(
        b["audit_req"]["license"][0],
        "https://img.example.com/license.jpg"
    );
}

/// 对应 Java: WxMaShopAuditServiceImplTest.testAuditCategory
#[tokio::test]
async fn g3_shop_audit_test_audit_category() {
    let s = MockServer::start(dispatch(|_| r#"{"errcode":0,"errmsg":"ok"}"#.into())).await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let mut req = wx_rust_miniapp::bean::shop::request::WxMaShopAuditCategoryRequest::default();
    req.audit_req.license = vec!["www.xxxxx.com".into()];
    req.audit_req.category_info.level1 = 7419;
    req.audit_req.category_info.level2 = 7439;
    req.audit_req.category_info.level3 = 7448;
    req.audit_req.category_info.certificate = vec!["www.xxxxx.com".into()];
    let r = svc
        .shop_audit_service()
        .unwrap()
        .audit_category(&req)
        .await
        .unwrap();
    assert_eq!(r.err_code, 0);
    let b = s.last_body_json();
    assert_eq!(b["audit_req"]["category_info"]["level1"], 7419);
    assert_eq!(b["audit_req"]["category_info"]["level2"], 7439);
    assert_eq!(b["audit_req"]["category_info"]["level3"], 7448);
}

/// 对应 Java: WxMaShopAuditServiceImplTest.testGetAuditResult
#[tokio::test]
async fn g3_shop_audit_test_get_audit_result() {
    let s = MockServer::start(dispatch(|_| r#"{"errcode":0,"errmsg":"ok","data":{"status":2,"brand_id":100,"reject_reason":"资质不全"}}"#.into())).await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let r = svc
        .shop_audit_service()
        .unwrap()
        .get_audit_result("RQAAAHIOW-QGAAAAveAUYQ")
        .await
        .unwrap();
    assert_eq!(r.err_code, 0);
    assert_eq!(r.data.status, 2);
    assert_eq!(r.data.brand_id, 100);
    assert_eq!(r.data.reject_reason, "资质不全");
}

/// 对应 Java: WxMaShopAuditServiceImplTest.testGetMiniappCertificate1
#[tokio::test]
async fn g3_shop_audit_test_get_miniapp_certificate_type1() {
    let s = MockServer::start(dispatch(|_| r#"{"errcode":0,"errmsg":"ok","data":{"qualification_list":[{"key":"license","value":"https://img.example.com/q.jpg"}]}}"#.into())).await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let r = svc
        .shop_audit_service()
        .unwrap()
        .get_miniapp_certificate(1)
        .await
        .unwrap();
    assert!(r.is_object());
    assert_eq!(r["errcode"], 0);
}

/// 对应 Java: WxMaShopAuditServiceImplTest.testGetMiniappCertificate2
#[tokio::test]
async fn g3_shop_audit_test_get_miniapp_certificate_type2() {
    let s = MockServer::start(dispatch(|_| {
        r#"{"errcode":0,"errmsg":"ok","data":{"qualification_list":[]}}"#.into()
    }))
    .await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let r = svc
        .shop_audit_service()
        .unwrap()
        .get_miniapp_certificate(2)
        .await
        .unwrap();
    assert!(r.is_object());
}

/// 对应 Java: WxMaShopDeliveryServiceImplTest.testGetCompanyList
#[tokio::test]
async fn g3_shop_delivery_test_get_company_list() {
    let s = MockServer::start(dispatch(|_| r#"{"errcode":0,"errmsg":"ok","company_list":[{"delivery_id":"ZTO","delivery_name":"中通快递"},{"delivery_id":"SF","delivery_name":"顺丰速运"}]}"#.into())).await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let r = svc
        .shop_delivery_service()
        .unwrap()
        .get_company_list()
        .await
        .unwrap();
    assert_eq!(r.err_code, 0);
    assert_eq!(r.company_list.len(), 2);
    assert_eq!(r.company_list[0].delivery_id, "ZTO");
    assert_eq!(r.company_list[0].delivery_name, "中通快递");
    assert_eq!(r.company_list[1].delivery_id, "SF");
}

/// 对应 Java: WxMaShopDeliveryServiceImplTest.testSend
#[tokio::test]
async fn g3_shop_delivery_test_send() {
    let s = MockServer::start(dispatch(|_| r#"{"errcode":0,"errmsg":"ok"}"#.into())).await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let mut req = wx_rust_miniapp::bean::shop::request::WxMaShopDeliverySendRequest::default();
    req.out_order_id = "318070290792415232".into();
    req.openid = "odIi15CuQ0IQviqsnUMy6CKNetrM".into();
    req.finish_all_delivery = 1;
    let mut dl = wx_rust_miniapp::bean::shop::request::DeliveryListBean::default();
    dl.delivery_id = "ZTO".into();
    dl.waybill_id = "73164691843558".into();
    req.delivery_list = vec![dl];
    let r = svc
        .shop_delivery_service()
        .unwrap()
        .send(&req)
        .await
        .unwrap();
    assert_eq!(r.err_code, 0);
    let b = s.last_body_json();
    assert_eq!(b["out_order_id"], "318070290792415232");
    assert_eq!(b["openid"], "odIi15CuQ0IQviqsnUMy6CKNetrM");
    assert_eq!(b["finish_all_delivery"], 1);
    assert_eq!(b["delivery_list"][0]["delivery_id"], "ZTO");
    assert_eq!(b["delivery_list"][0]["waybill_id"], "73164691843558");
}

/// 对应 Java: WxMaShopDeliveryServiceImplTest.testReceive
#[tokio::test]
async fn g3_shop_delivery_test_receive() {
    let s = MockServer::start(dispatch(|_| r#"{"errcode":0,"errmsg":"ok"}"#.into())).await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let mut req = wx_rust_miniapp::bean::shop::request::WxMaShopDeliveryRecieveRequest::default();
    req.openid = "oTVP50O53a7jgmawAmxKukNlq3XI".into();
    req.order_id = 123456;
    req.out_order_id = "xxxxx".into();
    let r = svc
        .shop_delivery_service()
        .unwrap()
        .receive(&req)
        .await
        .unwrap();
    assert_eq!(r.err_code, 0);
    let b = s.last_body_json();
    assert_eq!(b["openid"], "oTVP50O53a7jgmawAmxKukNlq3XI");
    assert_eq!(b["order_id"], 123456);
    assert_eq!(b["out_order_id"], "xxxxx");
}

/// 对应 Java: WxMaShopPayServiceImplTest.testCreateOrder
#[tokio::test]
async fn g3_shop_pay_test_create_order() {
    let s = MockServer::start(dispatch(|_| r#"{"errcode":0,"errmsg":"ok","payment_params":{"timeStamp":1700000000,"nonceStr":"abc123","package":"prepay_id=wx123","paySign":"sign123","signType":"RSA"}}"#.into())).await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let mut req = wx_rust_miniapp::bean::shop::request::WxMaShopPayCreateOrderRequest::default();
    req.openid = "oTVP50O53a7jgmawAmxKukNlq3XI".into();
    req.combine_trade_no = "COMBINE_001".into();
    req.expire_time = 1700086400;
    let mut sub = wx_rust_miniapp::bean::shop::request::SubOrdersDTO::default();
    sub.mchid = "1900001".into();
    sub.amount = 100;
    sub.trade_no = "TRADE_001".into();
    sub.description = "测试商品".into();
    req.sub_orders = vec![sub];
    let r = svc
        .shop_pay_service()
        .unwrap()
        .create_order(&req)
        .await
        .unwrap();
    assert_eq!(r.err_code, 0);
    assert_eq!(r.payment_params.nonce_str, "abc123");
    assert_eq!(r.payment_params.package_x, "prepay_id=wx123");
    assert_eq!(r.payment_params.pay_sign, "sign123");
    assert_eq!(r.payment_params.sign_type, "RSA");
    let b = s.last_body_json();
    assert_eq!(b["openid"], "oTVP50O53a7jgmawAmxKukNlq3XI");
    assert_eq!(b["combine_trade_no"], "COMBINE_001");
    assert_eq!(b["sub_orders"][0]["mchid"], "1900001");
    assert_eq!(b["sub_orders"][0]["amount"], 100);
}

/// 对应 Java: WxMaShopPayServiceImplTest.testGetOrder
#[tokio::test]
async fn g3_shop_pay_test_get_order() {
    let s = MockServer::start(dispatch(|_| r#"{"errcode":0,"errmsg":"ok","order":{"trade_no":"457243057210572800","transaction_id":"TX_001","amount":100}}"#.into())).await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let r = svc
        .shop_pay_service()
        .unwrap()
        .get_order("457243057210572800")
        .await
        .unwrap();
    assert_eq!(r.err_code, 0);
    assert_eq!(r.order.trade_no, "457243057210572800");
    assert_eq!(r.order.transaction_id, "TX_001");
}

/// 对应 Java: WxMaShopAfterSaleServiceImplTest.testGet
#[tokio::test]
async fn g3_shop_after_sale_test_get() {
    let s = MockServer::start(dispatch(|_| r#"{"errcode":0,"errmsg":"ok","aftersale_infos":[{"out_aftersale_id":"AS_001","status":1,"type":1}]}"#.into())).await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let mut req = wx_rust_miniapp::bean::shop::request::WxMaShopAfterSaleGetRequest::default();
    req.openid = "oTVP50O53a7jgmawAmxKukNlq3XI".into();
    req.order_id = 32434234;
    req.out_order_id = "xxxxx".into();
    let r = svc
        .shop_after_sale_service()
        .unwrap()
        .get(&req)
        .await
        .unwrap();
    assert_eq!(r.err_code, 0);
    let b = s.last_body_json();
    assert_eq!(b["openid"], "oTVP50O53a7jgmawAmxKukNlq3XI");
    assert_eq!(b["order_id"], 32434234);
    assert_eq!(b["out_order_id"], "xxxxx");
}

/// 对应 Java: WxMaShopAfterSaleServiceImplTest.testEcGet
#[tokio::test]
async fn g3_shop_after_sale_test_ec_get() {
    let s = MockServer::start(dispatch(|_| {
        r#"{"errcode":0,"errmsg":"ok","out_aftersale_id":"aso_123124341","status":1,"type":1}"#
            .into()
    }))
    .await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let mut req = wx_rust_miniapp::bean::shop::request::WxMaShopEcAfterSaleGetRequest::default();
    req.aftersale_id = 123;
    req.out_aftersale_id = "aso_123124341".into();
    let r = svc
        .shop_after_sale_service()
        .unwrap()
        .get_ec(&req)
        .await
        .unwrap();
    assert_eq!(r.err_code, 0);
    let b = s.last_body_json();
    assert_eq!(b["aftersale_id"], 123);
    assert_eq!(b["out_aftersale_id"], "aso_123124341");
}

/// 对应 Java: WxMaEmployeeRelationServiceImplTest.testSendEmployeeMsg
#[tokio::test]
async fn g3_employee_relation_test_send_employee_msg() {
    let s = MockServer::start(dispatch(|_| r#"{"errcode":0,"errmsg":"ok"}"#.into())).await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let mut req = wx_rust_miniapp::bean::employee::WxMaSendEmployeeMsgRequest::default();
    req.page = "/pages/index/index".into();
    req.touser = "o0uBr12b1zdgCk1qDoBivmSYb9GA".into();
    req.template_id = "nmO-O4V33TOREVLAlumwPCsHssqkt7mea".into();
    req.data = r#"{"data":{"thing1":{"value":"测试"}}}"#.into();
    svc.employee_relation_service()
        .unwrap()
        .send_employee_msg(&req)
        .await
        .unwrap();
    let b = s.last_body_json();
    assert_eq!(b["page"], "/pages/index/index");
    assert_eq!(b["touser"], "o0uBr12b1zdgCk1qDoBivmSYb9GA");
    assert!(b["data"].is_string());
}

/// 对应 Java: WxMaEmployeeRelationServiceImplTest.testUnbinduserb2cauthinfo
#[tokio::test]
async fn g3_employee_relation_test_unbind_employee() {
    let s = MockServer::start(dispatch(|_| r#"{"errcode":0,"errmsg":"ok"}"#.into())).await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let mut req = wx_rust_miniapp::bean::employee::WxMaUnbindEmployeeRequest::default();
    req.openid_list = vec!["o0uBr12b1zdgCk1qDoBivmSYb9GA".into()];
    svc.employee_relation_service()
        .unwrap()
        .unbind_employee(&req)
        .await
        .unwrap();
    let b = s.last_body_json();
    assert_eq!(b["openid_list"][0], "o0uBr12b1zdgCk1qDoBivmSYb9GA");
}

/// 对应 Java: WxMaImmediateDeliveryServiceImplTest.testCancelOrder
#[tokio::test]
async fn g3_immediate_delivery_test_cancel_order() {
    let s = MockServer::start(dispatch(|_| r#"{"resultcode":0,"resultmsg":"ok"}"#.into())).await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let mut req = wx_rust_miniapp::bean::delivery::CancelOrderRequest::default();
    req.shop_id = "shopId".into();
    req.shop_no = "shopNo_1".into();
    req.app_secret = "secret".into();
    req.delivery_id = "SFTC".into();
    req.cancel_reason_id = 1;
    req.shop_order_id = "order_001".into();
    req.waybill_id = "WB_001".into();
    let r = svc
        .immediate_delivery_service()
        .unwrap()
        .cancel_order(&req)
        .await
        .unwrap();
    assert_eq!(r.result_code, 0);
    let b = s.last_body_json();
    assert_eq!(b["shopid"], "shopId");
    assert_eq!(b["shop_order_id"], "order_001");
    assert_eq!(b["cancel_reason_id"], 1);
}

/// 对应 Java: WxMaImmediateDeliveryServiceImplTest.testAbnormalConfirm
#[tokio::test]
async fn g3_immediate_delivery_test_abnormal_confirm() {
    let s = MockServer::start(dispatch(|_| r#"{"resultcode":0,"resultmsg":"ok"}"#.into())).await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let mut req = wx_rust_miniapp::bean::delivery::AbnormalConfirmRequest::default();
    req.shop_id = "shopId".into();
    req.shop_no = "shopNo_1".into();
    req.app_secret = "secret".into();
    req.delivery_id = "SFTC".into();
    req.shop_order_id = "order_001".into();
    req.waybill_id = "WB_001".into();
    req.remark = "测试签收异常订单".into();
    let r = svc
        .immediate_delivery_service()
        .unwrap()
        .abnormal_confirm(&req)
        .await
        .unwrap();
    assert_eq!(r.result_code, 0);
    let b = s.last_body_json();
    assert_eq!(b["shopid"], "shopId");
    assert_eq!(b["remark"], "测试签收异常订单");
}

/// 对应 Java: WxMaImmediateDeliveryServiceImplTest.testMockUpdateOrder
#[tokio::test]
async fn g3_immediate_delivery_test_mock_update_order() {
    let s = MockServer::start(dispatch(|_| r#"{"resultcode":0,"resultmsg":"ok"}"#.into())).await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let mut req = wx_rust_miniapp::bean::delivery::MockUpdateOrderRequest::default();
    req.action_time = 1700000000;
    req.order_status = 102;
    req.shop_order_id = "".into();
    let r = svc
        .immediate_delivery_service()
        .unwrap()
        .mock_update_order(&req)
        .await
        .unwrap();
    assert_eq!(r.result_code, 0);
    let b = s.last_body_json();
    assert_eq!(b["action_time"], 1700000000);
    assert_eq!(b["order_status"], 102);
}

/// 对应 Java: WxMaShopCatServiceImplTest.testGetCat
#[tokio::test]
async fn g3_shop_cat_test_get_cat() {
    let s = MockServer::start(dispatch(|_| r#"{"errcode":0,"errmsg":"ok","third_cat_list":[{"third_cat_id":101,"third_cat_name":"测试类目","first_cat_id":1,"first_cat_name":"一级类目","second_cat_id":2,"second_cat_name":"二级类目"}]}"#.into())).await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let r = svc.shop_cat_service().unwrap().get_cat().await.unwrap();
    assert_eq!(r.err_code, 0);
    assert_eq!(r.third_cat_list.len(), 1);
    assert_eq!(r.third_cat_list[0].third_cat_id, 101);
    assert_eq!(r.third_cat_list[0].third_cat_name, "测试类目");
    assert_eq!(r.third_cat_list[0].first_cat_name, "一级类目");
}

// ═══ G4: 能力服务组 ═══

/// 对应 Java: WxMaLiveServiceImplTest.deleteRoom
#[tokio::test]
async fn g4_live_test_delete_room() {
    let s = MockServer::start(dispatch(|_| r#"{"errcode":0,"errmsg":"ok"}"#.into())).await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let r = svc.live_service().unwrap().delete_room(29).await.unwrap();
    assert!(r);
    let b = s.last_body_json();
    assert_eq!(b["id"], 29);
}

/// 对应 Java: WxMaLiveServiceImplTest.editRoom
#[tokio::test]
async fn g4_live_test_edit_room() {
    let s = MockServer::start(dispatch(|_| r#"{"errcode":0,"errmsg":"ok"}"#.into())).await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let mut room = wx_rust_miniapp::bean::live::WxMaLiveRoomInfo::default();
    room.id = 39;
    room.name = "修改订阅通知直播间".into();
    room.anchor_name = "鹏军_专业小程序开发".into();
    room.r#type = 1;
    let r = svc.live_service().unwrap().edit_room(&room).await.unwrap();
    assert!(r);
    let b = s.last_body_json();
    assert_eq!(b["id"], 39);
    assert_eq!(b["name"], "修改订阅通知直播间");
    assert_eq!(b["anchorName"], "鹏军_专业小程序开发");
}

/// 对应 Java: WxMaLiveServiceImplTest.getLiveReplay
#[tokio::test]
async fn g4_live_test_get_live_replay() {
    let s = MockServer::start(dispatch(|_| r#"{"errcode":0,"total":1,"room_info":[{"name":"回放直播间","roomid":3,"live_status":102,"start_time":1700000000,"end_time":1700003600}]}"#.into())).await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let r = svc
        .live_service()
        .unwrap()
        .get_live_replay_default(3, 0, 10)
        .await
        .unwrap();
    assert_eq!(r.total, 1);
    assert_eq!(r.room_infos.len(), 1);
    assert_eq!(r.room_infos[0].room_id, 3);
}

/// 对应 Java: WxMaLiveServiceImplTest.getLiveinfos
#[tokio::test]
async fn g4_live_test_get_live_infos() {
    let s = MockServer::start(dispatch(|_| r#"{"errcode":0,"total":2,"room_info":[{"name":"直播间A","roomid":1,"live_status":101},{"name":"直播间B","roomid":2,"live_status":104}]}"#.into())).await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let r = svc.live_service().unwrap().get_live_infos().await.unwrap();
    assert_eq!(r.len(), 2);
    assert_eq!(r[0].name, "直播间A");
    assert_eq!(r[0].room_id, 1);
    assert_eq!(r[1].room_id, 2);
}

/// 对应 Java: WxMaLiveGoodsServiceImplTest.deleteGoods
#[tokio::test]
async fn g4_live_goods_test_delete_goods() {
    let s = MockServer::start(dispatch(|_| r#"{"errcode":0,"errmsg":"ok"}"#.into())).await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let r = svc
        .live_goods_service()
        .unwrap()
        .delete_goods(9)
        .await
        .unwrap();
    assert!(r);
    let b = s.last_body_json();
    assert_eq!(b["goodsId"], 9);
}

/// 对应 Java: WxMaLiveMemberServiceImplTest.testAddRole
#[tokio::test]
async fn g4_live_member_test_add_role() {
    let s = MockServer::start(dispatch(|_| r#"{"errcode":0,"errmsg":"ok"}"#.into())).await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let r = svc
        .live_member_service()
        .unwrap()
        .add_role("abc", 1)
        .await
        .unwrap();
    assert!(!r.is_empty());
    let b = s.last_body_json();
    assert_eq!(b["username"], "abc");
    assert_eq!(b["role"], 1);
}

/// 对应 Java: WxMaLiveMemberServiceImplTest.testDeleteRole
#[tokio::test]
async fn g4_live_member_test_delete_role() {
    let s = MockServer::start(dispatch(|_| r#"{"errcode":0,"errmsg":"ok"}"#.into())).await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let r = svc
        .live_member_service()
        .unwrap()
        .delete_role("abc", 1)
        .await
        .unwrap();
    assert!(!r.is_empty());
    let b = s.last_body_json();
    assert_eq!(b["username"], "abc");
    assert_eq!(b["role"], 1);
}

/// 对应 Java: WxMaVodServiceImplTest.testListDrama
#[tokio::test]
async fn g4_vod_test_list_drama() {
    let s = MockServer::start(dispatch(|_| r#"{"errcode":0,"errmsg":"ok","drama_info_list":[{"drama_id":100000,"name":"我的中国梦","cover_url":"https://cdn/cover.jpg","create_time":1700000000,"media_count":10}]}"#.into())).await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let mut req = wx_rust_miniapp::bean::vod::WxMaVodListDramaRequest::default();
    req.offset = 0;
    req.limit = 100;
    let r = svc
        .vod_service()
        .unwrap()
        .list_drama(&req)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(r.len(), 1);
    assert_eq!(r[0].drama_id, 100000);
    assert_eq!(r[0].name, "我的中国梦");
}

/// 对应 Java: WxMaVodServiceImplTest.testGetTask
#[tokio::test]
async fn g4_vod_test_get_task() {
    let s = MockServer::start(dispatch(|_| r#"{"errcode":0,"errmsg":"ok","task_info":{"task_type":1,"status":2,"errcode":0,"errmsg":"ok","create_time":1700000000}}"#.into())).await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let mut req = wx_rust_miniapp::bean::vod::WxMaVodGetTaskRequest::default();
    req.task_id = 12345;
    let r = svc.vod_service().unwrap().get_task(&req).await.unwrap();
    assert_eq!(r.task_info.status, 2);
    assert_eq!(r.task_info.task_type, 1);
}

/// 对应 Java: WxMaXPayServiceImplTest.testQueryUserBalance
#[tokio::test]
async fn g4_xpay_test_query_user_balance() {
    let s = MockServer::start(dispatch(|_| r#"{"errcode":0,"errmsg":"ok","balance":500,"present_balance":100,"sum_save":600,"sum_present":100}"#.into())).await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let mut req = wx_rust_miniapp::bean::xpay::WxMaXPayQueryUserBalanceRequest::default();
    req.openid = "o1".into();
    req.env = 0;
    req.user_ip = "127.0.0.1".into();
    let mut sig = wx_rust_miniapp::bean::xpay::WxMaXPaySigParams::default();
    sig.session_key = "sk".into();
    sig.app_key = "ak".into();
    let r = svc
        .xpay_service()
        .unwrap()
        .query_user_balance(&req, &sig)
        .await
        .unwrap();
    assert_eq!(r.balance, 500);
    assert_eq!(r.present_balance, 100);
    assert_eq!(r.sum_save, 600);
    let b = s.last_body_json();
    assert_eq!(b["openid"], "o1");
    assert_eq!(b["env"], 0);
}

/// 对应 Java: WxMaXPayServiceImplTest.testQueryOrder
#[tokio::test]
async fn g4_xpay_test_query_order() {
    let s = MockServer::start(dispatch(|_| r#"{"errcode":0,"errmsg":"ok","order":{"order_id":"ORDER_1","status":1,"order_fee":100,"paid_fee":100}}"#.into())).await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let mut req = wx_rust_miniapp::bean::xpay::WxMaXPayQueryOrderRequest::default();
    req.openid = "o1".into();
    req.env = 0;
    req.order_id = "ORDER_1".into();
    let mut sig = wx_rust_miniapp::bean::xpay::WxMaXPaySigParams::default();
    sig.session_key = "sk".into();
    sig.app_key = "ak".into();
    let r = svc
        .xpay_service()
        .unwrap()
        .query_order(&req, &sig)
        .await
        .unwrap();
    assert_eq!(r.order.order_id, "ORDER_1");
    assert_eq!(r.order.status, 1);
    assert_eq!(r.order.paid_fee, 100);
}

/// 对应 Java: WxMaXPayServiceImplTest.testCancelCurrencyPay
#[tokio::test]
async fn g4_xpay_test_cancel_currency_pay() {
    let s = MockServer::start(dispatch(|_| {
        r#"{"errcode":0,"errmsg":"ok","order_id":"ORDER_1"}"#.into()
    }))
    .await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let mut req = wx_rust_miniapp::bean::xpay::WxMaXPayCancelCurrencyPayRequest::default();
    req.openid = "o1".into();
    req.env = 0;
    req.user_ip = "127.0.0.1".into();
    req.order_id = "ORDER_1".into();
    req.pay_order_id = "PAY_ORDER_1".into();
    req.amount = 1000;
    let mut sig = wx_rust_miniapp::bean::xpay::WxMaXPaySigParams::default();
    sig.session_key = "sk".into();
    sig.app_key = "ak".into();
    let r = svc
        .xpay_service()
        .unwrap()
        .cancel_currency_pay(&req, &sig)
        .await
        .unwrap();
    assert_eq!(r.order_id, "ORDER_1");
    let b = s.last_body_json();
    assert_eq!(b["openid"], "o1");
    assert_eq!(b["amount"], 1000);
}

/// 对应 Java: WxMaXPayServiceImplTest.testDownloadBill
#[tokio::test]
async fn g4_xpay_test_download_bill() {
    let s = MockServer::start(dispatch(|_| {
        r#"{"errcode":0,"errmsg":"ok","url":"https://cdn/bill.csv"}"#.into()
    }))
    .await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let mut req = wx_rust_miniapp::bean::xpay::WxMaXPayDownloadBillRequest::default();
    req.begin_ds = 20230801;
    req.end_ds = 20230810;
    let mut sig = wx_rust_miniapp::bean::xpay::WxMaXPaySigParams::default();
    sig.session_key = "sk".into();
    sig.app_key = "ak".into();
    let r = svc
        .xpay_service()
        .unwrap()
        .download_bill(&req, &sig)
        .await
        .unwrap();
    assert_eq!(r.url, "https://cdn/bill.csv");
    let b = s.last_body_json();
    assert_eq!(b["begin_ds"], 20230801);
    assert_eq!(b["end_ds"], 20230810);
}

/// 对应 Java: WxMaPromotionServiceTest.testGetRole
#[tokio::test]
async fn g4_promotion_test_get_role() {
    let s = MockServer::start(dispatch(|_| r#"{"errcode":0,"errmsg":"ok","role_list":[{"role_id":1,"name":"推广员1号","desc":"描述"}],"total_cnt":1}"#.into())).await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let mut req = wx_rust_miniapp::bean::promoter::WxMaPromotionGetRoleRequest::default();
    req.role_id = 1;
    let r = svc
        .promotion_service()
        .unwrap()
        .get_role(&req)
        .await
        .unwrap();
    assert_eq!(r.total_cnt, 1);
    assert_eq!(r.role_list.len(), 1);
    assert_eq!(r.role_list[0].role_id, 1);
    assert_eq!(r.role_list[0].name, "推广员1号");
}

/// 对应 Java: WxMaPromotionServiceTest.testUpdateRole
#[tokio::test]
async fn g4_promotion_test_update_role() {
    let s = MockServer::start(dispatch(|_| r#"{"errcode":0,"errmsg":"ok"}"#.into())).await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let mut req = wx_rust_miniapp::bean::promoter::WxMaPromoterUpdateRoleRequest::default();
    req.role_id = 1;
    req.name = "推广员1号名字".into();
    req.desc = "推广员1号描述".into();
    svc.promotion_service()
        .unwrap()
        .update_role(&req)
        .await
        .unwrap();
    let b = s.last_body_json();
    assert_eq!(b["role_id"], 1);
    assert_eq!(b["name"], "推广员1号名字");
    assert_eq!(b["desc"], "推广员1号描述");
}

/// 对应 Java: WxMaPromotionServiceTest.testSingleSendMsg
#[tokio::test]
async fn g4_promotion_test_single_send_msg() {
    let s = MockServer::start(dispatch(|_| r#"{"errcode":0,"errmsg":"ok"}"#.into())).await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let mut req = wx_rust_miniapp::bean::promoter::WxMaPromotionSingleSendMsgRequest::default();
    req.msg_type = 1;
    req.content = "{}".into();
    req.appid = "wxappid".into();
    req.path = "pages/index".into();
    req.openid = "o1".into();
    let r = svc
        .promotion_service()
        .unwrap()
        .single_send_msg(&req)
        .await
        .unwrap();
    assert_eq!(r.errcode, 0);
    let b = s.last_body_json();
    assert_eq!(b["msg_type"], 1);
    assert_eq!(b["openid"], "o1");
    assert_eq!(b["appid"], "wxappid");
}

/// 对应 Java: WxMaPromotionServiceTest.testGetMsg
#[tokio::test]
async fn g4_promotion_test_get_msg() {
    let s = MockServer::start(dispatch(|_| {
        r#"{"errcode":0,"errmsg":"ok","send_cnt":10,"percent":80,"fail_cnt":2}"#.into()
    }))
    .await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let mut req = wx_rust_miniapp::bean::promoter::WxMaPromotionGetMsgRequest::default();
    req.msg_id = "MSG-100".into();
    let r = svc
        .promotion_service()
        .unwrap()
        .get_msg(&req)
        .await
        .unwrap();
    assert_eq!(r.send_cnt, 10);
    assert_eq!(r.percent, 80);
    assert_eq!(r.fail_cnt, 2);
}

/// 对应 Java: WxMaDeviceSubscribeServiceImplTest.testCreateIotGroupId
#[tokio::test]
async fn g4_device_subscribe_test_create_iot_group_id() {
    let s = MockServer::start(dispatch(|_| {
        r#"{"errcode":0,"errmsg":"ok","group_id":"GROUP_1"}"#.into()
    }))
    .await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let mut req = wx_rust_miniapp::bean::device::WxMaCreateIotGroupIdRequest::default();
    req.model_id = "11111".into();
    req.group_name = "测试设备组".into();
    let r = svc
        .device_subscribe_service()
        .unwrap()
        .create_iot_group_id(&req)
        .await
        .unwrap();
    assert_eq!(r, "GROUP_1");
    let b = s.last_body_json();
    assert_eq!(b["model_id"], "11111");
    assert_eq!(b["group_name"], "测试设备组");
}

/// 对应 Java: WxMaReimburseInvoiceServiceImplTest.testUpdateInvoiceStatus
#[tokio::test]
async fn g4_reimburse_invoice_test_update_invoice_status() {
    let s = MockServer::start(dispatch(|_| r#"{"errcode":0,"errmsg":"ok"}"#.into())).await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let mut req = wx_rust_miniapp::bean::invoice::UpdateInvoiceStatusRequest::default();
    req.card_id = "CARD_1".into();
    req.encrypt_code = "ENC_1".into();
    req.reimburse_status = "INVOICE_REIMBURSE_INIT".into();
    svc.reimburse_invoice_service()
        .unwrap()
        .update_invoice_status(&req)
        .await
        .unwrap();
    let b = s.last_body_json();
    assert_eq!(b["card_id"], "CARD_1");
    assert_eq!(b["encrypt_code"], "ENC_1");
    assert_eq!(b["reimburse_status"], "INVOICE_REIMBURSE_INIT");
}

/// 对应 Java: WxMaReimburseInvoiceServiceImplTest.testUpdateStatusBatch
#[tokio::test]
async fn g4_reimburse_invoice_test_update_status_batch() {
    let s = MockServer::start(dispatch(|_| r#"{"errcode":0,"errmsg":"ok"}"#.into())).await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let mut r = wx_rust_miniapp::bean::invoice::InvoiceInfoRequest::default();
    r.card_id = "CARD_1".into();
    r.encrypt_code = "ENC_1".into();
    let mut req = wx_rust_miniapp::bean::invoice::UpdateStatusBatchRequest::default();
    req.invoice_list = vec![r];
    req.openid = "o1".into();
    req.reimburse_status = "INVOICE_REIMBURSE_LOCK".into();
    svc.reimburse_invoice_service()
        .unwrap()
        .update_status_batch(&req)
        .await
        .unwrap();
    let b = s.last_body_json();
    assert_eq!(b["openid"], "o1");
    assert_eq!(b["reimburse_status"], "INVOICE_REIMBURSE_LOCK");
    assert_eq!(b["invoice_list"][0]["card_id"], "CARD_1");
}

/// 对应 Java: WxMaQrcodeJumpServiceImplTest.testDeleteRule
#[tokio::test]
async fn g4_qrcode_jump_test_delete_rule() {
    let s = MockServer::start(dispatch(|_| r#"{"errcode":0,"errmsg":"ok"}"#.into())).await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let r = svc
        .qrcode_jump_service()
        .unwrap()
        .delete_rule("/pages/index")
        .await
        .unwrap();
    assert!(r.contains("errcode"));
    let b = s.last_body_json();
    assert_eq!(b["prefix"], "/pages/index");
}

/// 对应 Java: WxMaComplaintServiceImplTest 查询投诉 errcode!=0 场景
#[tokio::test]
async fn g4_complaint_query_errcode_nonzero() {
    let s = MockServer::start(dispatch(|_| {
        r#"{"errcode":40001,"errmsg":"invalid credential"}"#.into()
    }))
    .await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let mut q = wx_rust_miniapp::bean::complaint::WxMaComplaintRequest::default();
    q.begin_date = "2024-01-01".into();
    q.end_date = "2024-01-31".into();
    q.limit = 10;
    q.offset = 0;
    let err = svc
        .complaint_service()
        .unwrap()
        .query_complaints(&q)
        .await
        .expect_err("errcode!=0 应报错");
    assert_eq!(err.error_code(), Some(40001));
}

/// 对应 Java: WxMaXPayServiceImplTest 虚拟支付 errcode!=0 场景
#[tokio::test]
async fn g4_xpay_currency_pay_errcode_nonzero() {
    let s = MockServer::start(dispatch(|_| {
        r#"{"errcode":90001,"errmsg":"invalid env"}"#.into()
    }))
    .await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let mut req = wx_rust_miniapp::bean::xpay::WxMaXPayCurrencyPayRequest::default();
    req.openid = "o1".into();
    req.env = 0;
    req.user_ip = "127.0.0.1".into();
    req.amount = 100;
    let mut sig = wx_rust_miniapp::bean::xpay::WxMaXPaySigParams::default();
    sig.session_key = "sk".into();
    sig.app_key = "ak".into();
    let err = svc
        .xpay_service()
        .unwrap()
        .currency_pay(&req, &sig)
        .await
        .expect_err("errcode!=0 应报错");
    assert_eq!(err.error_code(), Some(90001));
}
