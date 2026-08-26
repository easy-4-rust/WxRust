#![allow(clippy::field_reassign_with_default)]
//! 存量语义审计修复验证测试。
//!
//! 验证以下 9 处缺陷修复的 HTTP 语义（URL + 方法动词 + 参数）：
//! 1. kefu: kf_account_add 使用 /customservice/kfaccount/add（非 /cgi-bin/message/custom/send）
//! 2. subscribe_msg: get_pub_template_title_list 使用 GET（非 POST）
//! 3. subscribe_msg: get_pub_template_key_words_by_id 使用 GET + query（非 POST + body）
//! 4. subscribe_msg: get_template_list 使用 GET（非 POST）
//! 5. subscribe_msg: get_category 使用 GET（非 POST）
//! 6. oauth2: sns_userinfo URL 包含 openid 参数
//! 7. material: material_video_info 使用 GET（非 POST）
//! 8. material: material_news_info 使用 GET（非 POST）
//! 9. material: material_delete 使用 GET（非 POST）

use std::sync::Arc;
use std::sync::atomic::Ordering;

use wx_rust_mp::api::WxMpService;
use wx_rust_mp::config::WxMpConfigStorage;
use wx_rust_mp::config::r#impl::WxMpDefaultConfig;

/// 极简 mock HTTP 服务器：记录最近请求方法 + 路径。
struct MockServer {
    addr: std::net::SocketAddr,
    last_request: Arc<std::sync::Mutex<String>>,
    stop: Arc<std::sync::atomic::AtomicBool>,
}

