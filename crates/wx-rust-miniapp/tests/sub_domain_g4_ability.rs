#![allow(clippy::field_reassign_with_default)]
//! miniapp G4 能力服务组集成测试（MockServer 模式，镜像 Java 测试语义）。
//!
//! 对应 Java `weixin-java-miniapp/src/test/java/cn/binarywang/wx/miniapp/api/impl/`
//! 下能力类子域测试：`WxMaLiveServiceImplTest` / `WxMaLiveGoodsServiceImplTest` /
//! `WxMaCloudServiceImplTest` / `WxMaVodServiceImplTest` / `WxMaXPayServiceImplTest` /
//! `WxMaMarketingServiceImplTest` / `WxMaPromotionServiceTest` /
//! `WxMaIntracityServiceImpleTest` / `WxMaComplaintServiceImplTest` /
//! `WxMaDeviceSubscribeServiceImplTest` / `WxMaFaceServiceImplTest` /
//! `WxMaReimburseInvoiceServiceImplTest` / `WxMaQrcodeJumpServiceImplTest`。
//!
//! 覆盖：直播（房间/商品）、云开发、短剧点播、虚拟支付（HMAC-SHA256 签名）、
//! 营销、推广、同城配送（snake_case 线格式）、交易投诉、设备订阅、人脸核身、
//! 报销发票、二维码快速跳转 13 个子服务的请求路径 / 请求体 / 响应解析。

use std::sync::Arc;
use std::sync::atomic::Ordering;

use wx_rust_miniapp::api::WxMaService;
use wx_rust_miniapp::bean::cloud::WxCloudSendSmsV2Request;
use wx_rust_miniapp::bean::complaint::{WxMaComplaintRequest, WxMaResponseRequest};
use wx_rust_miniapp::bean::device::{WxMaDeviceSubscribeMessageRequest, WxMaDeviceTicketRequest};
use wx_rust_miniapp::bean::face::{
    CertInfo, WxMaFaceGetVerifyIdRequest, WxMaFaceQueryVerifyInfoRequest,
};
use wx_rust_miniapp::bean::intractiy::{AddressInfo, WxMaPreAddOrderRequest, WxMaStore};
use wx_rust_miniapp::bean::invoice::{InvoiceBatchRequest, InvoiceInfoRequest};
use wx_rust_miniapp::bean::live::{WxMaLiveGoodInfo, WxMaLiveRoomInfo};
use wx_rust_miniapp::bean::marketing::WxMaUserAction;
use wx_rust_miniapp::bean::promoter::{WxMaPromotionAddRoleRequest, WxMaPromotionSendMsgRequest};
use wx_rust_miniapp::bean::qrcode::WxMaQrcodeJumpRule;
use wx_rust_miniapp::bean::vod::{WxMaVodListMediaRequest, WxMaVodPullUploadRequest};
use wx_rust_miniapp::bean::xpay::{WxMaXPayCurrencyPayRequest, WxMaXPaySigParams};
use wx_rust_miniapp::config::WxMaConfig;
use wx_rust_miniapp::config::r#impl::WxMaDefaultConfig;

/// 极简 mock HTTP 服务器：按请求路径返回固定响应，记录最近一次请求行与请求体。
struct MockServer {
    addr: std::net::SocketAddr,
    last_request_line: Arc<std::sync::Mutex<String>>,
    last_body: Arc<std::sync::Mutex<String>>,
    stop: Arc<std::sync::atomic::AtomicBool>,
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
        let last_request_line = Arc::new(std::sync::Mutex::new(String::new()));
        let last_body = Arc::new(std::sync::Mutex::new(String::new()));
        let stop = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let handler = Arc::new(handler);

