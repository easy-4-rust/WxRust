#![allow(clippy::field_reassign_with_default)]
//! 小程序覆盖率提升：`WxMaService` trait 默认方法（MockServer 模式）。
//!
//! 对应 Java `BaseWxMaServiceImpl` + 门面暴露的 user/security/qrcode/
//! subscribe/msg/internet/link 域方法（Java 三层继承链在 Rust 为 trait 默认
//! 实现）。覆盖：access_token 双通道（标准/稳定版/自定义 %s URL）、签名
//! 校验、登录会话、用户开放数据解密（AES-128-CBC 本地构造密文）、内容
//! 安全、二维码/小程序码全家族（二进制响应 + 落盘）、订阅消息、客服/
//! 统一/动态消息、用户加密 key、URL Link/短链，以及未装配子服务时
//! 默认 getter 返回 None。全程离线。

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::Ordering;

use wx_rust_miniapp::api::WxMaService;
use wx_rust_miniapp::api::r#impl::WxMaServiceImpl;
use wx_rust_miniapp::bean::{
    GenerateShortLinkRequest, GenerateUrlLinkRequest, MsgData, QueryUrlLinkRequest,
    WxMaCodeLineColor, WxMaGetUserNotifyRequest, WxMaKefuMessage, WxMaMediaSecCheckCheckRequest,
    WxMaMsgSecCheckCheckRequest, WxMaServiceNotifyExtRequest, WxMaServiceNotifyRequest,
    WxMaSubscribeMessage, WxMaUniformMessage, WxMaUpdatableMsg,
};
use wx_rust_miniapp::config::WxMaConfig;
use wx_rust_miniapp::config::r#impl::WxMaDefaultConfig;
use wx_rust_miniapp::message::KfText;
use wx_rust_miniapp::util::crypto::wx_ma_crypt_utils::encrypt_with_encrypt_key;

/// 极简 mock HTTP 服务器：按请求路径返回（Content-Type, 字节体），
/// 记录最近一次请求行与请求体（在 coverage_boost_ma_sub_services 的
/// MockServer 基础上扩展二进制响应，供二维码执行器识别图片）。
struct MockServer {
    addr: std::net::SocketAddr,
    last_request_line: Arc<std::sync::Mutex<String>>,
    last_body: Arc<std::sync::Mutex<String>>,
    stop: Arc<std::sync::atomic::AtomicBool>,
}

impl MockServer {
    async fn start<F>(handler: F) -> Self
    where
        F: Fn(&str) -> (String, Vec<u8>) + Send + Sync + 'static,
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
                    if let Some(line) = request.lines().next() {
                        *last_request_line_clone.lock().unwrap() = line.to_string();
                    }
                    if let Some(idx) = request.find("\r\n\r\n") {
                        let body = request[idx + 4..].to_string();
                        *last_body_clone.lock().unwrap() = body;
                    }
                    let path = request
                        .lines()
                        .next()
                        .map(|l| l.split_whitespace().nth(1).unwrap_or("/").to_string())
                        .unwrap_or_else(|| "/".to_string());
                    let (content_type, body) = handler(&path);
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                        body.len(),
                    );
                    let mut bytes = response.into_bytes();
                    bytes.extend_from_slice(&body);
                    let _ = socket.write_all(&bytes).await;
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

/// 构建指向 mock 服务器的小程序配置。
fn config_with_host(host: &str) -> Arc<dyn WxMaConfig> {
    let mut config = WxMaDefaultConfig::new("wxappid", "secret");
    config.set_token("tokentoken");
    config.set_aes_key("abcdefghijklmnopqrstuvwxyz0123456789ABCDEFG");
    let mut host_config = wx_rust_miniapp::config::WxMaHostConfig::new();
    host_config.api_host = host.to_string();
    config.set_host_config(host_config);
    Arc::new(config)
}

/// JSON 响应封装。
fn json(body: &str) -> (String, Vec<u8>) {
    ("application/json".to_string(), body.as_bytes().to_vec())
}

/// 通用路由 handler：token 请求 + 默认 ok + 各子域响应。
fn dispatch(
    handler: impl Fn(&str) -> (String, Vec<u8>) + Send + Sync + 'static,
) -> impl Fn(&str) -> (String, Vec<u8>) + Send + Sync + 'static {
    move |path: &str| {
        if path.contains("/cgi-bin/token") || path.contains("/stable_token") {
            return json(r#"{"access_token":"MOCK_TOKEN","expires_in":7200}"#);
        }
        handler(path)
    }
}

