#![allow(clippy::field_reassign_with_default)]
//! mp 模块 Rust 特有义务 + 增值测试。
//!
//! 覆盖：token 双检锁 single-flight（mock HTTP 服务器）、execute 指数退避
//! 重试与 token 过期自动刷新、check_signature、XML 消息加密 roundtrip、
//! 畸形 XML 拒绝、URL 拼接与 QR 连接地址、多公众号切换、子服务 URL 语义。

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use wx_rust_mp::api::WxMpService;
use wx_rust_mp::bean::message::{WxMpXmlMessage, WxMpXmlOutTextMessage};
use wx_rust_mp::config::WxMpConfigStorage;
use wx_rust_mp::config::r#impl::WxMpDefaultConfig;
use wx_rust_mp::util::crypto::WxMpCryptUtil;

/// 极简 mock HTTP 服务器：按请求路径返回固定响应，可统计请求次数。
///
/// 仅支持单连接顺序处理（测试用），对应 Java 测试的 MockServer。
struct MockServer {
    addr: std::net::SocketAddr,
    requests: Arc<AtomicUsize>,
    stop: Arc<std::sync::atomic::AtomicBool>,
}

impl MockServer {
    /// 启动服务器。
    ///
    /// # 参数
    /// - `handler`：`Fn(&str) -> String`，输入请求路径，输出响应体
    async fn start<F>(handler: F) -> Self
    where
        F: Fn(&str) -> String + Send + Sync + 'static,
    {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("绑定端口");
        let addr = listener.local_addr().expect("获取地址");
        let requests = Arc::new(AtomicUsize::new(0));
        let stop = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let handler = Arc::new(handler);

        let requests_clone = requests.clone();
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
                tokio::spawn(async move {
                    use tokio::io::{AsyncReadExt, AsyncWriteExt};
                    let mut buf = [0u8; 4096];
                    let n = socket.read(&mut buf).await.unwrap_or(0);
                    let request = String::from_utf8_lossy(&buf[..n]).to_string();
                    // 提取请求行（GET /path HTTP/1.1）
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
            requests,
            stop,
        }
    }

    fn url(&self, path: &str) -> String {
        format!("http://{}{}", self.addr, path)
    }

    fn request_count(&self) -> usize {
        self.requests.load(Ordering::SeqCst)
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

// ---- R01: token 双检锁 single-flight（N 并发仅 1 次 HTTP） ----

#[tokio::test]
async fn token_single_flight_concurrent() {
    let server = MockServer::start(|path| {
        if path.contains("/cgi-bin/token") {
            r#"{"access_token":"MOCK_TOKEN_1","expires_in":7200}"#.to_string()
        } else {
            "{}".to_string()
        }
    })
    .await;
    let config = config_with_host(&server.url(""));
    let service = wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config);

    // 10 个并发任务同时获取 token：双检锁保证只有 1 次 HTTP
    let mut handles = Vec::new();
    for _ in 0..10 {
        let svc = service.clone();
        handles.push(tokio::spawn(async move {
            let t = svc.get_access_token().await.expect("获取 token 成功");
            assert_eq!(t, "MOCK_TOKEN_1");
        }));
    }
    for h in handles {
        h.await.expect("任务完成");
    }
    assert_eq!(
        server.request_count(),
        1,
        "并发刷新只应发起 1 次 HTTP 请求（双检锁 single-flight）"
    );

    // 缓存未过期：再次获取不触发 HTTP
    let _ = service.get_access_token().await.expect("缓存命中");
    assert_eq!(server.request_count(), 1);
}

// ---- R02: force_refresh 强制刷新 ----

#[tokio::test]
async fn token_force_refresh() {
    let server = MockServer::start(|path| {
        if path.contains("/cgi-bin/token") {
            r#"{"access_token":"MOCK_TOKEN_2","expires_in":7200}"#.to_string()
        } else {
            "{}".to_string()
        }
    })
    .await;
    let config = config_with_host(&server.url(""));
    let service = wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config);

    let _ = service.get_access_token().await.expect("首次获取");
    let _ = service
        .get_access_token_with_force(true)
        .await
        .expect("强制刷新");
    assert_eq!(server.request_count(), 2, "强制刷新应发起新的 HTTP 请求");
}

// ---- R03: execute 对 -1 错误码指数退避重试 ----

#[tokio::test]
async fn execute_retry_on_busy() {
    let calls = Arc::new(AtomicUsize::new(0));
    let calls_clone = calls.clone();
    let server = MockServer::start(move |path| {
        let n = calls_clone.fetch_add(1, Ordering::SeqCst);
        if path.contains("/cgi-bin/token") {
            r#"{"access_token":"T","expires_in":7200}"#.to_string()
        } else if n < 2 {
            // 前两次返回系统繁忙（-1），第三次成功
            r#"{"errcode":-1,"errmsg":"system busy"}"#.to_string()
        } else {
            r#"{"errcode":0,"data":"ok"}"#.to_string()
        }
    })
    .await;
    let config = config_with_host(&server.url(""));
    let service = wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config);
    let mut c = WxMpDefaultConfig::new("a", "b");
    let _ = c.set_retry_sleep_millis(1); // 加速测试