        let last_request_line_clone = last_request_line.clone();
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
                let last_request_line_clone = last_request_line_clone.clone();
                let last_body_clone = last_body_clone.clone();
                tokio::spawn(async move {
                    use tokio::io::{AsyncReadExt, AsyncWriteExt};
                    let mut buf = [0u8; 65536];
                    let n = socket.read(&mut buf).await.unwrap_or(0);
                    let request = String::from_utf8_lossy(&buf[..n]).to_string();
                    // 记录请求行（含 query 串，如
                    // "POST /xpay/currency_pay?pay_sig=..&signature=..&access_token=.. HTTP/1.1"）
                    if let Some(line) = request.lines().next() {
                        *last_request_line_clone.lock().unwrap() = line.to_string();
                    }
                    // 记录请求体（POST / multipart 场景）
                    if let Some(idx) = request.find("\r\n\r\n") {
                        let body = request[idx + 4..].to_string();
                        *last_body_clone.lock().unwrap() = body;
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
            last_request_line,
            last_body,
            stop,
        }
    }

    fn url(&self, path: &str) -> String {
        format!("http://{}{}", self.addr, path)
    }

    fn last_request_line(&self) -> String {
        self.last_request_line.lock().unwrap().clone()
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

/// 构建指向 mock 服务器的小程序配置（可指定云环境 ID）。
fn config_with_host_and_cloud_env(host: &str, cloud_env: &str) -> Arc<dyn WxMaConfig> {
    let mut config = WxMaDefaultConfig::new("wxappid", "secret");
    config.set_cloud_env(cloud_env);
    let mut host_config = wx_rust_miniapp::config::WxMaHostConfig::new();
    host_config.api_host = host.to_string();
    config.set_host_config(host_config);
    Arc::new(config)
}

/// 构建指向 mock 服务器的小程序配置。
fn config_with_host(host: &str) -> Arc<dyn WxMaConfig> {
    config_with_host_and_cloud_env(host, "")
}

/// 通用路由 handler：token 请求 + 各子域响应。
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

/// 从请求行中提取 query 参数（`?key=value&...`）。
fn query_param(request_line: &str, key: &str) -> Option<String> {
    let query = request_line.split('?').nth(1)?;
    let query = query.split(' ').next()?;
    for pair in query.split('&') {
        let mut it = pair.splitn(2, '=');
        if it.next() == Some(key) {
            return it.next().map(|s| s.to_string());
        }
    }
    None
}

/// 校验字符串是 64 位十六进制小写（HMAC-SHA256 十六进制）。
fn is_lower_hex_64(s: &str) -> bool {
    s.len() == 64
        && s.chars().all(|c| c.is_ascii_hexdigit())
        && s.chars().all(|c| !c.is_ascii_uppercase())
}

// ---- 直播（镜像 Java WxMaLiveServiceImplTest.createRoom / getLiveInfo） ----

#[tokio::test]
async fn live_create_room_and_get_live_info() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/wxa/business/getliveinfo") {
            r#"{"errcode":0,"total":1,"room_info":[{"name":"直播间A","roomid":123,"cover_img":"COVER","live_status":101,"start_time":1700000000,"end_time":1700003600,"anchor_name":"主播"}]}"#.to_string()
        } else {
            r#"{"errcode":0,"roomId":123,"qrcode_url":"https://wx.qr"}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_miniapp::api::r#impl::WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let live = service.live_service().expect("直播服务存在");

    // createRoom：Java 语义为 post 序列化后的 WxMaLiveRoomInfo 到 /wxaapi/broadcast/room/create
    let mut room_info = WxMaLiveRoomInfo::default();
    room_info.name = "订阅通知直播间".to_string();
    room_info.cover_img = "MEDIA_1".to_string();
    room_info.anchor_name = "主播".to_string();
    room_info.r#type = 1;
    let result = live.create_room(&room_info).await.expect("创建直播间成功");
    assert_eq!(result.room_id, 123);
    assert_eq!(result.qrcode_url, "https://wx.qr");
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    // Java gson 序列化：camelCase 键
    assert_eq!(body["name"], "订阅通知直播间");
    assert_eq!(body["coverImg"], "MEDIA_1");
    assert_eq!(body["anchorName"], "主播");
    assert_eq!(body["type"], 1);

    // getLiveInfo(0, 10)：Java 请求体固定含 start/limit
    let list = live.get_live_info(0, 10).await.expect("获取直播间列表成功");
    assert_eq!(list.total, 1);
    assert_eq!(list.room_infos.len(), 1);
    assert_eq!(list.room_infos[0].room_id, 123);
    assert_eq!(list.room_infos[0].name, "直播间A");
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["start"], 0);
    assert_eq!(body["limit"], 10);
}

#[tokio::test]
async fn live_create_room_error_300036_recover() {
    let server = MockServer::start(dispatch(|_| {
        // 微信侧对重复房间名返回 errcode=300036 且携带房间数据；
        // Java 语义：createRoom 从错误报文（WxError.json）回解析出 WxMaCreateRoomResult
        r#"{"errcode":300036,"errmsg":"房间名称已存在","roomId":456,"qrcode_url":"https://wx.qr"}"#
            .to_string()
    }))
    .await;
    let service =
        wx_rust_miniapp::api::r#impl::WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let live = service.live_service().expect("直播服务存在");

    let mut room_info = WxMaLiveRoomInfo::default();
    room_info.name = "重复直播间".to_string();
    room_info.cover_img = "MEDIA_1".to_string();
    let result = live.create_room(&room_info).await;
    // Java 语义（WxMaLiveServiceImpl.createRoom）：errcode=300036 时从
    // `WxErrorException.getError().getJson()` 回解析出 WxMaCreateRoomResult；
    // 执行引擎（SimpleGetRequestExecutor::handle_response）保留原始报文 json。
    let result = result.expect("300036 应从错误报文回解析出房间数据（Java 语义）");
    assert_eq!(result.room_id, 456);
    assert_eq!(result.qrcode_url, "https://wx.qr");
    // 请求体线格式不受影响
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["name"], "重复直播间");
    assert_eq!(body["coverImg"], "MEDIA_1");
}

