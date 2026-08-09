#![allow(clippy::field_reassign_with_default)]
//! 企业微信门面服务（`WxCpService` / `WxCpServiceImpl`）集成测试。
//!
//! 镜像 Java `BaseWxCpServiceImplTest` / `WxCpServiceGetContactAccessTokenTest`
//! / `WxCpServiceGetMsgAuditAccessTokenTest` 的语义，经 MockServer 验证
//! （模式照抄 miniapp `tests/sub_domain_g1_core.rs`，自含无外部依赖）。
//!
//! 覆盖：
//! - token 双检锁并发（并发 20 个 `getAccessToken` 只刷新一次）与缓存命中
//! - 通讯录同步 / 会话存档 access_token 专用通道（secret 未配置报错、
//!   `getForContact`/`postForContact`/`postForMsgAudit` URL 拼接）
//! - jsapi_ticket / agent jsapi_ticket 获取与缓存
//! - `createJsapiSignature` / `createAgentJsapiSignature` 签名算法（重算比对）
//! - `getCallbackIp` / `getApiDomainIp` 响应解析
//! - 新实现方法 `jsCode2Session` / `getProviderToken`（请求路径 + 响应解析）
//! - batch 三件套（`replaceParty`/`syncUser`/`replaceUser`）、`getTaskResult`、
//!   `postWithoutToken`、`buildQrConnectUrl`、`checkSignature`

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use wx_rust_common::config::WxConfigStorage;
use wx_rust_common::util::crypto::Sha1;
use wx_rust_cp::api::WxCpService;
use wx_rust_cp::api::r#impl::WxCpServiceImpl;
use wx_rust_cp::config::r#impl::WxCpDefaultConfig;
use wx_rust_cp::config::{WxCpConfigStorage, WxCpHostConfig};

/// 极简 mock HTTP 服务器：按请求路径返回 (Content-Type, body)，记录
/// 最近一次请求路径（含 query）与请求体、请求计数。
struct MockServer {
    addr: std::net::SocketAddr,
    requests: Arc<AtomicUsize>,
    last_path: Arc<std::sync::Mutex<String>>,
    last_body: Arc<std::sync::Mutex<String>>,
    stop: Arc<std::sync::atomic::AtomicBool>,
}

