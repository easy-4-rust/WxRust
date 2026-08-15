#![allow(clippy::field_reassign_with_default)]
//! mp 子域服务测试（第二批：material/shake/card/memberCard/guide/marketing/
//! subscribeMsg/aiOpen/ocr/imgProc/reimburseInvoice/merchantInvoice/kefu 扩展）。
//!
//! 镜像 Java `WxMp*ServiceImplTest` 的 HTTP 语义（路径/payload/响应解析），
//! 经 MockServer 验证；线格式对齐 Java @SerializedName 与 Gson adapter。

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
        if path.contains("/cgi-bin/token") {
            return r#"{"access_token":"MOCK_TOKEN","expires_in":7200}"#.to_string();
        }
        handler(path)
    }
}

// ---- 素材（镜像 Java WxMpMaterialServiceTest） ----

#[tokio::test]
async fn material_count_and_batch_get() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/material/get_materialcount") {
            r#"{"voiceCount":1,"videoCount":2,"imageCount":3,"newsCount":4}"#.to_string()
        } else if path.contains("/cgi-bin/material/batchget_material") {
            r#"{"totalCount":1,"itemCount":1,"items":[{"mediaId":"M1","updateTime":"1700000000","content":{"news_item":[]}}]}"#.to_string()
        } else if path.contains("/cgi-bin/material/get_material") {
            r#"{"articles":[{"title":"标题"}]}"#.to_string()
        } else if path.contains("/cgi-bin/material/del_material") {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        } else {
            "{}".to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let material_service = service.material_service().expect("素材服务存在");

    let count = material_service
        .material_count()
        .await
        .expect("素材计数成功");
    assert_eq!(count.voice_count, 1);
    assert_eq!(count.news_count, 4);

    let batch = material_service
        .material_news_batch_get(0, 10)
        .await
        .expect("图文素材列表成功");
    assert_eq!(batch.total_count, 1);
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["type"], "news");
    assert_eq!(body["offset"], 0);

    let news = material_service
        .material_news_info("M1")
        .await
        .expect("图文素材详情成功");
    assert_eq!(news.articles.len(), 1);
    assert_eq!(news.articles[0].title, "标题");

    assert!(
        material_service
            .material_delete("M1")
            .await
            .expect("删除素材成功")
    );
}

// ---- 摇一摇周边（镜像 Java WxMpShakeServiceTest） ----

#[tokio::test]
async fn shake_page_add_and_relation_search() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/shakearound/page/add") {
            r#"{"pageId":100,"errcode":0,"errmsg":"ok"}"#.to_string()
        } else if path.contains("/shakearound/relation/search") {
            r#"{"data":{"relations":[{"device_id":1,"page_id":100}],"total_count":1}}"#.to_string()
        } else {
            "{}".to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let shake_service = service.shake_service().expect("摇一摇服务存在");

    let mut query = wx_rust_mp::bean::shake::WxMpShakeAroundPageAddQuery::default();
    query.title = "页面".to_string();
    let result = shake_service.page_add(&query).await.expect("页面添加成功");
    assert_eq!(result.page_id, 100);

    let mut rel = wx_rust_mp::bean::shake::WxMpShakeAroundRelationSearchQuery::default();
    rel.page_id = 100;
    let rel_result = shake_service
        .relation_search(&rel)
        .await
        .expect("关系查询成功");
    assert_eq!(rel_result.data.total_count, 1);
}

// ---- 卡券（镜像 Java WxMpCardServiceTest） ----