/// 默认 ok 响应。
fn ok() -> (String, Vec<u8>) {
    json(r#"{"errcode":0,"errmsg":"ok"}"#)
}

/// 图片字节响应（二维码执行器按 Content-Type 判定）。
fn jpeg() -> (String, Vec<u8>) {
    (
        "image/jpeg".to_string(),
        vec![0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46],
    )
}

/// 解析最近一次请求体为 JSON。
fn last_body_json(server: &MockServer) -> serde_json::Value {
    serde_json::from_str(&server.last_body()).expect("请求体 JSON")
}

/// AES-128-CBC 本地加密（session_key 通道）：密钥与 IV 同为
/// base64("0123456789abcdef")，hexIv 为其十六进制。
fn session_channel_encrypt(plain: &str) -> (String, String, String) {
    let key_b64 = base64::Engine::encode(
        &base64::engine::general_purpose::STANDARD,
        b"0123456789abcdef",
    );
    let hex_iv = "30313233343536373839616263646566".to_string();
    let encrypted = encrypt_with_encrypt_key(&key_b64, &hex_iv, plain).expect("本地加密成功");
    (key_b64.clone(), encrypted, key_b64)
}

// ══════════════════════════════════════════════════════════════════════════════
// SOURCE_PARITY: access_token 双通道 + 签名校验（镜像 BaseWxMaServiceImpl）
// ══════════════════════════════════════════════════════════════════════════════

/// 对应 Java: getAccessToken / getAccessToken(forceRefresh) / 稳定版 / 自定义 URL。
#[tokio::test]
async fn access_token_standard_stable_and_custom_url() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/custom-token") || path.contains("/stable-custom") {
            json(r#"{"access_token":"MOCK_TOKEN","expires_in":7200}"#)
        } else {
            ok()
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));

    // 标准通道：过期 → 请求 /cgi-bin/token 并缓存
    let token = service.get_access_token().await.expect("获取 token 成功");
    assert_eq!(token, "MOCK_TOKEN");
    assert!(server.last_request_line().contains("/cgi-bin/token"));

    // 缓存命中：不再发请求（返回缓存值）
    let again = service.get_access_token().await.expect("缓存 token");
    assert_eq!(again, "MOCK_TOKEN");

    // 强制刷新
    service
        .get_access_token_with_force(true)
        .await
        .expect("强制刷新成功");

    // 稳定版通道：POST /stable_token
    let stable_config = WxMaDefaultConfig::new("wxappid", "secret");
    stable_config.use_stable_access_token(true);
    let mut host_config = wx_rust_miniapp::config::WxMaHostConfig::new();
    host_config.api_host = server.url("");
    stable_config.set_host_config(host_config);
    let stable_service = WxMaServiceImpl::new_arc(Arc::new(stable_config));
    let token = stable_service
        .get_access_token_with_force(true)
        .await
        .expect("稳定版 token 成功");
    assert_eq!(token, "MOCK_TOKEN");
    assert!(server.last_request_line().contains("/stable_token"));

    // 自定义 accessTokenUrl（%s 依次替换 appid/secret）
    let custom = WxMaDefaultConfig::new("wxappid", "secret");
    custom.set_access_token_url(&format!(
        "{}/custom-token?appid=%s&secret=%s",
        server.url("")
    ));
    let mut host_config = wx_rust_miniapp::config::WxMaHostConfig::new();
    host_config.api_host = server.url("");
    custom.set_host_config(host_config);
    let custom_service = WxMaServiceImpl::new_arc(Arc::new(custom));
    custom_service
        .get_access_token_with_force(true)
        .await
        .expect("自定义 URL token 成功");
    assert!(
        server
            .last_request_line()
            .contains("/custom-token?appid=wxappid&secret=secret")
    );

    // 自定义 URL + 稳定版：URL 原样使用（不做 %s 替换）
    let custom_stable = WxMaDefaultConfig::new("wxappid", "secret");
    custom_stable.use_stable_access_token(true);
    custom_stable.set_access_token_url(&format!("{}/stable-custom", server.url("")));
    let mut host_config = wx_rust_miniapp::config::WxMaHostConfig::new();
    host_config.api_host = server.url("");
    custom_stable.set_host_config(host_config);
    let custom_stable_service = WxMaServiceImpl::new_arc(Arc::new(custom_stable));
    custom_stable_service
        .get_access_token_with_force(true)
        .await
        .expect("稳定版自定义 URL 成功");
    assert!(server.last_request_line().contains("/stable-custom"));
}