// ---- 直播商品（镜像 Java WxMaLiveGoodsServiceImplTest.addGoods） ----

#[tokio::test]
async fn live_goods_add_goods() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/wxaapi/broadcast/goods/update") {
            r#"{"errcode":0}"#.to_string()
        } else {
            r#"{"errcode":0,"auditId":12345,"goodsId":8}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_miniapp::api::r#impl::WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let goods_service = service.live_goods_service().expect("直播商品服务存在");

    let mut goods = WxMaLiveGoodInfo::default();
    goods.goods_id = 8;
    goods.name = "商品A".to_string();
    goods.cover_img_url = "http://cover/1.png".to_string();
    goods.price_type = 1;
    goods.price = "99.9".to_string();
    goods.goods_key = vec!["k1".to_string()];
    // Java 语义：请求体 {"goodsInfo":{...}}
    let result = goods_service.add_goods(&goods).await.expect("添加商品成功");
    assert_eq!(result.audit_id, 12345);
    assert_eq!(result.goods_id, 8);
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["goodsInfo"]["name"], "商品A");
    assert_eq!(body["goodsInfo"]["goodsId"], 8);
    assert_eq!(body["goodsInfo"]["coverImgUrl"], "http://cover/1.png");
    assert_eq!(body["goodsInfo"]["priceType"], 1);
    assert_eq!(body["goodsInfo"]["goodsKey"][0], "k1");

    // updateGoods：同一 goodsInfo 结构
    assert!(
        goods_service
            .update_goods(&goods)
            .await
            .expect("更新商品成功")
    );
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["goodsInfo"]["goodsId"], 8);
}

// ---- 云开发（镜像 Java WxMaCloudServiceImplTest.testInvokeCloudFunction /
// testAddList / testSendSmsV2） ----

#[tokio::test]
async fn cloud_invoke_cloud_function() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/tcb/invokecloudfunction") {
            r#"{"errcode":0,"resp_data":"{\"openid\":\"o1\",\"nick\":\"云函数返回\"}"}"#.to_string()
        } else {
            r#"{"errcode":0,"id_list":["id-1"]}"#.to_string()
        }
    }))
    .await;
    let service = wx_rust_miniapp::api::r#impl::WxMaServiceImpl::new_arc(
        config_with_host_and_cloud_env(&server.url(""), "test-env-1"),
    );
    let cloud = service.cloud_service().expect("云开发服务存在");

    // testInvokeCloudFunction：URL query 携带 env（配置 cloudEnv）与 name，
    // 请求体为原始 body 字符串（Java post(url, body)）
    let result = cloud
        .invoke_cloud_function("login", "{}")
        .await
        .expect("触发云函数成功");
    assert!(result.contains("openid"));
    assert!(result.contains("云函数返回"));
    let request_line = server.last_request_line();
    assert!(request_line.contains("/tcb/invokecloudfunction"));
    assert!(request_line.contains("env=test-env-1"));
    assert!(request_line.contains("name=login"));
    assert_eq!(server.last_body(), "{}");

    // add：Java 拼接 `db.collection('geo').add({data: [...]})` 查询串
    let docs = vec![serde_json::json!({"description": "item1", "price": 1.2})];
    let ids = cloud.add("geo", &docs).await.expect("批量添加成功");
    assert_eq!(ids, vec!["id-1"]);
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["env"], "test-env-1");
    let query = body["query"].as_str().expect("query 字段");
    assert!(query.starts_with("db.collection('geo').add({data: ["));
    assert!(query.contains("\"description\":\"item1\""));
}