impl MockServer {
    /// 启动服务器（`handler(path) -> (content_type, body)`）。
    async fn start<F>(handler: F) -> Self
    where
        F: Fn(&str) -> (String, String) + Send + Sync + 'static,
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
                    if let Some(path) = request
                        .lines()
                        .next()
                        .and_then(|l| l.split_whitespace().nth(1))
                    {
                        *last_path_clone.lock().unwrap() = path.to_string();
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
                        "HTTP/1.1 200 OK\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
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

/// JSON 响应快捷构造。
fn json(body: &str) -> (String, String) {
    ("application/json".to_string(), body.to_string())
}

/// 指向 mock 服务器的主机配置（api_host 覆盖）。
fn host_config(host: &str) -> WxCpHostConfig {
    let mut config = WxCpHostConfig::new();
    config.api_host = host.to_string();
    config
}

/// 构建指向 mock 服务器的默认配置（corpid=corpid, secret=secret,
/// token=token123, agentid=101）。
fn config_with_host(host: &str) -> Arc<dyn WxCpConfigStorage> {
    let mut config = WxCpDefaultConfig::new("corpid", "secret");
    config.set_token("token123");
    config.set_agent_id(Some(101));
    config.set_host_config(host_config(host));
    Arc::new(config)
}

/// 通用路由 handler：token 请求先应答（按 corpsecret 区分普通/通讯录同步/
/// 会话存档三类 token），业务路径按 contains 分派。
fn dispatch(
    handler: impl Fn(&str) -> (String, String) + Send + Sync + 'static,
) -> impl Fn(&str) -> (String, String) + Send + Sync + 'static {
    move |path: &str| {
        if path.contains("/cgi-bin/gettoken") {
            if path.contains("corpsecret=contact_secret") {
                return json(r#"{"access_token":"CONTACT_TOKEN","expires_in":7200}"#);
            }
            if path.contains("corpsecret=msg_audit_secret") {
                return json(r#"{"access_token":"MSG_AUDIT_TOKEN","expires_in":7200}"#);
            }
            return json(r#"{"access_token":"MOCK_TOKEN","expires_in":7200}"#);
        }
        handler(path)
    }
}

// ---- token：双检锁并发 + 缓存（镜像 Java getAccessToken(boolean)） ----

#[tokio::test]
async fn get_access_token_double_check_lock_concurrency() {
    // 并发 20 个 getAccessToken()：双检锁（isAccessTokenExpired 预检 +
    // accessTokenLock 锁内二次判断）保证只有 1 次 token HTTP 请求。
    let server = MockServer::start(dispatch(|_path| json("{}"))).await;
    let service = WxCpServiceImpl::new_arc(config_with_host(&server.url("")));

    let mut handles = Vec::new();
    for _ in 0..20 {
        let svc = service.clone();
        handles.push(tokio::spawn(async move { svc.get_access_token().await }));
    }
    for handle in handles {
        assert_eq!(handle.await.unwrap().expect("token 获取成功"), "MOCK_TOKEN");
    }
    assert_eq!(server.request_count(), 1, "双检锁下并发刷新只发生 1 次");
    let path = server.last_path();
    assert!(path.contains("/cgi-bin/gettoken"), "路径: {path}");
    assert!(path.contains("corpid=corpid"), "路径: {path}");
    assert!(path.contains("corpsecret=secret"), "路径: {path}");

    // forceRefresh=true 强制再刷新一次（镜像 Java getAccessToken(true)）
    let refreshed = service
        .get_access_token_with_force(true)
        .await
        .expect("强制刷新成功");
    assert_eq!(refreshed, "MOCK_TOKEN");
    assert_eq!(server.request_count(), 2, "强制刷新重新请求 token");
}

#[tokio::test]
async fn get_access_token_uses_cache_when_not_expired() {
    let server = MockServer::start(dispatch(|_path| json("{}"))).await;
    let config = WxCpDefaultConfig::new("corpid", "secret");
    config.set_host_config(host_config(&server.url("")));
    // 预置未过期 token（镜像 Java updateAccessToken("cached_token", 7200)）
    config.update_access_token("cached_token", 7200);
    let service = WxCpServiceImpl::new_arc(Arc::new(config));

    assert_eq!(
        service.get_access_token().await.expect("返回缓存 token"),
        "cached_token"
    );
    assert_eq!(
        server.request_count(),
        0,
        "未过期 token 直接走缓存，不发请求"
    );
}

// ---- 通讯录同步 access token 通道（镜像 WxCpServiceGetContactAccessTokenTest） ----

#[tokio::test]
async fn contact_access_token_channel_and_get_post_for_contact() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/user/get") {
            json(r#"{"errcode":0,"errmsg":"ok","userid":"zhangsan"}"#)
        } else if path.contains("/cgi-bin/user/add") {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        } else {
            json("{}")
        }
    }))
    .await;

    // 未配置通讯录同步 secret：报错（镜像 Java `new WxErrorException("通讯录同步secret未配置")`）
    let service = WxCpServiceImpl::new_arc(config_with_host(&server.url("")));
    let err = service
        .get_for_contact(&server.url("/cgi-bin/user/get"), "")
        .await
        .unwrap_err();
    assert!(
        format!("{err}").contains("通讯录同步secret未配置"),
        "错误信息: {err}"
    );

    // 配置通讯录同步 secret 后：用独立的通讯录同步 access_token 发请求
    let config = WxCpDefaultConfig::new("corpid", "secret");
    config.set_contact_secret("contact_secret");
    config.set_host_config(host_config(&server.url("")));
    let service = WxCpServiceImpl::new_arc(Arc::new(config));

