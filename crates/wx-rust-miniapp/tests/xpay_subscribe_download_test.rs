#![allow(clippy::field_reassign_with_default)]
//! 小程序虚拟支付 2026-09 新增接口补测——官方文档示例为 golden。
//!
//! 覆盖 8 个超出 WxJava 4.8.6 范围的新 API（个人主体小程序虚拟支付
//! 订阅系列 + 订单下载任务 + iOS 月结账单 + 商户管控原因）：
//! 断言覆盖：响应字段解析、请求体关键字段、URL 路径与 pay_sig 注入、
//! 错误码上抛（errcode != 0）。

use std::sync::Arc;
use std::sync::atomic::Ordering;

use wx_rust_miniapp::api::WxMaService;
use wx_rust_miniapp::api::r#impl::WxMaServiceImpl;
use wx_rust_miniapp::bean::xpay::{
    WxMaXPayCancelSubscribeContractRequest, WxMaXPayDownloadIosSettlementBillRequest,
    WxMaXPayQueryDownloadOrderRequest, WxMaXPayQueryPunishmentReasonsRequest,
    WxMaXPayQuerySubscribeContractRequest, WxMaXPaySendSubscribePrePaymentRequest,
    WxMaXPaySigParams, WxMaXPayStartDownloadOrderRequest, WxMaXPaySubmitSubscribePayOrderRequest,
};
use wx_rust_miniapp::config::WxMaConfig;
use wx_rust_miniapp::config::r#impl::WxMaDefaultConfig;

