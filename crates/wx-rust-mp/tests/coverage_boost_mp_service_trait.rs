#![allow(clippy::field_reassign_with_default)]
//! mp 服务门面（WxMpService trait 默认方法）覆盖率提升测试。
//!
//! 镜像 Java `BaseWxMpServiceImplTest` 的 HTTP 语义，经 MockServer 验证；
//! 覆盖 shortUrl / getCallbackIP / netCheck / getCurrentAutoReplyInfo /
//! clearQuota / genShorten / fetchShorten / buildQrConnectUrl /
//! checkSignature / createJsapiSignature / getJsapiTicket / getTicket /
//! getAccessToken（含 stable 版本）等默认方法。

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
// 长链接转短链接（对应 Java BaseWxMpServiceImpl.shortUrl）
// ========================================================================

/// 对应 Java: BaseWxMpServiceImplTest.shortUrl
#[tokio::test]
async fn short_url_success() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/shorturl") {
            r#"{"short_url":"https://w.url.cn/short123"}"#.to_string()
        } else {
            "{}".to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));

    let short = service
        .short_url("https://example.com/very/long/path?param=value")
        .await
        .expect("短链接成功");
    assert_eq!(short, "https://w.url.cn/short123");
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["action"], "long2short");
    assert_eq!(
        body["long_url"],
        "https://example.com/very/long/path?param=value"
    );
}

/// 对应 Java: BaseWxMpServiceImplTest.shortUrlWithAccessToken
#[tokio::test]
async fn short_url_with_access_token_param_rejected() {
    let server = MockServer::start(dispatch(|_path| "{}".to_string())).await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));

    // URL 包含 &access_token= 应被拒绝（短路校验）
    let err = service
        .short_url("https://example.com/path&access_token=secret&other=1")
        .await
        .expect_err("包含 &access_token= 应报错");
    let msg = err.to_string();
    assert!(
        msg.contains("非法字符") || msg.contains("access_token"),
        "错误信息: {msg}"
    );
}

// ========================================================================
// 获取微信服务器 IP（对应 Java BaseWxMpServiceImpl.getCallbackIP）
// ========================================================================

/// 对应 Java: BaseWxMpServiceImplTest.getCallbackIP
#[tokio::test]
async fn get_callback_ip_success() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/getcallbackip") {
            r#"{"ip_list":["101.226.125.115","101.226.125.116"]}"#.to_string()
        } else {
            "{}".to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));

    let ips = service.get_callback_ip().await.expect("获取服务器 IP 成功");
    assert_eq!(ips.len(), 2);
    assert_eq!(ips[0], "101.226.125.115");
}

// ========================================================================
// 网络检测（对应 Java BaseWxMpServiceImpl.netCheck）
// ========================================================================

/// 对应 Java: BaseWxMpServiceImplTest.netCheck
#[tokio::test]
async fn net_check_success() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/callback/check") {
            r#"{"dnsInfos":[{"ip":"101.226.125.115","realOperator":"电信"}],"pingInfos":[{"ip":"101.226.125.115","fromOperator":"联通","packageLoss":"0","time":"10ms"}]}"#.to_string()
        } else {
            "{}".to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));

    let result = service
        .net_check("all", "CHINA_UNICOM")
        .await
        .expect("网络检测成功");
    assert_eq!(result.dns_infos.len(), 1);
    assert_eq!(result.dns_infos[0].ip, "101.226.125.115");
    assert_eq!(result.ping_infos.len(), 1);
    assert_eq!(result.ping_infos[0].package_loss, "0");
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["action"], "all");
    assert_eq!(body["check_operator"], "CHINA_UNICOM");
}

// ========================================================================
// 自动回复信息（对应 Java BaseWxMpServiceImpl.getCurrentAutoReplyInfo）
// ========================================================================

/// 对应 Java: BaseWxMpServiceImplTest.getCurrentAutoReplyInfo
#[tokio::test]
async fn get_current_auto_reply_info_success() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/get_current_autoreply_info") {
            r#"{"is_add_friend_reply_open":1,"is_autoreply_open":1,"keyword_list_info":[{"type":1,"match_mode":1,"content":"你好","reply_list_info":[{"type":"text","content":"欢迎"}]}]}"#.to_string()
        } else {
            "{}".to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));

    let info = service
        .get_current_auto_reply_info()
        .await
        .expect("获取自动回复信息成功");
    assert!(info.is_add_friend_reply_open.unwrap_or(false));
}

// ========================================================================
// 清空 API 调用次数（对应 Java BaseWxMpServiceImpl.clearQuota）
// ========================================================================

/// 对应 Java: BaseWxMpServiceImplTest.clearQuota
#[tokio::test]
async fn clear_quota_success() {
    let server = MockServer::start(dispatch(|_path| {
        r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));

    service
        .clear_quota("wxappid")
        .await
        .expect("清空调用次数成功");
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["appid"], "wxappid");
}

