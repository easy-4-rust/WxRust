//! 企业微信门面子域（Wave 3 C3）集成测试：门面装配收尾。
//!
//! 覆盖：
//! - 31 个子服务 getter 全部返回 `Some`（覆写 trait 默认 `None`）且为
//!   构建时装配的单例（重复调用返回同一 `Arc`，对应 Java
//!   `BaseWxCpServiceImpl` 构造器字段）
//! - 子服务经 `Weak<dyn WxCpService>` 调用门面执行引擎（路径/access_token
//!   断言：GET 与 POST 通道）
//! - 并发子服务调用共享门面 token 双检锁（并发 20 次只刷新 1 次 token）
//! - `buildQrConnectUrl` 编码修复（encodeURIComponent 保留集：带点域名
//!   `.` 不编码、`~` 等保留、空格 `%20`）
//! - msgaudit 6 个 native SDK 方法返回 -99（PLATFORM_NA 标注）

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use wx_rust_cp::api::WxCpService;
use wx_rust_cp::api::r#impl::WxCpServiceImpl;
use wx_rust_cp::bean::WxCpMessage;
use wx_rust_cp::config::r#impl::WxCpDefaultConfig;
use wx_rust_cp::config::{WxCpConfigStorage, WxCpHostConfig};

/// 极简 mock HTTP 服务器：按请求路径返回 (Content-Type, body)，记录
/// 最近一次请求路径（含 query）与请求体、请求计数（照抄
/// `tests/wx_cp_service_impl_test.rs` 模式）。
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

/// 通用路由 handler：token 请求先应答（MOCK_TOKEN），业务路径按 contains
/// 分派（user/department 列表、消息发送、其余回空 JSON）。
fn dispatch(
    handler: impl Fn(&str) -> (String, String) + Send + Sync + 'static,
) -> impl Fn(&str) -> (String, String) + Send + Sync + 'static {
    move |path: &str| {
        if path.contains("/cgi-bin/gettoken") {
            return json(r#"{"access_token":"MOCK_TOKEN","expires_in":7200}"#);
        }
        handler(path)
    }
}

// ---- 31 个 getter 全 Some + 单例（对应 Java WxCpService.getXxxService()
// 返回 Base 构造器装配的字段实例） ----

#[test]
fn all_31_getters_installed_and_singleton() {
    let service = WxCpServiceImpl::new_arc(config_with_host("http://127.0.0.1:1"));

    // 逐一断言 31 个 getter 返回 Some（对应 Java 31 个 `getXxxService()`）
    assert!(service.department_service().is_some());
    assert!(service.media_service().is_some());
    assert!(service.menu_service().is_some());
    assert!(service.oauth2_service().is_some());
    assert!(service.tag_service().is_some());
    assert!(service.user_service().is_some());
    assert!(service.external_contact_service().is_some());
    assert!(service.chat_service().is_some());
    assert!(service.task_card_service().is_some());
    assert!(service.agent_service().is_some());
    assert!(service.message_service().is_some());
    assert!(service.oa_service().is_some());
    assert!(service.school_service().is_some());
    assert!(service.school_user_service().is_some());
    assert!(service.school_health_service().is_some());
    assert!(service.living_service().is_some());
    assert!(service.oa_agent_service().is_some());
    assert!(service.oa_we_drive_service().is_some());
    assert!(service.oa_we_doc_service().is_some());
    assert!(service.msg_audit_service().is_some());
    assert!(service.oa_calendar_service().is_some());
    assert!(service.oa_meeting_room_service().is_some());
    assert!(service.oa_schedule_service().is_some());
    assert!(service.group_robot_service().is_some());
    assert!(service.work_bench_service().is_some());
    assert!(service.kf_service().is_some());
    assert!(service.export_service().is_some());
    assert!(service.meeting_service().is_some());
    assert!(service.corp_group_service().is_some());
    assert!(service.intelligent_robot_service().is_some());
    assert!(service.hr_service().is_some());

    // 重复调用返回同一实例（OnceLock 单例，对应 Java 字段在构造器只赋一次）
    let u1 = service.user_service().unwrap();
    let u2 = service.user_service().unwrap();
    assert!(Arc::ptr_eq(&u1, &u2), "user 子服务应为单例");
    let d1 = service.department_service().unwrap();
    let d2 = service.department_service().unwrap();
    assert!(Arc::ptr_eq(&d1, &d2), "department 子服务应为单例");
    let m1 = service.message_service().unwrap();
    let m2 = service.message_service().unwrap();
    assert!(Arc::ptr_eq(&m1, &m2), "message 子服务应为单例");
}