#[tokio::test]
async fn card_create_and_query() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/card/create") {
            r#"{"card_id":"CARD_1","errcode":0,"errmsg":"ok"}"#.to_string()
        } else if path.contains("/card/code/get") {
            r#"{"openid":"o1","canConsume":true,"userCardStatus":"NORMAL"}"#.to_string()
        } else if path.contains("/card/code/decrypt") {
            r#"{"code":"CODE_1","errcode":0,"errmsg":"ok"}"#.to_string()
        } else if path.contains("/card/qrcode/create") {
            r#"{"ticket":"TICKET","url":"http://x"}"#.to_string()
        } else if path.contains("/card/testwhitelist/set") {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        } else {
            "{}".to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let card_service = service.card_service().expect("卡券服务存在");

    let request = wx_rust_mp::bean::card::WxMpCardCreateRequest::default();
    let result = card_service
        .create_card(&request)
        .await
        .expect("创建卡券成功");
    assert_eq!(result.card_id, "CARD_1");

    let result = card_service
        .query_card_code("CARD_1", "CODE_1", true)
        .await
        .expect("查询卡券成功");
    assert!(result.can_consume);

    let code = card_service
        .decrypt_card_code("ENCRYPT")
        .await
        .expect("解密卡券成功");
    assert_eq!(code, "CODE_1");

    let qr = card_service
        .create_qrcode_card("CARD_1", "outer", Some(1800))
        .await
        .expect("卡券二维码成功");
    assert_eq!(qr.ticket, "TICKET");
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["action_name"], "QR_CARD");
    assert_eq!(body["action_info"]["card"]["card_id"], "CARD_1");
    assert_eq!(body["expire_seconds"], 1800);

    card_service
        .add_test_white_list("o1")
        .await
        .expect("白名单成功");
}

// ---- 会员卡（镜像 Java WxMpMemberCardServiceTest） ----

#[tokio::test]
async fn member_card_create_and_activate() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/card/membercard/activate") {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        } else if path.contains("/card/membercard/userinfo/get") {
            r#"{"openId":"o1","nickname":"张三","membershipNumber":"100"}"#.to_string()
        } else {
            r#"{"card_id":"MCARD_1","errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let member_service = service.member_card_service().expect("会员卡服务存在");

    let message = wx_rust_mp::bean::card::membercard::WxMpMemberCardCreateMessage::default();
    let card_id = member_service
        .create_member_card(&message)
        .await
        .expect("创建会员卡成功");
    assert_eq!(card_id, "MCARD_1");

    let mut activated =
        wx_rust_mp::bean::card::membercard::WxMpMemberCardActivatedMessage::default();
    activated.membership_number = "100".to_string();
    member_service
        .activate_member_card(&activated)
        .await
        .expect("激活会员卡成功");

    let info = member_service
        .get_user_info("MCARD_1", "CODE_1")
        .await
        .expect("会员卡用户信息成功");
    assert_eq!(info.open_id, "o1");
}

// ---- 顾问（镜像 Java WxMpGuideServiceTest） ----

#[tokio::test]
async fn guide_list_and_acct_config() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/guide/getguideacctlist") {
            r#"{"total_num":1,"list":[{"guide_account":"acc1","guide_openid":"o1","guide_nickname":"顾问"}]}"#.to_string()
        } else if path.contains("/cgi-bin/guide/getguideacctconfig") {
            r#"{"black_keyword":{"values":["违禁"]},"guide_auto_reply":{"msg_type":"text","content":"不在线"}}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let guide_service = service.guide_service().expect("顾问服务存在");

    let list = guide_service.list_guide(0, 10).await.expect("顾问列表成功");
    assert_eq!(list.total_num, 1);
    assert_eq!(list.list[0].account, "acc1");

    let config = guide_service
        .get_guide_acct_config()
        .await
        .expect("顾问配置成功");
    assert_eq!(config.guide_sensitive_words.values.len(), 1);

    guide_service
        .del_guide("acc1", "o1")
        .await
        .expect("删除顾问成功");
}

// ---- 广告（镜像 Java WxMpMarketingServiceTest） ----

#[tokio::test]
async fn marketing_add_action_sets_and_ad_leads() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/marketing/user_action_sets/add") {
            r#"{"user_action_set_id":100}"#.to_string()
        } else if path.contains("/marketing/wechat_ad_leads/get") {
            r#"{"page_info":{"total_number":1},"list":[]}"#.to_string()
        } else {
            "{}".to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let marketing_service = service.marketing_service().expect("广告服务存在");

    let id = marketing_service
        .add_user_action_sets("WEB", "注册", "注册行为")
        .await
        .expect("创建行为数据源成功");
    assert_eq!(id, 100);

    let leads = marketing_service
        .get_ad_leads("2024-01-01", "2024-01-07", &[], 1, 10)
        .await
        .expect("广告数据成功");
    assert_eq!(leads.page_info.total_number, 1);
}

// ---- 订阅消息（镜像 Java WxMpSubscribeMsgServiceTest） ----