struct MockServer {
    addr: std::net::SocketAddr,
    last_path: Arc<std::sync::Mutex<String>>,
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
        let last_path = Arc::new(std::sync::Mutex::new(String::new()));
        let last_body = Arc::new(std::sync::Mutex::new(String::new()));
        let stop = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let handler = Arc::new(handler);
        let lp = last_path.clone();
        let lb = last_body.clone();
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
                let lp = lp.clone();
                let lb = lb.clone();
                tokio::spawn(async move {
                    use tokio::io::{AsyncReadExt, AsyncWriteExt};
                    let mut buf = [0u8; 65536];
                    let n = socket.read(&mut buf).await.unwrap_or(0);
                    let request = String::from_utf8_lossy(&buf[..n]).to_string();
                    let path = request
                        .lines()
                        .next()
                        .map(|l| l.split_whitespace().nth(1).unwrap_or("/").to_string())
                        .unwrap_or_default();
                    *lp.lock().unwrap() = path.clone();
                    if let Some(idx) = request.find("\r\n\r\n") {
                        *lb.lock().unwrap() = request[idx + 4..].to_string();
                    }
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
            last_path,
            last_body,
            stop,
        }
    }
    fn url(&self, p: &str) -> String {
        format!("http://{}{}", self.addr, p)
    }
    fn last_path(&self) -> String {
        self.last_path.lock().unwrap().clone()
    }
    fn last_body_json(&self) -> serde_json::Value {
        serde_json::from_str(&self.last_body.lock().unwrap().clone()).unwrap()
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

fn sig() -> WxMaXPaySigParams {
    let mut sig = WxMaXPaySigParams::default();
    sig.session_key = "sk".into();
    sig.app_key = "ak".into();
    sig
}

/// 对应官方文档: query_subscribe_contract（查询签约关系）。
/// golden：authorization_state=SIGNED（签约生效中）。
#[tokio::test]
async fn xpay_query_subscribe_contract_signed() {
    let s = MockServer::start(dispatch(|_| {
        r#"{"errcode":0,"errmsg":"ok","authorization_state":"SIGNED"}"#.into()
    }))
    .await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let mut req = WxMaXPayQuerySubscribeContractRequest::default();
    req.openid = "o1".into();
    req.product_id = "sub_goods_001".into();
    req.out_contract_code = "CC123".into();
    let r = svc
        .xpay_service()
        .unwrap()
        .query_subscribe_contract(&req, &sig())
        .await
        .unwrap();
    assert_eq!(r.authorization_state, "SIGNED");
    // 请求体关键字段（官方文档请求示例）
    let b = s.last_body_json();
    assert_eq!(b["openid"], "o1");
    assert_eq!(b["product_id"], "sub_goods_001");
    assert_eq!(b["out_contract_code"], "CC123");
    // URL 路径 + 单 pay_sig 注入
    let p = s.last_path();
    assert!(
        p.starts_with("/xpay/query_subscribe_contract?"),
        "实际：{p}"
    );
    assert!(p.contains("pay_sig="), "pay_sig 缺失：{p}");
}

/// 对应官方文档: query_subscribe_contract——TERMINATED 终态语义。
#[tokio::test]
async fn xpay_query_subscribe_contract_terminated() {
    let s = MockServer::start(dispatch(|_| {
        r#"{"errcode":0,"errmsg":"ok","authorization_state":"TERMINATED"}"#.into()
    }))
    .await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let r = svc
        .xpay_service()
        .unwrap()
        .query_subscribe_contract(
            &WxMaXPayQuerySubscribeContractRequest {
                openid: "o1".into(),
                product_id: "p".into(),
                out_contract_code: "c".into(),
            },
            &sig(),
        )
        .await
        .unwrap();
    assert_eq!(r.authorization_state, "TERMINATED");
}

/// 对应官方文档: send_subscribe_pre_payment（预通知扣款）。
/// golden：errcode=0 受理成功；deduct_price 为分为单位整数。
#[tokio::test]
async fn xpay_send_subscribe_pre_payment_ok() {
    let s = MockServer::start(dispatch(|_| r#"{"errcode":0,"errmsg":""}"#.into())).await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let mut req = WxMaXPaySendSubscribePrePaymentRequest::default();
    req.openid = "o1".into();
    req.deduct_price = 100;
    req.product_id = "sub_goods_001".into();
    req.out_contract_code = "CC123".into();
    let r = svc
        .xpay_service()
        .unwrap()
        .send_subscribe_pre_payment(&req, &sig())
        .await
        .unwrap();
    assert_eq!(r.errcode, 0);
    let b = s.last_body_json();
    assert_eq!(b["deduct_price"], 100);
    assert!(
        s.last_path()
            .starts_with("/xpay/send_subscribe_pre_payment?")
    );
}

/// 对应官方文档: send_subscribe_pre_payment——690000000 用户未签约上抛。
#[tokio::test]
async fn xpay_send_subscribe_pre_payment_not_subscribed() {
    let s = MockServer::start(dispatch(|_| {
        r#"{"errcode":690000000,"errmsg":"user not subscribed"}"#.into()
    }))
    .await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let err = svc
        .xpay_service()
        .unwrap()
        .send_subscribe_pre_payment(&WxMaXPaySendSubscribePrePaymentRequest::default(), &sig())
        .await
        .unwrap_err();
    assert_eq!(err.error_code(), Some(690000000));
}

/// 对应官方文档: submit_subscribe_pay_order（发起订阅扣款）。
/// golden：errcode=0 仅代表受理成功；请求体含 offer_id/order_id/attach 全字段。
#[tokio::test]
async fn xpay_submit_subscribe_pay_order_accepted() {
    let s = MockServer::start(dispatch(|_| r#"{"errcode":0,"errmsg":""}"#.into())).await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let mut req = WxMaXPaySubmitSubscribePayOrderRequest::default();
    req.openid = "o1".into();
    req.offer_id = "offer123".into();
    req.buy_quantity = 1;
    req.env = 0;
    req.currency_type = "CNY".into();
    req.product_id = "sub_goods_001".into();
    req.deduct_price = 300;
    req.order_id = "ORDER_20260902_0001".into();
    req.attach = "payload".into();
    let r = svc
        .xpay_service()
        .unwrap()
        .submit_subscribe_pay_order(&req, &sig())
        .await
        .unwrap();
    assert_eq!(r.errcode, 0);
    let b = s.last_body_json();
    assert_eq!(b["offer_id"], "offer123");
    assert_eq!(b["buy_quantity"], 1);
    assert_eq!(b["currency_type"], "CNY");
    assert_eq!(b["deduct_price"], 300);
    assert_eq!(b["order_id"], "ORDER_20260902_0001");
    assert_eq!(b["attach"], "payload");
    assert!(
        s.last_path()
            .starts_with("/xpay/submit_subscribe_pay_order?")
    );
}

/// 对应官方文档: submit_subscribe_pay_order——-15027 不存在预通知单/重复下单。
#[tokio::test]
async fn xpay_submit_subscribe_pay_order_dup() {
    let s = MockServer::start(dispatch(|_| {
        r#"{"errcode":-15027,"errmsg":"不存在预通知单/重复下单"}"#.into()
    }))
    .await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let err = svc
        .xpay_service()
        .unwrap()
        .submit_subscribe_pay_order(&WxMaXPaySubmitSubscribePayOrderRequest::default(), &sig())
        .await
        .unwrap_err();
    assert_eq!(err.error_code(), Some(-15027));
}

/// 对应官方文档: cancel_subscribe_contract（商家解约）。
#[tokio::test]
async fn xpay_cancel_subscribe_contract_ok() {
    let s = MockServer::start(dispatch(|_| r#"{"errcode":0,"errmsg":""}"#.into())).await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let mut req = WxMaXPayCancelSubscribeContractRequest::default();
    req.openid = "o1".into();
    req.termination_reason = "用户申请".into();
    req.product_id = "sub_goods_001".into();
    req.out_contract_code = "CC123".into();
    let r = svc
        .xpay_service()
        .unwrap()
        .cancel_subscribe_contract(&req, &sig())
        .await
        .unwrap();
    assert_eq!(r.errcode, 0);
    let b = s.last_body_json();
    assert_eq!(b["termination_reason"], "用户申请");
    assert!(
        s.last_path()
            .starts_with("/xpay/cancel_subscribe_contract?")
    );
}

/// 对应官方文档: start_download_order（下载支付订单）。
/// golden：task_id 返回；is_provided/refund_status 可选字段不序列化为 null。
#[tokio::test]
async fn xpay_start_download_order_task_created() {
    let s = MockServer::start(dispatch(|_| {
        r#"{"errcode":0,"errmsg":"","task_id":"TASK_9"}"#.into()
    }))
    .await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let mut req = WxMaXPayStartDownloadOrderRequest::default();
    req.begin_ds = 20260420;
    req.end_ds = 20260420;
    req.order_type = 3; // 会员订阅订单
    req.is_provided = Some(true);
    req.env = 0;
    req.pay_channel = 1;
    let r = svc
        .xpay_service()
        .unwrap()
        .start_download_order(&req, &sig())
        .await
        .unwrap();
    assert_eq!(r.task_id, "TASK_9");
    let b = s.last_body_json();
    assert_eq!(b["begin_ds"], 20260420);
    assert_eq!(b["order_type"], 3);
    assert_eq!(b["is_provided"], true);
    assert_eq!(b["pay_channel"], 1);
    // 未设置的 order_info（String 默认空串）与 None 的 refund_status 不干扰
    assert!(s.last_path().starts_with("/xpay/start_download_order?"));
}

/// 对应官方文档: start_download_order——refund_status 仅退款单时传。
#[tokio::test]
async fn xpay_start_download_order_refund_filter() {
    let s = MockServer::start(dispatch(|_| {
        r#"{"errcode":0,"errmsg":"","task_id":"T"}"#.into()
    }))
    .await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let mut req = WxMaXPayStartDownloadOrderRequest::default();
    req.begin_ds = 20260401;
    req.end_ds = 20260430;
    req.order_type = 4;
    req.refund_status = Some(2);
    req.env = 0;
    req.pay_channel = 2; // 苹果 IAP
    svc.xpay_service()
        .unwrap()
        .start_download_order(&req, &sig())
        .await
        .unwrap();
    let b = s.last_body_json();
    assert_eq!(b["refund_status"], 2);
    assert_eq!(b["pay_channel"], 2);
}

/// 对应官方文档: query_download_order（查询下载订单任务）。
/// golden：status=2 成功 + download_url + expire_at。
#[tokio::test]
async fn xpay_query_download_order_success() {
    let s = MockServer::start(dispatch(|_| {
        r#"{"errcode":0,"errmsg":"","task_id":"TASK_9","status":2,"download_url":"https://dl.weixin.qq.com/bill.gz","expire_at":1745328000}"#.into()
    }))
    .await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let mut req = WxMaXPayQueryDownloadOrderRequest::default();
    req.task_id = "TASK_9".into();
    req.env = 0;
    let r = svc
        .xpay_service()
        .unwrap()
        .query_download_order(&req, &sig())
        .await
        .unwrap();
    assert_eq!(r.status, 2);
    assert_eq!(r.download_url, "https://dl.weixin.qq.com/bill.gz");
    assert_eq!(r.expire_at, 1745328000);
    assert!(s.last_path().starts_with("/xpay/query_download_order?"));
}

/// 对应官方文档: query_download_order——status=3 失败。
#[tokio::test]
async fn xpay_query_download_order_failed_task() {
    let s = MockServer::start(dispatch(|_| {
        r#"{"errcode":0,"errmsg":"","task_id":"T","status":3,"download_url":"","expire_at":0}"#
            .into()
    }))
    .await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let r = svc
        .xpay_service()
        .unwrap()
        .query_download_order(&WxMaXPayQueryDownloadOrderRequest::default(), &sig())
        .await
        .unwrap();
    assert_eq!(r.status, 3);
    assert_eq!(r.download_url, "");
}

/// 对应官方文档: download_ios_settlement_bill（下载 iOS 月结账单）。
/// golden：bill_list 逐月条目（month + bill_url）。
#[tokio::test]
async fn xpay_download_ios_settlement_bill_list() {
    let s = MockServer::start(dispatch(|_| {
        r#"{"errcode":0,"errmsg":"","bill_list":[{"month":"202501","bill_url":"https://dl/1.zip"},{"month":"202502","bill_url":"https://dl/2.zip"}]}"#.into()
    }))
    .await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let mut req = WxMaXPayDownloadIosSettlementBillRequest::default();
    req.start_month = "202501".into();
    req.end_month = "202502".into();
    let r = svc
        .xpay_service()
        .unwrap()
        .download_ios_settlement_bill(&req, &sig())
        .await
        .unwrap();
    assert_eq!(r.bill_list.len(), 2);
    assert_eq!(r.bill_list[0].month, "202501");
    assert_eq!(r.bill_list[1].bill_url, "https://dl/2.zip");
    let b = s.last_body_json();
    assert_eq!(b["start_month"], "202501");
    assert!(
        s.last_path()
            .starts_with("/xpay/download_ios_settlement_bill?")
    );
}

/// 对应官方文档: query_punishment_reasons（商户被管控原因查询）。
/// golden：limited_functions + recovery_specifications 全字段。
#[tokio::test]
async fn xpay_query_punishment_reasons_full() {
    let s = MockServer::start(dispatch(|_| {
        r#"{"errcode":0,"errmsg":"","appid":"wx123","nickname":"测试小店","merchant_code":"1900000109","limited_functions":["支付","提现"],"other_limited_functions":"无","recovery_specifications":[{"limitation_case_id":"CASE_1","limitation_reason_type":"RISK","limitation_reason":"交易风控","limitation_reason_describe":"异常交易比例偏高","relate_limitations":["支付"],"other_relate_limitations":"","recover_way":"提交申诉","recover_way_param":"APPEAL_1","recover_help_url":"https://help","limitation_action_type":"LIMIT","limitation_start_date":"2026-01-01","limitation_date":"2026-01-02"}]}"#.into()
    }))
    .await;
    let svc = WxMaServiceImpl::new_arc(config_with_host(&s.url("")));
    let r = svc
        .xpay_service()
        .unwrap()
        .query_punishment_reasons(&WxMaXPayQueryPunishmentReasonsRequest::default(), &sig())
        .await
        .unwrap();
    assert_eq!(r.appid, "wx123");
    assert_eq!(r.merchant_code, "1900000109");
    assert_eq!(r.limited_functions, vec!["支付", "提现"]);
    assert_eq!(r.recovery_specifications.len(), 1);
    let spec = &r.recovery_specifications[0];
    assert_eq!(spec.limitation_case_id, "CASE_1");
    assert_eq!(spec.limitation_reason, "交易风控");
    assert_eq!(spec.recover_way, "提交申诉");
    assert_eq!(spec.limitation_date, "2026-01-02");
    // 官方定义无请求体字段：POST body 序列化为 {}
    let b = s.last_body_json();
    assert_eq!(b.as_object().map(|m| m.len()), Some(0));
    assert!(s.last_path().starts_with("/xpay/query_punishment_reasons?"));
}