/// 对应 Java: extractAccessToken（errcode / 缺 access_token / 成功缓存）+
/// checkSignature（SHA1 排序拼接）。
#[tokio::test]
async fn extract_access_token_and_check_signature() {
    let server = MockServer::start(dispatch(|_p| ok())).await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));

    // errcode != 0
    let err = service
        .extract_access_token(r#"{"errcode":40013,"errmsg":"invalid appid"}"#)
        .expect_err("errcode 应抛错");
    assert_eq!(err.error_code(), Some(40013));

    // 缺 access_token 字段
    let err = service
        .extract_access_token(r#"{"errcode":0,"errmsg":"ok"}"#)
        .expect_err("缺 access_token 应抛错");
    assert_eq!(err.error_code(), Some(-99));

    // 成功：解析并写回配置缓存
    let token = service
        .extract_access_token(r#"{"errcode":0,"access_token":"TK_9","expires_in":60}"#)
        .expect("提取成功");
    assert_eq!(token, "TK_9");
    assert_eq!(
        service.wx_ma_config().access_token().as_deref(),
        Some("TK_9")
    );

    // 非法 JSON
    assert!(service.extract_access_token("not json").is_err());

    // checkSignature：SHA1(token, timestamp, nonce) 排序无分隔拼接
    let timestamp = "1700000000";
    let nonce = "nonce-1";
    let expected = wx_rust_common::util::crypto::Sha1::digest(&["tokentoken", timestamp, nonce])
        .expect("签名计算");
    assert!(service.check_signature(timestamp, nonce, &expected));
    assert!(!service.check_signature(timestamp, nonce, "bad-signature"));

    // token 请求失败（errcode!=0）→ getAccessToken 抛错
    // （不走 dispatch：token 分支会拦截并返回成功响应）
    let err_server =
        MockServer::start(|_p| json(r#"{"errcode":40001,"errmsg":"invalid credential"}"#)).await;
    let err_service = WxMaServiceImpl::new_arc(config_with_host(&err_server.url("")));
    let err = err_service
        .get_access_token()
        .await
        .expect_err("token 接口报错应抛出");
    assert_eq!(err.error_code(), Some(40001));
}

// ══════════════════════════════════════════════════════════════════════════════
// SOURCE_PARITY: 会话 + 用户域（镜像 WxMaServiceImpl 的门面方法）
// ══════════════════════════════════════════════════════════════════════════════

/// 对应 Java: jsCode2SessionInfo / getSessionInfo / getPaidUnionId / setDynamicData。
#[tokio::test]
async fn session_paid_union_id_and_dynamic_data() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/sns/jscode2session") {
            json(r#"{"session_key":"sk-1","openid":"o1","unionid":"u1"}"#)
        } else if path.contains("/wxa/getpaidunionid") {
            json(r#"{"errcode":0,"unionid":"union-9"}"#)
        } else {
            ok()
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));

    // jsCode2SessionInfo
    let session = service
        .js_code_to_session("js-code-1")
        .await
        .expect("登录会话成功");
    assert_eq!(session.openid, "o1");
    assert_eq!(session.session_key, "sk-1");
    assert_eq!(session.unionid, "u1");
    assert!(
        server
            .last_request_line()
            .contains("js_code=js-code-1&grant_type=authorization_code")
            || server
                .last_request_line()
                .contains("grant_type=authorization_code")
    );

    // getSessionInfo 委托同一实现
    let session2 = service
        .get_session_info("js-code-2")
        .await
        .expect("会话信息成功");
    assert_eq!(session2.openid, "o1");

    // getPaidUnionId 全参
    let union_id = service
        .get_paid_union_id("o1", Some("TXN_1"), Some("MCH_1"), Some("NO_1"))
        .await
        .expect("unionid 成功");
    assert_eq!(union_id, "union-9");
    assert!(
        server
            .last_request_line()
            .contains("transaction_id=TXN_1&mch_id=MCH_1&out_trade_no=NO_1")
    );

    // 可选参数缺省（None 与空串均不拼接）
    let union_id = service
        .get_paid_union_id("o1", None, Some(""), None)
        .await
        .expect("unionid 成功");
    assert_eq!(union_id, "union-9");
    assert!(server.last_request_line().contains("/wxa/getpaidunionid"));
    assert!(!server.last_request_line().contains("transaction_id="));

    // setDynamicData：query 为内嵌 JSON 字符串
    service
        .set_dynamic_data(60, "testtype", 1, "[{\"value\":1}]")
        .await
        .expect("动态数据成功");
    let body = last_body_json(&server);
    assert_eq!(body["lifespan"], 60);
    assert_eq!(body["scene"], 1);
    // Java 语义：query 为 "{\"type\":\"...\"}" 的 JSON 字符串（非嵌套对象）
    assert_eq!(body["query"], r#"{"type":"testtype"}"#);
}

/// 对应 Java: getPaidUnionId 缺 unionid 字段抛错。
#[tokio::test]
async fn paid_union_id_missing_field_errors() {
    let server = MockServer::start(dispatch(|_p| json(r#"{"errcode":0}"#))).await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let err = service
        .get_paid_union_id("o1", None, None, None)
        .await
        .expect_err("缺 unionid 应抛错");
    assert_eq!(err.error_code(), Some(-99));
}

/// 对应 Java: getUserInfo / decryptSessionInfo / getPhoneNoInfo（AES-128-CBC）。
#[tokio::test]
async fn user_info_decrypt_and_phone_no_info() {
    let service = WxMaServiceImpl::new_arc(config_with_host("http://127.0.0.1:1"));

    // 用户信息：本地构造密文（gender 为字符串，线格式合法）
    let plain = r#"{"nickName":"小明","gender":"1","language":"zh_CN","city":"广州",
        "province":"广东","country":"CN","avatarUrl":"https://a/1.png",
        "unionId":"union-1","watermark":{"timestamp":1700000000,"appid":"wxappid"}}"#;
    let (key, encrypted, iv) = session_channel_encrypt(plain);
    let user_info = service
        .get_user_info(&key, &encrypted, &iv)
        .await
        .expect("用户信息解密成功");
    assert_eq!(user_info.nick_name, "小明");
    assert_eq!(user_info.union_id, "union-1");
    assert_eq!(user_info.watermark.appid, "wxappid");

    // decryptSessionInfo 返回原始 JSON
    let raw = service
        .decrypt_session_info(&key, &encrypted, &iv)
        .await
        .expect("解密会话信息成功");
    assert!(raw.contains("小明"));

    // getPhoneNoInfo：手机号 JSON
    let phone_plain = r#"{"phoneNumber":"13800000000","purePhoneNumber":"13800000000",
        "countryCode":"86","watermark":{"timestamp":1700000000,"appid":"wxappid"}}"#;
    let (key, encrypted, iv) = session_channel_encrypt(phone_plain);
    let phone = service
        .get_phone_no_info(&key, &encrypted, &iv)
        .await
        .expect("手机号解密成功");
    assert_eq!(phone.phone_number, "13800000000");
    assert_eq!(phone.country_code, "86");

    // 非法密文：解密失败 → Io 错误
    let err = service
        .get_user_info("####", &encrypted, &iv)
        .await
        .expect_err("非法 base64 应报错");
    assert!(matches!(
        err,
        wx_rust_common::error::WxErrorException::Io(_)
    ));
}

/// 对应 Java: setUserStorage（HmacSHA256 签名）+ checkUserInfo（SHA1）。
#[tokio::test]
async fn set_user_storage_and_check_user_info() {
    let server = MockServer::start(dispatch(|_p| ok())).await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));

    let mut kv = HashMap::new();
    kv.insert("key1".to_string(), "value1".to_string());
    kv.insert("key2".to_string(), "value2".to_string());
    service
        .set_user_storage(&kv, "session-key-1", "o1")
        .await
        .expect("上报用户数据成功");
    assert!(server.last_request_line().contains("/wxa/set_user_storage"));
    assert!(
        server
            .last_request_line()
            .contains("sig_method=hmac_sha256")
    );
    assert!(server.last_request_line().contains("openid=o1"));
    let body = last_body_json(&server);
    let kv_list = body["kv_list"].as_array().expect("kv_list 数组");
    assert_eq!(kv_list.len(), 2);

    // checkUserInfo：SHA1(rawData + sessionKey) 小写十六进制
    let raw_data = r#"{"nickName":"小明"}"#;
    let session_key = "session-key-1";
    let mut hasher = sha1::Sha1::new();
    use sha1::Digest as _;
    hasher.update(raw_data.as_bytes());
    hasher.update(session_key.as_bytes());
    let signature = hex::encode(hasher.finalize());
    assert!(service.check_user_info(session_key, raw_data, &signature));
    assert!(!service.check_user_info(session_key, raw_data, "bad"));
}

/// 对应 Java: getPhoneNumber / getPhoneNoInfo(code) / getCode2VerifyInfo / checkSessionKey。
#[tokio::test]
async fn phone_number_code2verify_and_check_session() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/wxa/business/getuserphonenumber") {
            json(
                r#"{"errcode":0,"phone_info":{"phoneNumber":"13900000000",
                "purePhoneNumber":"13900000000","countryCode":"86"}}"#,
            )
        } else if path.contains("code2verifyinfo") || path.contains("code2session") {
            json(r#"{"errcode":0,"session_key":"sk","openid":"o1","is_limit":true}"#)
        } else {
            ok()
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));

    // getPhoneNumber：含 phone_info
    let phone = service
        .get_phone_number("code-1")
        .await
        .expect("手机号成功");
    assert_eq!(phone.expect("phone_info 存在").phone_number, "13900000000");
    let body = last_body_json(&server);
    assert_eq!(body["code"], "code-1");

    // getPhoneNoInfo(code) 委托 getPhoneNumber
    let phone = service
        .get_phone_no_info_with_code("code-1")
        .await
        .expect("手机号（code）成功");
    assert!(phone.is_some());

    // getCode2VerifyInfo
    let verify = service
        .get_code2_verify_info("code-2", "check-1")
        .await
        .expect("多端登录验证成功");
    assert!(verify.is_limit);
    let body = last_body_json(&server);
    assert_eq!(body["code"], "code-2");
    assert_eq!(body["checkcode"], "check-1");

    // checkSessionKey：成功恒返回 true
    let checked = service
        .check_session_key("o1", "sk-1")
        .await
        .expect("检查会话成功");
    assert!(checked);
    assert!(
        server
            .last_request_line()
            .contains("sig_method=hmac_sha256")
    );
}