#[tokio::test]
async fn subscribe_msg_send_and_templates() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/message/subscribe/bizsend") {
            r#"{"errcode":0,"errmsg":"ok","msgid":123456}"#.to_string()
        } else if path.contains("/wxaapi/newtmpl/gettemplate") {
            r#"{"errcode":0,"errmsg":"ok","data":[{"priTmplId":"TPL_1","title":"模板"}]}"#
                .to_string()
        } else if path.contains("/wxaapi/newtmpl/getcategory") {
            r#"{"errcode":0,"errmsg":"ok","data":[{"id":1,"name":"教育"}]}"#.to_string()
        } else {
            "{}".to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let subscribe_service = service.subscribe_msg_service().expect("订阅消息服务存在");

    let mut message = wx_rust_mp::bean::subscribe::WxMpSubscribeMessage::default();
    message.to_user = Some("o1".to_string());
    message.template_id = Some("TPL_1".to_string());
    message.content_value = Some("内容".to_string());
    let msgid = subscribe_service
        .send(&message)
        .await
        .expect("订阅消息发送成功");
    assert!(msgid.contains("123456"));
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["touser"], "o1");
    assert_eq!(body["template_id"], "TPL_1");

    let templates = subscribe_service
        .get_template_list()
        .await
        .expect("模板列表成功");
    assert_eq!(templates.len(), 1);
    assert_eq!(templates[0].pri_tmpl_id, "TPL_1");

    let categories = subscribe_service
        .get_category()
        .await
        .expect("类目列表成功");
    assert_eq!(categories[0].name, "教育");
}

// ---- OCR / 图片处理（镜像 Java WxMpOcrServiceImplTest） ----

#[tokio::test]
async fn ocr_id_card_and_img_proc_qr() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/ocr/idcard") {
            r#"{"type":"idcard","name":"张三","id":"110101199001011234","addr":"北京市"}"#.to_string()
        } else if path.contains("/cgi-bin/imgproc/qrcode") {
            r#"{"img_size":{"w":100,"h":100},"code_results":[{"type_name":"QRCODE","data":"http://x","pos":{"left_top":{"x":0,"y":0}}}]}"#.to_string()
        } else {
            "{}".to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let ocr_service = service.ocr_service().expect("OCR 服务存在");

    let result = ocr_service
        .id_card("http://img/x.jpg")
        .await
        .expect("身份证识别成功");
    assert_eq!(result.name, "张三");
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["img_url"], "http://img/x.jpg");

    let img_proc_service = service.img_proc_service().expect("图片处理服务存在");
    let qr = img_proc_service
        .qr_code("http://img/x.jpg")
        .await
        .expect("二维码识别成功");
    assert_eq!(qr.code_results.len(), 1);
}

// ---- 发票（镜像 Java WxMpReimburseInvoiceServiceTest / WxMpMerchantInvoiceServiceTest） ----

#[tokio::test]
async fn invoice_reimburse_and_merchant() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/invoice/getinvoicedetail") {
            r#"{"card_id":"CARD_1","begin_time":1700000000,"end_time":1700000100,"payee":"腾讯","detail":"发票"}"#.to_string()
        } else if path.contains("/card/invoice/queryinvoceinfo") {
            r#"{"errcode":0,"errmsg":"ok","invoicedetail":{"fpqqlsh":"SH_1"}}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let reimburse_service = service
        .reimburse_invoice_service()
        .expect("报销发票服务存在");

    let mut request = wx_rust_mp::bean::invoice::reimburse::InvoiceInfoRequest::default();
    request.card_id = "CARD_1".to_string();
    let info = reimburse_service
        .get_invoice_info(&request)
        .await
        .expect("发票详情成功");
    assert_eq!(info.card_id, "CARD_1");

    let merchant_service = service
        .merchant_invoice_service()
        .expect("商户发票服务存在");
    let result = merchant_service
        .query_invoice_info("SH_1", "TAX_1")
        .await
        .expect("发票查询成功");
    assert_eq!(result.invoicedetail.fpqqlsh, "SH_1");
}

// ---- 客服扩展（镜像 Java WxMpKefuServiceImplTest） ----