#[tokio::test]
async fn cloud_database_and_send_sms() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/tcb/sendsmsv2") {
            r#"{"errcode":0,"send_status_list":[{"serial_no":"S-1","phone_number":"+8612345678910","errcode":0,"errmsg":"ok"}]}"#.to_string()
        } else if path.contains("/tcb/databasequery") {
            // Java WxCloudDatabaseQueryResult.data 为 String[]（JSON 字符串数组）；
            // Pager 键为大写 Offset/Limit/Total
            r#"{"errcode":0,"data":["{\"description\":\"item1\"}"],"pager":{"Offset":1,"Limit":10,"Total":1}}"#.to_string()
        } else if path.contains("/tcb/databasecount") {
            r#"{"errcode":0,"count":3}"#.to_string()
        } else {
            r#"{"errcode":0,"id_list":["id-1"]}"#.to_string()
        }
    }))
    .await;
    let service = wx_rust_miniapp::api::r#impl::WxMaServiceImpl::new_arc(
        config_with_host_and_cloud_env(&server.url(""), "test-env-1"),
    );
    let cloud = service.cloud_service().expect("云开发服务存在");

    // databaseQuery：请求体 {"env":..., "query":...}
    let query_result = cloud
        .database_query("db.collection(\"geo\").where({done:false}).limit(10).skip(1).get()")
        .await
        .expect("数据库查询成功");
    assert_eq!(query_result.pager.total, 1);
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["env"], "test-env-1");
    assert_eq!(
        body["query"],
        "db.collection(\"geo\").where({done:false}).limit(10).skip(1).get()"
    );

    // databaseCount
    let count = cloud
        .database_count("db.collection(\"geo\").where({done:false}).count()")
        .await
        .expect("数据库计数成功");
    assert_eq!(count, 3);

    // sendSmsV2：request 未指定 env 时补默认云环境 ID（Java request.getEnv()==null 语义）
    let mut sms = WxCloudSendSmsV2Request::default();
    sms.url_link = "https://wxaurl.cn/xxxxxx".to_string();
    sms.template_id = "844110".to_string();
    sms.template_param_list = vec!["能力上新".to_string()];
    sms.phone_number_list = vec!["+8612345678910".to_string()];
    let sms_result = cloud.send_sms_v2(&sms).await.expect("发送短信成功");
    assert_eq!(sms_result.send_status_list.len(), 1);
    assert_eq!(sms_result.send_status_list[0].serial_no, "S-1");
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["env"], "test-env-1");
    assert_eq!(body["template_id"], "844110");
    assert_eq!(body["url_link"], "https://wxaurl.cn/xxxxxx");
    assert_eq!(body["phone_number_list"][0], "+8612345678910");
}

// ---- 短剧点播（镜像 Java WxMaVodServiceImplTest.testListMedia /
// testPullUpload / testUploadSingleFile） ----

#[tokio::test]
async fn vod_list_media_and_pull_upload() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/wxa/sec/vod/listmedia") {
            r#"{"errcode":0,"media_info_list":[{"media_id":100,"name":"短剧A","drama_id":100000,"description":"简介","create_time":1700000000,"file_size":"1024","duration":60,"expire_time":0}]}"#.to_string()
        } else if path.contains("/wxa/sec/vod/pullupload") {
            r#"{"errcode":0,"task_id":12345}"#.to_string()
        } else {
            r#"{"errcode":0}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_miniapp::api::r#impl::WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let vod = service.vod_service().expect("点播服务存在");

    // testListMedia
    let mut list_req = WxMaVodListMediaRequest::default();
    list_req.drama_id = 100000;
    list_req.offset = 0;
    list_req.limit = 100;
    list_req.media_name = "短剧".to_string();
    let list = vod.list_media(&list_req).await.expect("获取媒体列表成功");
    let list = list.expect("媒体列表非空");
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].media_id, 100);
    assert_eq!(list[0].name, "短剧A");
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["drama_id"], 100000);
    assert_eq!(body["offset"], 0);
    assert_eq!(body["limit"], 100);

    // testPullUpload
    let mut pull = WxMaVodPullUploadRequest::default();
    pull.media_url = "https://cdn/1.mp4".to_string();
    pull.cover_url = "https://cdn/1.jpg".to_string();
    pull.media_name = "短剧A".to_string();
    let result = vod.pull_upload(&pull).await.expect("拉取上传成功");
    assert_eq!(result.task_id, 12345);
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["media_url"], "https://cdn/1.mp4");
    assert_eq!(body["cover_url"], "https://cdn/1.jpg");
    assert_eq!(body["media_name"], "短剧A");
}

#[tokio::test]
async fn vod_upload_single_file_multipart() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/wxa/sec/vod/singlefileupload") {
            r#"{"errcode":0,"media_id":100}"#.to_string()
        } else {
            r#"{"errcode":0}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_miniapp::api::r#impl::WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let vod = service.vod_service().expect("点播服务存在");

    // uploadSingleFile：multipart 字段照搬 OkHttpVodSingleUploadRequestExecutor
    // （media_data/media_type/media_name，可带 cover_type/cover_data/source_context）
    let tmp = std::env::temp_dir().join("wx_rust_ma_vod_test_media.bin");
    std::fs::write(&tmp, b"0123456789").expect("写入临时文件");
    let result = vod
        .upload_single_file(tmp.to_str().expect("路径"), "短剧A", "mp4")
        .await
        .expect("单文件上传成功");
    assert_eq!(result.media_id, 100);
    let multipart = server.last_body();
    assert!(
        multipart.contains("name=\"media_data\""),
        "multipart 应含 media_data 字段: {multipart}"
    );
    assert!(
        multipart.contains("name=\"media_type\""),
        "应含 media_type 字段"
    );
    assert!(
        multipart.contains("name=\"media_name\""),
        "应含 media_name 字段"
    );
    assert!(multipart.contains("短剧A"), "media_name 值应为 短剧A");
    std::fs::remove_file(&tmp).ok();
}

