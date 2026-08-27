//! WxRust Alpha 内部接入示例：miniapp-text-sender
//!
//! 场景：内部通知服务通过 wx-rust-miniapp 发送小程序订阅消息。
//! 演示：配置存储、服务构建、订阅消息发送、错误处理。
//!
//! 本 crate 不依赖真实微信服务端，使用内置 MockServer 进行端到端验证。

use std::sync::Arc;

use wx_rust_miniapp::api::r#impl::WxMaServiceImpl;
use wx_rust_miniapp::bean::{MsgData, WxMaSubscribeMessage};
use wx_rust_miniapp::config::r#impl::WxMaDefaultConfig;
use wx_rust_miniapp::config::WxMaConfig;

/// 构建指向指定 host 的小程序服务实例。
///
/// 模拟接入方的配置初始化流程：设置 appid/secret/token/host。
pub fn build_service(host: &str, appid: &str, secret: &str) -> Arc<WxMaServiceImpl> {
    let mut config = WxMaDefaultConfig::new(appid, secret);
    config.set_token("alpha-test-token");
    config.set_msg_data_format("json");
    let mut host_config = wx_rust_miniapp::config::WxMaHostConfig::new();
    host_config.api_host = host.to_string();
    config.set_host_config(host_config);
    let config_arc: Arc<dyn WxMaConfig> = Arc::new(config);
    WxMaServiceImpl::new_arc(config_arc)
}

/// 构造一条订阅消息（内部通知场景：订单状态变更通知）。
pub fn build_order_notify_msg(
    to_user: &str,
    template_id: &str,
    order_no: &str,
    status: &str,
) -> WxMaSubscribeMessage {
    let mut msg = WxMaSubscribeMessage::new();
    msg.to_user = Some(to_user.to_string());
    msg.template_id = Some(template_id.to_string());
    msg.page = Some("pages/order/detail".to_string());
    msg.add_data(MsgData {
        name: "character_string1".to_string(),
        value: order_no.to_string(),
    });
    msg.add_data(MsgData {
        name: "thing2".to_string(),
        value: status.to_string(),
    });
    msg
}

/// 构造一条客服文本消息（内部通知场景：简单文本推送）。
pub fn build_kefu_text_msg(
    to_user: &str,
    content: &str,
) -> wx_rust_miniapp::message::WxMaKefuMessage {
    wx_rust_miniapp::builder::TextMessageBuilder::new()
        .to_user(to_user)
        .content(content)
        .build()
}

// ============================================================================
// Mock HTTP 服务器（测试 / 示例共用）
// ============================================================================

use std::net::SocketAddr;
use std::sync::atomic::{AtomicUsize, Ordering};

/// 极简 mock HTTP 服务器：按请求路径返回预设响应。
pub struct MockServer {
    pub addr: SocketAddr,
    pub request_count: Arc<AtomicUsize>,
    pub last_path: Arc<std::sync::Mutex<String>>,
    pub last_body: Arc<std::sync::Mutex<String>>,
    stop: Arc<std::sync::atomic::AtomicBool>,
}

impl MockServer {
    /// 启动 mock 服务器。`handler` 接收请求路径，返回 (Content-Type, body)。
    pub async fn start<F>(handler: F) -> Self
    where
        F: Fn(&str) -> (String, String) + Send + Sync + 'static,
    {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("绑定端口");
        let addr = listener.local_addr().expect("获取地址");
        let request_count = Arc::new(AtomicUsize::new(0));
        let last_path = Arc::new(std::sync::Mutex::new(String::new()));
        let last_body = Arc::new(std::sync::Mutex::new(String::new()));
        let stop = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let handler = Arc::new(handler);

        let rc = request_count.clone();
        let lp = last_path.clone();
        let lb = last_body.clone();
        let stop_c = stop.clone();

        tokio::spawn(async move {
            loop {
                if stop_c.load(Ordering::SeqCst) {
                    break;
                }
                let Ok((mut socket, _)) = listener.accept().await else {
                    continue;
                };
                rc.fetch_add(1, Ordering::SeqCst);
                let handler = handler.clone();
                let lp = lp.clone();
                let lb = lb.clone();
                tokio::spawn(async move {
                    use tokio::io::{AsyncReadExt, AsyncWriteExt};
                    let mut buf = [0u8; 16384];
                    let n = socket.read(&mut buf).await.unwrap_or(0);
                    let request = String::from_utf8_lossy(&buf[..n]).to_string();

                    // 提取路径
                    let path = request
                        .lines()
                        .next()
                        .and_then(|l| l.split_whitespace().nth(1))
                        .unwrap_or("/")
                        .to_string();

                    // 提取 POST body
                    if let Some(idx) = request.find("\r\n\r\n") {
                        *lb.lock().unwrap() = request[idx + 4..].to_string();
                    }
                    *lp.lock().unwrap() = path.clone();

                    let (content_type, body) = handler(&path);
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                        body.len()
                    );
                    let _ = socket.write_all(response.as_bytes()).await;
                });
            }
        });

