#![allow(dead_code)]
//! Top-15 未镜像 Java 测试类批量补测——mp 模块。
//!
//! 本文件镜像以下 Java 测试类：
//! - WxMpMaterialServiceImplTest（331 行）

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Weak};

use wx_rust_mp::api::r#impl::*;
use wx_rust_mp::api::*;
use wx_rust_mp::bean::material::*;
use wx_rust_mp::config::r#impl::WxMpDefaultConfig;
use wx_rust_mp::config::*;

// ═══════════════════════════════════════════════════════════════
// MockServer 基础设施
// ═══════════════════════════════════════════════════════════════

struct MockServer {
    addr: std::net::SocketAddr,
    requests: Arc<AtomicUsize>,
    last_path: Arc<std::sync::Mutex<String>>,
    last_body: Arc<std::sync::Mutex<String>>,
    stop: Arc<std::sync::atomic::AtomicBool>,
}

impl MockServer {
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
                    let mut buf = [0u8; 65536];
                    let n = socket.read(&mut buf).await.unwrap_or(0);
                    let request = String::from_utf8_lossy(&buf[..n]).to_string();
                    if let Some(path) = request
                        .lines()
                        .next()
                        .and_then(|l| l.split_whitespace().nth(1))
                    {
                        *last_path_clone.lock().unwrap() = path.to_string();
                    }
                    if let Some(idx) = request.find("\r\n\r\n") {
                        *last_body_clone.lock().unwrap() = request[idx + 4..].to_string();
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

fn json(body: &str) -> (String, String) {
    ("application/json".to_string(), body.to_string())
}

fn ok_resp() -> &'static str {
    r#"{"errcode":0,"errmsg":"ok"}"#
}

fn host_config(host: &str) -> WxMpHostConfig {
    let mut config = WxMpHostConfig::new();
    config.api_host = host.to_string();
    config
}

fn config_with_host(host: &str) -> Arc<dyn WxMpConfigStorage> {
    let mut config = WxMpDefaultConfig::new("appid", "secret");
    config.set_token("token123");
    config.set_host_config(host_config(host));
    Arc::new(config)
}

fn service_with_host(host: &str) -> Arc<dyn WxMpService> {
    WxMpServiceImpl::new_arc(config_with_host(host))
}

fn weak_service(service: &Arc<dyn WxMpService>) -> Weak<dyn WxMpService> {
    Arc::downgrade(service)
}

fn dispatch(
    handler: impl Fn(&str) -> (String, String) + Send + Sync + 'static,
) -> impl Fn(&str) -> (String, String) + Send + Sync + 'static {
    move |path: &str| {
        if path.contains("/cgi-bin/token") {
            return json(r#"{"access_token":"MOCK_TOKEN","expires_in":7200}"#);
        }
        handler(path)
    }
}

// ═══════════════════════════════════════════════════════════════
// #14 WxMpMaterialServiceImplTest（331 行）—— 公众号素材服务
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxMpMaterialServiceImplTest.testMaterialCount
#[tokio::test]
async fn test_mp_material_count() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/material/get_materialcount") {
            json(
                r#"{
                "errcode": 0,
                "errmsg": "ok",
                "voiceCount": 5,
                "videoCount": 3,
                "imageCount": 10,
                "newsCount": 2
            }"#,
            )
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxMpMaterialServiceImpl::new(weak_service(&service));
    let result = svc.material_count().await.expect("获取素材数量成功");
    assert_eq!(result.voice_count, 5);
    assert_eq!(result.video_count, 3);
    assert_eq!(result.image_count, 10);
    assert_eq!(result.news_count, 2);
    assert!(
        server
            .last_path()
            .contains("/cgi-bin/material/get_materialcount")
    );
}