// ---- 虚拟支付（镜像 Java WxMaXPayServiceImplTest.testCurrencyPay） ----

#[tokio::test]
async fn xpay_currency_pay_signed() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/xpay/currency_pay") {
            r#"{"errcode":0,"balance":100,"used_present_amount":0,"order_id":"ORDER_1"}"#
                .to_string()
        } else {
            r#"{"errcode":0}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_miniapp::api::r#impl::WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let xpay = service.xpay_service().expect("虚拟支付服务存在");

    let mut request = WxMaXPayCurrencyPayRequest::default();
    request.openid = "o1".to_string();
    request.env = 0;
    request.user_ip = "127.0.0.1".to_string();
    request.amount = 100;
    let mut sig_params = WxMaXPaySigParams::default();
    sig_params.app_key = "test-app-key".to_string();
    sig_params.session_key = "test-session-key".to_string();

    // currencyPay 为双签名：URL query 携带 pay_sig（pay 签名）与 signature（登录态签名）
    let response = xpay
        .currency_pay(&request, &sig_params)
        .await
        .expect("虚拟币充值下单成功");
    assert_eq!(response.balance, 100);
    assert_eq!(response.order_id, "ORDER_1");

    let request_line = server.last_request_line();
    assert!(
        request_line.contains("/xpay/currency_pay"),
        "{request_line}"
    );
    let pay_sig = query_param(&request_line, "pay_sig").expect("pay_sig 参数");
    let signature = query_param(&request_line, "signature").expect("signature 参数");
    // HMAC-SHA256 十六进制小写，64 位
    assert!(
        is_lower_hex_64(&pay_sig),
        "pay_sig 应为 64 位小写十六进制: {pay_sig}"
    );
    assert!(
        is_lower_hex_64(&signature),
        "signature 应为 64 位小写十六进制: {signature}"
    );

    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["openid"], "o1");
    assert_eq!(body["env"], 0);
    assert_eq!(body["user_ip"], "127.0.0.1");
    assert_eq!(body["amount"], 100);
}

// ---- 营销（镜像 Java WxMaMarketingServiceImplTest.addUserActionSets /
// addUserAction） ----

#[tokio::test]
async fn marketing_user_action_sets_and_actions() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/marketing/user_action_sets/add") {
            r#"{"errcode":0,"data":{"user_action_set_id":123456}}"#.to_string()
        } else {
            r#"{"errcode":0,"message":"ok"}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_miniapp::api::r#impl::WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let marketing = service.marketing_service().expect("营销服务存在");

    // addUserActionSets：请求体 {"type","name","description"}，URL 带 version=v1.0
    let set_id = marketing
        .add_user_action_sets("WEB", "测试数据源", "描述")
        .await
        .expect("创建数据源成功");
    assert_eq!(set_id, 123456);
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["type"], "WEB");
    assert_eq!(body["name"], "测试数据源");
    assert_eq!(body["description"], "描述");
    assert!(server.last_request_line().contains("version=v1.0"));

    // addUserAction：请求体按 Java WxMaUserAction.listToJson 手工组装
    // （action_time/action_type 下划线键、trace.click_id、action_param.value）
    let mut action = WxMaUserAction::default();
    action.url = "https://wxaurl.cn/page".to_string();
    action.action_time = 1700000000;
    action.action_type = "COMPLETE_ORDER".to_string();
    action.click_id = "CLICK-1".to_string();
    action.action_param = 99;
    action.leads_type = "LEADS".to_string();
    let response = marketing
        .add_user_action(&[action], Some(123456))
        .await
        .expect("回传数据成功");
    assert!(response.contains("message"));
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["user_action_set_id"], 123456);
    assert_eq!(body["actions"][0]["url"], "https://wxaurl.cn/page");
    assert_eq!(body["actions"][0]["action_time"], 1700000000);
    assert_eq!(body["actions"][0]["action_type"], "COMPLETE_ORDER");
    assert_eq!(body["actions"][0]["trace"]["click_id"], "CLICK-1");
    assert_eq!(body["actions"][0]["action_param"]["value"], 99);
    assert_eq!(body["actions"][0]["action_param"]["leads_type"], "LEADS");
}

// ---- 推广员（镜像 Java WxMaPromotionServiceTest.addRole / sendMsg） ----