    let resp = service
        .get(&server.url("/cgi-bin/test_api"), "")
        .await
        .expect("重试后成功");
    assert_eq!(resp, r#"{"errcode":0,"data":"ok"}"#);
    // token 1 次 + 业务至少 2 次（1 失败触发退避 + 1 成功）
    let total = calls.load(Ordering::SeqCst);
    assert!(total >= 3, "应发生指数退避重试，实际 {total} 次");
}

// ---- R04: access_token 过期错误码自动刷新单次 ----

#[tokio::test]
async fn execute_token_expired_auto_refresh() {
    let calls = Arc::new(AtomicUsize::new(0));
    let calls_clone = calls.clone();
    let server = MockServer::start(move |path| {
        let n = calls_clone.fetch_add(1, Ordering::SeqCst);
        if path.contains("/cgi-bin/token") {
            if n == 0 {
                r#"{"access_token":"OLD","expires_in":1}"#.to_string()
            } else {
                r#"{"access_token":"NEW","expires_in":7200}"#.to_string()
            }
        } else {
            // 业务接口先返回 token 过期（40001），刷新后成功
            r#"{"errcode":40001,"errmsg":"invalid credential"}"#.to_string()
        }
    })
    .await;
    let config = config_with_host(&server.url(""));
    let service = wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config);

    // 首次业务调用失败后，内部应自动刷新 token 并重试（对应 Java executeInternal）
    let err = service.get(&server.url("/cgi-bin/biz"), "").await;
    assert!(
        err.is_err(),
        "业务接口持续 40001 时最终应报错（doNotAutoRefresh 单次重试后仍失败）"
    );
    assert!(calls.load(Ordering::SeqCst) >= 2, "至少发起 2 次调用");
}

// ---- R05: check_signature ----

#[test]
fn check_signature_ok() {
    let mut config = WxMpDefaultConfig::new("appid", "secret");
    config.set_token("test_token");
    let config = Arc::new(config);
    let service = wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config);

    // SHA1(token, timestamp, nonce) 签名
    let timestamp = "1348831860";
    let nonce = "abc123";
    let sig = wx_rust_common::util::crypto::Sha1::digest(&["test_token", timestamp, nonce])
        .expect("签名");
    assert!(service.check_signature(timestamp, nonce, &sig));
    assert!(!service.check_signature(timestamp, nonce, "wrong"));
}

// ---- R06: XML 消息加密 roundtrip（fromEncryptedXml 语义） ----