    let resp = service
        .get_for_contact(&server.url("/cgi-bin/user/get"), "userid=zhangsan")
        .await
        .expect("getForContact 成功");
    assert_eq!(resp, r#"{"errcode":0,"errmsg":"ok","userid":"zhangsan"}"#);
    let path = server.last_path();
    assert!(path.contains("/cgi-bin/user/get"), "路径: {path}");
    assert!(
        path.contains("access_token=CONTACT_TOKEN"),
        "使用通讯录同步 token，路径: {path}"
    );
    assert!(path.contains("userid=zhangsan"), "路径: {path}");

    let resp = service
        .post_for_contact(&server.url("/cgi-bin/user/add"), r#"{"userid":"zhangsan"}"#)
        .await
        .expect("postForContact 成功");
    assert_eq!(resp, r#"{"errcode":0,"errmsg":"ok"}"#);
    let path = server.last_path();
    assert!(path.contains("/cgi-bin/user/add"), "路径: {path}");
    assert!(
        path.contains("access_token=CONTACT_TOKEN"),
        "使用通讯录同步 token，路径: {path}"
    );
    assert!(
        server.last_body().contains("\"userid\":\"zhangsan\""),
        "body: {}",
        server.last_body()
    );
}

// ---- 会话存档 access token 通道（镜像 WxCpServiceGetMsgAuditAccessTokenTest） ----

#[tokio::test]
async fn msg_audit_access_token_channel_and_post_for_msg_audit() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/msgaudit/get_room_data") {
            json(r#"{"errcode":0,"errmsg":"ok","roomid":"r1"}"#)
        } else {
            json("{}")
        }
    }))
    .await;

    // 未配置会话存档 secret：报错（镜像 Java `new WxErrorException("会话存档secret未配置")`）
    let service = WxCpServiceImpl::new_arc(config_with_host(&server.url("")));
    let err = service
        .post_for_msg_audit(&server.url("/cgi-bin/msgaudit/get_room_data"), "{}")
        .await
        .unwrap_err();
    assert!(
        format!("{err}").contains("会话存档secret未配置"),
        "错误信息: {err}"
    );

    // 配置会话存档 secret 后：用独立的会话存档 access_token 发请求
    let config = WxCpDefaultConfig::new("corpid", "secret");
    config.set_msg_audit_secret("msg_audit_secret");
    config.set_host_config(host_config(&server.url("")));
    let service = WxCpServiceImpl::new_arc(Arc::new(config));

    let resp = service
        .post_for_msg_audit(
            &server.url("/cgi-bin/msgaudit/get_room_data"),
            r#"{"roomid":"r1"}"#,
        )
        .await
        .expect("postForMsgAudit 成功");
    assert_eq!(resp, r#"{"errcode":0,"errmsg":"ok","roomid":"r1"}"#);
    let path = server.last_path();
    assert!(
        path.contains("/cgi-bin/msgaudit/get_room_data"),
        "路径: {path}"
    );
    assert!(
        path.contains("access_token=MSG_AUDIT_TOKEN"),
        "使用会话存档 token，路径: {path}"
    );
    assert!(
        server.last_body().contains("\"roomid\":\"r1\""),
        "body: {}",
        server.last_body()
    );
}

// ---- jsapi_ticket / agent jsapi_ticket（镜像 Java getJsapiTicket(boolean)） ----

#[tokio::test]
async fn jsapi_ticket_and_agent_jsapi_ticket() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/get_jsapi_ticket") {
            json(r#"{"errcode":0,"errmsg":"ok","ticket":"TICKET_JSAPI","expires_in":7200}"#)
        } else if path.contains("/cgi-bin/ticket/get") {
            json(r#"{"errcode":0,"errmsg":"ok","ticket":"TICKET_AGENT","expires_in":7200}"#)
        } else {
            json("{}")
        }
    }))
    .await;
    let service = WxCpServiceImpl::new_arc(config_with_host(&server.url("")));

    // jsapi ticket：token 请求 + ticket 请求共 2 次
    let ticket = service.get_jsapi_ticket().await.expect("jsapi ticket");
    assert_eq!(ticket, "TICKET_JSAPI");
    let path = server.last_path();
    assert!(path.contains("/cgi-bin/get_jsapi_ticket"), "路径: {path}");
    assert!(path.contains("access_token=MOCK_TOKEN"), "路径: {path}");

    // 缓存生效：再次获取不再请求
    let before = server.request_count();
    assert_eq!(
        service.get_jsapi_ticket().await.expect("jsapi ticket"),
        "TICKET_JSAPI"
    );
    assert_eq!(server.request_count(), before, "未过期 ticket 走缓存");

    // agent jsapi ticket：走 GET_AGENT_CONFIG_TICKET（ticket 请求 1 次）
    assert_eq!(
        service
            .get_agent_jsapi_ticket()
            .await
            .expect("agent jsapi ticket"),
        "TICKET_AGENT"
    );
    let path = server.last_path();
    assert!(path.contains("/cgi-bin/ticket/get"), "路径: {path}");
    assert!(path.contains("type=agent_config"), "路径: {path}");

    // 强制刷新 agent jsapi ticket：重新请求
    assert_eq!(
        service
            .get_agent_jsapi_ticket_with_force(true)
            .await
            .expect("agent jsapi ticket"),
        "TICKET_AGENT"
    );
}

