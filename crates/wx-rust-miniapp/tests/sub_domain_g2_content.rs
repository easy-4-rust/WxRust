//! 小程序 G2 内容服务组集成测试（镜像 Java
//! `WxMaSubscribeServiceImplTest` / `WxMaShareServiceImplTest` /
//! `WxMaSchemeServiceImplTest` / `WxMaLinkServiceImplTest` /
//! `WxMaQrcodeServiceImplTest` / `WxMaJsapiServiceImplTest` /
//! `WxMaPluginServiceImplTest` / `WxMaOpenApiServiceImplTest` /
//! `WxMaInternetServiceImplTest` 的 HTTP 语义；`WxMaRunServiceImplTest`
//! 在 Java 侧不存在，按 Java `WxMaRunServiceImpl.getRunStepInfo` 文档语义
//! ADAPTED 测试）。
//!
//! 覆盖 10 个子服务：subscribe / qrcode / jsapi / share / scheme / link /
//! plugin / run / open_api / internet，均经 MockServer 验证请求路径、请求体
//! 线格式与响应解析。字节响应（二维码图片）场景走 `QrcodeBytesRequestExecutor`
//! 语义：Content-Type 非 `application/json` 时返回原始字节。

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use base64::Engine as _;
use hmac::{Hmac, KeyInit, Mac};
use sha1::Digest as _;
use sha2::Sha256;

use wx_rust_miniapp::api::WxMaService;
use wx_rust_miniapp::api::r#impl::WxMaServiceImpl;
use wx_rust_miniapp::bean::{
    GenerateShortLinkRequest, GenerateUrlLinkRequest, MsgData, QueryUrlLinkRequest,
    WxMaCodeLineColor, WxMaGenerateSchemeRequest, WxMaInternetResponse, WxMaRunStepInfo,
    WxMaShareInfo, WxMaSubscribeMessage,
};
use wx_rust_miniapp::config::WxMaConfig;
use wx_rust_miniapp::config::WxMaHostConfig;
use wx_rust_miniapp::config::r#impl::WxMaDefaultConfig;

/// mock HTTP 响应（内容类型 + 字节体；`QrcodeBytesRequestExecutor` 依赖
/// Content-Type 区分 JSON 错误报文与图片字节）。
struct MockResponse {
    content_type: &'static str,
    body: Vec<u8>,
}

fn json_body(s: &str) -> MockResponse {
    MockResponse {
        content_type: "application/json",
        body: s.as_bytes().to_vec(),
    }
}

fn image_body(bytes: &[u8]) -> MockResponse {
    MockResponse {
        content_type: "image/png",
        body: bytes.to_vec(),
    }
}

/// 极简 mock HTTP 服务器：按请求路径（含 query）返回固定响应，记录最近一次
/// 请求路径与请求体。
struct MockServer {
    addr: std::net::SocketAddr,
    requests: Arc<AtomicUsize>,
    last_path: Arc<std::sync::Mutex<String>>,
    last_body: Arc<std::sync::Mutex<String>>,
    stop: Arc<std::sync::atomic::AtomicBool>,
}