/// 对应 Java: getPhoneNumber 无 phone_info 字段返回 None。
#[tokio::test]
async fn phone_number_without_info_returns_none() {
    let server = MockServer::start(dispatch(|_p| json(r#"{"errcode":0,"errmsg":"ok"}"#))).await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let phone = service.get_phone_number("code-1").await.expect("请求成功");
    assert_eq!(phone, None);
}

// ══════════════════════════════════════════════════════════════════════════════
// SOURCE_PARITY: 内容安全域（镜像 WxMaSecurityServiceImpl 的门面方法）
// ══════════════════════════════════════════════════════════════════════════════

/// 对应 Java: checkImage(File) / checkMessage / checkMessage(v2) / mediaCheckAsync / getUserRiskRank。
#[tokio::test]
async fn security_image_message_and_media_check() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/wxa/img_sec_check") {
            ok()
        } else if path.contains("/wxa/msg_sec_check") {
            json(
                r#"{"errcode":0,"errmsg":"ok","result":{"suggest":"pass","label":"100"},
                "detail":[]}"#,
            )
        } else if path.contains("/wxa/media_check_async") {
            json(
                r#"{"errcode":0,"errmsg":"ok","trace_id":"trace-1",
                "result":{"suggest":"pass","label":"100"},"detail":[]}"#,
            )
        } else if path.contains("/wxa/getuserriskrank") {
            json(r#"{"errcode":0,"errmsg":"ok","risk_rank":2,"unoin_id":0}"#)
        } else {
            ok()
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));

    // checkImage(File)：本地临时文件 multipart 上传
    let tmp = std::env::temp_dir().join("cov_boost_sec_img.png");
    std::fs::write(&tmp, b"\x89PNG\r\n\x1a\nfake-image-bytes").expect("写入临时文件");
    assert!(
        service
            .check_image_file(tmp.to_str().unwrap())
            .await
            .expect("图片检测成功")
    );
    assert!(server.last_request_line().contains("/wxa/img_sec_check"));
    assert!(server.last_body().contains("media"));

    // checkMessage(String)
    assert!(
        service
            .check_message("待检测文本")
            .await
            .expect("文本检测成功")
    );
    let body = last_body_json(&server);
    assert_eq!(body["content"], "待检测文本");

    // checkMessage(WxMaMsgSecCheckCheckRequest)
    let mut request = WxMaMsgSecCheckCheckRequest::default();
    request.version = "2".to_string();
    request.openid = "o1".to_string();
    request.scene = 1;
    request.content = "v2 内容".to_string();
    let response = service
        .check_message_with_request(&request)
        .await
        .expect("v2 文本检测成功");
    assert_eq!(response.result.suggest, "pass");

    // mediaCheckAsync(String, int)（门面版）
    let result = service
        .media_check_async("https://img.example.com/a.png", 2)
        .await
        .expect("异步媒体检测成功");
    assert_eq!(result.trace_id, "trace-1");

    // mediaCheckAsync(WxMaMediaSecCheckCheckRequest)
    let mut request = WxMaMediaSecCheckCheckRequest::default();
    request.media_url = "https://img.example.com/b.png".to_string();
    request.media_type = 2;
    request.version = 2;
    request.openid = "o1".to_string();
    request.scene = 1;
    let result = service
        .media_check_async_with_request(&request)
        .await
        .expect("v2 异步媒体检测成功");
    assert_eq!(result.trace_id, "trace-1");
    let body = last_body_json(&server);
    assert_eq!(body["version"], 2);

    // getUserRiskRank（门面版）
    let mut risk_request =
        wx_rust_miniapp::bean::safety::request::WxMaUserSafetyRiskRankRequest::default();
    risk_request.appid = "wxappid".to_string();
    risk_request.openid = "o1".to_string();
    risk_request.scene = 1;
    let risk = service
        .get_user_risk_rank(&risk_request)
        .await
        .expect("风险等级成功");
    assert_eq!(risk.risk_rank, 2);
}