impl MockServer {
    async fn start<F>(handler: F) -> Self
    where
        F: Fn(&str, &str) -> String + Send + Sync + 'static,
    {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("绑定端口");
        let addr = listener.local_addr().expect("获取地址");
        let last_request = Arc::new(std::sync::Mutex::new(String::new()));
        let stop = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let handler = Arc::new(handler);

        let last_request_clone = last_request.clone();
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
                let last_request_clone = last_request_clone.clone();
                tokio::spawn(async move {
                    use tokio::io::{AsyncReadExt, AsyncWriteExt};
                    let mut buf = [0u8; 16384];
                    let n = socket.read(&mut buf).await.unwrap_or(0);
                    let request = String::from_utf8_lossy(&buf[..n]).to_string();
                    let first_line = request.lines().next().unwrap_or("").to_string();
                    let method = first_line
                        .split_whitespace()
                        .next()
                        .unwrap_or("GET")
                        .to_string();
                    let path = first_line
                        .split_whitespace()
                        .nth(1)
                        .unwrap_or("/")
                        .to_string();
                    *last_request_clone.lock().unwrap() = format!("{method} {path}");
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
            last_request,
            stop,
        }
    }

    fn url(&self, path: &str) -> String {
        format!("http://{}{}", self.addr, path)
    }

    fn last_request(&self) -> String {
        self.last_request.lock().unwrap().clone()
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
    handler: impl Fn(&str, &str) -> String + Send + Sync + 'static,
) -> impl Fn(&str, &str) -> String + Send + Sync + 'static {
    move |method: &str, path: &str| {
        if path.contains("/cgi-bin/token") || path.contains("/stable_token") {
            return r#"{"access_token":"MOCK_TOKEN","expires_in":7200}"#.to_string();
        }
        handler(method, path)
    }
}

// ---- 修复 1: kefu kf_account_add URL ----

#[tokio::test]
async fn kefu_kf_account_add_uses_correct_url() {
    let server = MockServer::start(dispatch(|_method, _path| {
        r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let kefu_service = service.kefu_service().expect("客服服务存在");

    let request = wx_rust_mp::bean::kefu::request::WxMpKfAccountRequest {
        kf_account: "test@test".to_string(),
        nick_name: "客服".to_string(),
        invite_wx: String::new(),
    };
    let result = kefu_service
        .kf_account_add(&request)
        .await
        .expect("添加客服成功");
    assert!(result);
    // 验证 URL 包含 /customservice/kfaccount/add，不包含 /cgi-bin/message/custom/send
    let req = server.last_request();
    assert!(
        req.contains("/customservice/kfaccount/add"),
        "kf_account_add 应使用 /customservice/kfaccount/add，实际: {req}"
    );
    assert!(
        !req.contains("/cgi-bin/message/custom/send"),
        "kf_account_add 不应使用 /cgi-bin/message/custom/send，实际: {req}"
    );
}

// ---- 修复 2-5: subscribe_msg GET vs POST ----

#[tokio::test]
async fn subscribe_get_pub_template_title_list_uses_get() {
    let server = MockServer::start(dispatch(|method, path| {
        if path.contains("/wxaapi/newtmpl/getpubtemplatetitles") {
            // 验证是 GET 请求
            assert_eq!(method, "GET", "get_pub_template_title_list 应使用 GET");
            r#"{"list":[],"total_count":0}"#.to_string()
        } else {
            "{}".to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let subscribe_service = service.subscribe_msg_service().expect("订阅消息服务存在");

    let _ = subscribe_service
        .get_pub_template_title_list(&[], 0, 10)
        .await
        .expect("获取模板标题列表成功");
    let req = server.last_request();
    assert!(
        req.starts_with("GET"),
        "get_pub_template_title_list 应使用 GET，实际: {req}"
    );
}

#[tokio::test]
async fn subscribe_get_pub_template_key_words_by_id_uses_get() {
    let server = MockServer::start(dispatch(|method, path| {
        if path.contains("/wxaapi/newtmpl/getpubtemplatekeywords") {
            assert_eq!(method, "GET", "get_pub_template_key_words_by_id 应使用 GET");
            // 验证 query 参数包含 tid
            assert!(path.contains("tid="), "URL 应包含 tid 参数，实际: {path}");
            r#"{"data":[{"kid":1,"name":"keyword1","example":"example1"}]}"#.to_string()
        } else {
            "{}".to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let subscribe_service = service.subscribe_msg_service().expect("订阅消息服务存在");

    let _ = subscribe_service
        .get_pub_template_key_words_by_id("TMPL_ID_1")
        .await
        .expect("获取模板关键词成功");
    let req = server.last_request();
    assert!(
        req.starts_with("GET"),
        "get_pub_template_key_words_by_id 应使用 GET，实际: {req}"
    );
    assert!(
        req.contains("tid=TMPL_ID_1"),
        "URL 应包含 tid=TMPL_ID_1，实际: {req}"
    );
}

#[tokio::test]
async fn subscribe_get_template_list_uses_get() {
    let server = MockServer::start(dispatch(|method, path| {
        if path.contains("/wxaapi/newtmpl/gettemplate") {
            assert_eq!(method, "GET", "get_template_list 应使用 GET");
            r#"{"data":[]}"#.to_string()
        } else {
            "{}".to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let subscribe_service = service.subscribe_msg_service().expect("订阅消息服务存在");

    let _ = subscribe_service
        .get_template_list()
        .await
        .expect("获取模板列表成功");
    let req = server.last_request();
    assert!(
        req.starts_with("GET"),
        "get_template_list 应使用 GET，实际: {req}"
    );
}

#[tokio::test]
async fn subscribe_get_category_uses_get() {
    let server = MockServer::start(dispatch(|method, path| {
        if path.contains("/wxaapi/newtmpl/getcategory") {
            assert_eq!(method, "GET", "get_category 应使用 GET");
            r#"{"data":[]}"#.to_string()
        } else {
            "{}".to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let subscribe_service = service.subscribe_msg_service().expect("订阅消息服务存在");

    let _ = subscribe_service
        .get_category()
        .await
        .expect("获取分类成功");
    let req = server.last_request();
    assert!(
        req.starts_with("GET"),
        "get_category 应使用 GET，实际: {req}"
    );
}

// ---- 修复 6: oauth2 sns_userinfo URL 包含 openid ----

#[tokio::test]
async fn oauth2_get_user_info_url_contains_openid() {
    let server = MockServer::start(dispatch(|_method, path| {
        if path.contains("/sns/userinfo") {
            // 验证 URL 包含 openid 参数
            assert!(
                path.contains("openid="),
                "sns_userinfo URL 应包含 openid 参数，实际: {path}"
            );
            r#"{"openid":"oX1","nickname":"测试用户","headimgurl":"http://img"}"#.to_string()
        } else {
            "{}".to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let dyn_arc: Arc<dyn WxMpService> = service.clone();
    let oauth2_service =
        wx_rust_mp::api::r#impl::WxMpOAuth2ServiceImpl::new(Arc::downgrade(&dyn_arc));

    let token = wx_rust_common::bean::oauth2::WxOAuth2AccessToken {
        access_token: "AT1".to_string(),
        open_id: "oX1".to_string(),
        refresh_token: "RT1".to_string(),
        expires_in: 7200,
        scope: "snsapi_userinfo".to_string(),
        ..Default::default()
    };
    use wx_rust_common::service::WxOAuth2Service;
    let _ = oauth2_service
        .get_user_info(&token, "zh_CN")
        .await
        .expect("获取用户信息成功");
    let req = server.last_request();
    assert!(
        req.contains("openid=oX1"),
        "URL 应包含 openid=oX1，实际: {req}"
    );
    assert!(
        req.contains("access_token=AT1"),
        "URL 应包含 access_token=AT1，实际: {req}"
    );
}

// ---- 修复 7-9: material GET vs POST ----

#[tokio::test]
async fn material_video_info_uses_get() {
    let server = MockServer::start(dispatch(|method, path| {
        if path.contains("/cgi-bin/material/get_material") {
            assert_eq!(method, "GET", "material_video_info 应使用 GET");
            assert!(
                path.contains("media_id="),
                "URL 应包含 media_id 参数，实际: {path}"
            );
            r#"{"title":"视频标题","description":"视频描述","down_url":"http://video"}"#.to_string()
        } else {
            "{}".to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let material_service = service.material_service().expect("素材服务存在");

    let _ = material_service
        .material_video_info("MEDIA_ID_1")
        .await
        .expect("获取视频信息成功");
    let req = server.last_request();
    assert!(
        req.starts_with("GET"),
        "material_video_info 应使用 GET，实际: {req}"
    );
    assert!(
        req.contains("media_id=MEDIA_ID_1"),
        "URL 应包含 media_id=MEDIA_ID_1，实际: {req}"
    );
}

#[tokio::test]
async fn material_news_info_uses_get() {
    let server = MockServer::start(dispatch(|method, path| {
        if path.contains("/cgi-bin/material/get_material") {
            assert_eq!(method, "GET", "material_news_info 应使用 GET");
            assert!(
                path.contains("media_id="),
                "URL 应包含 media_id 参数，实际: {path}"
            );
            r#"{"news_item":[{"title":"图文标题","content":"图文内容"}]}"#.to_string()
        } else {
            "{}".to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let material_service = service.material_service().expect("素材服务存在");

    let _ = material_service
        .material_news_info("MEDIA_ID_2")
        .await
        .expect("获取图文信息成功");
    let req = server.last_request();
    assert!(
        req.starts_with("GET"),
        "material_news_info 应使用 GET，实际: {req}"
    );
    assert!(
        req.contains("media_id=MEDIA_ID_2"),
        "URL 应包含 media_id=MEDIA_ID_2，实际: {req}"
    );
}

#[tokio::test]
async fn material_delete_uses_get() {
    let server = MockServer::start(dispatch(|method, path| {
        if path.contains("/cgi-bin/material/del_material") {
            assert_eq!(method, "GET", "material_delete 应使用 GET");
            assert!(
                path.contains("media_id="),
                "URL 应包含 media_id 参数，实际: {path}"
            );
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        } else {
            "{}".to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let material_service = service.material_service().expect("素材服务存在");

    let result = material_service
        .material_delete("MEDIA_ID_3")
        .await
        .expect("删除素材成功");
    assert!(result);
    let req = server.last_request();
    assert!(
        req.starts_with("GET"),
        "material_delete 应使用 GET，实际: {req}"
    );
    assert!(
        req.contains("media_id=MEDIA_ID_3"),
        "URL 应包含 media_id=MEDIA_ID_3，实际: {req}"
    );
}
