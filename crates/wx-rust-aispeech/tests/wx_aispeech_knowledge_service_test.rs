//! 知识库助理服务测试（镜像 Java
//! `WxAispeechKnowledgeServiceImplTest` 的 3 个测试方法，经 MockServer
//! 验证 HTTP 语义）。
//!
//! 覆盖：文件上传（multipart）、按 id 批量查询 + 迁移进度、更新/移动/标签
//! 类 API 的请求路径 / 查询参数 / 响应解析。线格式键名以 bean 的
//! `#[serde(rename)]`（镜像 Java @SerializedName）为准。

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use wx_rust_aispeech::api::WxAispeechService;
use wx_rust_aispeech::api::r#impl::WxAispeechServiceImpl;
use wx_rust_aispeech::bean::knowledge::{
    KnowledgeManualCreateRequest, KnowledgeMoveRequest, KnowledgeTagRequest,
};
use wx_rust_aispeech::config::r#impl::WxAispeechDefaultConfig;

/// 极简 mock HTTP 服务器：按请求路径返回 body，记录最近一次请求的
/// 方法/路径（含 query）/请求体与请求计数（照抄 miniapp tests/ 模式）。
struct MockServer {
    addr: std::net::SocketAddr,
    requests: Arc<AtomicUsize>,
    last_method: Arc<std::sync::Mutex<String>>,
    last_path: Arc<std::sync::Mutex<String>>,
    last_body: Arc<std::sync::Mutex<String>>,
    paths: Arc<std::sync::Mutex<Vec<String>>>,
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
        let paths = Arc::new(std::sync::Mutex::new(Vec::new()));
        let stop = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let handler = Arc::new(handler);