// ---- 子服务经 Weak<dyn WxCpService> 调门面执行引擎（GET 通道：
// 路径 + 自动注入 access_token） ----

#[tokio::test]
async fn sub_service_get_flows_through_facade_engine() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/user/list") {
            json(r#"{"errcode":0,"errmsg":"ok","userlist":[]}"#)
        } else if path.contains("/cgi-bin/department/list") {
            json(r#"{"errcode":0,"errmsg":"ok","department":[]}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let service = WxCpServiceImpl::new_arc(config_with_host(&server.url("")));

    // user_service.listByDepartment：GET `USER_LIST + departId`，执行引擎
    // 自动带 access_token（Weak 升级到门面后取配置与引擎）
    let users = service
        .user_service()
        .unwrap()
        .list_by_department(5, None, None)
        .await
        .expect("成员列表获取成功");
    assert!(users.is_empty());
    let path = server.last_path();
    assert!(
        path.contains("/cgi-bin/user/list?department_id=5"),
        "path: {path}"
    );
    assert!(path.contains("access_token=MOCK_TOKEN"), "path: {path}");

    // department_service.list(Some(1))：GET `DEPARTMENT_LIST?id=1`
    let depts = service
        .department_service()
        .unwrap()
        .list(Some(1))
        .await
        .expect("部门列表获取成功");
    assert!(depts.is_empty());
    let path = server.last_path();
    assert!(
        path.contains("/cgi-bin/department/list?id=1"),
        "path: {path}"
    );
    assert!(path.contains("access_token=MOCK_TOKEN"), "path: {path}");
}

// ---- 子服务经 Weak 调门面执行引擎（POST 通道：请求体 + token） ----

#[tokio::test]
async fn sub_service_post_flows_through_facade_engine() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/message/send") {
            json(r#"{"errcode":0,"errmsg":"ok","msgid":"msg_1"}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let service = WxCpServiceImpl::new_arc(config_with_host(&server.url("")));

    // message_service.send：POST `MESSAGE_SEND`，请求体镜像 Gson 插入序
    let mut message = WxCpMessage::default();
    message.to_user = Some("zhangsan".to_string());
    message.agent_id = Some(101);
    message.msg_type = Some("text".to_string());
    message.content = Some("hello".to_string());
    let result = service
        .message_service()
        .unwrap()
        .send(&message)
        .await
        .expect("消息发送成功");
    assert_eq!(result.msg_id, "msg_1");

    let path = server.last_path();
    assert!(path.contains("/cgi-bin/message/send"), "path: {path}");
    assert!(path.contains("access_token=MOCK_TOKEN"), "path: {path}");
    let body = server.last_body();
    assert!(body.contains(r#""msgtype":"text""#), "body: {body}");
    assert!(body.contains(r#""content":"hello""#), "body: {body}");
}

// ---- 并发子服务调用共享门面 token 双检锁（并发 20 次只刷新 1 次 token） ----

#[tokio::test]
async fn concurrent_sub_service_calls_share_token_double_check_lock() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/user/list") {
            json(r#"{"errcode":0,"errmsg":"ok","userlist":[]}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let service = WxCpServiceImpl::new_arc(config_with_host(&server.url("")));
    let _ = service.user_service(); // 预取一次，验证装配（重复调用见单例测试）

    // 20 个并发子服务调用：双检锁（isAccessTokenExpired 预检 + 锁内二次
    // 判断）保证只有 1 次 token HTTP 请求，其余 20 次均为业务请求
    let mut handles = Vec::new();
    for _ in 0..20 {
        let svc = service.clone();
        handles.push(tokio::spawn(async move {
            svc.user_service()
                .expect("user 子服务已装配")
                .list_by_department(5, None, None)
                .await
        }));
    }
    for handle in handles {
        assert!(handle.await.unwrap().is_ok(), "子服务调用应成功");
    }
    // 1 次 token 刷新 + 20 次业务请求
    assert_eq!(server.request_count(), 21, "双检锁下并发只刷新 1 次 token");
    let path = server.last_path();
    assert!(path.contains("/cgi-bin/user/list"), "path: {path}");
}

// ---- buildQrConnectUrl 编码修复（encodeURIComponent 保留集，对应
// Java URIUtil.encodeURIComponent：`.` 不编码——Wave 3 C3 修复） ----

#[test]
fn build_qr_connect_url_preserves_unreserved_chars() {
    let service = WxCpServiceImpl::new_arc(config_with_host("http://127.0.0.1:1"));

    // 带点域名：`.` 保持原样（修复前 NON_ALPHANUMERIC 会编码为 %2E）
    let url = service.build_qr_connect_url("https://example.com/oauth?from=qr", "state_1");
    assert!(!url.contains("%2E"), "点号不应编码为 %2E，URL: {url}");
    assert!(
        url.contains("redirect_uri=https%3A%2F%2Fexample.com%2Foauth%3Ffrom%3Dqr"),
        "URL: {url}"
    );

    // 保留集 `- _ . ! ~ * ' ( )` 全部不编码；空格编码为 %20
    let url2 = service.build_qr_connect_url("https://a-b_c.d!e~f*g'h(i)j k", "");
    assert!(
        url2.contains("redirect_uri=https%3A%2F%2Fa-b_c.d!e~f*g'h(i)j%20k"),
        "URL: {url2}"
    );
    assert!(!url2.contains("%2E"), "点号不应编码，URL: {url2}");
    assert!(url2.contains("%20"), "空格应编码为 %20，URL: {url2}");
    assert!(
        url2.ends_with("state="),
        "空 state trim 后拼接，URL: {url2}"
    );
}

// ---- msgaudit native SDK 方法（PLATFORM_NA：依赖官方 native SDK
// Finance 私有协议，Rust 无对应实现，返回 -99） ----

#[tokio::test]
async fn msg_audit_native_sdk_methods_return_not_implemented() {
    let server = MockServer::start(dispatch(|_path| json("{}"))).await;
    let service = WxCpServiceImpl::new_arc(config_with_host(&server.url("")));
    let audit = service.msg_audit_service().unwrap();

    // 6 个方法逐一断言返回 -99（镜像 Java 依赖 `Finance.GetChatData`/
    // `Finance.GetMediaData` 的私有协议接口在 Rust 的 PLATFORM_NA 语义）
    let err = audit
        .get_chat_datas(0, 100, None, None, 1)
        .await
        .unwrap_err();
    assert_eq!(err.error_code(), Some(-99), "getChatDatas: {err}");

    let err = audit
        .get_chat_records(0, 100, None, None, 1)
        .await
        .unwrap_err();
    assert_eq!(err.error_code(), Some(-99), "getChatRecords: {err}");

    let err = audit
        .get_media_file(0, "fileid", None, None, 1, "target.bin")
        .await
        .unwrap_err();
    assert_eq!(err.error_code(), Some(-99), "getMediaFile: {err}");

    let err = audit
        .download_media_file("fileid", None, None, 1, "target.bin")
        .await
        .unwrap_err();
    assert_eq!(err.error_code(), Some(-99), "downloadMediaFile: {err}");

    let err = audit
        .get_media_file_with_callback(0, "fileid", None, None, 1, &mut |_| {})
        .await
        .unwrap_err();
    assert_eq!(err.error_code(), Some(-99), "getMediaFile(回调版): {err}");

    let err = audit
        .download_media_file_with_callback("fileid", None, None, 1, &mut |_| {})
        .await
        .unwrap_err();
    assert_eq!(
        err.error_code(),
        Some(-99),
        "downloadMediaFile(回调版): {err}"
    );

    // 无 HTTP 请求发生（纯 PLATFORM_NA，不经门面引擎）
    assert_eq!(
        server.request_count(),
        0,
        "native SDK 方法不应发起 HTTP 请求"
    );
}