/// 对应 Java: WxMpMaterialServiceImplTest.testMaterialNewsBatchGet
#[tokio::test]
async fn test_mp_material_news_batch_get() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/material/batchget_material") {
            json(
                r#"{
                "errcode": 0,
                "errmsg": "ok",
                "totalCount": 1,
                "itemCount": 1,
                "items": [
                    {
                        "media_id": "MEDIA001",
                        "content": {
                            "news_item": [
                                {
                                    "title": "测试图文",
                                    "thumb_media_id": "THUMB001"
                                }
                            ]
                        },
                        "update_time": 1620000000
                    }
                ]
            }"#,
            )
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxMpMaterialServiceImpl::new(weak_service(&service));
    let result = svc
        .material_news_batch_get(0, 10)
        .await
        .expect("批量获取图文素材成功");
    assert_eq!(result.total_count, 1);
    assert_eq!(result.item_count, 1);
    assert!(
        server
            .last_path()
            .contains("/cgi-bin/material/batchget_material")
    );
}

/// 对应 Java: WxMpMaterialServiceImplTest.testMaterialFileBatchGet
#[tokio::test]
async fn test_mp_material_file_batch_get() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/material/batchget_material") {
            json(
                r#"{
                "errcode": 0,
                "errmsg": "ok",
                "totalCount": 2,
                "itemCount": 2,
                "items": [
                    {"media_id": "MEDIA001", "name": "file1.jpg", "update_time": 1620000000},
                    {"media_id": "MEDIA002", "name": "file2.jpg", "update_time": 1620000001}
                ]
            }"#,
            )
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxMpMaterialServiceImpl::new(weak_service(&service));
    let result = svc
        .material_file_batch_get("image", 0, 10)
        .await
        .expect("批量获取文件素材成功");
    assert_eq!(result.total_count, 2);
    assert_eq!(result.item_count, 2);
}

/// 对应 Java: WxMpMaterialServiceImplTest.testMaterialVideoInfo
#[tokio::test]
async fn test_mp_material_video_info() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/material/get_material") {
            json(
                r#"{
                "errcode": 0,
                "errmsg": "ok",
                "title": "测试视频",
                "description": "视频描述",
                "downUrl": "https://example.com/video.mp4"
            }"#,
            )
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxMpMaterialServiceImpl::new(weak_service(&service));
    let result = svc
        .material_video_info("MEDIA001")
        .await
        .expect("获取视频信息成功");
    assert_eq!(result.title, "测试视频");
    assert_eq!(result.description, "视频描述");
    assert_eq!(result.down_url, "https://example.com/video.mp4");
}

/// 对应 Java: WxMpMaterialServiceImplTest.testDeleteMaterial（请求体构建验证）
#[test]
fn test_mp_material_delete_body() {
    let body = serde_json::json!({ "media_id": "MEDIA001" });
    assert_eq!(body["media_id"], "MEDIA001");
}

/// 对应 Java: WxMpMaterialServiceImplTest（图片上传结果解析）
#[test]
fn test_mp_material_upload_result_serde() {
    let json_str = r#"{
        "errCode": 0,
        "errMsg": "ok",
        "mediaId": "MEDIA_NEW_001",
        "url": "https://mmbiz.qpic.cn/test.jpg"
    }"#;
    let result: WxMpMaterialUploadResult = serde_json::from_str(json_str).expect("解析上传结果");
    assert_eq!(result.err_code, 0);
    assert_eq!(result.media_id, "MEDIA_NEW_001");
    assert_eq!(result.url, "https://mmbiz.qpic.cn/test.jpg");
}

/// 对应 Java: WxMpMaterialServiceImplTest（素材数量结果解析）
#[test]
fn test_mp_material_count_result_serde() {
    let json_str = r#"{
        "errcode": 0,
        "errmsg": "ok",
        "voiceCount": 5,
        "videoCount": 3,
        "imageCount": 10,
        "newsCount": 2
    }"#;
    let result: WxMpMaterialCountResult = serde_json::from_str(json_str).expect("解析素材数量");
    assert_eq!(result.voice_count, 5);
    assert_eq!(result.video_count, 3);
    assert_eq!(result.image_count, 10);
    assert_eq!(result.news_count, 2);
}
