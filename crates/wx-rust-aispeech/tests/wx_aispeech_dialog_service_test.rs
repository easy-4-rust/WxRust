//! 对话机器人服务测试（补齐接线复核缺口：Java 侧无
//! `WxAispeechDialogServiceImplTest`，Rust 侧此前同样零覆盖，本文件以
//! MockServer 验证全部 6 个 trait 方法的 HTTP 语义与签名头注入）。
//!
//! 覆盖：`getAccessToken`（含 `setOpenAiToken` 副作用）、`importBotJson`、
//! `publishBot`、`getPublishProgress`、`queryAsyncTask`、`query`
//! （请求 AES 加密 + 密文/明文两种响应分支 + `rawAnswer` 解析），以及
//! 执行引擎 `executeDialogPost` 的 `X-OPENAI-TOKEN`/`X-APPID` 缺失报错
//! 与 MD5 链式签名头校验。

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use wx_rust_aispeech::api::WxAispeechService;
use wx_rust_aispeech::api::r#impl::WxAispeechServiceImpl;
use wx_rust_aispeech::bean::dialog::{BotIntent, DialogQueryRequest};
use wx_rust_aispeech::config::WxAispeechConfigStorage;
use wx_rust_aispeech::config::r#impl::WxAispeechDefaultConfig;
use wx_rust_aispeech::util::WxAispeechSignUtil;

/// 极简 mock HTTP 服务器：按请求路径返回 body，记录最近一次请求的方法/
/// 路径/请求体与请求头（照抄 knowledge tests/ 的 MockServer 模式，
/// 追加请求头记录以校验对话 API 的签名头注入）。
struct MockServer {
    addr: std::net::SocketAddr,
    requests: Arc<AtomicUsize>,
    last_method: Arc<std::sync::Mutex<String>>,
    last_path: Arc<std::sync::Mutex<String>>,
    last_body: Arc<std::sync::Mutex<String>>,
    last_headers: Arc<std::sync::Mutex<HashMap<String, String>>>,
    stop: Arc<std::sync::atomic::AtomicBool>,
}

impl MockServer {
    /// 启动服务器（`handler(method, path) -> body`）。
    async fn start<F>(handler: F) -> Self
    where
        F: Fn(&str, &str) -> String + Send + Sync + 'static,
    {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("绑定端口");
        let addr = listener.local_addr().expect("获取地址");
        let requests = Arc::new(AtomicUsize::new(0));
        let last_method = Arc::new(std::sync::Mutex::new(String::new()));
        let last_path = Arc::new(std::sync::Mutex::new(String::new()));
        let last_body = Arc::new(std::sync::Mutex::new(String::new()));
        let last_headers = Arc::new(std::sync::Mutex::new(HashMap::new()));
        let stop = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let handler = Arc::new(handler);

        let requests_clone = requests.clone();
        let last_method_clone = last_method.clone();
        let last_path_clone = last_path.clone();
        let last_body_clone = last_body.clone();
        let last_headers_clone = last_headers.clone();
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
                let last_method_clone = last_method_clone.clone();
                let last_path_clone = last_path_clone.clone();
                let last_body_clone = last_body_clone.clone();
                let last_headers_clone = last_headers_clone.clone();
                tokio::spawn(async move {
                    use tokio::io::{AsyncReadExt, AsyncWriteExt};
                    let mut buf = [0u8; 65536];
                    let n = socket.read(&mut buf).await.unwrap_or(0);
                    let request = String::from_utf8_lossy(&buf[..n]).to_string();
                    // 记录请求方法、路径（含 query）、请求体与请求头（键小写）
                    let mut lines = request.lines();
                    if let Some(request_line) = lines.next() {
                        let mut parts = request_line.split_whitespace();
                        if let Some(method) = parts.next() {
                            *last_method_clone.lock().unwrap() = method.to_string();
                        }
                        if let Some(path) = parts.next() {
                            *last_path_clone.lock().unwrap() = path.to_string();
                        }
                    }
                    let (head, body) = match request.find("\r\n\r\n") {
                        Some(idx) => (&request[..idx], request[idx + 4..].to_string()),
                        None => (request.as_str(), String::new()),
                    };
                    let mut headers = HashMap::new();
                    for line in head.lines().skip(1) {
                        if let Some((name, value)) = line.split_once(':') {
                            headers
                                .insert(name.trim().to_ascii_lowercase(), value.trim().to_string());
                        }
                    }
                    *last_headers_clone.lock().unwrap() = headers;
                    *last_body_clone.lock().unwrap() = body;
                    let method = last_method_clone.lock().unwrap().clone();
                    let path = last_path_clone.lock().unwrap().clone();
                    let body = handler(&method, &path);
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
            last_method,
            last_path,
            last_body,
            last_headers,
            stop,
        }
    }

    fn url(&self, path: &str) -> String {
        format!("http://{}{}", self.addr, path)
    }

    fn request_count(&self) -> usize {
        self.requests.load(Ordering::SeqCst)
    }

    fn last_method(&self) -> String {
        self.last_method.lock().unwrap().clone()
    }

    fn last_path(&self) -> String {
        self.last_path.lock().unwrap().clone()
    }

    fn last_body(&self) -> String {
        self.last_body.lock().unwrap().clone()
    }

    /// 最近一次请求的请求头（键小写）。
    fn header(&self, name: &str) -> Option<String> {
        self.last_headers
            .lock()
            .unwrap()
            .get(&name.to_ascii_lowercase())
            .cloned()
    }
}

impl Drop for MockServer {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
    }
}