// ---- jsapi 签名（镜像 Java createJsapiSignature / createAgentJsapiSignature） ----

#[tokio::test]
async fn create_jsapi_signature_matches_sha1_algorithm() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/get_jsapi_ticket") {
            json(r#"{"errcode":0,"errmsg":"ok","ticket":"TICKET_JSAPI","expires_in":7200}"#)
        } else {
            json("{}")
        }
    }))
    .await;
    let service = WxCpServiceImpl::new_arc(config_with_host(&server.url("")));

    let url = "https://example.com/page?a=1&b=2";
    let sig = service
        .create_jsapi_signature(url)
        .await
        .expect("jsapi 签名");
    // 对应 Java：appId 固定取 corpid
    assert_eq!(sig.app_id, "corpid");
    assert_eq!(sig.url, url);
    // SHA1.genWithAmple(jsapi_ticket=.., noncestr=.., timestamp=.., url=..)
    // 排序后以 & 连接；用返回值重算比对（noncestr/timestamp 随机，无法预置 golden）
    let expected = Sha1::digest_with_amp(&[
        &format!("jsapi_ticket={}", "TICKET_JSAPI"),
        &format!("noncestr={}", sig.nonce_str),
        &format!("timestamp={}", sig.timestamp),
        &format!("url={url}"),
    ])
    .expect("sha1 计算");
    assert_eq!(sig.signature, expected);
}

#[tokio::test]
async fn create_agent_jsapi_signature_matches_sha1_algorithm() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/ticket/get") {
            json(r#"{"errcode":0,"errmsg":"ok","ticket":"TICKET_AGENT","expires_in":7200}"#)
        } else {
            json("{}")
        }
    }))
    .await;
    let service = WxCpServiceImpl::new_arc(config_with_host(&server.url("")));

    let url = "https://example.com/agent/page";
    let sig = service
        .create_agent_jsapi_signature(url)
        .await
        .expect("agent jsapi 签名");
    // 对应 Java：携带 corpid/agentid（config 中 agentid=101）
    assert_eq!(sig.corpid, "corpid");
    assert_eq!(sig.agentid, 101);
    assert_eq!(sig.url, url);
    let expected = Sha1::digest_with_amp(&[
        &format!("jsapi_ticket={}", "TICKET_AGENT"),
        &format!("noncestr={}", sig.nonce_str),
        &format!("timestamp={}", sig.timestamp),
        &format!("url={url}"),
    ])
    .expect("sha1 计算");
    assert_eq!(sig.signature, expected);
}

// ---- 服务器 IP 段（镜像 Java getCallbackIp / getApiDomainIp） ----

#[tokio::test]
async fn get_callback_ip_and_api_domain_ip() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/getcallbackip") {
            json(r#"{"errcode":0,"errmsg":"ok","ip_list":["101.226.103.*","101.226.62.*"]}"#)
        } else if path.contains("/cgi-bin/get_api_domain_ip") {
            json(r#"{"errcode":0,"errmsg":"ok","ip_list":["10.0.0.1","10.0.0.2"]}"#)
        } else {
            json("{}")
        }
    }))
    .await;
    let service = WxCpServiceImpl::new_arc(config_with_host(&server.url("")));

    let ips = service.get_callback_ip().await.expect("回调 IP");
    assert_eq!(ips, vec!["101.226.103.*", "101.226.62.*"]);
    assert!(server.last_path().contains("/cgi-bin/getcallbackip"));

    let ips = service.get_api_domain_ip().await.expect("接口 IP");
    assert_eq!(ips, vec!["10.0.0.1", "10.0.0.2"]);
    assert!(server.last_path().contains("/cgi-bin/get_api_domain_ip"));

    // 响应缺 ip_list 字段：报错（镜像 getIp 的 Gson 解析异常）
    let bad_server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/bad_ip") {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        } else {
            json("{}")
        }
    }))
    .await;
    let service = WxCpServiceImpl::new_arc(config_with_host(&bad_server.url("")));
    let err = service.get_callback_ip().await.unwrap_err();
    assert!(
        format!("{err}").contains("ip_list 字段缺失"),
        "错误信息: {err}"
    );
}