impl MockServer {
    /// 启动服务器（`handler(path) -> MockResponse`）。
    async fn start<F>(handler: F) -> Self
    where
        F: Fn(&str) -> MockResponse + Send + Sync + 'static,
    {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("绑定端口");
        let addr = listener.local_addr().expect("获取地址");
        let requests = Arc::new(AtomicUsize::new(0));
        let last_path = Arc::new(std::sync::Mutex::new(String::new()));
        let last_body = Arc::new(std::sync::Mutex::new(String::new()));
        let stop = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let handler = Arc::new(handler);

        let requests_clone = requests.clone();
        let last_path_clone = last_path.clone();
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
                requests_clone.fetch_add(1, Ordering::SeqCst);
                let handler = handler.clone();
                let last_path_clone = last_path_clone.clone();
                let last_body_clone = last_body_clone.clone();
                tokio::spawn(async move {
                    use tokio::io::{AsyncReadExt, AsyncWriteExt};
                    let mut buf = [0u8; 16384];
                    let n = socket.read(&mut buf).await.unwrap_or(0);
                    let request = String::from_utf8_lossy(&buf[..n]).to_string();
                    // 记录请求路径（含 query）与请求体（POST 场景）
                    let path = request
                        .lines()
                        .next()
                        .map(|l| l.split_whitespace().nth(1).unwrap_or("/").to_string())
                        .unwrap_or_else(|| "/".to_string());
                    *last_path_clone.lock().unwrap() = path.clone();
                    if let Some(idx) = request.find("\r\n\r\n") {
                        let body = request[idx + 4..].to_string();
                        *last_body_clone.lock().unwrap() = body;
                    }
                    let response = handler(&path);
                    let head = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                        response.content_type,
                        response.body.len()
                    );
                    let mut out = head.into_bytes();
                    out.extend_from_slice(&response.body);
                    let _ = socket.write_all(&out).await;
                });
            }
        });

        Self {
            addr,
            requests,
            last_path,
            last_body,
            stop,
        }
    }

    fn url(&self, path: &str) -> String {
        format!("http://{}{}", self.addr, path)
    }

    fn request_count(&self) -> usize {
        self.requests.load(Ordering::SeqCst)
    }

    fn last_path(&self) -> String {
        self.last_path.lock().unwrap().clone()
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

/// 构建指向 mock 服务器的小程序配置（`WxMaDefaultConfig` + host 配置覆盖
/// api_host；对应 mp 样板 `config_with_host`）。
fn config_with_host(host: &str) -> Arc<dyn WxMaConfig> {
    let mut config = WxMaDefaultConfig::new("wxappid", "secret");
    config
        .set_token("token123")
        .set_aes_key("abcdefghijklmnopqrstuvwxyz0123456789ABCDEFG");
    let mut host_config = WxMaHostConfig::new();
    host_config.api_host = host.to_string();
    config.set_host_config(host_config);
    Arc::new(config)
}

/// 通用路由 handler：token 请求 + 各子域响应。
///
/// 注意子串冲突顺序：`/cgi-bin/clear_quota` 必须先判 `/v2` 变体；
/// `/wxa/getwxacode` 必须先判 `/wxa/getwxacodeunlimit` 变体。
fn dispatch(
    handler: impl Fn(&str) -> MockResponse + Send + Sync + 'static,
) -> impl Fn(&str) -> MockResponse + Send + Sync + 'static {
    move |path: &str| {
        if path.contains("/cgi-bin/token") || path.contains("/stable_token") {
            return json_body(r#"{"access_token":"MOCK_TOKEN","expires_in":7200}"#);
        }
        handler(path)
    }
}

/// 二维码字节固定样本（PNG 文件头 + IHDR 块头）。
const PNG_BYTES: &[u8] = &[
    0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52,
    0x00, 0x00, 0x00, 0x2A, 0x00, 0x00, 0x00, 0x2A, 0x08,
];

// ---- 解密 golden fixture（Java 测试同款 session_key / iv，AES-128-CBC +
// PKCS7，由 Python pycryptodome 生成并回验） ----

/// Java `WxMaShareServiceImplTest` / `WxMaRunServiceImpl` 使用的 session key。
const SESSION_KEY: &str = "tiihtNczf5v6AKRyjwEUhQ==";
/// Java 测试使用的 iv。
const IV_STR: &str = "r7BXXKkLb8qrSNn05n0qiA==";
/// `{"openGId":"opengid_demo_001"}` 的 AES-128-CBC 密文。
const SHARE_ENCRYPTED: &str = "MzSIcRSG9gB4fs5k+UvtAsLvRPKvBoS0AaXMJ9OuIdQ=";
/// `{"opengid":"opengid_demo_002","open_single_roomid":"room_1","group_openid":"gh_xxx","chat_type":2}`
/// 的 AES-128-CBC 密文。
const GROUP_ENTER_ENCRYPTED: &str = "Aq6ocLML5+c60oLbC3SVF7kaYHhz+W6J812lh74MSBMCe3/ko7++1KD2SUjwmCTX1qilbJkWDMgQI088sGk2BuUPRVwR/a29TtB2VgGjfJfwrwT04E0+2XNCfbmBQsJZUycvMSpjXONUSe8uAdkulQ==";
/// `{"stepInfoList":[{"timestamp":1547212800,"step":90},{"timestamp":1547299200,"step":120}]}`
/// 的 AES-128-CBC 密文。
const RUN_ENCRYPTED: &str = "XoLUxF76jN/OsfTGUqF/ZjmZu0B4O82Y3eYgcmoqKbPoiRwklVh3sh4c0gcnHbdGom4gzI9PXgbDzzN2zDn2F0FL+ElGQIbjR8coAeG91SGUeRuLm+vW9Hy+8LVily6Z";

// ---- 订阅消息（镜像 Java WxMaSubscribeServiceImplTest） ----

#[tokio::test]
async fn subscribe_send_subscribe_msg_wire_format() {
    // 镜像 Java `testSendSubscribeMsg`：Java 以 GsonAdapter 输出
    // touser/template_id/page/data/miniprogram_state/lang，Rust 手写
    // Serialize 须输出同一线格式；响应含 msgid 时调用成功（Java 返回 void）。
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/message/subscribe/send") {
            json_body(r#"{"errcode":0,"errmsg":"ok","msgid":123456}"#)
        } else {
            json_body(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let subscribe_service = service.subscribe_service().expect("订阅消息服务存在");

    let mut message = WxMaSubscribeMessage::new();
    message.to_user = Some("openid_1".to_string());
    message.template_id = Some("TMPL_ID_001".to_string());
    message.page = Some("pages/index/index".to_string());
    // thing1 超长：Java `resetValue` 截断为前 17 字符 + "..."
    message.add_data(MsgData {
        name: "thing1".to_string(),
        value: "abcdefghijklmnopqrstuvwxyz".to_string(),
    });
    // number1 非法字符：Java `resetValue` 置 "0"
    message.add_data(MsgData {
        name: "number1".to_string(),
        value: "abc".to_string(),
    });
    // 默认值：Java 默认 miniprogramState=formal、lang=zh_CN
    subscribe_service
        .send_subscribe_msg(&message)
        .await
        .expect("发送订阅消息成功");

    assert!(
        server
            .last_path()
            .contains("/cgi-bin/message/subscribe/send")
    );
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["touser"], "openid_1");
    assert_eq!(body["template_id"], "TMPL_ID_001");
    assert_eq!(body["page"], "pages/index/index");
    assert_eq!(body["miniprogram_state"], "formal");
    assert_eq!(body["lang"], "zh_CN");
    assert_eq!(body["data"]["thing1"]["value"], "abcdefghijklmnopq...");
    assert_eq!(body["data"]["number1"]["value"], "0");
}

#[tokio::test]
async fn subscribe_get_pub_template_title_list() {
    // 镜像 Java `testGetPubTemplateTitleList`：GET 查询串
    // ids=2,616&start=0&limit=30，响应解析 count/data。
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/wxaapi/newtmpl/getpubtemplatetitles") {
            json_body(
                r#"{"errcode":0,"errmsg":"ok","count":1,"data":[{"type":2,"tid":99,"categoryId":"616","title":"快递服务通知"}]}"#,
            )
        } else {
            json_body(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let subscribe_service = service.subscribe_service().expect("订阅消息服务存在");

    let result = subscribe_service
        .get_pub_template_title_list(&["2", "616"], 0, 30)
        .await
        .expect("获取公共模板标题成功");
    assert_eq!(result.count, 1);
    assert_eq!(result.data.len(), 1);
    assert_eq!(result.data[0].tid, 99);
    assert_eq!(result.data[0].title, "快递服务通知");
    assert_eq!(result.data[0].category_id, "616");

    let path = server.last_path();
    assert!(path.contains("/wxaapi/newtmpl/getpubtemplatetitles?"));
    assert!(path.contains("ids=2,616"));
    assert!(path.contains("start=0"));
    assert!(path.contains("limit=30"));
}

#[tokio::test]
async fn subscribe_keywords_add_template_and_template_ops() {
    // 镜像 Java `testGetPubTemplateKeyWordsById` / `testAddTemplate` /
    // `testGetTemplateList` / `testDelTemplate` / `testGetCategory`。
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/wxaapi/newtmpl/getpubtemplatekeywords") {
            json_body(r#"{"errcode":0,"errmsg":"ok","data":[{"kid":1,"name":"姓名","example":"张伟","rule":"thing"}]}"#)
        } else if path.contains("/wxaapi/newtmpl/addtemplate") {
            json_body(r#"{"errcode":0,"errmsg":"ok","priTmplId":"TMPL_ADDED_1"}"#)
        } else if path.contains("/wxaapi/newtmpl/gettemplate") {
            json_body(r#"{"errcode":0,"errmsg":"ok","data":[{"priTmplId":"TMPL_ADDED_1","title":"模板","content":"内容","example":"例子","type":2}]}"#)
        } else if path.contains("/wxaapi/newtmpl/getcategory") {
            json_body(r#"{"errcode":0,"errmsg":"ok","data":[{"id":616,"name":"快递业"}]}"#)
        } else {
            json_body(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let subscribe_service = service.subscribe_service().expect("订阅消息服务存在");

    // 关键词库
    let keywords = subscribe_service
        .get_pub_template_keywords_by_id("99")
        .await
        .expect("获取关键词成功");
    assert_eq!(keywords.len(), 1);
    assert_eq!(keywords[0].kid, 1);
    assert_eq!(keywords[0].name, "姓名");
    assert!(server.last_path().contains("tid=99"));

    // 组合模板
    let template_id = subscribe_service
        .add_template("401", &[1, 2], "测试数据")
        .await
        .expect("添加模板成功");
    assert_eq!(template_id, "TMPL_ADDED_1");
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["tid"], "401");
    assert_eq!(body["kidList"][0], 1);
    assert_eq!(body["kidList"][1], 2);
    assert_eq!(body["sceneDesc"], "测试数据");

    // 模板列表
    let templates = subscribe_service
        .get_template_list()
        .await
        .expect("模板列表成功");
    assert_eq!(templates.len(), 1);
    assert_eq!(templates[0].pri_tmpl_id, "TMPL_ADDED_1");
    assert_eq!(templates[0].title, "模板");

    // 删除模板
    assert!(
        subscribe_service
            .del_template("TMPL_ADDED_1")
            .await
            .expect("删除模板成功")
    );
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["priTmplId"], "TMPL_ADDED_1");

    // 类目
    let categories = subscribe_service
        .get_category()
        .await
        .expect("获取类目成功");
    assert_eq!(categories.len(), 1);
    assert_eq!(categories[0].id, 616);
    assert_eq!(categories[0].name, "快递业");
}

// ---- 二维码/小程序码（镜像 Java WxMaQrcodeServiceImplTest） ----

#[tokio::test]
async fn qrcode_create_qrcode_bytes_and_save_file() {
    // 镜像 Java `testCreateQrcodeBytes` + `testCreateQrcode`：
    // POST /cgi-bin/wxaapp/createwxaqrcode，请求体 {"path","width"}；
    // 图片字节响应（Content-Type 非 JSON）经 QrcodeBytesRequestExecutor
    // 原样返回。
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/wxaapp/createwxaqrcode") {
            image_body(PNG_BYTES)
        } else {
            json_body(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let qrcode_service = service.qrcode_service().expect("二维码服务存在");

    // createQrcodeBytes("111", 122)
    let bytes = qrcode_service
        .create_qrcode_bytes("111", 122)
        .await
        .expect("获取二维码字节成功");
    assert_eq!(bytes, PNG_BYTES);
    assert!(
        server
            .last_path()
            .contains("/cgi-bin/wxaapp/createwxaqrcode")
    );
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["path"], "111");
    assert_eq!(body["width"], 122);

    // createQrcode("111", 122)：字节保存为临时 .jpg 文件（Java
    // FileUtils.createTmpFile 语义），文件内容与字节一致
    let file_path = qrcode_service
        .create_qrcode("111", 122)
        .await
        .expect("保存二维码文件成功");
    let saved = std::fs::read(&file_path).expect("读取二维码文件");
    assert_eq!(saved, PNG_BYTES);
    let _ = std::fs::remove_file(&file_path);
}

#[tokio::test]
async fn qrcode_create_wxa_code_unlimit_wire_and_bytes() {
    // 镜像 Java `testCreateWxaCodeUnlimitBytes`：POST /wxa/getwxacodeunlimit，
    // env_version 非空时输出、line_color 为 None 时省略（Gson null 省略语义）。
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/wxa/getwxacodeunlimit") {
            image_body(PNG_BYTES)
        } else {
            json_body(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let qrcode_service = service.qrcode_service().expect("二维码服务存在");

    let bytes = qrcode_service
        .create_wxa_code_unlimit_bytes(
            "111",
            "pages/unknown",
            false,
            Some("trial"),
            122,
            true,
            None,
            false,
        )
        .await
        .expect("获取不限制数量小程序码成功");
    assert_eq!(bytes, PNG_BYTES);
    assert!(server.last_path().contains("/wxa/getwxacodeunlimit"));
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["scene"], "111");
    assert_eq!(body["page"], "pages/unknown");
    assert_eq!(body["check_path"], false);
    assert_eq!(body["env_version"], "trial");
    assert_eq!(body["width"], 122);
    assert_eq!(body["auto_color"], true);
    assert_eq!(body["is_hyaline"], false);
    // Java：lineColor 为 null 时 Gson 省略该字段
    assert!(body.get("line_color").is_none());

    // line_color 显式传入 + env_version 缺省：env_version 省略、
    // line_color 序列化（Java WxMaCodeLineColor 含 r/g/b 三字段，默认 "0"）
    let bytes = qrcode_service
        .create_wxa_code_unlimit_bytes(
            "111",
            "pages/unknown",
            false,
            None,
            122,
            true,
            Some(WxMaCodeLineColor {
                r: "1".to_string(),
                g: "0".to_string(),
                b: "0".to_string(),
            }),
            false,
        )
        .await
        .expect("获取不限制数量小程序码成功");
    assert_eq!(bytes, PNG_BYTES);
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert!(body.get("env_version").is_none());
    assert_eq!(body["line_color"]["r"], "1");
    assert_eq!(body["line_color"]["g"], "0");
    assert_eq!(body["line_color"]["b"], "0");
}

#[tokio::test]
async fn qrcode_json_error_response_raises() {
    // 镜像 Java `QrcodeBytesRequestExecutor` 语义：响应 Content-Type 为
    // application/json 时视为微信错误报文并抛错（即使 errcode==0 也抛）。
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/wxa/getwxacodeunlimit") {
            json_body(r#"{"errcode":40001,"errmsg":"invalid credential"}"#)
        } else {
            json_body(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let qrcode_service = service.qrcode_service().expect("二维码服务存在");

    let err = qrcode_service
        .create_wxa_code_unlimit_bytes("111", "pages/unknown", false, None, 122, true, None, false)
        .await
        .expect_err("JSON 错误响应应抛错");
    assert_eq!(err.error_code(), Some(40001));
}

// ---- JSAPI（镜像 Java WxMaJsapiServiceImplTest） ----

#[tokio::test]
async fn jsapi_get_jsapi_ticket_cached_and_force_refresh() {
    // 镜像 Java `testGetJsapiTicket` / `testGetJsapiTicket1`：ticket 双检锁
    // 缓存——3 次调用仅 1 次 HTTP；forceRefresh 强制过期后再次 HTTP。
    let ticket_hits = Arc::new(AtomicUsize::new(0));
    let ticket_hits_clone = ticket_hits.clone();
    let server = MockServer::start(dispatch(move |path| {
        if path.contains("/cgi-bin/ticket/getticket") {
            let hit = ticket_hits_clone.fetch_add(1, Ordering::SeqCst);
            if hit == 0 {
                json_body(
                    r#"{"errcode":0,"errmsg":"ok","ticket":"TICKET_CACHED","expires_in":7200}"#,
                )
            } else {
                json_body(
                    r#"{"errcode":0,"errmsg":"ok","ticket":"TICKET_REFRESHED","expires_in":7200}"#,
                )
            }
        } else {
            json_body(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let jsapi_service = service.jsapi_service().expect("jsapi 服务存在");

    // 3 次非强制调用：缓存命中，仅 1 次 HTTP
    let t1 = jsapi_service
        .get_jsapi_ticket()
        .await
        .expect("获取 jsapi ticket 成功");
    let t2 = jsapi_service
        .get_jsapi_ticket()
        .await
        .expect("获取 jsapi ticket 成功");
    let t3 = jsapi_service
        .get_jsapi_ticket()
        .await
        .expect("获取 jsapi ticket 成功");
    assert_eq!(t1, "TICKET_CACHED");
    assert_eq!(t2, "TICKET_CACHED");
    assert_eq!(t3, "TICKET_CACHED");
    assert_eq!(
        ticket_hits.load(Ordering::SeqCst),
        1,
        "缓存未生效：HTTP 次数 > 1"
    );
    assert!(server.last_path().contains("/cgi-bin/ticket/getticket"));
    assert!(server.last_path().contains("type=jsapi"));
    // 全局计数：1 次 access_token + 1 次 ticket
    assert_eq!(server.request_count(), 2);

    // 强制刷新：过期缓存后再次 HTTP
    let t4 = jsapi_service
        .get_jsapi_ticket_with_force(true)
        .await
        .expect("强制刷新 jsapi ticket 成功");
    assert_eq!(t4, "TICKET_REFRESHED");
    assert_eq!(ticket_hits.load(Ordering::SeqCst), 2);
    assert_eq!(server.request_count(), 3);

    // wx_card 类型走同一缓存通道（type=wx_card）
    let card = jsapi_service
        .get_card_api_ticket()
        .await
        .expect("获取卡券 ticket 成功");
    assert_eq!(card, "TICKET_REFRESHED");
    assert!(server.last_path().contains("type=wx_card"));
}

#[tokio::test]
async fn jsapi_create_jsapi_signature_golden() {
    // 镜像 Java `testCreateJsapiSignature`：签名 =
    // SHA1(排序后以 `&` 连接 jsapi_ticket/noncestr/timestamp/url)。
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/ticket/getticket") {
            json_body(r#"{"errcode":0,"errmsg":"ok","ticket":"TICKET_SIG","expires_in":7200}"#)
        } else {
            json_body(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let jsapi_service = service.jsapi_service().expect("jsapi 服务存在");

    let url = "http://www.qq.com";
    let sig = jsapi_service
        .create_jsapi_signature(url)
        .await
        .expect("生成 jsapi 签名成功");

    // 结构断言
    assert_eq!(sig.app_id, "wxappid");
    assert_eq!(sig.url, url);
    assert_eq!(
        sig.nonce_str.chars().count(),
        16,
        "随机串长度（Java RandomUtils.getRandomStr 16 位）"
    );
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    assert!((now - sig.timestamp).abs() <= 10, "时间戳应为当前秒");

    // 黄金向量重算（与 impl 同一算法：排序后 join("&") 再 SHA1 小写）
    let mut parts = [
        format!("jsapi_ticket={}", "TICKET_SIG"),
        format!("noncestr={}", sig.nonce_str),
        format!("timestamp={}", sig.timestamp),
        format!("url={}", url),
    ];
    parts.sort();
    let expected = hex::encode(sha1::Sha1::digest(parts.join("&").as_bytes()));
    assert_eq!(sig.signature, expected);
    assert_eq!(sig.signature.len(), 40);
}

// ---- 分享（镜像 Java WxMaShareServiceImplTest） ----

#[tokio::test]
async fn share_get_share_info_decrypt() {
    // 镜像 Java `testGetShareInfo`：AES-128-CBC 解密（Java 测试同款
    // session_key/iv），golden fixture 由 pycryptodome 生成。
    let service = WxMaServiceImpl::new_arc(config_with_host("http://127.0.0.1:1"));
    let share_service = service.share_service().expect("分享服务存在");

    let info = share_service
        .get_share_info(SESSION_KEY, SHARE_ENCRYPTED, IV_STR)
        .await
        .expect("解密分享信息成功");
    assert_eq!(info.open_g_id, "opengid_demo_001");
    assert_eq!(
        info,
        WxMaShareInfo {
            open_g_id: "opengid_demo_001".to_string()
        }
    );
}

#[tokio::test]
async fn share_get_group_enter_info_decrypt() {
    // 镜像 Java `testGetGroupEnterInfo`：群入口信息解密解析。
    let service = WxMaServiceImpl::new_arc(config_with_host("http://127.0.0.1:1"));
    let share_service = service.share_service().expect("分享服务存在");

    let info = share_service
        .get_group_enter_info(SESSION_KEY, GROUP_ENTER_ENCRYPTED, IV_STR)
        .await
        .expect("解密群入口信息成功");
    assert_eq!(info.open_g_id, "opengid_demo_002");
    assert_eq!(info.group_openid, "gh_xxx");
    assert_eq!(info.chat_type, 2);
}

// ---- Scheme（镜像 Java WxMaSchemeServiceImplTest） ----

#[tokio::test]
async fn scheme_generate_and_missing_openlink_error() {
    // 镜像 Java `testGenerate`：POST /wxa/generatescheme，请求体
    // jump_wxa.path/query/env_version + is_expire/expire_time，响应取
    // `openlink` 字段。
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/wxa/generatescheme") {
            json_body(r#"{"errcode":0,"errmsg":"ok","openlink":"https://wxaurl.cn/SCHEME_1"}"#)
        } else if path.contains("/wxa/generatenfcscheme") {
            json_body(r#"{"errcode":0,"errmsg":"ok","openlink":"https://wxaurl.cn/SCHEME_NFC"}"#)
        } else {
            json_body(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let scheme_service = service.scheme_service().expect("scheme 服务存在");

    let mut request = WxMaGenerateSchemeRequest::default();
    request.jump_wxa.path = "pages/productView/editPhone/editPhone".to_string();
    request.jump_wxa.query = "".to_string();
    request.is_expire = true;
    request.expire_time = 1700000000;

    let link = scheme_service
        .generate(&request)
        .await
        .expect("生成 scheme 成功");
    assert_eq!(link, "https://wxaurl.cn/SCHEME_1");
    assert!(server.last_path().contains("/wxa/generatescheme"));
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(
        body["jump_wxa"]["path"],
        "pages/productView/editPhone/editPhone"
    );
    assert_eq!(body["jump_wxa"]["query"], "");
    assert_eq!(body["is_expire"], true);
    assert_eq!(body["expire_time"], 1700000000);

    // generateNFC（Java `testGenerateNfc`）
    let nfc = scheme_service
        .generate_nfc(&wx_rust_miniapp::bean::scheme::WxMaGenerateNfcSchemeRequest::default())
        .await
        .expect("生成 NFC scheme 成功");
    assert_eq!(nfc, "https://wxaurl.cn/SCHEME_NFC");
    assert!(server.last_path().contains("/wxa/generatenfcscheme"));

    // 响应缺 openlink：Java `getAsString` 返回 null，Rust 抛
    // `openlink 字段缺失`
    let server2 = MockServer::start(dispatch(|_path| {
        json_body(r#"{"errcode":0,"errmsg":"ok"}"#)
    }))
    .await;
    let service2 = WxMaServiceImpl::new_arc(config_with_host(&server2.url("")));
    let scheme_service2 = service2.scheme_service().expect("scheme 服务存在");
    let err = scheme_service2
        .generate(&request)
        .await
        .expect_err("缺少 openlink 应抛错");
    assert_eq!(err.error_code(), Some(-99));
    assert!(err.to_string().contains("openlink 字段缺失"));
}

// ---- URL Link / Short Link（镜像 Java WxMaLinkServiceImplTest） ----

#[tokio::test]
async fn link_generate_url_link_and_short_link() {
    // 镜像 Java `testGenerateUrlLink` / `testGenerateMultiEnvUrlLink` /
    // `testGenerateShortLink`。
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/wxa/generate_urllink") {
            json_body(r#"{"errcode":0,"errmsg":"ok","url_link":"https://wxaurl.cn/URL_LINK_1"}"#)
        } else if path.contains("/wxa/genwxashortlink") {
            json_body(r#"{"errcode":0,"errmsg":"ok","link":"https://w.url.cn/s/ABCDEF"}"#)
        } else {
            json_body(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let link_service = service.link_service().expect("链接服务存在");

    let mut request = GenerateUrlLinkRequest::default();
    request.path = "pages/tabBar/home/home".to_string();
    request.query = "channel=test".to_string();
    request.is_expire = true;
    request.expire_time = 1700000000;

    let url_link = link_service
        .generate_url_link(&request)
        .await
        .expect("生成 URL Link 成功");
    assert_eq!(url_link, "https://wxaurl.cn/URL_LINK_1");
    assert!(server.last_path().contains("/wxa/generate_urllink"));
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["path"], "pages/tabBar/home/home");
    assert_eq!(body["query"], "channel=test");
    assert_eq!(body["is_expire"], true);
    assert_eq!(body["expire_time"], 1700000000);

    // 短链接：page_url/page_title/is_permanent（Java `testGenerateShortLink`）
    let mut short_request = GenerateShortLinkRequest::default();
    short_request.page_url = "pages/productView/editPhone/editPhone?id=31832".to_string();
    short_request.page_title = "productView".to_string();
    short_request.is_permanent = false;
    let link = link_service
        .generate_short_link(&short_request)
        .await
        .expect("生成短链接成功");
    assert_eq!(link, "https://w.url.cn/s/ABCDEF");
    assert!(server.last_path().contains("/wxa/genwxashortlink"));
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(
        body["page_url"],
        "pages/productView/editPhone/editPhone?id=31832"
    );
    assert_eq!(body["page_title"], "productView");
    assert_eq!(body["is_permanent"], false);
}

#[tokio::test]
async fn link_missing_url_link_field_raises() {
    // 镜像 Java `WxMaLinkServiceImpl.generateUrlLink`：响应缺 `url_link`
    // 抛 `无url_link`（Java `new WxErrorException("无url_link")`）。
    let server = MockServer::start(dispatch(|_path| {
        json_body(r#"{"errcode":0,"errmsg":"ok"}"#)
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let link_service = service.link_service().expect("链接服务存在");

    let request = GenerateUrlLinkRequest::default();
    let err = link_service
        .generate_url_link(&request)
        .await
        .expect_err("缺少 url_link 应抛错");
    assert_eq!(err.error_code(), Some(-99));
    assert!(err.to_string().contains("无url_link"));

    let short_request = GenerateShortLinkRequest::default();
    let err = link_service
        .generate_short_link(&short_request)
        .await
        .expect_err("缺少 link 应抛错");
    assert_eq!(err.error_code(), Some(-99));
    assert!(err.to_string().contains("无link"));
}

#[tokio::test]
async fn link_query_url_link_response() {
    // 镜像 Java `testQueryUrlLink`：POST /wxa/query_urllink，
    // url_link_info.path 与生成时一致。
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/wxa/query_urllink") {
            json_body(
                r#"{"errcode":0,"errmsg":"ok","visit_openid":"openid_1","url_link_info":{"appid":"wxappid","path":"pages/index","query":"","create_time":1700000000,"expire_time":1800000000,"env_version":"release"}}"#,
            )
        } else {
            json_body(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let link_service = service.link_service().expect("链接服务存在");

    let request = QueryUrlLinkRequest {
        url_link: "https://wxaurl.cn/URL_LINK_1".to_string(),
    };
    let result = link_service
        .query_url_link(&request)
        .await
        .expect("查询 URL Link 成功");
    assert_eq!(result.errcode, 0);
    assert_eq!(result.url_link_info.path, "pages/index");
    assert_eq!(result.url_link_info.appid, "wxappid");
    assert!(server.last_path().contains("/wxa/query_urllink"));
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["url_link"], "https://wxaurl.cn/URL_LINK_1");
}

// ---- 插件（镜像 Java WxMaPluginServiceImplTest） ----

#[tokio::test]
async fn plugin_apply_and_get_list() {
    // 镜像 Java `testApplyPlugin` + `testGetPluginList`：动作由请求体
    // `action` 字段区分，共用 /wxa/plugin。
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/wxa/plugin") {
            json_body(r#"{"errcode":0,"errmsg":"ok","plugin_list":[{"appid":"wx4418e3e031e551be","status":"done","nickname":"插件昵称","headimgurl":"https://x"}]}"#)
        } else {
            json_body(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let plugin_service = service.plugin_service().expect("插件服务存在");

    // applyPlugin("wx4418e3e031e551be", null)：Java 传 null reason，
    // Rust 以空串表达；Java Gson 对 null 字段省略输出，Rust 同步省略
    plugin_service
        .apply_plugin("wx4418e3e031e551be", "")
        .await
        .expect("申请插件成功");
    assert!(server.last_path().contains("/wxa/plugin"));
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["action"], "apply");
    assert_eq!(body["plugin_appid"], "wx4418e3e031e551be");
    assert!(
        body.get("reason").is_none(),
        "Java null reason 应省略该字段"
    );

    // getPluginList
    let result = plugin_service
        .get_plugin_list()
        .await
        .expect("获取插件列表成功");
    assert_eq!(result.plugin_list.len(), 1);
    assert_eq!(result.plugin_list[0].app_id, "wx4418e3e031e551be");
    assert_eq!(result.plugin_list[0].status, "done");
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["action"], "list");
}

#[tokio::test]
async fn plugin_unbind_and_update() {
    // 镜像 Java `testUnbindPlugin` + `testUpdatePlugin`。
    let server = MockServer::start(dispatch(|_path| {
        json_body(r#"{"errcode":0,"errmsg":"ok"}"#)
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let plugin_service = service.plugin_service().expect("插件服务存在");

    plugin_service
        .unbind_plugin("wx4418e3e031e551be")
        .await
        .expect("解绑插件成功");
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["action"], "unbind");
    assert_eq!(body["plugin_appid"], "wx4418e3e031e551be");

    plugin_service
        .update_plugin("wx4418e3e031e551be", "2.0.2")
        .await
        .expect("更新插件成功");
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["action"], "update");
    assert_eq!(body["plugin_appid"], "wx4418e3e031e551be");
    assert_eq!(body["user_version"], "2.0.2");
}

// ---- 微信运动（Java 无对应测试类；ADAPTED：镜像
// WxMaRunServiceImpl.getRunStepInfo 解密语义） ----

#[tokio::test]
async fn run_get_run_step_info_decrypt() {
    // 对应 Java `WxMaRunServiceImpl.getRunStepInfo`：
    // `WxMaRunStepInfo.fromJson(WxMaCryptUtils.decrypt(...))`，响应取
    // stepInfoList 数组。
    let service = WxMaServiceImpl::new_arc(config_with_host("http://127.0.0.1:1"));
    let run_service = service.run_service().expect("微信运动服务存在");

    let steps = run_service
        .get_run_step_info(SESSION_KEY, RUN_ENCRYPTED, IV_STR)
        .await
        .expect("解密运动数据成功");
    assert_eq!(steps.len(), 2);
    assert_eq!(steps[0].timestamp, 1547212800);
    assert_eq!(steps[0].step, 90);
    assert_eq!(steps[1].timestamp, 1547299200);
    assert_eq!(steps[1].step, 120);
    assert_eq!(
        steps,
        vec![
            WxMaRunStepInfo {
                timestamp: 1547212800,
                step: 90
            },
            WxMaRunStepInfo {
                timestamp: 1547299200,
                step: 120
            },
        ]
    );
}

// ---- openApi 管理（镜像 Java WxMaOpenApiServiceImplTest） ----

#[tokio::test]
async fn open_api_clear_quota_and_get_api_quota() {
    // 镜像 Java `clearQuota` + `getApiQuota`：clear_quota 请求体
    // {"appid":...}；get_api_quota 请求体 {"cgi_path":...} 并解析 quota。
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/openapi/quota/get") {
            json_body(
                r#"{"errcode":0,"errmsg":"ok","quota":{"daily_limit":10000,"used":10,"remain":9990},"rateLimit":{"call_count":0,"refresh_second":0},"componentRateLimit":{"call_count":0,"refresh_second":0}}"#,
            )
        } else {
            json_body(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let open_api_service = service.open_api_service().expect("openApi 服务存在");

    assert!(open_api_service.clear_quota().await.expect("清空配额成功"));
    assert!(server.last_path().contains("/cgi-bin/clear_quota"));
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["appid"], "wxappid");

    let quota = open_api_service
        .get_api_quota("/cgi-bin/openapi/quota/get")
        .await
        .expect("查询配额成功");
    assert_eq!(quota.quota.daily_limit, 10000);
    assert_eq!(quota.quota.used, 10);
    assert_eq!(quota.quota.remain, 9990);
    assert!(server.last_path().contains("/cgi-bin/openapi/quota/get"));
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["cgi_path"], "/cgi-bin/openapi/quota/get");
}

#[tokio::test]
async fn open_api_rid_info_and_clear_quota_by_app_secret() {
    // 镜像 Java `getApiQuotaInfo` + `clearQuotaByAppSecret`：
    // rid 查询响应含 request 时解析返回，无则返回 None；
    // v2 清空配额 URL 带 appid/appsecret query，请求体为空。
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/openapi/rid/get") {
            json_body(
                r#"{"errcode":0,"errmsg":"ok","request":{"invoke_time":1700000000,"cost_in_ms":10,"request_url":"https://api.weixin.qq.com/cgi-bin/openapi/quota/get","request_body":"","response_body":"","client_ip":"1.2.3.4"}}"#,
            )
        } else {
            json_body(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let open_api_service = service.open_api_service().expect("openApi 服务存在");

    let rid_info = open_api_service
        .get_rid_info("658723fa-2d3a0086-64bc7215")
        .await
        .expect("查询 rid 成功")
        .expect("rid 信息存在");
    assert_eq!(rid_info.invoke_time, 1700000000);
    assert_eq!(
        rid_info.request_url,
        "https://api.weixin.qq.com/cgi-bin/openapi/quota/get"
    );
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["rid"], "658723fa-2d3a0086-64bc7215");

    assert!(
        open_api_service
            .clear_quota_by_app_secret()
            .await
            .expect("按 AppSecret 清空配额成功")
    );
    let path = server.last_path();
    assert!(path.contains("/cgi-bin/clear_quota/v2"));
    assert!(path.contains("appid=wxappid"));
    assert!(path.contains("appsecret=secret"));
    assert!(server.last_body().is_empty(), "v2 清空配额请求体应为空串");
}

// ---- 服务端网络（镜像 Java WxMaInternetServiceImplTest） ----

#[tokio::test]
async fn internet_get_user_encrypt_key_signature_and_query() {
    // 镜像 Java `testGetUserEncryptKey2`：签名 =
    // HmacSHA256(key=Base64 解码后 sessionKey, msg="") 十六进制大写；
    // POST /wxa/business/getuserencryptkey，请求体为空串。
    let openid = "ogu-84hVFTbTt-myGisQESoDJ6BM";
    let session_key = "9ny8n3t0KULoi0deF7T9pw==";
    let server = MockServer::start(dispatch(|_path| {
        json_body(r#"{"errcode":0,"errmsg":"ok"}"#)
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let internet_service = service.internet_service().expect("互联网服务存在");

    let response = internet_service
        .get_user_encrypt_key(openid, session_key)
        .await
        .expect("获取用户加密 key 成功");
    assert_eq!(
        response,
        WxMaInternetResponse {
            errcode: 0,
            errmsg: "ok".to_string()
        }
    );

    // 黄金签名重算（与 impl 同一算法）
    let key = base64::engine::general_purpose::STANDARD
        .decode(session_key)
        .expect("session key base64 解码");
    let mut mac = Hmac::<Sha256>::new_from_slice(&key).expect("HMAC key");
    mac.update(b"");
    let expected_signature = hex::encode_upper(mac.finalize().into_bytes());

    let path = server.last_path();
    assert!(path.contains("/wxa/business/getuserencryptkey"));
    assert!(path.contains("sig_method=hmac_sha256"));
    assert!(path.contains(&format!("openid={openid}")));
    assert!(path.contains(&format!("signature={expected_signature}")));
    assert!(
        server.last_body().is_empty(),
        "Java `post(url, \"\")` 空请求体"
    );

    // 已废弃三参重载（Java `getUserEncryptKey(String, String, String)`）：
    // query 顺序 openid/signature/sig_method
    internet_service
        .get_user_encrypt_key_with_signature(openid, "SIG_FIXED", "hmac_sha256")
        .await
        .expect("带签名获取用户加密 key 成功");
    let path = server.last_path();
    assert!(path.contains("/wxa/business/getuserencryptkey"));
    assert!(path.contains(&format!("openid={openid}")));
    assert!(path.contains("signature=SIG_FIXED"));
    assert!(path.contains("sig_method=hmac_sha256"));
}