/// 测试用 AES 密钥（43 位 base64，解码 32 字节 → AES-256；与
/// `wx_aispeech_sign_util_test.rs` 的 Java 测试密钥同源）。
const AES_KEY: &str = "q1Os1ZMe0nG28KUEx9lg3HjK7V5QyXvi212fzsgDqgz";

/// 构建指向 mock 服务器的服务（appid=appid1, token=token1，对话 API
/// 指向 mock；`with_token` 控制是否预置 openAiToken）。
fn service_with_host(host: &str, with_token: bool) -> Arc<WxAispeechServiceImpl> {
    let mut config = WxAispeechDefaultConfig::new();
    config
        .set_appid("appid1")
        .set_token("token1")
        .set_aes_key(AES_KEY)
        .set_dialog_api_base_url(host);
    if with_token {
        config.set_open_ai_token("tok-1");
    }
    WxAispeechServiceImpl::new_arc(Arc::new(config))
}

/// 对应 Java `getAccessToken`：token 获取 + `setOpenAiToken` 副作用 +
/// `X-APPID` 头与 MD5 链式签名头校验。
#[tokio::test]
async fn test_get_access_token_writes_open_ai_token() {
    let server = MockServer::start(|_method, path| {
        if path.contains("/v2/token") {
            r#"{"code":0,"request_id":"r0","data":{"access_token":"tok-123"}}"#.to_string()
        } else {
            "{}".to_string()
        }
    })
    .await;
    let service = service_with_host(&server.url(""), false);
    let dialog_service = service.dialog_service().expect("对话服务存在");

    let token = dialog_service
        .get_access_token(Some(""), Some("acc-1"))
        .await
        .expect("获取 token 成功");
    assert_eq!(token, "tok-123");

    // 对应 Java `service.getConfigStorage().setOpenAiToken(token)` 副作用
    assert_eq!(
        service.config_storage().open_ai_token(),
        Some("tok-123".to_string())
    );

    // 请求路径 / 方法 / 请求体（account 非空才放入请求体）
    assert_eq!(server.last_path(), "/v2/token");
    assert_eq!(server.last_method(), "POST");
    assert_eq!(server.last_body(), r#"{"account":"acc-1"}"#);

    // 请求头：X-APPID（appid 为空回落配置 appid）；签名头按
    // md5(token + timestamp + nonce + md5(body)) 复算一致
    assert_eq!(server.header("x-appid").as_deref(), Some("appid1"));
    let timestamp: i64 = server
        .header("timestamp")
        .expect("timestamp 头存在")
        .parse()
        .expect("timestamp 为数字");
    let nonce = server.header("nonce").expect("nonce 头存在");
    let expected_sign = WxAispeechSignUtil::calc_dialog_sign(
        Some("token1"),
        timestamp,
        &nonce,
        &server.last_body(),
    );
    assert_eq!(
        server.header("sign").as_deref(),
        Some(expected_sign.as_str())
    );
    assert_eq!(server.request_count(), 1);
}

/// 对应 Java `importBotJson`/`publishBot`/`queryAsyncTask`：任务导入、
/// 发布与异步任务查询（`X-OPENAI-TOKEN` 头注入）。
#[tokio::test]
async fn test_import_publish_and_query_async_task() {
    let server = MockServer::start(|_method, path| {
        if path.contains("/v2/bot/import/json") {
            r#"{"code":0,"request_id":"r1","data":{"task_id":"task-9"}}"#.to_string()
        } else if path.contains("/v2/bot/publish") {
            r#"{"code":0,"request_id":"req-publish"}"#.to_string()
        } else if path.contains("/v2/async/fetch") {
            r#"{"code":0,"data":{"state":2,"msg":"done","progress":100,"total_count":3,"success_count":3,"fail_count":0}}"#.to_string()
        } else {
            "{}".to_string()
        }
    })
    .await;
    let service = service_with_host(&server.url(""), true);
    let dialog_service = service.dialog_service().expect("对话服务存在");

    // importBotJson：POST /v2/bot/import/json，返回 data.task_id
    let intents = vec![BotIntent {
        skill: Some("技能A".to_string()),
        intent: Some("意图1".to_string()),
        questions: Some(vec!["问题1".to_string()]),
        answers: Some(vec!["答案1".to_string()]),
        ..Default::default()
    }];
    let task_id = dialog_service
        .import_bot_json(1, &intents)
        .await
        .expect("导入成功");
    assert_eq!(task_id, "task-9");
    assert_eq!(server.last_path(), "/v2/bot/import/json");
    // 请求体含 mode 与 data 内容（对应 Java {"mode":1,"data":[...]）
    assert!(
        server.last_body().contains(r#""mode":1"#),
        "body: {}",
        server.last_body()
    );
    assert!(
        server.last_body().contains(r#""skill":"技能A""#),
        "body: {}",
        server.last_body()
    );
    // X-OPENAI-TOKEN 注入
    assert_eq!(
        server.header("x-openai-token").as_deref(),
        Some("tok-1"),
        "token 头: {:?}",
        server.header("x-openai-token")
    );

    // publishBot：POST /v2/bot/publish，返回 request_id
    let request_id = dialog_service.publish_bot().await.expect("发布成功");
    assert_eq!(request_id, "req-publish");
    assert_eq!(server.last_path(), "/v2/bot/publish");
    assert_eq!(server.last_body(), "{}");

    // queryAsyncTask：POST /v2/async/fetch，返回 data
    let task = dialog_service
        .query_async_task("task-9")
        .await
        .expect("查询任务成功");
    assert_eq!(task.state, Some(2));
    assert_eq!(task.progress, Some(100));
    assert_eq!(task.total_count, Some(3));
    assert_eq!(task.success_count, Some(3));
    assert_eq!(task.fail_count, Some(0));
    assert_eq!(server.last_path(), "/v2/async/fetch");
    assert!(server.last_body().contains(r#""task_id":"task-9""#));
    assert_eq!(server.request_count(), 3);
}

/// 对应 Java `getPublishProgress`：POST /v2/bot/effective_progress。
#[tokio::test]
async fn test_get_publish_progress() {
    let server = MockServer::start(|_method, path| {
        if path.contains("/v2/bot/effective_progress") {
            r#"{"code":0,"data":{"progress":60,"status":1,"end_time":"2026-08-25"}}"#.to_string()
        } else {
            "{}".to_string()
        }
    })
    .await;
    let service = service_with_host(&server.url(""), true);
    let dialog_service = service.dialog_service().expect("对话服务存在");

    let progress = dialog_service
        .get_publish_progress("online")
        .await
        .expect("查询发布进度成功");
    assert_eq!(progress.progress, Some(60));
    assert_eq!(progress.status, Some(1));
    assert_eq!(progress.end_time.as_deref(), Some("2026-08-25"));
    assert_eq!(server.last_path(), "/v2/bot/effective_progress");
    assert!(server.last_body().contains(r#""env":"online""#));
}

/// 对应 Java `query`（密文响应分支）：请求体 AES-CBC 加密、响应为非 JSON
/// 密文时解密后解析。
#[tokio::test]
async fn test_query_encrypted_roundtrip() {
    // 预计算密文响应（对应 Java 服务端返回加密报文）
    let plain_response = r#"{"code":0,"data":{"answer":"{\"k\":1}","skill_name":"技能A"}}"#;
    let encrypted_response =
        WxAispeechSignUtil::encrypt_aes_cbc_to_base64(plain_response, AES_KEY).expect("预加密响应");
    let server = MockServer::start(move |_method, path| {
        if path.contains("/v2/bot/query") {
            encrypted_response.clone()
        } else {
            "{}".to_string()
        }
    })
    .await;
    let service = service_with_host(&server.url(""), true);
    let dialog_service = service.dialog_service().expect("对话服务存在");

    let result = dialog_service
        .query(&DialogQueryRequest {
            query: Some("你好".to_string()),
            env: Some("online".to_string()),
            ..Default::default()
        })
        .await
        .expect("对话查询成功");

    // 响应解析：answer 保留原文，rawAnswer 解析为 JSON（对应 Java setRawAnswer）
    assert_eq!(result.answer.as_deref(), Some(r#"{"k":1}"#));
    assert_eq!(result.skill_name.as_deref(), Some("技能A"));
    assert_eq!(
        result.raw_answer.as_ref().and_then(|v| v.get("k")),
        Some(&serde_json::json!(1))
    );

    // 请求体为 AES 密文（非 JSON）：解密后还原 DialogQueryRequest
    let sent_body = server.last_body();
    assert!(!sent_body.starts_with('{'), "请求体应为密文: {sent_body}");
    let decrypted =
        WxAispeechSignUtil::decrypt_aes_cbc_from_base64(&sent_body, AES_KEY).expect("解密请求体");
    let request: DialogQueryRequest = serde_json::from_str(&decrypted).expect("还原请求结构");
    assert_eq!(request.query.as_deref(), Some("你好"));
    assert_eq!(request.env.as_deref(), Some("online"));
    assert_eq!(server.last_path(), "/v2/bot/query");
    assert_eq!(server.request_count(), 1);
}

/// 对应 Java `query`（明文响应分支）：响应本身为 JSON 时不解密直接解析。
#[tokio::test]
async fn test_query_plain_json_response() {
    let server = MockServer::start(|_method, path| {
        if path.contains("/v2/bot/query") {
            r#"{"code":0,"data":{"answer":"纯文本回答","msg_id":"m-1"}}"#.to_string()
        } else {
            "{}".to_string()
        }
    })
    .await;
    let service = service_with_host(&server.url(""), true);
    let dialog_service = service.dialog_service().expect("对话服务存在");

    let result = dialog_service
        .query(&DialogQueryRequest {
            query: Some("问题".to_string()),
            ..Default::default()
        })
        .await
        .expect("对话查询成功");
    // answer 非 JSON 时 rawAnswer 保持 None
    assert_eq!(result.answer.as_deref(), Some("纯文本回答"));
    assert_eq!(result.msg_id.as_deref(), Some("m-1"));
    assert!(result.raw_answer.is_none());
}

/// 对应 Java `executeDialogPost` 报错分支：`X-OPENAI-TOKEN` 与 `X-APPID`
/// 缺失时报错（不发起网络请求）。
#[tokio::test]
async fn test_dialog_error_branches() {
    // 未配置 openAiToken 时（with_open_token=true）报错
    let server = MockServer::start(|_method, _path| "{}".to_string()).await;
    let service = service_with_host(&server.url(""), false);
    let dialog_service = service.dialog_service().expect("对话服务存在");
    let err = dialog_service.publish_bot().await;
    assert!(err.is_err(), "缺 X-OPENAI-TOKEN 应报错");
    assert_eq!(server.request_count(), 0, "报错前不应发起请求");

    // 未配置 appid 且入参 appid 为空时（with_open_token=false）报错
    let mut config = WxAispeechDefaultConfig::new();
    config.set_dialog_api_base_url(server.url(""));
    let service = WxAispeechServiceImpl::new_arc(Arc::new(config));
    let dialog_service = service.dialog_service().expect("对话服务存在");
    let err = dialog_service.get_access_token(None, None).await;
    assert!(err.is_err(), "缺 X-APPID 应报错");
    assert_eq!(server.request_count(), 0, "报错前不应发起请求");
}