// ---- 新实现方法（镜像 Java BaseWxCpServiceImpl.jsCode2Session） ----

#[tokio::test]
async fn js_code_2_session_request_and_parse() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/miniprogram/jscode2session") {
            json(
                r#"{"errcode":0,"errmsg":"ok","session_key":"sk_abc","userid":"zhangsan","corpid":"corpid"}"#,
            )
        } else {
            json("{}")
        }
    }))
    .await;
    let service = WxCpServiceImpl::new_arc(config_with_host(&server.url("")));

    let result = service
        .js_code_2_session("code_111")
        .await
        .expect("登录凭证校验");
    // 对应 Java fromJson：session_key/userid/corpid
    assert_eq!(result.session_key, "sk_abc");
    assert_eq!(result.user_id, "zhangsan");
    assert_eq!(result.corp_id, "corpid");

    let path = server.last_path();
    assert!(
        path.contains("/cgi-bin/miniprogram/jscode2session"),
        "路径: {path}"
    );
    assert!(path.contains("js_code=code_111"), "路径: {path}");
    assert!(
        path.contains("grant_type=authorization_code"),
        "路径: {path}"
    );
    assert!(path.contains("access_token=MOCK_TOKEN"), "路径: {path}");
}

// ---- 新实现方法（镜像 Java BaseWxCpServiceImpl.getProviderToken） ----

#[tokio::test]
async fn get_provider_token_request_and_parse() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/service/get_provider_token") {
            json(
                r#"{"errcode":0,"errmsg":"ok","provider_access_token":"enLSZ5xxxxxxJRL","expires_in":7200}"#,
            )
        } else {
            json("{}")
        }
    }))
    .await;
    let service = WxCpServiceImpl::new_arc(config_with_host(&server.url("")));

    let token = service
        .get_provider_token("corp_id_1", "provider_secret_1")
        .await
        .expect("服务商凭证");
    assert_eq!(token.provider_access_token, "enLSZ5xxxxxxJRL");
    assert_eq!(token.expires_in, 7200);

    let path = server.last_path();
    assert!(
        path.contains("/cgi-bin/service/get_provider_token"),
        "路径: {path}"
    );
    // 镜像 Java `this.post(...)`：走标准执行引擎，自动带 access_token
    assert!(path.contains("access_token=MOCK_TOKEN"), "路径: {path}");
    // 镜像 Java Gson JsonObject 插入序：corpid 在前，provider_secret 在后
    assert_eq!(
        server.last_body(),
        r#"{"corpid":"corp_id_1","provider_secret":"provider_secret_1"}"#,
        "请求体应与 Java Gson 序列化一致"
    );
}

// ---- batch 三件套 + 异步任务结果（镜像 Java replaceParty/syncUser/replaceUser/getTaskResult） ----

#[tokio::test]
async fn batch_replace_party_sync_user_replace_user() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/batch/syncuser") {
            json(r#"{"errcode":0,"errmsg":"ok","jobid":"job_123"}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let service = WxCpServiceImpl::new_arc(config_with_host(&server.url("")));

    // replaceParty：POST {media_id}
    let resp = service
        .replace_party("media_1")
        .await
        .expect("replaceParty");
    assert_eq!(resp, r#"{"errcode":0,"errmsg":"ok"}"#);
    assert!(server.last_path().contains("/cgi-bin/batch/replaceparty"));
    assert!(
        server.last_body().contains("\"media_id\":\"media_1\""),
        "body: {}",
        server.last_body()
    );

    // syncUser：返回 jobid
    let job = service.sync_user("media_2").await.expect("syncUser");
    assert_eq!(job, "job_123");
    assert!(server.last_path().contains("/cgi-bin/batch/syncuser"));
    assert!(
        server.last_body().contains("\"media_id\":\"media_2\""),
        "body: {}",
        server.last_body()
    );

    // replaceUser：POST {media_id}
    let resp = service.replace_user("media_3").await.expect("replaceUser");
    assert_eq!(resp, r#"{"errcode":0,"errmsg":"ok"}"#);
    assert!(server.last_path().contains("/cgi-bin/batch/replaceuser"));
}

#[tokio::test]
async fn get_task_result_appends_jobid() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/batch/getresult") {
            json(r#"{"errcode":0,"errmsg":"ok","status":1}"#)
        } else {
            json("{}")
        }
    }))
    .await;
    let service = WxCpServiceImpl::new_arc(config_with_host(&server.url("")));

    let resp = service.get_task_result("job_abc").await.expect("任务结果");
    assert_eq!(resp, r#"{"errcode":0,"errmsg":"ok","status":1}"#);
    let path = server.last_path();
    assert!(
        path.contains("/cgi-bin/batch/getresult?jobid=job_abc"),
        "jobid 拼在路径后，路径: {path}"
    );
    assert!(path.contains("access_token=MOCK_TOKEN"), "路径: {path}");
}