// ========================================================================
// 短 key 托管（对应 Java BaseWxMpServiceImpl.genShorten / fetchShorten）
// ========================================================================

/// 对应 Java: BaseWxMpServiceImplTest.genShorten
#[tokio::test]
async fn gen_shorten_success() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/shorten/gen") {
            r#"{"short_key":"abc123"}"#.to_string()
        } else {
            "{}".to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));

    let key = service
        .gen_shorten("https://example.com/long/path", 3600)
        .await
        .expect("生成短 key 成功");
    assert_eq!(key, "abc123");
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["long_data"], "https://example.com/long/path");
    assert_eq!(body["expire_seconds"], 3600);
}

/// 对应 Java: BaseWxMpServiceImplTest.fetchShorten
#[tokio::test]
async fn fetch_shorten_success() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/shorten/fetch") {
            r#"{"long_data":"https://example.com/long/path"}"#.to_string()
        } else {
            "{}".to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));

    let result = service
        .fetch_shorten("abc123")
        .await
        .expect("解析短 key 成功");
    assert_eq!(result.long_data, "https://example.com/long/path");
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["short_key"], "abc123");
}

// ========================================================================
// 构建扫码连接地址（对应 Java BaseWxMpServiceImpl.buildQrConnectUrl）
// ========================================================================

/// 对应 Java: BaseWxMpServiceImplTest.buildQrConnectUrl
#[tokio::test]
async fn build_qr_connect_url_success() {
    let server = MockServer::start(dispatch(|_path| "{}".to_string())).await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));

    let url = service.build_qr_connect_url("https://example.com/callback", "snsapi_login", "STATE");
    assert!(url.contains("appid=wxappid"));
    assert!(url.contains("scope=snsapi_login"));
    assert!(url.contains("state=STATE"));
    assert!(url.contains("#wechat_redirect"));
    // redirect_uri 应被编码
    assert!(url.contains("redirect_uri="));
}

// ========================================================================
// 校验签名（对应 Java BaseWxMpServiceImpl.checkSignature）
// ========================================================================

/// 对应 Java: BaseWxMpServiceImplTest.checkSignature
#[tokio::test]
async fn check_signature_valid_and_invalid() {
    let server = MockServer::start(dispatch(|_path| "{}".to_string())).await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));

    // 手动计算预期签名
    let timestamp = "1700000000";
    let nonce = "nonce123";
    let expected = wx_rust_common::util::crypto::Sha1::digest(&["token123", timestamp, nonce])
        .expect("SHA1 成功");

    assert!(service.check_signature(timestamp, nonce, &expected));
    assert!(!service.check_signature(timestamp, nonce, "wrong_signature"));
}

// ========================================================================
// JSAPI 签名（对应 Java BaseWxMpServiceImpl.createJsapiSignature）
// ========================================================================

/// 对应 Java: BaseWxMpServiceImplTest.createJsapiSignature
#[tokio::test]
async fn create_jsapi_signature_success() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/ticket/getticket") {
            r#"{"ticket":"jsapi_ticket_value","expires_in":7200}"#.to_string()
        } else {
            "{}".to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));

    let sig = service
        .create_jsapi_signature("https://example.com/page")
        .await
        .expect("创建 JSAPI 签名成功");
    assert_eq!(sig.app_id, "wxappid");
    assert_eq!(sig.url, "https://example.com/page");
    assert!(!sig.signature.is_empty());
    assert!(!sig.nonce_str.is_empty());
    assert!(sig.timestamp > 0);
}

// ========================================================================
// Ticket 获取（对应 Java BaseWxMpServiceImpl.getTicket / getJsapiTicket）
// ========================================================================

/// 对应 Java: BaseWxMpServiceImplTest.getTicket
#[tokio::test]
async fn get_ticket_force_refresh() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/ticket/getticket") {
            r#"{"ticket":"jsapi_ticket_value","expires_in":7200}"#.to_string()
        } else {
            "{}".to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));

    let ticket = service
        .get_jsapi_ticket(true)
        .await
        .expect("获取 jsapi ticket 成功");
    assert_eq!(ticket, "jsapi_ticket_value");
}

/// 对应 Java: BaseWxMpServiceImplTest.getJsapiTicket
#[tokio::test]
async fn get_jsapi_ticket_cached() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/ticket/getticket") {
            r#"{"ticket":"jsapi_ticket_value","expires_in":7200}"#.to_string()
        } else {
            "{}".to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));

    // 首次获取（强制刷新）
    let ticket1 = service
        .get_jsapi_ticket(true)
        .await
        .expect("获取 jsapi ticket 成功");
    assert_eq!(ticket1, "jsapi_ticket_value");

    // 第二次获取（应使用缓存，不刷新）
    let ticket2 = service
        .get_jsapi_ticket(false)
        .await
        .expect("获取缓存 jsapi ticket 成功");
    assert_eq!(ticket2, "jsapi_ticket_value");
}