        Self { addr, request_count, last_path, last_body, stop }
    }

    /// 构造 base URL。
    pub fn base_url(&self) -> String {
        format!("http://{}", self.addr)
    }

    /// 已接收的请求总数。
    pub fn requests(&self) -> usize {
        self.request_count.load(Ordering::SeqCst)
    }

    /// 最后一次请求的路径。
    pub fn last_path(&self) -> String {
        self.last_path.lock().unwrap().clone()
    }

    /// 最后一次请求的 body。
    pub fn last_body(&self) -> String {
        self.last_body.lock().unwrap().clone()
    }
}

impl Drop for MockServer {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
    }
}

/// 通用路由：token 请求自动应答，其余走自定义 handler。
pub fn wechat_dispatch(
    handler: impl Fn(&str) -> (String, String) + Send + Sync + 'static,
) -> impl Fn(&str) -> (String, String) + Send + Sync + 'static {
    move |path: &str| {
        if path.contains("/cgi-bin/token") || path.contains("/cgi-bin/stable_token") {
            return (
                "application/json".to_string(),
                r#"{"access_token":"MOCK_ACCESS_TOKEN_7200","expires_in":7200}"#.to_string(),
            );
        }
        handler(path)
    }
}

/// 通用路由（带 token 请求计数器）：token 请求自动应答并计数，其余走自定义 handler。
pub fn wechat_dispatch_with_counter(
    handler: impl Fn(&str) -> (String, String) + Send + Sync + 'static,
    token_counter: Arc<AtomicUsize>,
) -> impl Fn(&str) -> (String, String) + Send + Sync + 'static {
    move |path: &str| {
        if path.contains("/cgi-bin/token") || path.contains("/cgi-bin/stable_token") {
            token_counter.fetch_add(1, Ordering::SeqCst);
            return (
                "application/json".to_string(),
                r#"{"access_token":"MOCK_ACCESS_TOKEN_7200","expires_in":7200}"#.to_string(),
            );
        }
        handler(path)
    }
}