#[tokio::test]
async fn kefu_account_and_session() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/customservice/getkflist") {
            r#"{"kf_list":[{"kf_account":"test1@test","kf_nick":"客服1","kf_id":"1001"}]}"#.to_string()
        } else if path.contains("/customservice/kfsession/getwaitcase") {
            r#"{"count":1,"waitcaselist":[{"openid":"o1","kf_account":"test1@test","createtime":1700000000}]}"#.to_string()
        } else if path.contains("/customservice/kfsession/getsession?openid=o1") {
            r#"{"kf_account":"test1@test","createtime":1700000000}"#.to_string()
        } else if path.contains("/cgi-bin/customservice/msgrecord/getmsglist") {
            r#"{"recordlist":[{"openid":"o1","opercode":2002,"text":"你好"}]}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let kefu_service = service.kefu_service().expect("客服服务存在");

    let list = kefu_service.kf_list().await.expect("客服列表成功");
    assert_eq!(list.kf_list.len(), 1);
    assert_eq!(list.kf_list[0].account, "test1@test");

    let wait = kefu_service
        .kf_session_get_wait_case()
        .await
        .expect("未接入会话成功");
    assert_eq!(wait.count, 1);

    let session = kefu_service
        .kf_session_get("o1")
        .await
        .expect("会话查询成功");
    assert_eq!(session.kf_account, "test1@test");

    let mut request = wx_rust_mp::bean::kefu::request::WxMpKfSessionRequest::default();
    request.openid = "o1".to_string();
    request.kf_account = "test1@test".to_string();
    assert!(
        kefu_service
            .kf_session_create(&request)
            .await
            .expect("会话创建成功")
    );

    let msg_list = kefu_service
        .kf_msg_list(1700000000, 1700000100, 0, 10)
        .await
        .expect("聊天记录成功");
    assert_eq!(msg_list.records.len(), 1);
    assert_eq!(msg_list.records[0].text, "你好");

    // Java 参数校验：number 超限/起始晚于结束报错
    let err = kefu_service
        .kf_msg_list(1700000000, 1700000100, 0, 10001)
        .await
        .expect_err("number 超限应报错");
    assert!(err.to_string().contains("10000"));
}

// ---- 顾问子服务（镜像 Java WxMpGuideBuyerServiceTest 等） ----

#[tokio::test]
async fn guide_buyer_and_tag_sub_services() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/guide/addguidebuyerrelation") {
            r#"{"buyer_resp":[{"openid":"o1","errcode":0,"errmsg":"ok"}]}"#.to_string()
        } else if path.contains("/cgi-bin/guide/getguidebuyerrelationlist") {
            r#"{"buyer_list":{"total_num":1,"list":[{"openid":"o1","buyer_nickname":"买家"}]}}"#
                .to_string()
        } else if path.contains("/cgi-bin/guide/newguidetagoption") {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        } else if path.contains("/cgi-bin/guide/getguidetagoption") {
            r#"{"tag_option":[{"tag_name":"VIP","tag_values":["金卡","银卡"]}]}"#.to_string()
        } else if path.contains("/cgi-bin/guide/getguidebuyertag") {
            r#"{"tag_values":["VIP"]}"#.to_string()
        } else if path.contains("/cgi-bin/guide/queryguidebuyerbytag") {
            r#"{"openid_list":["o1","o2"]}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));

    let buyer_service = service.guide_buyer_service().expect("顾问买家服务存在");
    let mut info = wx_rust_mp::bean::guide::WxMpAddGuideBuyerInfo::default();
    info.openid = "o1".to_string();
    info.nickname = "买家".to_string();
    let resp = buyer_service
        .add_guide_buyer_relation("acc1", "o1", &[info])
        .await
        .expect("添加买家关系成功");
    assert_eq!(resp.len(), 1);
    assert_eq!(resp[0].openid, "o1");

    let list = buyer_service
        .get_guide_buyer_relation_list("acc1", "o1", 0, 10)
        .await
        .expect("买家列表成功");
    assert_eq!(list.list[0].nickname, "买家");

    let tag_service = service.guide_tag_service().expect("顾问标签服务存在");
    tag_service
        .new_guide_tag_option("VIP", &["金卡".to_string(), "银卡".to_string()])
        .await
        .expect("新建标签成功");
    let options = tag_service
        .get_guide_tag_option()
        .await
        .expect("标签选项成功");
    assert_eq!(options[0].tag_name, "VIP");
    assert_eq!(options[0].values.len(), 2);

    let tags = tag_service
        .get_guide_buyer_tag("acc1", "o1", "o1", false)
        .await
        .expect("买家标签成功");
    assert_eq!(tags, vec!["VIP"]);

    let buyers = tag_service
        .query_guide_buyer_by_tag("acc1", "o1", 10, &["VIP".to_string()])
        .await
        .expect("按标签查询成功");
    assert_eq!(buyers.len(), 2);
}