#[tokio::test]
async fn promotion_add_role_and_send_msg() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/promoter/sendmsg") {
            r#"{"errcode":0,"msg_id":"MSG-100"}"#.to_string()
        } else {
            r#"{"errcode":0,"role_id":123,"name":"推广员"}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_miniapp::api::r#impl::WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let promotion = service.promotion_service().expect("推广服务存在");

    // addRole：请求体 {"name","desc"}
    let mut role = WxMaPromotionAddRoleRequest::default();
    role.name = "推广员".to_string();
    role.desc = "角色描述".to_string();
    let result = promotion.add_role(&role).await.expect("新增角色成功");
    assert_eq!(result.role_id, 123);
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["name"], "推广员");
    assert_eq!(body["desc"], "角色描述");

    // sendMsg：请求体 msg_type/content/appid/path/list_type/role_id
    let mut msg = WxMaPromotionSendMsgRequest::default();
    msg.msg_type = 1;
    msg.content = "群发内容".to_string();
    msg.appid = "wxappid".to_string();
    msg.path = "pages/index".to_string();
    msg.list_type = 1;
    msg.role_id = vec![123];
    let result = promotion.send_msg(&msg).await.expect("群发消息成功");
    assert_eq!(result.msg_id, "MSG-100");
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["msg_type"], 1);
    assert_eq!(body["content"], "群发内容");
    assert_eq!(body["appid"], "wxappid");
    assert_eq!(body["list_type"], 1);
    assert_eq!(body["role_id"][0], 123);
}

// ---- 同城配送（镜像 Java WxMaIntracityServiceImpleTest.createStore /
// preAddOrder；请求体经 LOWER_CASE_WITH_UNDERSCORES 转 snake_case） ----

#[tokio::test]
async fn intracity_create_store_and_pre_add_order() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/intracity/createstore") {
            r#"{"wx_store_id":"WX-STORE-1"}"#.to_string()
        } else {
            r#"{"service_trans_id":"ST-1","distance":100,"est_fee":1200,"expected_finished_time":1700000000,"promise_delivery_time":30}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_miniapp::api::r#impl::WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let intracity = service.intracity_service().expect("同城配送服务存在");

    // createStore：Java 校验 outStoreId 非空、wxStoreId 为空；
    // 线格式键为 snake_case（gson LOWER_CASE_WITH_UNDERSCORES）
    let mut store = WxMaStore::default();
    store.out_store_id = "OUT-1".to_string();
    store.store_name = "测试门店".to_string();
    store.city_id = "440300".to_string();
    store.order_pattern = 1;
    store.service_trans_prefer = "TRANS-1".to_string();
    let mut address = AddressInfo::default();
    address.province = "广东省".to_string();
    address.city = "深圳市".to_string();
    address.area = "南山区".to_string();
    address.street = "科技园".to_string();
    address.house = "1号楼".to_string();
    address.phone = "13800000000".to_string();
    store.address_info = address;
    let wx_store_id = intracity.create_store(&store).await.expect("创建门店成功");
    assert_eq!(wx_store_id, "WX-STORE-1");
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["out_store_id"], "OUT-1");
    assert_eq!(body["store_name"], "测试门店");
    assert_eq!(body["city_id"], "440300");
    assert_eq!(body["order_pattern"], 1);
    assert_eq!(body["service_trans_prefer"], "TRANS-1");
    assert_eq!(body["address_info"]["province"], "广东省");
    assert_eq!(body["address_info"]["house"], "1号楼");
    // bean 中 wxStoreId 为空串也按线格式输出
    assert_eq!(body["wx_store_id"], "");

    // preAddOrder：请求体 snake_case；响应键反向转换回 bean（serviceTransId）
    let mut pre = WxMaPreAddOrderRequest::default();
    pre.wx_store_id = "WX-STORE-1".to_string();
    pre.user_name = "张三".to_string();
    pre.user_phone = "13800000000".to_string();
    pre.user_lng = 113.0;
    pre.user_lat = 23.0;
    pre.user_address = "深圳市南山区".to_string();
    pre.use_sandbox = 0;
    pre.cargo.cargo_name = "苹果".to_string();
    pre.cargo.cargo_weight = 1;
    pre.cargo.cargo_num = 1;
    pre.cargo.cargo_price = 1000;
    let result = intracity.pre_add_order(&pre).await.expect("查询运费成功");
    assert_eq!(result.service_trans_id, "ST-1");
    assert_eq!(result.distance, 100);
    assert_eq!(result.est_fee, 1200);
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["wx_store_id"], "WX-STORE-1");
    assert_eq!(body["user_name"], "张三");
    assert_eq!(body["user_lng"], 113.0);
    assert_eq!(body["cargo"]["cargo_name"], "苹果");
    assert_eq!(body["cargo"]["cargo_price"], 1000);
}

// ---- 交易投诉（镜像 Java WxMaComplaintServiceImplTest.queryComplaints /
// submitResponse / uploadResponseImage） ----