// ============================================================================
// 集成测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use wx_rust_miniapp::api::WxMaService;

    /// 测试 1：token 获取 + 订阅消息发送正常路径
    #[tokio::test]
    async fn test_send_subscribe_msg_success() {
        let server = MockServer::start(wechat_dispatch(|path| {
            if path.contains("subscribe/send") {
                return (
                    "application/json".to_string(),
                    r#"{"errcode":0,"errmsg":"ok"}"#.to_string(),
                );
            }
            (
                "application/json".to_string(),
                r#"{"errcode":0,"errmsg":"ok"}"#.to_string(),
            )
        }))
        .await;

        let service = build_service(&server.base_url(), "wx_test_appid", "test_secret");
        let msg = build_order_notify_msg("ox_user123", "tpl_001", "ORD-20260827-001", "已发货");

        let result = service.send_subscribe_msg(&msg).await;
        assert!(result.is_ok(), "订阅消息发送应成功: {:?}", result.err());

        // 验证请求确实到达了 mock server（至少 2 次：token + subscribe）
        assert!(server.requests() >= 2, "应有至少 2 次请求");

        // 验证最后请求路径包含 subscribe/send
        let last = server.last_path();
        assert!(
            last.contains("subscribe/send"),
            "最后请求应为 subscribe/send, 实际: {last}"
        );

        // 验证请求 body 包含正确字段
        let body = server.last_body();
        assert!(body.contains("ox_user123"), "body 应含 touser");
        assert!(body.contains("tpl_001"), "body 应含 template_id");
        assert!(body.contains("ORD-20260827-001"), "body 应含订单号");
    }

    /// 测试 2：微信错误码应返回 Err
    #[tokio::test]
    async fn test_send_subscribe_msg_error_response() {
        let server = MockServer::start(wechat_dispatch(|path| {
            if path.contains("subscribe/send") {
                return (
                    "application/json".to_string(),
                    r#"{"errcode":40003,"errmsg":"invalid openid"}"#.to_string(),
                );
            }
            (
                "application/json".to_string(),
                r#"{"errcode":0,"errmsg":"ok"}"#.to_string(),
            )
        }))
        .await;

        let service = build_service(&server.base_url(), "wx_test_appid", "test_secret");
        let msg = build_order_notify_msg("invalid_openid", "tpl_001", "ORD-001", "已发货");

        let result = service.send_subscribe_msg(&msg).await;
        assert!(result.is_err(), "错误 openid 应返回 Err");

        let err = result.unwrap_err();
        let err_str = format!("{err}");
        assert!(
            err_str.contains("40003") || err_str.contains("invalid"),
            "错误应包含 40003 或 invalid, 实际: {err_str}"
        );
    }

    /// 测试 3：客服消息发送正常路径
    #[tokio::test]
    async fn test_send_kefu_msg_success() {
        let server = MockServer::start(wechat_dispatch(|path| {
            if path.contains("message/custom/send") || path.contains("cgi-bin/message/custom/send") {
                return (
                    "application/json".to_string(),
                    r#"{"errcode":0,"errmsg":"ok"}"#.to_string(),
                );
            }
            (
                "application/json".to_string(),
                r#"{"errcode":0,"errmsg":"ok"}"#.to_string(),
            )
        }))
        .await;

        let service = build_service(&server.base_url(), "wx_test_appid", "test_secret");
        let msg = build_kefu_text_msg("ox_user456", "Alpha 测试消息：订单 ORD-001 已发货");

        let result = service.send_kefu_msg(&msg).await;
        assert!(result.is_ok(), "客服消息发送应成功: {:?}", result.err());
    }

    /// 测试 4：check_signature 签名校验
    #[test]
    fn test_check_signature() {
        // 用已知输入验证 SHA1 签名逻辑
        let service = tokio::runtime::Runtime::new().unwrap().block_on(async {
            build_service("http://127.0.0.1:1", "wx_appid", "secret")
        });
        // token = "alpha-test-token", timestamp = "1234567890", nonce = "nonce123"
        // SHA1(sort("alpha-test-token" + "1234567890" + "nonce123"))
        let valid = service.check_signature("1234567890", "nonce123", "invalid_sig");
        assert!(!valid, "错误签名应返回 false");
    }

    /// 测试 5：多次调用 token 只请求一次（双检锁单飞）
    #[tokio::test]
    async fn test_token_single_flight() {
        let token_call_count = Arc::new(AtomicUsize::new(0));

        let server = MockServer::start(wechat_dispatch_with_counter(
            |path| {
                if path.contains("subscribe/send") {
                    return (
                        "application/json".to_string(),
                        r#"{"errcode":0,"errmsg":"ok"}"#.to_string(),
                    );
                }
                (
                    "application/json".to_string(),
                    r#"{"errcode":0,"errmsg":"ok"}"#.to_string(),
                )
            },
            token_call_count.clone(),
        ))
        .await;

        let service = build_service(&server.base_url(), "wx_appid", "secret");

        // 并发发送 3 条消息，token 请求应该只发生 1 次
        let msg1 = build_order_notify_msg("u1", "tpl", "ORD-1", "状态A");
        let msg2 = build_order_notify_msg("u2", "tpl", "ORD-2", "状态B");
        let msg3 = build_order_notify_msg("u3", "tpl", "ORD-3", "状态C");

        let (r1, r2, r3) = tokio::join!(
            service.send_subscribe_msg(&msg1),
            service.send_subscribe_msg(&msg2),
            service.send_subscribe_msg(&msg3),
        );
        assert!(r1.is_ok());
        assert!(r2.is_ok());
        assert!(r3.is_ok());

        // token 端点应答次数应为 1（双检锁单飞）
        let token_calls = token_call_count.load(Ordering::SeqCst);
        assert_eq!(token_calls, 1, "token 请求应只发生 1 次, 实际: {token_calls}");
    }
}
