#![allow(clippy::field_reassign_with_default, clippy::if_same_then_else)]
//! mp 子域服务覆盖率提升测试（第三批：merchantInvoice/guide 补全/ocr/
//! imgProc/menu/templateMsg/qrcode/user/kefu 扩展/card 扩展/device 扩展/
//! aiOpen/draft 扩展/freePublish 扩展/reimburseInvoice 扩展/material 扩展/
//! store 扩展/comment 扩展/shake 扩展/subscribeMsg 扩展/marketing 扩展/
//! dataCube 扩展/wifi 扩展/massMsg 扩展）。
//!
//! 镜像 Java `WxMp*ServiceImplTest` 的 HTTP 语义，经 MockServer 验证；
//! 覆盖先前零覆盖的方法路径，提升 api/impl 层行覆盖率。

use std::sync::Arc;
use std::sync::atomic::Ordering;

use wx_rust_mp::api::WxMpService;
use wx_rust_mp::config::WxMpConfigStorage;
use wx_rust_mp::config::r#impl::WxMpDefaultConfig;

/// 极简 mock HTTP 服务器（记录最近请求体）。
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
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("绑定端口");
        let addr = listener.local_addr().expect("获取地址");
        let last_body = Arc::new(std::sync::Mutex::new(String::new()));
        let stop = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let handler = Arc::new(handler);

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
                let handler = handler.clone();
                let last_body_clone = last_body_clone.clone();
                tokio::spawn(async move {
                    use tokio::io::{AsyncReadExt, AsyncWriteExt};
                    let mut buf = [0u8; 16384];
                    let n = socket.read(&mut buf).await.unwrap_or(0);
                    let request = String::from_utf8_lossy(&buf[..n]).to_string();
                    if let Some(idx) = request.find("\r\n\r\n") {
                        *last_body_clone.lock().unwrap() = request[idx + 4..].to_string();
                    }
                    let path = request
                        .lines()
                        .next()
                        .map(|l| l.split_whitespace().nth(1).unwrap_or("/").to_string())
                        .unwrap_or_else(|| "/".to_string());
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
            last_body,
            stop,
        }
    }

    fn url(&self, path: &str) -> String {
        format!("http://{}{}", self.addr, path)
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

/// 构建指向 mock 服务器的公众号配置。
fn config_with_host(host: &str) -> Arc<dyn WxMpConfigStorage> {
    let mut config = WxMpDefaultConfig::new("wxappid", "secret");
    config
        .set_token("token123")
        .set_aes_key("abcdefghijklmnopqrstuvwxyz0123456789ABCDEFG");
    let mut host_config = wx_rust_mp::config::WxMpHostConfig::new();
    host_config.api_host = host.to_string();
    config.set_host_config(host_config);
    Arc::new(config)
}

/// 通用路由：token + 业务分派。
fn dispatch(
    handler: impl Fn(&str) -> String + Send + Sync + 'static,
) -> impl Fn(&str) -> String + Send + Sync + 'static {
    move |path: &str| {
        if path.contains("/cgi-bin/token") || path.contains("/stable_token") {
            return r#"{"access_token":"MOCK_TOKEN","expires_in":7200}"#.to_string();
        }
        handler(path)
    }
}

// ========================================================================
// 商户发票（镜像 Java WxMpMerchantInvoiceServiceImplTest）
// 覆盖 api/impl/wx_mp_merchant_invoice_service_impl.rs 7.81% → 目标 90%+
// ========================================================================

/// 对应 Java: WxMpMerchantInvoiceServiceImplTest.getAuthPageUrl
#[tokio::test]
async fn merchant_invoice_get_auth_page_url() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/card/invoice/getauthurl") {
            r#"{"authUrl":"https://auth.example.com","appid":"wxappid"}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let merchant = service
        .merchant_invoice_service()
        .expect("商户发票服务存在");

    let mut req = wx_rust_mp::bean::invoice::merchant::InvoiceAuthPageRequest::default();
    req.s_pappid = "SPAPPID".to_string();
    req.order_id = "ORDER_1".to_string();
    req.money = 100;
    req.source = "web".to_string();
    req.redirect_url = "https://redirect.example.com".to_string();
    req.r#type = 1;
    req.timestamp = 1700000000;
    req.ticket = "TICKET".to_string();
    let result = merchant
        .get_auth_page_url(&req)
        .await
        .expect("获取授权页 URL 成功");
    assert_eq!(result.appid, "wxappid");
    assert_eq!(result.auth_url, "https://auth.example.com");
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["s_pappid"], "SPAPPID");
    assert_eq!(body["order_id"], "ORDER_1");
    assert_eq!(body["money"], 100);
}

/// 对应 Java: WxMpMerchantInvoiceServiceImplTest.getAuthData
#[tokio::test]
async fn merchant_invoice_get_auth_data() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/card/invoice/getauthdata") {
            r#"{"invoiceStatus":"auth_time","authTime":1700000000,"userAuthInfo":{"userField":{"title":"个人","phone":"138","email":"a@b.com","customField":[]},"bizField":{"title":"公司","taxNo":"TAX","addr":"地址","phone":"010","bankType":"对公","bankNo":"622","customField":[]}}}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let merchant = service
        .merchant_invoice_service()
        .expect("商户发票服务存在");

    let mut req = wx_rust_mp::bean::invoice::merchant::InvoiceAuthDataRequest::default();
    req.s_pappid = "SPAPPID".to_string();
    req.order_id = "ORDER_1".to_string();
    let result = merchant
        .get_auth_data(&req)
        .await
        .expect("获取授权数据成功");
    assert_eq!(result.invoice_status, "auth_time");
    assert_eq!(result.user_auth_info.user_field.title, "个人");
    assert_eq!(result.user_auth_info.biz_field.tax_no, "TAX");
}