// ========================================================================
// Access Token 获取（含 stable 版本）
// ========================================================================

/// 对应 Java: BaseWxMpServiceImplTest.getAccessToken
#[tokio::test]
async fn get_access_token_normal() {
    let server = MockServer::start(dispatch(|_path| {
        r#"{"access_token":"MOCK_TOKEN","expires_in":7200}"#.to_string()
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));

    let token = service
        .get_access_token()
        .await
        .expect("获取 access_token 成功");
    assert_eq!(token, "MOCK_TOKEN");
}

/// 对应 Java: BaseWxMpServiceImplTest.getAccessTokenStable
#[tokio::test]
async fn get_access_token_stable() {
    // 不使用 dispatch，直接返回稳定版 token
    let server = MockServer::start(|_path: &str| {
        r#"{"access_token":"STABLE_TOKEN","expires_in":7200}"#.to_string()
    })
    .await;
    let mut config = WxMpDefaultConfig::new("wxappid", "secret");
    config
        .set_token("token123")
        .set_aes_key("abcdefghijklmnopqrstuvwxyz0123456789ABCDEFG");
    let mut host_config = wx_rust_mp::config::WxMpHostConfig::new();
    host_config.api_host = server.url("");
    config.set_host_config(host_config);
    config.use_stable_access_token(true);
    let config: Arc<dyn WxMpConfigStorage> = Arc::new(config);

    let service = wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config);
    let token = service
        .get_access_token()
        .await
        .expect("获取稳定版 access_token 成功");
    assert_eq!(token, "STABLE_TOKEN");
}

/// 对应 Java: BaseWxMpServiceImplTest.getAccessTokenForceRefresh
#[tokio::test]
async fn get_access_token_force_refresh() {
    let server = MockServer::start(dispatch(|_path| {
        r#"{"access_token":"REFRESHED_TOKEN","expires_in":7200}"#.to_string()
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));

    // 首次获取（dispatch 拦截 /cgi-bin/token 返回 MOCK_TOKEN）
    let token1 = service
        .get_access_token()
        .await
        .expect("获取 access_token 成功");
    assert_eq!(token1, "MOCK_TOKEN");

    // 强制刷新（dispatch 仍返回 MOCK_TOKEN，验证强制刷新路径执行）
    let token2 = service
        .get_access_token_with_force(true)
        .await
        .expect("强制刷新 access_token 成功");
    assert_eq!(token2, "MOCK_TOKEN");
}

// ========================================================================
// OAuth2 扩展（覆盖 oauth2 41.94% → 80%+）
// ========================================================================

/// 对应 Java: WxMpOAuth2ServiceImplTest 增量：buildAuthorizationUrl / getAccessTokenWith /
/// refreshAccessToken
#[tokio::test]
async fn oauth2_build_url_and_refresh() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/sns/oauth2/access_token") {
            r#"{"access_token":"OAUTH_TOKEN","expires_in":7200,"refresh_token":"REFRESH","openid":"o1","scope":"snsapi_userinfo"}"#.to_string()
        } else if path.contains("/sns/oauth2/refresh_token") {
            r#"{"access_token":"REFRESHED_TOKEN","expires_in":7200,"refresh_token":"REFRESH2","openid":"o1","scope":"snsapi_userinfo"}"#.to_string()
        } else if path.contains("/sns/userinfo") {
            r#"{"openid":"o1","nickname":"NICK","sex":1}"#.to_string()
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

    // 构建授权 URL
    let url =
        oauth2.build_authorization_url("https://example.com/callback", "snsapi_userinfo", "STATE");
    assert!(url.contains("appid=wxappid"));
    assert!(url.contains("scope=snsapi_userinfo"));

    // 使用指定 app_id 获取 token
    let token = oauth2
        .get_access_token_with("wxappid", "secret", "CODE_1")
        .await
        .expect("获取 OAuth2 token（指定 app_id）成功");
    assert_eq!(token.access_token, "OAUTH_TOKEN");
    assert_eq!(token.open_id, "o1");

    // 刷新 token
    let refreshed = oauth2
        .refresh_access_token("REFRESH")
        .await
        .expect("刷新 OAuth2 token 成功");
    assert_eq!(refreshed.access_token, "REFRESHED_TOKEN");

    // 获取用户信息（用新 token）
    let user = oauth2
        .get_user_info(&refreshed, "zh_CN")
        .await
        .expect("获取用户信息成功");
    assert_eq!(user.nickname, "NICK");
}