#[tokio::test]
async fn complaint_query_list_submit_and_upload() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/miniapp/complaint/list") {
            r#"{"errcode":0,"data":[{"complaint_id":"C-1","complaint_time":"1700000000","complaint_detail":"未收到货","complaint_state":"1","openid":"o1","phone_number":"13800000000"}],"total_count":1}"#.to_string()
        } else if path.contains("/cgi-bin/miniapp/complaint/upload") {
            r#"{"errcode":0,"media_id":"MEDIA-1"}"#.to_string()
        } else {
            r#"{"errcode":0}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_miniapp::api::r#impl::WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let complaint = service.complaint_service().expect("投诉服务存在");

    // queryComplaints：请求体 begin_date/end_date/limit/offset
    let mut query = WxMaComplaintRequest::default();
    query.begin_date = "2024-01-01".to_string();
    query.end_date = "2024-01-31".to_string();
    query.limit = 10;
    query.offset = 0;
    let result = complaint
        .query_complaints(&query)
        .await
        .expect("查询投诉列表成功");
    assert_eq!(result.total_count, 1);
    assert_eq!(result.data.len(), 1);
    assert_eq!(result.data[0].complaint_id, "C-1");
    assert_eq!(result.data[0].complaint_state, "1");
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["begin_date"], "2024-01-01");
    assert_eq!(body["end_date"], "2024-01-31");
    assert_eq!(body["limit"], 10);
    assert_eq!(body["offset"], 0);

    // submitResponse：请求体 complaint_id/response_content/response_images
    let mut response = WxMaResponseRequest::default();
    response.complaint_id = "C-1".to_string();
    response.response_content = "已退款".to_string();
    response.response_images = vec!["MEDIA-1".to_string()];
    complaint
        .submit_response(&response)
        .await
        .expect("提交回复成功");
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["complaint_id"], "C-1");
    assert_eq!(body["response_content"], "已退款");
    assert_eq!(body["response_images"][0], "MEDIA-1");

    // uploadResponseImage：multipart 字段名 image（Java CommonUploadParam.fromFile("image", ...)）
    let tmp = std::env::temp_dir().join("wx_rust_ma_complaint_test.png");
    std::fs::write(&tmp, b"fake-image-bytes").expect("写入临时文件");
    let media_id = complaint
        .upload_response_image(tmp.to_str().expect("路径"))
        .await
        .expect("上传反馈图片成功");
    assert_eq!(media_id, "MEDIA-1");
    let multipart = server.last_body();
    assert!(
        multipart.contains("name=\"image\""),
        "multipart 应含 image 字段: {multipart}"
    );
    std::fs::remove_file(&tmp).ok();
}

// ---- 设备订阅消息（镜像 Java WxMaDeviceSubscribeServiceImplTest.getSnTicket /
// sendDeviceSubscribeMsg） ----

#[tokio::test]
async fn device_subscribe_sn_ticket_and_send_msg() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/wxa/getsnticket") {
            r#"{"errcode":0,"sn_ticket":"TICKET-1"}"#.to_string()
        } else {
            r#"{"errcode":0}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_miniapp::api::r#impl::WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let device = service
        .device_subscribe_service()
        .expect("设备订阅服务存在");

    // getSnTicket：请求体 {"model_id","sn"}，解析 sn_ticket
    let mut ticket = WxMaDeviceTicketRequest::default();
    ticket.model_id = "MODEL-1".to_string();
    ticket.sn = "SN-1".to_string();
    let sn_ticket = device
        .get_sn_ticket(&ticket)
        .await
        .expect("获取设备票据成功");
    assert_eq!(sn_ticket, "TICKET-1");
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["model_id"], "MODEL-1");
    assert_eq!(body["sn"], "SN-1");

    // sendDeviceSubscribeMsg：请求体 to_openid_list/template_id/sn/data
    let mut msg = WxMaDeviceSubscribeMessageRequest::default();
    msg.to_openid_list = vec!["o1".to_string()];
    msg.template_id = "TEMPLATE-1".to_string();
    msg.sn = "SN-1".to_string();
    msg.page = "pages/index".to_string();
    msg.data = serde_json::json!({"thing1": {"value": "温度过高"}});
    device
        .send_device_subscribe_msg(&msg)
        .await
        .expect("发送设备订阅消息成功");
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["to_openid_list"][0], "o1");
    assert_eq!(body["template_id"], "TEMPLATE-1");
    assert_eq!(body["sn"], "SN-1");
    assert_eq!(body["data"]["thing1"]["value"], "温度过高");
}

// ---- 人脸核身（镜像 Java WxMaFaceServiceImplTest.getVerifyId / queryVerifyInfo） ----