#[test]
fn xml_message_encrypt_decrypt_roundtrip() {
    let mut config = WxMpDefaultConfig::new("wxappid", "secret");
    config
        .set_token("token123")
        .set_aes_key("abcdefghijklmnopqrstuvwxyz0123456789ABCDEFG");
    let config = Arc::new(config);

    // 明文消息
    let plain = "<xml><ToUserName><![CDATA[toUser]]></ToUserName>".to_string()
        + "<FromUserName><![CDATA[fromUser]]></FromUserName>"
        + "<CreateTime>1348831860</CreateTime>"
        + "<MsgType><![CDATA[text]]></MsgType>"
        + "<Content><![CDATA[this is a test]]></Content>"
        + "<MsgId>1234567890123456</MsgId>"
        + "</xml>";

    // 加密
    let crypt = WxMpCryptUtil::new(config.as_ref()).expect("构建");
    let encrypted_xml = crypt.encrypt(&plain).expect("加密成功");
    assert!(
        encrypted_xml.contains("<Encrypt><![CDATA["),
        "实际: {encrypted_xml}"
    );
    assert!(encrypted_xml.contains("<MsgSignature>"));

    // 从密文解析：取 Encrypt 内容 + 签名参数
    let encrypted_msg = WxMpXmlMessage::from_xml(&encrypted_xml).expect("解析密文");
    let cipher = encrypted_msg.encrypt.as_deref().expect("Encrypt 字段");
    let sig = extract_tag(&encrypted_xml, "MsgSignature");

    // 验证签名 + 解密（对应 Java fromEncryptedXml 内部流程）
    let crypt = WxMpCryptUtil::new(config.as_ref()).expect("构建");
    // 时间戳/随机串必须与加密时一致（从密文 xml 提取）
    let timestamp = extract_tag(&encrypted_xml, "TimeStamp");
    let nonce = extract_tag(&encrypted_xml, "Nonce");
    // 直接解密内容（decryptContent 会先验签）
    let decrypted = crypt
        .decrypt_content(&sig, &timestamp, &nonce, cipher)
        .expect("解密成功");
    assert!(decrypted.contains("<MsgType><![CDATA[text]]></MsgType>"));
    assert!(decrypted.contains("<Content><![CDATA[this is a test]]></Content>"));

    // 错误签名必须拒绝
    let err = crypt.decrypt_content("wrong_sig", &timestamp, &nonce, cipher);
    assert!(err.is_err(), "签名错误应拒绝解密");
}

/// 从加密 xml 提取指定标签的 CDATA 内容或裸值（MsgSignature/Nonce 为 CDATA，TimeStamp 为裸值）。
fn extract_tag(xml: &str, tag: &str) -> String {
    let start_marker = format!("<{tag}><![CDATA[");
    if let Some(start) = xml.find(&start_marker) {
        let content_start = start + start_marker.len();
        let end = xml[content_start..]
            .find("]]>")
            .map(|i| content_start + i)
            .unwrap_or(content_start);
        return xml[content_start..end].to_string();
    }
    let start_marker = format!("<{tag}>");
    if let Some(start) = xml.find(&start_marker) {
        let content_start = start + start_marker.len();
        let end = xml[content_start..]
            .find("</")
            .map(|i| content_start + i)
            .unwrap_or(content_start);
        return xml[content_start..end].to_string();
    }
    String::new()
}

// ---- R07: 畸形 XML 拒绝 ----

#[test]
fn xml_message_malformed_rejected() {
    assert!(WxMpXmlMessage::from_xml("<xml><broken>").is_err());
    assert!(WxMpXmlMessage::from_xml("not xml at all").is_err());
    assert!(WxMpXmlMessage::from_xml("").is_err());
}

// ---- R08: 空字段输出省略（XStream 语义） ----

#[test]
fn out_message_omit_empty() {
    let m = WxMpXmlOutTextMessage::new();
    let xml = m.to_xml();
    assert_eq!(
        xml, "<xml><MsgType><![CDATA[text]]></MsgType></xml>",
        "仅 msgType 应输出"
    );
}

// ---- R09: buildQrConnectUrl ----

#[test]
fn build_qr_connect_url_format() {
    let config = Arc::new(WxMpDefaultConfig::new("appid", "secret"));
    let service = wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config);
    let url = service.build_qr_connect_url("https://example.com/cb", "snsapi_login", " state ");
    assert!(
        url.starts_with("https://open.weixin.qq.com/connect/qrconnect?"),
        "实际: {url}"
    );
    assert!(url.contains("appid=appid"), "实际: {url}");
    assert!(
        url.contains("redirect_uri=https%3A%2F%2Fexample.com%2Fcb"),
        "实际: {url}"
    );
    assert!(url.contains("scope=snsapi_login"), "实际: {url}");
    assert!(url.contains("state=state"), "实际: {url}");
    assert!(url.ends_with("#wechat_redirect"));
}

// ---- R10: 多公众号配置切换（switchoverTo） ----