#[tokio::test]
async fn guide_material_and_massed_job_sub_services() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/guide/getguidecardmaterial") {
            r#"{"card_material_list":[{"media_id":"M1","type":1,"title":"卡券"}]}"#.to_string()
        } else if path.contains("/cgi-bin/guide/getguideimagematerial") {
            r#"{"total_num":1,"model_list":[{"media_id":"IMG1","picurl":"http://x"}]}"#.to_string()
        } else if path.contains("/cgi-bin/guide/getguidemassendjoblist") {
            r#"{"list":[{"task_id":"T1","task_name":"群发任务"}]}"#.to_string()
        } else if path.contains("/cgi-bin/guide/getguidemassendjob") {
            r#"{"task_id":"T1","task_name":"群发任务","task_status":2}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));

    let material_service = service.guide_material_service().expect("顾问素材服务存在");
    material_service
        .set_guide_card_material("M1", 1, "卡券", "/path", "app1")
        .await
        .expect("设置卡券素材成功");
    let cards = material_service
        .get_guide_card_material(1)
        .await
        .expect("卡券素材列表成功");
    assert_eq!(cards[0].title, "卡券");

    let imgs = material_service
        .get_guide_image_material(1, 0, 10)
        .await
        .expect("图片素材列表成功");
    assert_eq!(imgs.list[0].pic_url, "http://x");

    let massed_service = service
        .guide_massed_job_service()
        .expect("顾问群发服务存在");
    let job = massed_service
        .get_guide_massed_job("T1")
        .await
        .expect("群发任务查询成功");
    assert_eq!(job.task_name, "群发任务");
    let jobs = massed_service
        .get_guide_massed_job_list("acc1", "o1", &[2], 0, 10)
        .await
        .expect("群发任务列表成功");
    assert_eq!(jobs.len(), 1);
    massed_service
        .cancel_guide_massed_job("T1")
        .await
        .expect("取消任务成功");
}

// ---- OAuth2（镜像 Java WxMpOAuth2ServiceImplTest） ----

#[tokio::test]
async fn oauth2_access_token_and_user_info() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/sns/oauth2/access_token") {
            r#"{"access_token":"OAUTH_TOKEN","expires_in":7200,"refresh_token":"REFRESH","openid":"o1","scope":"snsapi_userinfo"}"#.to_string()
        } else if path.contains("/sns/userinfo") {
            r#"{"openid":"o1","nickname":"NICK","sex":1,"language":"zh_CN","city":"深圳","province":"广东","country":"中国","headimgurl":"http://h","privilege":[]}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let dyn_arc: Arc<dyn WxMpService> = service.clone();
    let oauth2 = wx_rust_mp::api::r#impl::WxMpOAuth2ServiceImpl::new(Arc::downgrade(&dyn_arc));
    use wx_rust_common::service::WxOAuth2Service;

    let token = oauth2
        .get_access_token("CODE_1")
        .await
        .expect("获取 OAuth2 token 成功");
    assert_eq!(token.access_token, "OAUTH_TOKEN");
    assert_eq!(token.open_id, "o1");

    let user = oauth2
        .get_user_info(&token, "zh_CN")
        .await
        .expect("获取用户信息成功");
    assert_eq!(user.nickname, "NICK");
    assert_eq!(user.city, "深圳");
}

// ---- 客服 builder（对应 Java builder/kefu/*） ----

#[tokio::test]
async fn kefu_builders_text_and_news() {
    use wx_rust_mp::builder::kefu::NewsBuilder;
    use wx_rust_mp::builder::kefu::TextBuilder;

    let msg = TextBuilder::start()
        .to_user("OPENID")
        .content("sfsfdsdf")
        .build();
    assert_eq!(
        msg.to_json().expect("序列化成功"),
        "{\"touser\":\"OPENID\",\"msgtype\":\"text\",\"text\":{\"content\":\"sfsfdsdf\"}}"
    );

    let news = NewsBuilder::start().to_user("OPENID").build();
    assert_eq!(news.get_msg_type(), "news");
}