#[tokio::test]
async fn face_get_verify_id_and_query_verify_info() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cityservice/face/identify/getverifyid") {
            r#"{"errcode":0,"verify_id":"VERIFY-1","expires_in":600}"#.to_string()
        } else {
            r#"{"errcode":0,"verify_ret":1}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_miniapp::api::r#impl::WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let face = service.face_service().expect("人脸核身服务存在");

    // getVerifyId：请求体 out_seq_no/cert_info/openid
    let mut request = WxMaFaceGetVerifyIdRequest::default();
    request.out_seq_no = "SEQ-1".to_string();
    request.openid = "o1".to_string();
    let mut cert = CertInfo::default();
    cert.cert_type = "IDENTITY_CARD".to_string();
    cert.cert_name = "张三".to_string();
    cert.cert_no = "440300000000000000".to_string();
    request.cert_info = cert;
    let result = face
        .get_verify_id(&request)
        .await
        .expect("获取核身会话成功");
    assert_eq!(result.verify_id, "VERIFY-1");
    assert_eq!(result.expires_in, 600);
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["out_seq_no"], "SEQ-1");
    assert_eq!(body["openid"], "o1");
    assert_eq!(body["cert_info"]["cert_type"], "IDENTITY_CARD");
    assert_eq!(body["cert_info"]["cert_name"], "张三");

    // queryVerifyInfo：请求体 verify_id，解析 verify_ret
    let mut info = WxMaFaceQueryVerifyInfoRequest::default();
    info.verify_id = "VERIFY-1".to_string();
    let result = face
        .query_verify_info(&info)
        .await
        .expect("查询核身结果成功");
    assert_eq!(result.verify_ret, 1);
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["verify_id"], "VERIFY-1");
}

// ---- 报销发票（镜像 Java WxMaReimburseInvoiceServiceImplTest.testGetInvoiceInfo /
// testGetInvoiceBatch） ----

#[tokio::test]
async fn invoice_get_info_and_batch() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/card/invoice/reimburse/getinvoicebatch") {
            r#"{"errcode":0,"item_list":[{"card_id":"CARD-2","type":"in","payee":"测试公司","detail":"明细2"}]}"#.to_string()
        } else {
            r#"{"errcode":0,"card_id":"CARD-1","begin_time":1700000000,"end_time":1700003600,"openid":"o1","type":"in","payee":"测试公司","detail":"明细"}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_miniapp::api::r#impl::WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let invoice = service.reimburse_invoice_service().expect("发票服务存在");

    // testGetInvoiceInfo：请求体 {"card_id","encrypt_code"}
    let mut request = InvoiceInfoRequest::default();
    request.card_id = "CARD-1".to_string();
    request.encrypt_code = "ENC-1".to_string();
    let result = invoice
        .get_invoice_info(&request)
        .await
        .expect("查询发票信息成功");
    assert_eq!(result.card_id, "CARD-1");
    assert_eq!(result.payee, "测试公司");
    assert_eq!(result.r#type, "in");
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["card_id"], "CARD-1");
    assert_eq!(body["encrypt_code"], "ENC-1");

    // testGetInvoiceBatch：响应取 item_list 数组（Java InvoiceInfoResponse.toList）
    let mut batch = InvoiceBatchRequest::default();
    batch.item_list = vec![request];
    let list = invoice
        .get_invoice_batch(&batch)
        .await
        .expect("批量查询发票信息成功");
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].card_id, "CARD-2");
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["item_list"][0]["card_id"], "CARD-1");
}

// ---- 二维码快速跳转（镜像 Java WxMaQrcodeJumpServiceImplTest.addRule / getRules） ----

#[tokio::test]
async fn qrcode_jump_add_rule_and_get_rules() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/wxaapi/wxaqrcodefast/getcategory") {
            r#"{"errcode":0,"rule_list":[{"prefix":"pages/","permit_sub_rule":true,"open_version":1,"path":"pages/index","debug_wxa_info":[],"is_expire":false}]}"#.to_string()
        } else {
            r#"{"errcode":0}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_miniapp::api::r#impl::WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let qrcode_jump = service.qrcode_jump_service().expect("二维码跳转服务存在");

    // addRule：返回原始响应报文；请求体 prefix/permit_sub_rule/open_version/path
    let mut rule = WxMaQrcodeJumpRule::default();
    rule.prefix = "pages/".to_string();
    rule.permit_sub_rule = true;
    rule.open_version = 1;
    rule.path = "pages/index".to_string();
    let response = qrcode_jump.add_rule(&rule).await.expect("添加跳转规则成功");
    assert!(response.contains("errcode"));
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["prefix"], "pages/");
    assert_eq!(body["permit_sub_rule"], true);
    assert_eq!(body["open_version"], 1);
    assert_eq!(body["path"], "pages/index");

    // getRules：请求体仅携带非 null 的 is_default/prefix；解析 rule_list
    let rules = qrcode_jump
        .get_rules(Some(true), Some("pages/"))
        .await
        .expect("获取跳转规则成功");
    assert_eq!(rules.len(), 1);
    assert_eq!(rules[0].prefix, "pages/");
    assert_eq!(rules[0].path, "pages/index");
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["is_default"], true);
    assert_eq!(body["prefix"], "pages/");
}