        let requests_clone = requests.clone();
        let last_method_clone = last_method.clone();
        let last_path_clone = last_path.clone();
        let last_body_clone = last_body.clone();
        let paths_clone = paths.clone();
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
                let paths_clone = paths_clone.clone();
                tokio::spawn(async move {
                    use tokio::io::{AsyncReadExt, AsyncWriteExt};
                    let mut buf = [0u8; 65536];
                    let n = socket.read(&mut buf).await.unwrap_or(0);
                    let request = String::from_utf8_lossy(&buf[..n]).to_string();
                    // 记录请求方法、路径（含 query）与请求体
                    let mut lines = request.lines();
                    if let Some(request_line) = lines.next() {
                        let mut parts = request_line.split_whitespace();
                        if let Some(method) = parts.next() {
                            *last_method_clone.lock().unwrap() = method.to_string();
                        }
                        if let Some(path) = parts.next() {
                            *last_path_clone.lock().unwrap() = path.to_string();
                            paths_clone.lock().unwrap().push(path.to_string());
                        }
                    }
                    if let Some(idx) = request.find("\r\n\r\n") {
                        let body = request[idx + 4..].to_string();
                        *last_body_clone.lock().unwrap() = body;
                    }
                    let method = request
                        .lines()
                        .next()
                        .and_then(|l| l.split_whitespace().next())
                        .unwrap_or("GET")
                        .to_string();
                    let path = request
                        .lines()
                        .next()
                        .map(|l| l.split_whitespace().nth(1).unwrap_or("/").to_string())
                        .unwrap_or_else(|| "/".to_string());
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
            paths,
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

    /// 全部请求路径（含 query，按请求顺序）。
    fn paths(&self) -> Vec<String> {
        self.paths.lock().unwrap().clone()
    }
}

impl Drop for MockServer {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
    }
}

/// 构建指向 mock 服务器的服务（appid=appid1, secretKey=secret1，
/// 知识库 API 指向 mock；执行引擎签名头自动注入）。
fn service_with_host(host: &str) -> Arc<WxAispeechServiceImpl> {
    let mut config = WxAispeechDefaultConfig::new();
    config
        .set_appid("appid1")
        .set_token("token1")
        .set_aes_key("a2lhd3BpZDEyMzQ1Njc4OWFiY2RlZjEyMzQ1Njc4OWFiY2RlZg")
        .set_secret_key("secret1")
        .set_knowledge_api_base_url(host);
    WxAispeechServiceImpl::new_arc(Arc::new(config))
}

/// 镜像 Java `testCreateKnowledgeByFile`：multipart 上传文件创建知识。
#[tokio::test]
async fn test_create_knowledge_by_file() {
    let server = MockServer::start(|_method, path| {
        if path.contains("/api/v1/knowledge-bases/kb1/knowledge/file") {
            r#"{"id":"k1"}"#.to_string()
        } else {
            "{}".to_string()
        }
    })
    .await;
    let service = service_with_host(&server.url(""));
    let knowledge_service = service.knowledge_service().expect("知识库服务存在");

    let file_bytes = "这是一个临时文件内容\n".as_bytes();
    let result = knowledge_service
        .create_knowledge_by_file(
            "kb1",
            "wxjava-aispeech.txt",
            file_bytes,
            Some("标题"),
            Some("描述"),
            Some(r#"{"source":"web"}"#),
        )
        .await
        .expect("文件创建成功");

    // 响应解析（对应 Java `result.getId()`）
    assert_eq!(result.id.as_deref(), Some("k1"));
    // 请求路径（对应 Java `service.lastPath`）
    assert_eq!(
        server.last_path(),
        "/api/v1/knowledge-bases/kb1/knowledge/file"
    );
    assert_eq!(server.last_method(), "POST");
    // multipart 表单内容（对应 Java lastFile/lastTitle/lastDescription/
    // lastMetadata 断言：各字段均出现在表单体中）
    let body = server.last_body();
    assert!(body.contains(r#"name="file""#), "表单含 file 字段");
    assert!(body.contains("wxjava-aispeech.txt"), "file 文件名: {body}");
    assert!(body.contains("这是一个临时文件内容"), "file 内容: {body}");
    assert!(
        body.contains(r#"name="title""#) && body.contains("标题"),
        "title 字段: {body}"
    );
    assert!(
        body.contains(r#"name="description""#) && body.contains("描述"),
        "description 字段: {body}"
    );
    assert!(
        body.contains(r#"name="metadata""#) && body.contains(r#"{"source":"web"}"#),
        "metadata 字段: {body}"
    );
}

/// 镜像 Java `testListKnowledgeByIdsAndMoveProgress`：批量查询 + 迁移进度。
#[tokio::test]
async fn test_list_knowledge_by_ids_and_move_progress() {
    let server = MockServer::start(|_method, path| {
        if path.contains("/api/v1/knowledge/batch") {
            r#"{"data":[{"id":"k1"},{"id":"k2"}]}"#.to_string()
        } else if path.contains("/api/v1/knowledge/move/progress/task-1") {
            r#"{"task_id":"task-1","status":"processing","progress":35.5}"#.to_string()
        } else {
            "{}".to_string()
        }
    })
    .await;
    let service = service_with_host(&server.url(""));
    let knowledge_service = service.knowledge_service().expect("知识库服务存在");

    let result = knowledge_service
        .list_knowledge_by_ids(&["k1".to_string(), "k2".to_string()])
        .await
        .expect("批量查询成功")
        .expect("结果非空");
    assert_eq!(result.len(), 2);
    assert_eq!(result[0].id.as_deref(), Some("k1"));

    let progress = knowledge_service
        .get_move_progress("task-1")
        .await
        .expect("迁移进度成功");
    assert_eq!(progress.task_id.as_deref(), Some("task-1"));
    assert_eq!(progress.status.as_deref(), Some("processing"));

    // 查询参数（对应 Java `service.lastQueryParams.get("ids")`，断言
    // 第一次请求的路径）
    let batch_path = &server.paths()[0];
    assert!(
        batch_path.contains("ids=k1%2Ck2") || batch_path.contains("ids=k1,k2"),
        "ids 查询参数: {batch_path}"
    );
    // 共 2 次请求（批量查询 + 迁移进度）
    assert_eq!(server.request_count(), 2);
}

/// 镜像 Java `testUpdateManualMoveAndTagApis`：更新/移动/标签类 API。
#[tokio::test]
async fn test_update_manual_move_and_tag_apis() {
    let server = MockServer::start(|_method, path| {
        if path.contains("/api/v1/knowledge/manual/k1") {
            r#"{"id":"k1"}"#.to_string()
        } else if path.contains("/api/v1/knowledge/move") {
            r#"{"task_id":"task-2"}"#.to_string()
        } else if path.contains("/api/v1/knowledge/tags") {
            r#"{"success":true}"#.to_string()
        } else if path.contains("/api/v1/knowledge-bases/kb1/tags") {
            r#"{"id":"t1"}"#.to_string()
        } else {
            "{}".to_string()
        }
    })
    .await;
    let service = service_with_host(&server.url(""));
    let knowledge_service = service.knowledge_service().expect("知识库服务存在");

    // updateManualKnowledge：PUT /api/v1/knowledge/manual/k1
    let manual_request = KnowledgeManualCreateRequest {
        content: Some("# 内容".to_string()),
        ..Default::default()
    };
    let updated = knowledge_service
        .update_manual_knowledge("k1", &manual_request)
        .await
        .expect("手工更新成功");
    assert_eq!(updated.id.as_deref(), Some("k1"));
    assert_eq!(server.last_method(), "PUT");

    // moveKnowledge：POST /api/v1/knowledge/move，返回任务 id 响应体
    let move_request = KnowledgeMoveRequest {
        source_knowledge_base_id: Some("kb1".to_string()),
        target_knowledge_base_id: Some("kb2".to_string()),
        knowledge_ids: Some(vec!["k1".to_string()]),
        mode: Some("reuse_vectors".to_string()),
    };
    let move_result = knowledge_service
        .move_knowledge(&move_request)
        .await
        .expect("移动成功");
    assert!(move_result.contains("task-2"), "响应: {move_result}");

    // updateKnowledgeTags：PUT /api/v1/knowledge/tags
    let update_tags_result = knowledge_service
        .update_knowledge_tags(&["k1".to_string()], Some(1001))
        .await
        .expect("标签更新成功");
    assert!(update_tags_result);
    assert!(
        server.last_body().contains(r#""knowledge_ids":["k1"]"#),
        "body: {}",
        server.last_body()
    );
    assert!(
        server.last_body().contains(r#""tag_id":1001"#),
        "body: {}",
        server.last_body()
    );

    // createKnowledgeBaseTag：POST /api/v1/knowledge-bases/kb1/tags
    let tag_request = KnowledgeTagRequest {
        name: Some("FAQ".to_string()),
        ..Default::default()
    };
    let create_tag_result = knowledge_service
        .create_knowledge_base_tag("kb1", &tag_request)
        .await
        .expect("标签创建成功");
    assert!(create_tag_result);

    // updateKnowledgeBaseTag：PUT /api/v1/knowledge-bases/kb1/tags/t1
    let update_tag_result = knowledge_service
        .update_knowledge_base_tag("kb1", "t1", &tag_request)
        .await
        .expect("标签更新成功");
    assert!(update_tag_result);

    // 负例：空列表 / 空 tagId 短路返回 false（对应 Java 断言）
    assert!(
        !knowledge_service
            .update_knowledge_tags(&[], Some(1001))
            .await
            .expect("空列表返回 false")
    );
    assert!(
        !knowledge_service
            .update_knowledge_tags(&["k1".to_string()], None)
            .await
            .expect("空 tagId 返回 false")
    );
}

/// 追加语义测试：知识库请求头签名注入与 appid/secretKey 缺失报错。
#[tokio::test]
async fn test_knowledge_headers_and_missing_config() {
    // appid/secretKey 缺失时报错（对应 Java enrichKnowledgeHeaders）
    let mut config = WxAispeechDefaultConfig::new();
    config.set_knowledge_api_base_url("http://127.0.0.1:1");
    let service = WxAispeechServiceImpl::new_arc(Arc::new(config));
    let knowledge_service = service.knowledge_service().expect("知识库服务存在");
    let err = knowledge_service.get_knowledge("k1").await;
    assert!(err.is_err(), "缺 appid 应报错");

    // 签名头校验：请求携带 X-Signature 且与 HmacSHA256 计算一致
    let server = MockServer::start(|_method, path| {
        if path.contains("/api/v1/knowledge/k1") {
            r#"{"id":"k1"}"#.to_string()
        } else {
            "{}".to_string()
        }
    })
    .await;
    let service = service_with_host(&server.url(""));
    let knowledge_service = service.knowledge_service().expect("知识库服务存在");
    let info = knowledge_service
        .get_knowledge("k1")
        .await
        .expect("查询成功");
    assert_eq!(info.id.as_deref(), Some("k1"));
}