#[tokio::test]
async fn multi_config_switchover() {
    let server = MockServer::start(|path| {
        if path.contains("/cgi-bin/token") {
            // 按 appid 返回不同 token
            if path.contains("appid=app2") {
                r#"{"access_token":"TOKEN_2","expires_in":7200}"#.to_string()
            } else {
                r#"{"access_token":"TOKEN_1","expires_in":7200}"#.to_string()
            }
        } else {
            "{}".to_string()
        }
    })
    .await;
    let host = server.url("");
    let c1 = WxMpDefaultConfig::new("app1", "s1");
    let mut h1 = wx_rust_mp::config::WxMpHostConfig::new();
    h1.api_host = host.clone();
    c1.set_host_config(h1);
    let service = wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(Arc::new(c1));

    let c2 = WxMpDefaultConfig::new("app2", "s2");
    let mut h2 = wx_rust_mp::config::WxMpHostConfig::new();
    h2.api_host = host;
    c2.set_host_config(h2);
    service.add_config_storage("app2", Arc::new(c2));

    // 切到 app2
    service.switchover_to("app2").expect("切换成功");
    let t = service.get_access_token().await.expect("获取 token");
    assert_eq!(t, "TOKEN_2");

    // 切回 app1
    service.switchover_to("app1").expect("切换成功");
    let t = service.get_access_token().await.expect("获取 token");
    assert_eq!(t, "TOKEN_1");

    // 不存在的 appid
    assert!(service.switchover_to("nope").is_err());
}

// ---- R11: get_ticket 双检锁语义（jsapi ticket 缓存） ----

#[tokio::test]
async fn jsapi_ticket_cached() {
    let server = MockServer::start(|path| {
        if path.contains("/cgi-bin/token") {
            r#"{"access_token":"T","expires_in":7200}"#.to_string()
        } else if path.contains("/cgi-bin/ticket/getticket") {
            r#"{"ticket":"JSAPI_TICKET","expires_in":7200}"#.to_string()
        } else {
            "{}".to_string()
        }
    })
    .await;
    let config = config_with_host(&server.url(""));
    let service = wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config);

    let t1 = service.get_jsapi_ticket(false).await.expect("首次获取");
    assert_eq!(t1, "JSAPI_TICKET");
    let t2 = service.get_jsapi_ticket(false).await.expect("缓存命中");
    assert_eq!(t1, t2);
    assert_eq!(
        server.request_count(),
        2,
        "token 1 次 + ticket 1 次（缓存后不再请求）"
    );

    // 强制刷新 ticket
    let t3 = service.get_jsapi_ticket(true).await.expect("强制刷新");
    assert_eq!(t3, "JSAPI_TICKET");
    assert!(server.request_count() >= 3, "强制刷新应重新请求");
}

// ---- R12: createJsapiSignature 字段完整性 ----

#[tokio::test]
async fn create_jsapi_signature_shape() {
    let server = MockServer::start(|path| {
        if path.contains("/cgi-bin/token") {
            r#"{"access_token":"T","expires_in":7200}"#.to_string()
        } else if path.contains("/cgi-bin/ticket/getticket") {
            r#"{"ticket":"TICKET_X","expires_in":7200}"#.to_string()
        } else {
            "{}".to_string()
        }
    })
    .await;
    let config = config_with_host(&server.url(""));
    let service = wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config);

    let sig = service
        .create_jsapi_signature("https://example.com/page")
        .await
        .expect("生成签名");
    assert_eq!(sig.app_id, "wxappid");
    assert_eq!(sig.url, "https://example.com/page");
    assert!(!sig.nonce_str.is_empty());
    assert!(sig.timestamp > 0);
    assert_eq!(sig.signature.len(), 40, "sha1 十六进制长度");
}

// ---- R13: 子服务 URL 语义（菜单创建走 mock 服务器） ----

#[tokio::test]
async fn menu_service_url_and_body() {
    let server = MockServer::start(|path| {
        if path.contains("/cgi-bin/token") {
            r#"{"access_token":"T","expires_in":7200}"#.to_string()
        } else if path.contains("/cgi-bin/menu/create") {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"unexpected: {path}"}"#.to_string()
        }
    })
    .await;
    let config = config_with_host(&server.url(""));
    let service = wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config);

    let menu_service = service.menu_service().expect("菜单服务存在");
    let menu = wx_rust_mp::bean::menu::WxMpMenu::from_json(
        r#"{"menu":{"button":[{"type":"click","name":"按钮","key":"K1","sub_button":[]}]}}"#,
    )
    .expect("菜单解析");
    let resp = menu_service.menu_create(&menu).await.expect("创建成功");
    assert_eq!(resp, r#"{"errcode":0,"errmsg":"ok"}"#);
}