// ---- postWithoutToken（镜像 Java postWithoutToken：不自动带 accessToken） ----

#[tokio::test]
async fn post_without_token_does_not_inject_token() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/custom_api") {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        } else {
            json("{}")
        }
    }))
    .await;
    let service = WxCpServiceImpl::new_arc(config_with_host(&server.url("")));

    let resp = service
        .post_without_token(&server.url("/cgi-bin/custom_api"), r#"{"k":"v"}"#)
        .await
        .expect("postWithoutToken");
    assert_eq!(resp, r#"{"errcode":0,"errmsg":"ok"}"#);
    let path = server.last_path();
    assert!(path.contains("/cgi-bin/custom_api"), "路径: {path}");
    assert!(
        !path.contains("access_token="),
        "postWithoutToken 不应自动带 token，路径: {path}"
    );
    assert!(
        server.last_body().contains("\"k\":\"v\""),
        "body: {}",
        server.last_body()
    );
    // execute_normal 通道不触发 token 请求
    assert_eq!(server.request_count(), 1, "仅业务请求 1 次");
}

// ---- buildQrConnectUrl + checkSignature（镜像 Java 二维码链接与消息签名） ----

#[tokio::test]
async fn build_qr_connect_url_and_check_signature() {
    let server = MockServer::start(dispatch(|_path| json("{}"))).await;
    let service = WxCpServiceImpl::new_arc(config_with_host(&server.url("")));

    // buildQrConnectUrl：redirect_uri 需 URL 编码，agentid 取配置值
    // （encodeURIComponent 语义，对应 Java URIUtil.encodeURIComponent：
    // '.'/'~' 等保留字符不编码——Wave 3 C3 修复，见 sub_domain_cp_facade.rs
    // 的带点域名断言）
    let url = service.build_qr_connect_url("https://example/oauth?from=qr", "state_1");
    assert!(
        url.starts_with("https://open.work.weixin.qq.com/wwopen/sso/qrConnect"),
        "URL: {url}"
    );
    assert!(url.contains("appid=corpid"), "URL: {url}");
    assert!(url.contains("agentid=101"), "URL: {url}");
    assert!(
        url.contains("redirect_uri=https%3A%2F%2Fexample%2Foauth%3Ffrom%3Dqr"),
        "URL: {url}"
    );
    assert!(url.contains("state=state_1"), "URL: {url}");
    // 空 state：trim 后为空（Java StringUtils.trimToEmpty 语义）；
    // 带点域名 `example.com` 的 '.' 不编码（encodeURIComponent 保留集）
    let url2 = service.build_qr_connect_url("https://example.com", "  ");
    assert!(url2.ends_with("state="), "URL: {url2}");
    assert!(
        url2.contains("redirect_uri=https%3A%2F%2Fexample.com"),
        "点号不应编码为 %2E，URL: {url2}"
    );

    // checkSignature：SHA1.gen(token, timestamp, nonce, data) 排序后无分隔符拼接
    let expected =
        Sha1::digest(&["token123", "1409304348", "1409349580", "hello"]).expect("sha1 计算");
    assert!(service.check_signature(&expected, "1409349580", "1409304348", "hello"));
    assert!(!service.check_signature("deadbeef", "1409349580", "1409304348", "hello"));
}