/// 对应 Java: WxMpMerchantInvoiceServiceImplTest.rejectInvoice
#[tokio::test]
async fn merchant_invoice_reject_invoice() {
    let server = MockServer::start(dispatch(|_path| {
        r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let merchant = service
        .merchant_invoice_service()
        .expect("商户发票服务存在");

    let mut req = wx_rust_mp::bean::invoice::merchant::InvoiceRejectRequest::default();
    req.s_pappid = "SPAPPID".to_string();
    req.order_id = "ORDER_1".to_string();
    req.reason = "信息不完整".to_string();
    req.url = "https://reject.example.com".to_string();
    merchant.reject_invoice(&req).await.expect("拒绝发票成功");
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["reason"], "信息不完整");
}

/// 对应 Java: WxMpMerchantInvoiceServiceImplTest.makeOutInvoice
#[tokio::test]
async fn merchant_invoice_make_out_invoice() {
    let server = MockServer::start(dispatch(|_path| {
        r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let merchant = service
        .merchant_invoice_service()
        .expect("商户发票服务存在");

    let mut req = wx_rust_mp::bean::invoice::merchant::MakeOutInvoiceRequest::default();
    req.invoiceinfo.wxopenid = "o1".to_string();
    req.invoiceinfo.fpqqlsh = "INV_1".to_string();
    req.invoiceinfo.nsrsbh = "TAX_1".to_string();
    req.invoiceinfo.nsrmc = "公司名".to_string();
    req.invoiceinfo.jshj = "100.00".to_string();
    req.invoiceinfo.hjje = "90.00".to_string();
    req.invoiceinfo.hjse = "10.00".to_string();
    merchant.make_out_invoice(&req).await.expect("开票成功");
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["invoiceinfo"]["fpqqlsh"], "INV_1");
}

/// 对应 Java: WxMpMerchantInvoiceServiceImplTest.clearOutInvoice
#[tokio::test]
async fn merchant_invoice_clear_out_invoice() {
    let server = MockServer::start(dispatch(|_path| {
        r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let merchant = service
        .merchant_invoice_service()
        .expect("商户发票服务存在");

    let mut req = wx_rust_mp::bean::invoice::merchant::ClearOutInvoiceRequest::default();
    req.invoiceinfo.fpqqlsh = "INV_1".to_string();
    req.invoiceinfo.nsrsbh = "TAX_1".to_string();
    merchant
        .clear_out_invoice(&req)
        .await
        .expect("作废发票成功");
}

/// 对应 Java: WxMpMerchantInvoiceServiceImplTest.setMerchantContactInfo
#[tokio::test]
async fn merchant_invoice_set_contact_info() {
    let server = MockServer::start(dispatch(|_path| {
        r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let merchant = service
        .merchant_invoice_service()
        .expect("商户发票服务存在");

    let mut contact = wx_rust_mp::bean::invoice::merchant::MerchantContactInfo::default();
    contact.phone = "13800138000".to_string();
    contact.timeout = 30;
    merchant
        .set_merchant_contact_info(&contact)
        .await
        .expect("设置联系方式成功");
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["contact"]["phone"], "13800138000");
}

/// 对应 Java: WxMpMerchantInvoiceServiceImplTest.getMerchantContactInfo
#[tokio::test]
async fn merchant_invoice_get_contact_info() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/card/invoice/setbizattr?action=query_contact") {
            r#"{"contact":{"phone":"13800138000","time_out":30}}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let merchant = service
        .merchant_invoice_service()
        .expect("商户发票服务存在");

    let contact = merchant
        .get_merchant_contact_info()
        .await
        .expect("获取联系方式成功");
    assert_eq!(contact.phone, "13800138000");
    assert_eq!(contact.timeout, 30);
}

/// 对应 Java: WxMpMerchantInvoiceServiceImplTest.setAuthPageSetting
#[tokio::test]
async fn merchant_invoice_set_auth_page_setting() {
    let server = MockServer::start(dispatch(|_path| {
        r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let merchant = service
        .merchant_invoice_service()
        .expect("商户发票服务存在");

    let setting = wx_rust_mp::bean::invoice::merchant::InvoiceAuthPageSetting::default();
    merchant
        .set_auth_page_setting(&setting)
        .await
        .expect("设置授权页成功");
}

/// 对应 Java: WxMpMerchantInvoiceServiceImplTest.getAuthPageSetting
#[tokio::test]
async fn merchant_invoice_get_auth_page_setting() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("query_auth_field") {
            // impl 提取 "auth_field" 值后用 serde 反序列化（字段用 camelCase rename）
            r#"{"auth_field":{"authField":{"userField":{"showTitle":1,"showPhone":1,"showEmail":0,"requirePhone":0,"requireEmail":0,"customField":[]},"bizField":{"showTitle":1,"showTaxNo":1,"showAddr":1,"showPhone":1,"showBankType":1,"showBankNo":1,"requireTaxNo":1,"requireAddr":0,"requirePhone":0,"requireBankType":0,"requireBankNo":0,"customField":[]}}}}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let merchant = service
        .merchant_invoice_service()
        .expect("商户发票服务存在");

    let setting = merchant
        .get_auth_page_setting()
        .await
        .expect("获取授权页设置成功");
    assert_eq!(setting.auth_field.user_field.show_title, 1);
    assert_eq!(setting.auth_field.biz_field.show_tax_no, 1);
}

/// 对应 Java: WxMpMerchantInvoiceServiceImplTest.setMerchantInvoicePlatform
#[tokio::test]
async fn merchant_invoice_set_platform() {
    let server = MockServer::start(dispatch(|_path| {
        r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let merchant = service
        .merchant_invoice_service()
        .expect("商户发票服务存在");

    let mut info = wx_rust_mp::bean::invoice::merchant::MerchantInvoicePlatformInfo::default();
    info.mchid = "MCH_1".to_string();
    info.s_pappid = "SPAPPID".to_string();
    merchant
        .set_merchant_invoice_platform(&info)
        .await
        .expect("设置平台信息成功");
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["pay_mch"]["mchid"], "MCH_1");
}

/// 对应 Java: WxMpMerchantInvoiceServiceImplTest.getMerchantInvoicePlatform
#[tokio::test]
async fn merchant_invoice_get_platform() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/card/invoice/setbizattr?action=query_pay_mch") {
            r#"{"pay_mch":{"mchid":"MCH_1","s_pappid":"SPAPPID"}}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let merchant = service
        .merchant_invoice_service()
        .expect("商户发票服务存在");

    let mut info = wx_rust_mp::bean::invoice::merchant::MerchantInvoicePlatformInfo::default();
    info.mchid = "MCH_1".to_string();
    info.s_pappid = "SPAPPID".to_string();
    let result = merchant
        .get_merchant_invoice_platform(&info)
        .await
        .expect("获取平台信息成功");
    assert_eq!(result.mchid, "MCH_1");
}

// ========================================================================
// 顾问主服务（镜像 Java WxMpGuideServiceImplTest 补全）
// 覆盖 api/impl/wx_mp_guide_service_impl.rs 11.84% → 目标 90%+
// ========================================================================

/// 对应 Java: WxMpGuideServiceImplTest.addGuide
#[tokio::test]
async fn guide_add_and_update() {
    let server = MockServer::start(dispatch(|_path| {
        r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let guide_service = service.guide_service().expect("顾问服务存在");

    let mut info = wx_rust_mp::bean::guide::WxMpGuideInfo::default();
    info.account = "acc1".to_string();
    info.openid = "o1".to_string();
    info.nick_name = "顾问A".to_string();
    guide_service.add_guide(&info).await.expect("添加顾问成功");
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["guide_account"], "acc1");
    assert_eq!(body["guide_nickname"], "顾问A");

    guide_service
        .update_guide(&info)
        .await
        .expect("更新顾问成功");
}

/// 对应 Java: WxMpGuideServiceImplTest.getGuide
#[tokio::test]
async fn guide_get_and_del() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/guide/getguideacct") {
            r#"{"guide_info":{"guide_account":"acc1","guide_openid":"o1","guide_nickname":"顾问A","guide_headimgurl":"http://img","status":1}}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let guide_service = service.guide_service().expect("顾问服务存在");

    let info = guide_service
        .get_guide("acc1", "o1")
        .await
        .expect("获取顾问成功");
    assert_eq!(info.account, "acc1");
    assert_eq!(info.nick_name, "顾问A");
    assert_eq!(info.status, 1);

    guide_service
        .del_guide("acc1", "o1")
        .await
        .expect("删除顾问成功");
}

/// 对应 Java: WxMpGuideServiceImplTest.createGuideQrCode
#[tokio::test]
async fn guide_create_qr_code() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/guide/guidecreateqrcode") {
            r#"{"qrcode_url":"https://qr.example.com/guide"}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let guide_service = service.guide_service().expect("顾问服务存在");

    let url = guide_service
        .create_guide_qr_code("acc1", "o1", "info")
        .await
        .expect("创建顾问二维码成功");
    assert_eq!(url, "https://qr.example.com/guide");
}

/// 对应 Java: WxMpGuideServiceImplTest.getGuideChatRecord
#[tokio::test]
async fn guide_get_chat_record() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/guide/getguidebuyerchatrecord") {
            r#"{"guide_msg_list":{"total_num":1,"msg_list":[{"guide_account":"acc1","guide_openid":"go1","content":"你好","create_time":1700000000,"content_type":1,"direction":1}]}}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let guide_service = service.guide_service().expect("顾问服务存在");

    let list = guide_service
        .get_guide_chat_record("acc1", "go1", "o1", 1700000000, 1700000100, 0, 10)
        .await
        .expect("获取聊天记录成功");
    assert_eq!(list.msg_list.len(), 1);
    assert_eq!(list.msg_list[0].content, "你好");
}

/// 对应 Java: WxMpGuideServiceImplTest.setGuideConfig / getGuideConfig
#[tokio::test]
async fn guide_set_and_get_config() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/guide/getguideconfig") {
            r#"{"guide_config":{"guide_fast_reply_list":[{"content":"快速回复"}],"guide_auto_reply":{"content":"自动回复","msgtype":1},"guide_auto_reply_plus":{"content":"增强回复","msgtype":1}}}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let guide_service = service.guide_service().expect("顾问服务存在");

    let auto_reply = wx_rust_mp::bean::guide::WxMpAddGuideAutoReply {
        content: "自动回复".to_string(),
        msg_type: 1,
    };
    guide_service
        .set_guide_config(
            "acc1",
            "o1",
            false,
            &["快速回复".to_string()],
            &auto_reply,
            &auto_reply,
        )
        .await
        .expect("设置顾问配置成功");

    let config = guide_service
        .get_guide_config("acc1", "o1")
        .await
        .expect("获取顾问配置成功");
    assert_eq!(config.guide_fast_reply_list.len(), 1);
    assert_eq!(config.guide_auto_reply.content, "自动回复");
}

/// 对应 Java: WxMpGuideServiceImplTest.setGuideAcctConfig
#[tokio::test]
async fn guide_set_acct_config() {
    let server = MockServer::start(dispatch(|_path| {
        r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let guide_service = service.guide_service().expect("顾问服务存在");

    guide_service
        .set_guide_acct_config(false, &["违禁词".to_string()], "自动回复内容")
        .await
        .expect("设置顾问账号配置成功");
}

/// 对应 Java: WxMpGuideServiceImplTest.newGuideGroup / getGuideGroupList
#[tokio::test]
async fn guide_group_operations() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/guide/newguidegroup") {
            r#"{"group_id":12345}"#.to_string()
        } else if path.contains("/cgi-bin/guide/getguidegrouplist") {
            r#"{"group_list":{"total_num":1,"guide_list":[{"guide_account":"acc1","guide_openid":"o1","guide_nickname":"顾问A","create_time":1700000000}]}}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let guide_service = service.guide_service().expect("顾问服务存在");

    let group_id = guide_service
        .new_guide_group("VIP群")
        .await
        .expect("创建顾问群组成功");
    assert_eq!(group_id, 12345);
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["group_name"], "VIP群");

    let list = guide_service
        .get_guide_group_list(0, 10)
        .await
        .expect("获取顾问群组列表成功");
    assert_eq!(list.list.len(), 1);
    assert_eq!(list.list[0].nick_name, "顾问A");
}

// ========================================================================
// OCR 扩展（镜像 Java WxMpOcrServiceImplTest 补全）
// 覆盖 api/impl/wx_mp_ocr_service_impl.rs 34.88% → 目标 90%+
// ========================================================================

/// 对应 Java: WxMpOcrServiceImplTest.bankCard / driving / drivingLicense / bizLicense / comm
#[tokio::test]
async fn ocr_bank_card_driving_biz_comm() {
    let server = MockServer::start(dispatch(|path| {
        // 注意：drivinglicense 包含 driving 子串，必须先匹配更长的路径
        if path.contains("/cgi-bin/ocr/drivinglicense") {
            r#"{"id_num":"110101199001011234","name":"张三","vehicle_type":"C1"}"#.to_string()
        } else if path.contains("/cgi-bin/ocr/driving") {
            r#"{"plate_num":"粤B12345","vehicle_type":"小型汽车","owner":"张三"}"#.to_string()
        } else if path.contains("/cgi-bin/ocr/bankcard") {
            r#"{"number":"6222021234567890","valid_from":"2020-01","valid_to":"2030-01"}"#
                .to_string()
        } else if path.contains("/cgi-bin/ocr/bizlicense") {
            r#"{"reg_num":"91440300MA5F123456","serial":"12345","legal_representative":"张三"}"#
                .to_string()
        } else if path.contains("/cgi-bin/ocr/comm") {
            r#"{"items":[{"text":"识别文字","pos":{"left_top":{"x":0,"y":0}}}]}"#.to_string()
        } else {
            "{}".to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let ocr = service.ocr_service().expect("OCR 服务存在");

    let bank = ocr
        .bank_card("http://img/bank.jpg")
        .await
        .expect("银行卡识别成功");
    assert_eq!(bank.number, "6222021234567890");

    let driving = ocr
        .driving("http://img/driving.jpg")
        .await
        .expect("行驶证识别成功");
    assert_eq!(driving.plate_num, "粤B12345");

    let license = ocr
        .driving_license("http://img/license.jpg")
        .await
        .expect("驾驶证识别成功");
    assert_eq!(license.id_num, "110101199001011234");

    let biz = ocr
        .biz_license("http://img/biz.jpg")
        .await
        .expect("营业执照识别成功");
    assert_eq!(biz.legal_representative, "张三");

    let comm = ocr.comm("http://img/comm.jpg").await.expect("通用识别成功");
    assert_eq!(comm.items.len(), 1);
}

// ========================================================================
// 图片处理扩展（覆盖 img_proc 27.78% → 90%+）
// ========================================================================

/// 对应 Java: WxMpImgProcServiceImplTest.superResolution / aiCrop
#[tokio::test]
async fn img_proc_super_resolution_and_ai_crop() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/imgproc/superresolution") {
            r#"{"media_id":"MEDIA_SR"}"#.to_string()
        } else if path.contains("/cgi-bin/imgproc/aicrop") {
            r#"{"results":[{"crop_left":0,"crop_top":0,"crop_right":100,"crop_bottom":100}]}"#
                .to_string()
        } else {
            "{}".to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let img = service.img_proc_service().expect("图片处理服务存在");

    let sr = img
        .super_resolution("http://img/sr.jpg")
        .await
        .expect("超分辨率成功");
    assert_eq!(sr.media_id, "MEDIA_SR");

    let crop = img
        .ai_crop("http://img/crop.jpg", Some("1:1"))
        .await
        .expect("智能裁剪成功");
    assert_eq!(crop.results.len(), 1);
    assert_eq!(crop.results[0].crop_right, 100);
}

// ========================================================================
// 菜单（覆盖 menu 29.41% → 90%+）
// ========================================================================

/// 对应 Java: WxMpMenuServiceImplTest.createGetDeleteSelfMenuInfo
#[tokio::test]
async fn menu_create_get_delete_self_info() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/menu/create") {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        } else if path.contains("/cgi-bin/menu/get") {
            r#"{"menu":{"button":[{"type":"click","name":"菜单","key":"V1001"}]}}"#.to_string()
        } else if path.contains("/cgi-bin/menu/delete") {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        } else if path.contains("/cgi-bin/get_current_selfmenu_info") {
            r#"{"selfmenu_info":{"button":[{"type":"click","name":"菜单","key":"V1001"}]}}"#
                .to_string()
        } else {
            "{}".to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let menu = service.menu_service().expect("菜单服务存在");

    let menu_data = wx_rust_mp::bean::menu::WxMpMenu::default();
    let result = menu.menu_create(&menu_data).await.expect("创建菜单成功");
    assert!(result.contains("ok"));

    let get_result = menu.menu_get().await.expect("获取菜单成功");
    assert!(get_result.contains("V1001"));

    let del_result = menu.menu_delete().await.expect("删除菜单成功");
    assert!(del_result.contains("ok"));

    let self_info = menu.get_self_menu_info().await.expect("获取自定义菜单成功");
    assert!(self_info.contains("selfmenu_info"));
}

// ========================================================================
// 模板消息（覆盖 template_msg 50% → 100%）
// ========================================================================

/// 对应 Java: WxMpTemplateMsgServiceImplTest.sendTemplateMsg
#[tokio::test]
async fn template_msg_send() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/message/template/send") {
            r#"{"errcode":0,"errmsg":"ok","msgid":123456}"#.to_string()
        } else {
            "{}".to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let tpl = service.template_msg_service().expect("模板消息服务存在");

    let mut msg = wx_rust_mp::bean::template::WxMpTemplateMessage::default();
    msg.to_user = Some("o1".to_string());
    msg.template_id = Some("TPL_1".to_string());
    msg.url = Some("https://example.com".to_string());
    let result = tpl.send_template_msg(&msg).await.expect("发送模板消息成功");
    assert!(result.contains("123456"));
}

// ========================================================================
// 二维码（覆盖 qrcode 50% → 100%）
// ========================================================================

/// 对应 Java: WxMpQrcodeServiceImplTest.createQrCodeTicket
#[tokio::test]
async fn qrcode_create_ticket() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/qrcode/create") {
            r#"{"ticket":"TICKET_VALUE","expire_seconds":1800,"url":"https://mp.weixin.qq.com/cgi-bin/showqrcode?ticket=TICKET_VALUE"}"#.to_string()
        } else {
            "{}".to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let qr = service.qrcode_service().expect("二维码服务存在");

    let ticket = qr
        .qrcode_create_ticket("QR_STR_SCENE", "test_scene")
        .await
        .expect("创建二维码成功");
    assert_eq!(ticket.ticket, "TICKET_VALUE");
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["action_name"], "QR_STR_SCENE");
    assert_eq!(body["action_info"]["scene"]["scene_str"], "test_scene");
}

// ========================================================================
// 用户（覆盖 user 50% → 100%）
// ========================================================================

/// 对应 Java: WxMpUserServiceImplTest.userInfo
#[tokio::test]
async fn user_info_get() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/user/info") {
            r#"{"openid":"o1","nickname":"张三","sex":1,"language":"zh_CN","city":"深圳","province":"广东","country":"中国","headimgurl":"http://h","subscribe":1,"subscribe_time":1700000000}"#.to_string()
        } else {
            "{}".to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let user = service.user_service().expect("用户服务存在");

    let info = user.user_info("o1").await.expect("获取用户信息成功");
    assert_eq!(info.open_id, "o1");
    assert_eq!(info.nickname, "张三");
}

// ========================================================================
// 客服扩展（覆盖 kefu 38.24% → 80%+）
// ========================================================================

/// 对应 Java: WxMpKefuServiceImplTest 增量：sendKefuMessage / kfOnlineList /
/// kfAccountAdd / kfAccountUpdate / kfAccountInviteWorker / kfAccountDel /
/// kfSessionClose / kfSessionList
#[tokio::test]
async fn kefu_send_message_and_account_ops() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/message/custom/send") {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        } else if path.contains("/cgi-bin/customservice/getonlinekflist") {
            r#"{"kf_online_list":[{"kf_account":"test1@test","status":1,"kf_id":"1001"}]}"#
                .to_string()
        } else if path.contains("/customservice/kfaccount/add") {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        } else if path.contains("/customservice/kfaccount/update") {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        } else if path.contains("/customservice/kfaccount/inviteworker") {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        } else if path.contains("/customservice/kfaccount/del") {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        } else if path.contains("/customservice/kfsession/close") {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        } else if path.contains("/customservice/kfsession/getsessionlist") {
            r#"{"sessionlist":[{"openid":"o1","createtime":1700000000}]}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let kefu = service.kefu_service().expect("客服服务存在");

    // 发送客服消息
    let mut msg = wx_rust_mp::bean::kefu::WxMpKefuMessage::default();
    msg.to_user = Some("o1".to_string());
    msg.msg_type = Some("text".to_string());
    msg.content = Some("你好".to_string());
    let result = kefu
        .send_kefu_message(&msg)
        .await
        .expect("发送客服消息成功");
    assert!(result.contains("ok"));

    // 在线客服列表
    let online = kefu.kf_online_list().await.expect("在线客服列表成功");
    assert_eq!(online.kf_online_list.len(), 1);
    assert_eq!(online.kf_online_list[0].account, "test1@test");

    // 添加客服账号
    let mut req = wx_rust_mp::bean::kefu::request::WxMpKfAccountRequest::default();
    req.kf_account = "new@test".to_string();
    req.nick_name = "新客服".to_string();
    assert!(kefu.kf_account_add(&req).await.expect("添加客服成功"));

    // 更新客服账号
    assert!(kefu.kf_account_update(&req).await.expect("更新客服成功"));

    // 邀请客服
    assert!(
        kefu.kf_account_invite_worker(&req)
            .await
            .expect("邀请客服成功")
    );

    // 删除客服
    assert!(kefu.kf_account_del("old@test").await.expect("删除客服成功"));

    // 关闭会话
    let mut session_req = wx_rust_mp::bean::kefu::request::WxMpKfSessionRequest::default();
    session_req.openid = "o1".to_string();
    session_req.kf_account = "test1@test".to_string();
    assert!(
        kefu.kf_session_close(&session_req)
            .await
            .expect("关闭会话成功")
    );

    // 会话列表
    let session_list = kefu
        .kf_session_list("test1@test")
        .await
        .expect("会话列表成功");
    assert_eq!(session_list.kf_session_list.len(), 1);
    assert_eq!(session_list.kf_session_list[0].openid, "o1");
}

// ========================================================================
// 卡券扩展（覆盖 card 27.27% → 75%+）
// ========================================================================

/// 对应 Java: WxMpCardServiceImplTest 增量：consumeCardCode / markCardCode /
/// getCardDetail / deleteCard / createLandingPage
#[tokio::test]
async fn card_consume_mark_detail_delete_landing() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/card/code/consume") {
            r#"{"card":{"card_id":"CARD_1"},"errcode":0,"errmsg":"ok"}"#.to_string()
        } else if path.contains("/card/code/mark") {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        } else if path.contains("/card/get") {
            r#"{"card":{"card_id":"CARD_1","card_type":"GROUPON","groupon":{"base_info":{"code_type":"CODE_TYPE_TEXT"}}}}"#.to_string()
        } else if path.contains("/card/delete") {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        } else if path.contains("/card/landingpage/create") {
            r#"{"url":"https://card.landing.page","errcode":0,"errmsg":"ok"}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let card = service.card_service().expect("卡券服务存在");

    // 消费卡券
    let card_id = card
        .consume_card_code("CODE_1")
        .await
        .expect("消费卡券成功");
    assert_eq!(card_id, "CARD_1");

    // 标记卡券
    card.mark_card_code("CODE_1", "CARD_1", "o1", true)
        .await
        .expect("标记卡券成功");

    // 获取卡券详情
    let detail = card
        .get_card_detail("CARD_1")
        .await
        .expect("获取卡券详情成功");
    assert!(detail.contains("CARD_1"));

    // 删除卡券
    card.delete_card("CARD_1").await.expect("删除卡券成功");

    // 创建落地页
    let req = wx_rust_mp::bean::card::WxMpCardLandingPageCreateRequest::default();
    card.create_landing_page(&req)
        .await
        .expect("创建落地页成功");
}

// ========================================================================
// 设备扩展（覆盖 device 16.67% → 85%+）
// ========================================================================

/// 对应 Java: WxMpDeviceServiceImplTest 增量：transMsg / authorize / compelBind /
/// unbind / compelUnbind / getOpenId / getBindDevice
#[tokio::test]
async fn device_trans_msg_authorize_bind_unbind() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/device/transmsg") {
            r#"{"base_resp":{"errcode":0,"errmsg":"ok"}}"#.to_string()
        } else if path.contains("/device/authorize_device") {
            r#"{"base_resp":{"errcode":0,"errmsg":"ok"}}"#.to_string()
        } else if path.contains("/device/compel_bind") {
            r#"{"base_resp":{"base_info":{"device_type":"gh_xxx","device_id":"D1"},"errcode":0,"errmsg":"ok"}}"#.to_string()
        } else if path.contains("/device/unbind") {
            r#"{"base_resp":{"base_info":{"device_type":"gh_xxx","device_id":"D1"},"errcode":0,"errmsg":"ok"}}"#.to_string()
        } else if path.contains("/device/compel_unbind") {
            r#"{"base_resp":{"base_info":{"device_type":"gh_xxx","device_id":"D1"},"errcode":0,"errmsg":"ok"}}"#.to_string()
        } else if path.contains("/device/get_openid") {
            r#"{"open_id":["o1"],"device_type":"gh_xxx","device_id":"D1","errcode":0,"errmsg":"ok"}"#.to_string()
        } else if path.contains("/device/get_bind_device") {
            r#"{"device_list":[{"device_id":"D1","device_type":"gh_xxx"}],"resp_msg":{"errcode":0,"errmsg":"ok"}}"#.to_string()
        } else {
            r#"{"base_resp":{"errcode":0,"errmsg":"ok"}}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let device = service.device_service().expect("设备服务存在");

    // 设备消息透传
    let mut msg = wx_rust_mp::bean::device::WxDeviceMsg::default();
    msg.device_type = "gh_xxx".to_string();
    msg.device_id = "D1".to_string();
    msg.open_id = "o1".to_string();
    msg.content = "aGVsbG8=".to_string(); // base64 "hello"
    let resp = device.trans_msg(&msg).await.expect("消息透传成功");
    assert_eq!(resp.err_code, 0);

    // 设备授权
    let mut auth = wx_rust_mp::bean::device::WxDeviceAuthorize::default();
    auth.device_num = "1".to_string();
    let auth_resp = device.authorize(&auth).await.expect("设备授权成功");
    assert_eq!(auth_resp.resp.len(), 0); // Vec<BaseResp> 默认空

    // 强制绑定
    let mut bind = wx_rust_mp::bean::device::WxDeviceBind::default();
    bind.ticket = "TICKET".to_string();
    bind.device_id = "D1".to_string();
    bind.open_id = "o1".to_string();
    let bind_resp = device.compel_bind(&bind).await.expect("强制绑定成功");
    assert_eq!(bind_resp.base_resp.err_code, 0);

    // 解绑
    let unbind_resp = device.unbind(&bind).await.expect("解绑成功");
    assert_eq!(unbind_resp.base_resp.err_code, 0);

    // 强制解绑
    let compel_unbind_resp = device.compel_unbind(&bind).await.expect("强制解绑成功");
    assert_eq!(compel_unbind_resp.base_resp.err_code, 0);

    // 获取 OpenId
    let openid_resp = device
        .get_open_id("gh_xxx", "D1")
        .await
        .expect("获取 OpenId 成功");
    assert!(openid_resp.open_ids.contains(&"o1".to_string()));

    // 获取绑定设备
    let bind_dev = device
        .get_bind_device("o1")
        .await
        .expect("获取绑定设备成功");
    assert_eq!(bind_dev.devices.len(), 1);
    assert_eq!(bind_dev.devices[0].device_id, "D1");
}

// ========================================================================
// AI 开放（覆盖 ai_open 10.71% → 80%+）
// ========================================================================

/// 对应 Java: WxMpAiOpenServiceImplTest.queryRecognitionResult / translate
#[tokio::test]
async fn ai_open_query_and_translate() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/media/voice/queryrecoresultfortext") {
            r#"{"result":"识别结果文本","errcode":0,"errmsg":"ok"}"#.to_string()
        } else if path.contains("/cgi-bin/media/voice/translatecontent") {
            r#"{"to_content":"Translated text","errcode":0,"errmsg":"ok"}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let ai = service.ai_open_service().expect("AI 开放服务存在");

    let result = ai
        .query_recognition_result("voice_1", "zh_CN")
        .await
        .expect("查询识别结果成功");
    assert_eq!(result, "识别结果文本");

    let translated = ai
        .translate("zh_CN", "en_US", "你好世界")
        .await
        .expect("翻译成功");
    assert_eq!(translated, "Translated text");
}

// ========================================================================
// 草稿箱扩展（覆盖 draft 40.85% → 85%+）
// ========================================================================

/// 对应 Java: WxMpDraftServiceImplTest 增量：updateDraft / getDraft / delDraft
#[tokio::test]
async fn draft_update_get_delete() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/draft/update") {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        } else if path.contains("/cgi-bin/draft/get") {
            r#"{"news_item":[{"title":"更新标题","content":"内容","thumb_media_id":"THUMB","author":"作者","show_cover_pic":1}]}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let draft = service.draft_service().expect("草稿箱服务存在");

    // 更新草稿
    let mut update = wx_rust_mp::bean::draft::WxMpUpdateDraft::default();
    update.media_id = "DRAFT_MEDIA_ID".to_string();
    update.articles = wx_rust_mp::bean::draft::WxMpDraftArticles::default();
    assert!(draft.update_draft(&update).await.expect("更新草稿成功"));

    // 获取草稿
    let info = draft
        .get_draft("DRAFT_MEDIA_ID")
        .await
        .expect("获取草稿成功");
    assert_eq!(info.news_item.len(), 1);
    assert_eq!(info.news_item[0].title, "更新标题");

    // 删除草稿
    assert!(
        draft
            .del_draft("DRAFT_MEDIA_ID")
            .await
            .expect("删除草稿成功")
    );
}

// ========================================================================
// 发布扩展（覆盖 free_publish 42.37% → 85%+）
// ========================================================================

/// 对应 Java: WxMpFreePublishServiceImplTest 增量：deletePush / getArticle / getPublicationRecords
#[tokio::test]
async fn free_publish_delete_get_article_records() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/freepublish/delete") {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        } else if path.contains("/cgi-bin/freepublish/getarticle") {
            r#"{"news_item":[{"title":"已发布标题","author":"作者"}]}"#.to_string()
        } else if path.contains("/cgi-bin/freepublish/batchget") {
            r#"{"total_count":1,"item_count":1,"item":[{"article_id":"ART_1","content":{"news_item":[{"title":"标题"}]},"update_time":"1700000000"}]}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let publish = service.free_publish_service().expect("发布服务存在");

    // 删除发布
    assert!(publish.delete_push("ART_1", 0).await.expect("删除发布成功"));

    // 获取已发布文章
    let info = publish
        .get_article_from_id("ART_1")
        .await
        .expect("获取已发布文章成功");
    assert_eq!(info.news_item[0].title, "已发布标题");

    // 获取发布记录列表
    let list = publish
        .get_publication_records(0, 10)
        .await
        .expect("获取发布记录成功");
    assert_eq!(list.total_count, 1);
    assert_eq!(list.items[0].article_id, "ART_1");
}

// ========================================================================
// 报销发票扩展（覆盖 reimburse_invoice 17.86% → 85%+）
// ========================================================================

/// 对应 Java: WxMpReimburseInvoiceServiceImplTest 增量：getInvoiceBatch /
/// updateInvoiceStatus / updateStatusBatch
#[tokio::test]
async fn reimburse_invoice_batch_update() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/invoice/getinvoicebatch") {
            r#"{"invoice_list":[{"card_id":"CARD_1","begin_time":1700000000,"end_time":1700000100,"payee":"腾讯","detail":"发票"}]}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let reimburse = service
        .reimburse_invoice_service()
        .expect("报销发票服务存在");

    // 批量获取发票
    let mut batch_req = wx_rust_mp::bean::invoice::reimburse::InvoiceBatchRequest::default();
    let mut item = wx_rust_mp::bean::invoice::reimburse::InvoiceInfoRequest::default();
    item.card_id = "CARD_1".to_string();
    item.encrypt_code = "CODE_1".to_string();
    batch_req.item_list.push(item);
    let batch = reimburse
        .get_invoice_batch(&batch_req)
        .await
        .expect("批量获取发票成功");
    assert_eq!(batch.len(), 1);
    assert_eq!(batch[0].card_id, "CARD_1");

    // 更新发票状态
    let mut status_req =
        wx_rust_mp::bean::invoice::reimburse::UpdateInvoiceStatusRequest::default();
    status_req.card_id = "CARD_1".to_string();
    status_req.encrypt_code = "CODE_1".to_string();
    status_req.reimburse_status = "INVOICE_REIMBURSE_INIT".to_string();
    reimburse
        .update_invoice_status(&status_req)
        .await
        .expect("更新发票状态成功");

    // 批量更新状态
    let mut batch_status =
        wx_rust_mp::bean::invoice::reimburse::UpdateStatusBatchRequest::default();
    batch_status.openid = "o1".to_string();
    batch_status.reimburse_status = "INVOICE_REIMBURSE_INIT".to_string();
    batch_status
        .invoice_list
        .push(wx_rust_mp::bean::invoice::reimburse::InvoiceInfoRequest {
            card_id: "CARD_1".to_string(),
            encrypt_code: "CODE_1".to_string(),
        });
    reimburse
        .update_status_batch(&batch_status)
        .await
        .expect("批量更新发票状态成功");
}

// ========================================================================
// 门店扩展（覆盖 store 56.10% → 90%+）
// ========================================================================

/// 对应 Java: WxMpStoreServiceImplTest 增量：del / update
#[tokio::test]
async fn store_del_and_update() {
    let server = MockServer::start(dispatch(|_path| {
        r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let store = service.store_service().expect("门店服务存在");

    let mut info = wx_rust_mp::bean::store::WxMpStoreBaseInfo::default();
    info.business_name = "更新门店".to_string();
    store.add(&info).await.expect("添加门店成功");
    store.update(&info).await.expect("更新门店成功");
    store.delete("POI_ID").await.expect("删除门店成功");
}

// ========================================================================
// 评论扩展（覆盖 comment 66.04% → 90%+）
// ========================================================================

/// 对应 Java: WxMpCommentServiceImplTest 增量：unmarkElect / delete / replyDelete
#[tokio::test]
async fn comment_unmark_delete_reply_delete() {
    let server = MockServer::start(dispatch(|_path| {
        r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let comment = service.comment_service().expect("评论服务存在");

    comment
        .unmark_elect("MSG_1", Some(0), 100)
        .await
        .expect("取消精选成功");
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["user_comment_id"], 100);

    comment
        .delete("MSG_1", Some(0), 200)
        .await
        .expect("删除评论成功");

    comment
        .reply_delete("MSG_1", Some(0), 100)
        .await
        .expect("删除回复成功");
}

// ========================================================================
// 摇一摇扩展（覆盖 shake 30.43% → 80%+）
// ========================================================================

/// 对应 Java: WxMpShakeServiceImplTest 增量：getShakeInfo / deviceBindPage
#[tokio::test]
async fn shake_get_info_and_device_bind_page() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/shakearound/user/getshakeinfo") {
            r#"{"data":{"page_id":"100","beacon_info":{"distance":1.5},"openid":"o1"}}"#.to_string()
        } else if path.contains("/shakearound/device/bindpage") {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let shake = service.shake_service().expect("摇一摇服务存在");

    let mut query = wx_rust_mp::bean::WxMpShakeQuery::default();
    query.ticket = "ticket_value".to_string();
    let info = shake
        .get_shake_info(&query)
        .await
        .expect("获取摇一摇信息成功");
    assert_eq!(info.data.page_id, "100");
    assert_eq!(info.data.openid, "o1");

    let bind_query = wx_rust_mp::bean::shake::WxMpShakeAroundDeviceBindPageQuery::default();
    shake
        .device_bind_page_query(&bind_query)
        .await
        .expect("设备关联页面成功");
}

// ========================================================================
// 订阅消息扩展（覆盖 subscribe_msg 15.71% → 80%+）
// ========================================================================

/// 对应 Java: WxMpSubscribeMsgServiceImplTest 增量：sendOnce /
/// getPubTemplateTitleList / getPubTemplateKeyWordsById / addTemplate / delTemplate
#[tokio::test]
async fn subscribe_msg_send_once_and_template_ops() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/message/template/subscribe") {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        } else if path.contains("/wxaapi/newtmpl/getpubtemplatetitles") {
            r#"{"data":[{"tid":1,"title":"公共模板","type":1,"category":"教育"}],"count":1}"#.to_string()
        } else if path.contains("/wxaapi/newtmpl/getpubtemplatekeywords") {
            r#"{"data":[{"kid":1,"name":"内容","example":"示例"},{"kid":2,"name":"备注","example":"备注示例"}]}"#.to_string()
        } else if path.contains("/wxaapi/newtmpl/addtemplate") {
            r#"{"priTmplId":"PRI_TPL_1","errcode":0,"errmsg":"ok"}"#.to_string()
        } else if path.contains("/wxaapi/newtmpl/deltemplate") {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let subscribe = service.subscribe_msg_service().expect("订阅消息服务存在");

    // 一次性订阅
    let mut msg = wx_rust_mp::bean::subscribe::WxMpSubscribeMessage::default();
    msg.to_user = Some("o1".to_string());
    msg.template_id = Some("TPL_1".to_string());
    msg.content_value = Some("内容".to_string());
    let ok = subscribe.send_once(&msg).await.expect("一次性订阅成功");
    assert!(ok);

    // 获取公共模板标题列表
    let titles = subscribe
        .get_pub_template_title_list(&[], 0, 10)
        .await
        .expect("获取公共模板标题成功");
    assert_eq!(titles.count, 1);
    assert_eq!(titles.data[0].tid, 1);

    // 获取公共模板关键词
    let keywords = subscribe
        .get_pub_template_key_words_by_id("T1")
        .await
        .expect("获取公共模板关键词成功");
    assert_eq!(keywords.len(), 2);
    assert_eq!(keywords[0].name, "内容");

    // 添加模板
    let pri_tmpl_id = subscribe
        .add_template("T1", &[1, 2], "场景描述")
        .await
        .expect("添加模板成功");
    assert_eq!(pri_tmpl_id, "PRI_TPL_1");

    // 删除模板
    let ok = subscribe
        .del_template("PRI_TPL_1")
        .await
        .expect("删除模板成功");
    assert!(ok);
}

// ========================================================================
// 广告扩展（覆盖 marketing 36% → 80%+）
// ========================================================================

/// 对应 Java: WxMpMarketingServiceImplTest 增量：getUserActionSets / addUserAction
#[tokio::test]
async fn marketing_get_action_sets_and_add_action() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/marketing/user_action_sets/get") {
            r#"{"user_action_set":[{"user_action_set_id":100,"description":"注册行为","activate_status":true}]}"#.to_string()
        } else if path.contains("/cgi-bin/marketing/user_actions/add") {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let marketing = service.marketing_service().expect("广告服务存在");

    // 获取行为数据源
    let result = marketing
        .get_user_action_sets(100)
        .await
        .expect("获取行为数据源成功");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].user_action_set_id, 100);

    // 添加用户行为
    let action = wx_rust_mp::bean::marketing::WxMpUserAction {
        user_action_set_id: 100,
        url: "https://example.com".to_string(),
        action_time: 1700000000,
        action_type: "WEB".to_string(),
        ..Default::default()
    };
    marketing
        .add_user_action(&[action])
        .await
        .expect("添加用户行为成功");
}

// ========================================================================
// 数据统计扩展（覆盖 datacube 69.23% → 90%+）
// ========================================================================

/// 对应 Java: WxMpDataCubeServiceImplTest 增量：getUserCumulate / getArticleSummary
#[tokio::test]
async fn datacube_user_cumulate_and_article_summary() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/datacube/getusercumulate") {
            r#"{"list":[{"ref_date":"2024-01-01","cumulate_user":1000}]}"#.to_string()
        } else if path.contains("/datacube/getarticlesummary") {
            r#"{"list":[{"ref_date":"2024-01-01","msgid":"12003_3","int_page_read_user":50,"int_page_read_count":100}]}"#.to_string()
        } else {
            r#"{"list":[]}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let datacube = service.data_cube_service().expect("数据统计服务存在");

    let cumulate = datacube
        .get_user_cumulate("2024-01-01", "2024-01-01")
        .await
        .expect("累计用户数据成功");
    assert_eq!(cumulate.len(), 1);
    assert_eq!(cumulate[0].cumulate_user, 1000);

    let summary = datacube
        .get_article_summary("2024-01-01", "2024-01-01")
        .await
        .expect("图文概况数据成功");
    assert_eq!(summary.len(), 1);
    assert_eq!(summary[0].int_page_read_user, 50);
}

// ========================================================================
// 群发扩展（覆盖 mass_message 72.69% → 90%+）
// ========================================================================

/// 对应 Java: WxMpMassMessageServiceImplTest 增量：delete / speedGet / speedSet / get
#[tokio::test]
async fn mass_message_delete_speed_get_set() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/message/mass/delete") {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        } else if path.contains("/cgi-bin/message/mass/speed/get") {
            r#"{"speed":0,"realspeed":10}"#.to_string()
        } else if path.contains("/cgi-bin/message/mass/speed/set") {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        } else if path.contains("/cgi-bin/message/mass/get") {
            r#"{"msg_id":34182,"msg_status":"SEND_SUCCESS"}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let mass = service.mass_message_service().expect("群发服务存在");

    // 删除群发
    mass.delete(34182, 0).await.expect("删除群发成功");

    // 获取群发速度
    let speed = mass
        .message_mass_speed_get()
        .await
        .expect("获取群发速度成功");
    assert_eq!(speed.speed, 0);
    assert_eq!(speed.realspeed, 10);

    // 设置群发速度
    mass.message_mass_speed_set(10)
        .await
        .expect("设置群发速度成功");

    // 获取群发状态
    let get_result = mass
        .message_mass_get(34182)
        .await
        .expect("获取群发状态成功");
    assert_eq!(get_result.msgstatus, "SEND_SUCCESS");
}

// ========================================================================
// Wi-Fi 扩展（覆盖 wifi 75% → 100%）
// ========================================================================

/// 对应 Java: WxMpWifiServiceImplTest 增量：updateShopWifiInfo 扩展
#[tokio::test]
async fn wifi_update_shop_wifi() {
    let server = MockServer::start(dispatch(|_path| {
        r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let wifi = service.wifi_service().expect("Wi-Fi 服务存在");

    assert!(
        wifi.update_shop_wifi_info(100, "OLD_SSID", "NEW_SSID", Some("newpass"))
            .await
            .expect("更新 Wi-Fi 成功")
    );
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["shop_id"], 100);
    assert_eq!(body["old_ssid"], "OLD_SSID");
    assert_eq!(body["ssid"], "NEW_SSID");
    assert_eq!(body["password"], "newpass");
}