/// 对应 Java: QrcodeBytesRequestExecutor —— 响应为 JSON 时视为错误报文。
#[tokio::test]
async fn security_image_file_error_json_response() {
    let server = MockServer::start(dispatch(|_p| {
        json(r#"{"errcode":40001,"errmsg":"invalid credential"}"#)
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let tmp = std::env::temp_dir().join("cov_boost_sec_img_err.png");
    std::fs::write(&tmp, b"fake").expect("写入临时文件");
    let err = service
        .check_image_file(tmp.to_str().unwrap())
        .await
        .expect_err("JSON 响应应抛错");
    assert_eq!(err.error_code(), Some(40001));
}

// ══════════════════════════════════════════════════════════════════════════════
// SOURCE_PARITY: 二维码/小程序码全家族（镜像 WxMaQrcodeServiceImpl）
// ══════════════════════════════════════════════════════════════════════════════

/// 对应 Java: createQrcode 家族（字节 / 临时文件 / 指定目录 / 默认宽度）。
#[tokio::test]
async fn qrcode_create_family() {
    let server = MockServer::start(dispatch(|_p| jpeg())).await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let out_dir = std::env::temp_dir().join("cov-boost-qrcode-dir");
    std::fs::create_dir_all(&out_dir).expect("创建输出目录");

    // createQrcodeBytes
    let bytes = service
        .create_qrcode_bytes("pages/index", 430)
        .await
        .expect("二维码字节成功");
    assert_eq!(bytes[0], 0xFF);
    assert!(
        server
            .last_request_line()
            .contains("/cgi-bin/wxaapp/createwxaqrcode")
    );
    let body = last_body_json(&server);
    assert_eq!(body["path"], "pages/index");
    assert_eq!(body["width"], 430);

    // createQrcode（临时文件）
    let file = service
        .create_qrcode("pages/index", 300)
        .await
        .expect("二维码落盘成功");
    assert!(file.ends_with(".jpg"));

    // createQrcode(path, width, dir)
    let file = service
        .create_qrcode_to_path("pages/index", 300, out_dir.to_str().unwrap())
        .await
        .expect("二维码写入目录成功");
    assert!(file.starts_with(out_dir.to_str().unwrap()));

    // createQrcode(path)（默认 430）
    let file = service
        .create_qrcode_default("pages/index")
        .await
        .expect("默认宽度二维码成功");
    assert!(file.ends_with(".jpg"));

    // createQrcode(path, dir)（默认 430）
    let file = service
        .create_qrcode_default_to_path("pages/index", out_dir.to_str().unwrap())
        .await
        .expect("默认宽度目录二维码成功");
    assert!(file.starts_with(out_dir.to_str().unwrap()));

    // saveQrcodeFile 直调（None → 系统临时目录）
    let file = service
        .save_qrcode_file(&[1, 2, 3], None)
        .await
        .expect("保存二维码成功");
    assert!(file.ends_with(".jpg"));
}

/// 对应 Java: createWxaCode 家族（env_version 默认 release、line_color 可空）。
#[tokio::test]
async fn wxa_code_create_family() {
    let server = MockServer::start(dispatch(|_p| jpeg())).await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let out_dir = std::env::temp_dir().join("cov-boost-wxacode-dir");
    std::fs::create_dir_all(&out_dir).expect("创建输出目录");
    let line_color = WxMaCodeLineColor {
        r: "10".to_string(),
        g: "20".to_string(),
        b: "30".to_string(),
    };

    // 全参字节版：显式 env_version + line_color
    let bytes = service
        .create_wxa_code_bytes(
            "pages/index",
            Some("trial"),
            280,
            false,
            Some(line_color.clone()),
            true,
        )
        .await
        .expect("小程序码字节成功");
    assert_eq!(bytes[0], 0xFF);
    assert!(server.last_request_line().contains("/wxa/getwxacode"));
    let body = last_body_json(&server);
    assert_eq!(body["env_version"], "trial");
    assert_eq!(body["line_color"]["r"], "10");
    assert_eq!(body["is_hyaline"], true);

    // env_version 空 → 默认 release；无 line_color → 字段省略
    let _ = service
        .create_wxa_code_bytes("pages/index", Some(""), 280, true, None, false)
        .await
        .expect("默认 env_version 成功");
    let body = last_body_json(&server);
    assert_eq!(body["env_version"], "release");
    assert!(body.get("line_color").is_none());

    // createWxaCode（临时文件）/ ToPath / 各默认参数便捷版
    let file = service
        .create_wxa_code(
            "pages/index",
            Some("trial"),
            280,
            false,
            Some(line_color),
            true,
        )
        .await
        .expect("小程序码落盘成功");
    assert!(file.ends_with(".jpg"));
    let file = service
        .create_wxa_code_to_path(
            "pages/index",
            None,
            280,
            out_dir.to_str().unwrap(),
            true,
            None,
            false,
        )
        .await
        .expect("小程序码目录成功");
    assert!(file.starts_with(out_dir.to_str().unwrap()));
    let _ = service
        .create_wxa_code_default("pages/index", 430)
        .await
        .expect("默认小程序码成功");
    let _ = service
        .create_wxa_code_default_simple("pages/index")
        .await
        .expect("极简小程序码成功");
    let file = service
        .create_wxa_code_width_to_path("pages/index", 430, out_dir.to_str().unwrap())
        .await
        .expect("宽度+目录小程序码成功");
    assert!(file.starts_with(out_dir.to_str().unwrap()));
    let file = service
        .create_wxa_code_simple_to_path("pages/index", out_dir.to_str().unwrap())
        .await
        .expect("极简+目录小程序码成功");
    assert!(file.starts_with(out_dir.to_str().unwrap()));
}

/// 对应 Java: createWxaCodeUnlimit 家族（scene/check_path/env_version 语义）。
#[tokio::test]
async fn wxa_code_unlimit_family() {
    let server = MockServer::start(dispatch(|_p| jpeg())).await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let out_dir = std::env::temp_dir().join("cov-boost-unlimit-dir");
    std::fs::create_dir_all(&out_dir).expect("创建输出目录");

    // 全参字节版：env_version 与 line_color 均输出
    let bytes = service
        .create_wxa_code_unlimit_bytes(
            "a=1",
            "pages/idx",
            true,
            Some("develop"),
            200,
            false,
            Some(WxMaCodeLineColor {
                r: "1".to_string(),
                g: "2".to_string(),
                b: "3".to_string(),
            }),
            false,
        )
        .await
        .expect("不限量小程序码字节成功");
    assert_eq!(bytes[0], 0xFF);
    assert!(
        server
            .last_request_line()
            .contains("/wxa/getwxacodeunlimit")
    );
    let body = last_body_json(&server);
    assert_eq!(body["scene"], "a=1");
    assert_eq!(body["page"], "pages/idx");
    assert_eq!(body["check_path"], true);
    assert_eq!(body["env_version"], "develop");
    assert_eq!(body["line_color"]["g"], "2");

    // env_version None → 字段省略
    let _ = service
        .create_wxa_code_unlimit_bytes("a=1", "pages/idx", false, None, 200, true, None, true)
        .await
        .expect("省略 env_version 成功");
    let body = last_body_json(&server);
    assert!(body.get("env_version").is_none());
    assert!(body.get("line_color").is_none());
    assert_eq!(body["check_path"], false);

    // 落盘家族
    let file = service
        .create_wxa_code_unlimit("a=1", "pages/idx", true, None, 430, true, None, false)
        .await
        .expect("不限量码落盘成功");
    assert!(file.ends_with(".jpg"));
    let file = service
        .create_wxa_code_unlimit_to_path(
            "a=1",
            "pages/idx",
            out_dir.to_str().unwrap(),
            true,
            None,
            430,
            true,
            None,
            false,
        )
        .await
        .expect("不限量码目录成功");
    assert!(file.starts_with(out_dir.to_str().unwrap()));
    let _ = service
        .create_wxa_code_unlimit_default("a=1", "pages/idx")
        .await
        .expect("默认不限量码成功");
    let file = service
        .create_wxa_code_unlimit_default_to_path("a=1", "pages/idx", out_dir.to_str().unwrap())
        .await
        .expect("默认不限量码目录成功");
    assert!(file.starts_with(out_dir.to_str().unwrap()));
}

/// 对应 Java: 小程序码响应为 JSON 错误报文时抛错。
#[tokio::test]
async fn wxa_code_json_response_errors() {
    let server = MockServer::start(dispatch(|_p| {
        json(r#"{"errcode":40001,"errmsg":"invalid credential"}"#)
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let err = service
        .create_wxa_code_unlimit_bytes("a=1", "pages/idx", true, None, 430, true, None, false)
        .await
        .expect_err("JSON 响应应抛错");
    assert_eq!(err.error_code(), Some(40001));
}

// ══════════════════════════════════════════════════════════════════════════════
// SOURCE_PARITY: 订阅消息域（镜像 WxMaSubscribeServiceImpl 的门面方法）
// ══════════════════════════════════════════════════════════════════════════════

/// 对应 Java: getPubTemplateTitleList / getPubTemplateKeyWordsById / addTemplate /
/// getTemplateList / delTemplate / getCategory / sendSubscribeMsg /
/// setUserNotify / setUserNotifyExt / getUserNotify。
#[tokio::test]
async fn subscribe_domain_facade_methods() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("getpubtemplatetitles") {
            json(r#"{"errcode":0,"count":1,"data":[{"tid":1001,"title":"购买成功通知"}]}"#)
        } else if path.contains("getpubtemplatekeywords") {
            json(r#"{"errcode":0,"data":[{"kid":1,"name":"订单编号","example":"E1"}]}"#)
        } else if path.contains("addtemplate") {
            json(r#"{"errcode":0,"priTmplId":"PRI_1"}"#)
        } else if path.contains("gettemplate") || path.contains("getcategory") {
            json(
                r#"{"errcode":0,"data":[{"priTmplId":"PRI_1","title":"购买成功通知",
                "content":"内容","example":"示例","type":2}]}"#,
            )
        } else if path.contains("getusernotify") {
            json(
                r#"{"errcode":0,"errmsg":"ok","notify_info":{"notify_type":1,
                "content_json":"{}","code_state":0,"code_expire_time":0}}"#,
            )
        } else {
            ok()
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));

    // getPubTemplateTitleList
    let titles = service
        .get_pub_template_title_list(&["1001", "1002"], 0, 10)
        .await
        .expect("公共模板标题成功");
    assert_eq!(titles.count, 1);
    assert_eq!(titles.data[0].tid, 1001);
    assert!(
        server
            .last_request_line()
            .contains("ids=1001,1002&start=0&limit=10")
    );

    // getPubTemplateKeyWordsById
    let keywords = service
        .get_pub_template_keywords_by_id("1001")
        .await
        .expect("关键词列表成功");
    assert_eq!(keywords.len(), 1);
    assert_eq!(keywords[0].kid, 1);

    // addTemplate
    let pri_tmpl_id = service
        .add_template("1001", &[1, 2], "购买场景")
        .await
        .expect("添加模板成功");
    assert_eq!(pri_tmpl_id, "PRI_1");
    let body = last_body_json(&server);
    assert_eq!(body["tid"], "1001");
    assert_eq!(body["kidList"], serde_json::json!([1, 2]));

    // getTemplateList
    let templates = service.get_template_list().await.expect("模板列表成功");
    assert_eq!(templates.len(), 1);
    assert_eq!(templates[0].pri_tmpl_id, "PRI_1");

    // delTemplate
    assert!(service.del_template("PRI_1").await.expect("删除模板成功"));

    // getCategory
    let categories = service.get_category().await.expect("类目成功");
    assert_eq!(categories.len(), 1);

    // sendSubscribeMsg
    let mut message = WxMaSubscribeMessage::default();
    message.to_user = Some("o1".to_string());
    message.template_id = Some("PRI_1".to_string());
    message.data = vec![MsgData {
        name: "thing1".to_string(),
        value: "订单已发货".to_string(),
    }];
    service
        .send_subscribe_msg(&message)
        .await
        .expect("发送订阅消息成功");
    let body = last_body_json(&server);
    assert_eq!(body["touser"], "o1");
    assert_eq!(body["template_id"], "PRI_1");

    // setUserNotify / setUserNotifyExt / getUserNotify
    let notify = WxMaServiceNotifyRequest {
        openid: "o1".to_string(),
        notify_type: 1,
        notify_code: "CODE_1".to_string(),
        content_json: "{}".to_string(),
        check_json: "{}".to_string(),
    };
    service
        .set_user_notify(&notify)
        .await
        .expect("激活服务卡片成功");
    let ext = WxMaServiceNotifyExtRequest {
        openid: "o1".to_string(),
        notify_type: 1,
        notify_code: "CODE_1".to_string(),
        ext_json: "{}".to_string(),
    };
    service
        .set_user_notify_ext(&ext)
        .await
        .expect("更新卡片扩展成功");
    let query = WxMaGetUserNotifyRequest {
        openid: "o1".to_string(),
        notify_code: "CODE_1".to_string(),
        notify_type: 1,
    };
    let result = service.get_user_notify(&query).await.expect("查询卡片成功");
    assert_eq!(result.notify_info.notify_type, 1);
}

// ══════════════════════════════════════════════════════════════════════════════
// SOURCE_PARITY: 消息域 + 网络域 + 链接域（镜像 WxMaMsgService/Internet/Link）
// ══════════════════════════════════════════════════════════════════════════════

/// 对应 Java: sendKefuMsg / sendUniformMsg / createUpdatableMessageActivityId /
/// setUpdatableMsg / getUserEncryptKey ×2 / generateUrlLink / generateShortLink /
/// queryUrlLink。
#[tokio::test]
async fn msg_internet_and_link_facade_methods() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("activityid/create") {
            json(r#"{"errcode":0,"activity_id":"ACT_1"}"#)
        } else if path.contains("generate_urllink") {
            json(r#"{"errcode":0,"url_link":"https://wxaurl.cn/abc"}"#)
        } else if path.contains("genwxashortlink") {
            json(r#"{"errcode":0,"link":"https://wxaurl.cn/short"}"#)
        } else if path.contains("query_urllink") {
            json(
                r#"{"errcode":0,"visit_openid":"o1","url_link_info":{"appid":"wxappid",
                "path":"pages/index","query":"a=1","create_time":1,"expire_time":2,
                "env_version":"release"}}"#,
            )
        } else if path.contains("getuserencryptkey") {
            json(r#"{"errcode":0,"errmsg":"ok","key_info":{"errcode":0,"errmsg":"ok","key":"K","version":1,"expire_in":100,"iv":"abcd"}}"#)
        } else {
            ok()
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));

    // sendKefuMsg
    let mut kefu = WxMaKefuMessage::default();
    kefu.to_user = Some("o1".to_string());
    kefu.msg_type = Some("text".to_string());
    kefu.text = Some(KfText {
        content: Some("你好".to_string()),
    });
    assert!(service.send_kefu_msg(&kefu).await.expect("客服消息成功"));
    let body = last_body_json(&server);
    assert_eq!(body["touser"], "o1");
    assert_eq!(body["text"]["content"], "你好");

    // sendUniformMsg
    let mut uniform = WxMaUniformMessage::default();
    uniform.to_user = Some("o1".to_string());
    uniform.template_id = Some("TMPL_1".to_string());
    service
        .send_uniform_msg(&uniform)
        .await
        .expect("统一消息成功");

    // createUpdatableMessageActivityId
    let activity = service
        .create_updatable_message_activity_id()
        .await
        .expect("动态消息 activity_id 成功");
    assert_eq!(activity["activity_id"], "ACT_1");

    // setUpdatableMsg
    let mut updatable = WxMaUpdatableMsg::default();
    updatable.activity_id = "ACT_1".to_string();
    service
        .set_updatable_msg(&updatable)
        .await
        .expect("动态消息更新成功");

    // getUserEncryptKey（显式签名）
    let response = service
        .get_user_encrypt_key_with_signature("o1", "sig-1", "hmac_sha256")
        .await
        .expect("加密 key（签名）成功");
    assert_eq!(response.errcode, 0);
    assert!(
        server
            .last_request_line()
            .contains("sig_method=hmac_sha256&openid=o1&signature=sig-1")
            || server.last_request_line().contains("openid=o1")
    );

    // getUserEncryptKey（sessionKey 派生签名：Base64 解码后 HmacSHA256 空串）
    let session_key = base64::Engine::encode(
        &base64::engine::general_purpose::STANDARD,
        b"0123456789abcdef",
    );
    let response = service
        .get_user_encrypt_key("o1", &session_key)
        .await
        .expect("加密 key 成功");
    assert_eq!(response.errcode, 0);
    assert!(
        server
            .last_request_line()
            .contains("sig_method=hmac_sha256")
    );

    // 修复 3be78af 后：sessionKey 直接用 UTF-8 字节，不再 Base64 解码，
    // 所以 "####" 不再报错（合法 UTF-8）
    let response = service
        .get_user_encrypt_key("o1", "####")
        .await
        .expect("修复后非 base64 sessionKey 也可用");
    assert_eq!(response.errcode, 0);

    // generateUrlLink
    let mut link_request = GenerateUrlLinkRequest::default();
    link_request.path = "pages/index".to_string();
    link_request.query = "a=1".to_string();
    let url_link = service
        .generate_url_link(&link_request)
        .await
        .expect("URL Link 成功");
    assert_eq!(url_link, "https://wxaurl.cn/abc");

    // generateShortLink
    let mut short_request = GenerateShortLinkRequest::default();
    short_request.page_url = "pages/index".to_string();
    short_request.page_title = "首页".to_string();
    let short_link = service
        .generate_short_link(&short_request)
        .await
        .expect("短链成功");
    assert_eq!(short_link, "https://wxaurl.cn/short");

    // queryUrlLink
    let query = QueryUrlLinkRequest {
        url_link: "https://wxaurl.cn/abc".to_string(),
    };
    let info = service
        .query_url_link(&query)
        .await
        .expect("查询 URL Link 成功");
    assert_eq!(info.visit_openid, "o1");
    assert_eq!(info.url_link_info.path, "pages/index");
}

/// 对应 Java: generateUrlLink / generateShortLink 缺字段抛错。
#[tokio::test]
async fn link_missing_field_errors() {
    let server = MockServer::start(dispatch(|_p| json(r#"{"errcode":0,"errmsg":"ok"}"#))).await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));

    let err = service
        .generate_url_link(&GenerateUrlLinkRequest::default())
        .await
        .expect_err("缺 url_link 应抛错");
    assert_eq!(err.error_code(), Some(-99));

    let err = service
        .generate_short_link(&GenerateShortLinkRequest::default())
        .await
        .expect_err("缺 link 应抛错");
    assert_eq!(err.error_code(), Some(-99));
}

// ══════════════════════════════════════════════════════════════════════════════
// RUST_OBLIGATION: trait 默认子服务 getter（未装配时返回 None）
// ══════════════════════════════════════════════════════════════════════════════

/// 仅实现配置与 HTTP 客户端的最小服务（不覆写任何子服务 getter）。
struct BareMaService {
    config: Arc<dyn WxMaConfig>,
    client: reqwest::Client,
}

#[async_trait::async_trait]
impl WxMaService for BareMaService {
    fn wx_ma_config(&self) -> Arc<dyn WxMaConfig> {
        self.config.clone()
    }

    fn http_client(&self) -> &reqwest::Client {
        &self.client
    }
}

/// 对应 Java: WxMaService 的 getXxxService 覆盖检查 —— 未装配实现恒为 None。
#[tokio::test]
async fn default_sub_service_getters_return_none() {
    let service = BareMaService {
        config: config_with_host("http://127.0.0.1:1"),
        client: reqwest::Client::new(),
    };
    assert!(service.user_service().is_none());
    assert!(service.msg_service().is_none());
    assert!(service.media_service().is_none());
    assert!(service.kefu_service().is_none());
    assert!(service.analysis_service().is_none());
    assert!(service.code_service().is_none());
    assert!(service.express_service().is_none());
    assert!(service.security_service().is_none());
    assert!(service.setting_service().is_none());
    assert!(service.subscribe_service().is_none());
    assert!(service.share_service().is_none());
    assert!(service.scheme_service().is_none());
    assert!(service.link_service().is_none());
    assert!(service.qrcode_service().is_none());
    assert!(service.jsapi_service().is_none());
    assert!(service.plugin_service().is_none());
    assert!(service.run_service().is_none());
    assert!(service.open_api_service().is_none());
    assert!(service.internet_service().is_none());
    assert!(service.shop_account_service().is_none());
    assert!(service.shop_after_sale_service().is_none());
    assert!(service.shop_audit_service().is_none());
    assert!(service.shop_cat_service().is_none());
    assert!(service.shop_coupon_service().is_none());
    assert!(service.shop_delivery_service().is_none());
    assert!(service.shop_img_service().is_none());
    assert!(service.shop_order_service().is_none());
    assert!(service.shop_pay_service().is_none());
    assert!(service.shop_register_service().is_none());
    assert!(service.shop_sharer_service().is_none());
    assert!(service.shop_spu_service().is_none());
    assert!(service.product_service().is_none());
    assert!(service.product_order_service().is_none());
    assert!(service.order_management_service().is_none());
    assert!(service.order_shipping_service().is_none());
    assert!(service.express_delivery_return_service().is_none());
    assert!(service.immediate_delivery_service().is_none());
    assert!(service.employee_relation_service().is_none());
    assert!(service.customservice_work_service().is_none());
    assert!(service.live_service().is_none());
    assert!(service.live_goods_service().is_none());
    assert!(service.live_member_service().is_none());
    assert!(service.cloud_service().is_none());
    assert!(service.vod_service().is_none());
    assert!(service.xpay_service().is_none());
    assert!(service.marketing_service().is_none());
    assert!(service.promotion_service().is_none());
    assert!(service.intracity_service().is_none());
    assert!(service.complaint_service().is_none());
    assert!(service.device_subscribe_service().is_none());
    assert!(service.face_service().is_none());
    assert!(service.reimburse_invoice_service().is_none());
    assert!(service.qrcode_jump_service().is_none());
    assert!(service.ocr_service().is_none());
    assert!(service.img_proc_service().is_none());
}
