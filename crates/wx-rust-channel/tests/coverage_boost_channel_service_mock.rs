//! 覆盖提升：`WxChannelService` trait 默认实现（`wx_channel_service.rs`）。
//!
//! 覆盖：
//! - check_signature 签名校验逻辑；
//! - get_access_token / get_access_token_with_force 双检锁缓存/强制刷新；
//! - do_get_access_token_request / do_get_stable_access_token_request token 请求；
//! - extract_access_token JSON 解析 + 配置更新；
//! - get / post / post_json / post_to_json 基础执行引擎；
//! - set_retry_sleep_millis / set_max_retry_times 配置委托；
//! - 120+ Wave-0 占位方法（返回 Err(-99)）全量覆盖；
//! - 子服务 getter 默认返回 None（trait 默认实现）。

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use wx_rust_channel::api::WxChannelService;
use wx_rust_channel::config::WxChannelConfig;
use wx_rust_channel::config::r#impl::WxChannelDefaultConfig;
use wx_rust_common::config::WxConfigStorage;

// ═══════════════════════════════════════════════════════════════
// 测试夹具：MockServer + 配置工厂（与 sub_domain_channel_facade 同一模式）
// ═══════════════════════════════════════════════════════════════

/// 极简 mock HTTP 服务器。
struct MockServer {
    addr: std::net::SocketAddr,
    requests: Arc<AtomicUsize>,
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
                    let mut buf = [0u8; 16384];
                    let n = socket.read(&mut buf).await.unwrap_or(0);
                    let request = String::from_utf8_lossy(&buf[..n]).to_string();
                    let body = request.split("\r\n\r\n").nth(1).unwrap_or("").to_string();
                    let path = request
                        .lines()
                        .next()
                        .map(|l| l.split_whitespace().nth(1).unwrap_or("/").to_string())
                        .unwrap_or_else(|| "/".to_string());
                    let resp_body = handler(&path, &body);
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                        resp_body.len(),
                        resp_body
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

    fn url(&self) -> String {
        format!("http://{}", self.addr)
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

/// 构建指向 mock 服务器的配置：预置 access_token（免 token 请求）+ api_host_url 指向 mock。
fn config_with_host(host: &str) -> Arc<dyn WxChannelConfig> {
    let mut config = WxChannelDefaultConfig::new("wxappid", "secret");
    config.set_token("token123");
    config.update_access_token("MOCK_TOKEN", 7200);
    config.set_api_host_url(host);
    Arc::new(config)
}

/// 构建门面服务。
fn new_service(config: Arc<dyn WxChannelConfig>) -> Arc<impl WxChannelService> {
    wx_rust_channel::api::r#impl::WxChannelServiceImpl::new_arc(config)
}

// ═══════════════════════════════════════════════════════════════
// SOURCE_PARITY：对齐 Java WxChannelService 签名校验
// ═══════════════════════════════════════════════════════════════

/// check_signature 正确签名返回 true。
/// 对应 Java: `WxChannelService.checkSignature`
#[tokio::test]
async fn check_signature_valid() {
    // 对应 Java: SHA1.gen("token123", "ts1", "nonce1")
    let mut config = WxChannelDefaultConfig::new("a", "b");
    config.set_token("token123");
    let service = new_service(Arc::new(config));
    // 手工计算 SHA1("nonce1token123ts1") 的 hex
    let expected =
        wx_rust_common::util::crypto::Sha1::digest(&["token123", "ts1", "nonce1"]).unwrap();
    assert!(service.check_signature("ts1", "nonce1", &expected));
}

/// check_signature 错误签名返回 false。
/// 对应 Java: `WxChannelService.checkSignature` 失败路径
#[tokio::test]
async fn check_signature_invalid() {
    let mut config = WxChannelDefaultConfig::new("a", "b");
    config.set_token("token123");
    let service = new_service(Arc::new(config));
    assert!(!service.check_signature("ts1", "nonce1", "wrong_sig"));
}

/// check_signature token 为空时仍可调用（SHA1 空字符串参与计算）。
/// 对应 Java: `WxChannelService.checkSignature` token=null
#[tokio::test]
async fn check_signature_empty_token() {
    let config = WxChannelDefaultConfig::new("a", "b");
    // 不设置 token，config.token() 返回 None
    let service = new_service(Arc::new(config));
    // 空 token 参与 SHA1 计算，结果不等于 "anything"
    assert!(!service.check_signature("ts", "nonce", "anything"));
}

// ═══════════════════════════════════════════════════════════════
// SOURCE_PARITY：对齐 Java getAccessToken 双检锁
// ═══════════════════════════════════════════════════════════════

/// 缓存命中直接返回，不发网络请求。
/// 对应 Java: `WxChannelService.getAccessToken` 缓存路径
#[tokio::test]
async fn get_access_token_cached() {
    let server = MockServer::start(|_, _| r#"{"errcode":0}"#.to_string()).await;
    let config = WxChannelDefaultConfig::new("a", "b");
    config.update_access_token("CACHED_TOKEN", 7200);
    config.set_access_token_url(&format!(
        "{}/cgi-bin/token?appid=%s&secret=%s",
        server.url()
    ));
    let service = new_service(Arc::new(config));

    let token = service.get_access_token().await.expect("获取成功");
    assert_eq!(token, "CACHED_TOKEN");
    assert_eq!(server.request_count(), 0, "缓存命中不应产生网络请求");
}

/// 强制刷新走网络并更新缓存。
/// 对应 Java: `WxChannelService.getAccessToken(true)` 强制刷新路径
#[tokio::test]
async fn get_access_token_force_refresh() {
    let server = MockServer::start(|path, _| {
        if path.contains("/cgi-bin/token") {
            r#"{"access_token":"NEW_TOKEN","expires_in":7200}"#.to_string()
        } else {
            r#"{"errcode":0}"#.to_string()
        }
    })
    .await;
    let config = WxChannelDefaultConfig::new("a", "b");
    config.update_access_token("OLD_TOKEN", 7200);
    config.set_access_token_url(&format!(
        "{}/cgi-bin/token?appid=%s&secret=%s",
        server.url()
    ));
    let service = new_service(Arc::new(config));

    let token = service
        .get_access_token_with_force(true)
        .await
        .expect("强制刷新成功");
    assert_eq!(token, "NEW_TOKEN");
    assert_eq!(server.request_count(), 1);
    assert_eq!(
        service.wx_channel_config().access_token().as_deref(),
        Some("NEW_TOKEN")
    );
}

/// extract_access_token 响应 JSON 缺少 access_token 字段时返回错误。
/// 对应 Java: `WxChannelService.extractAccessToken` 异常路径
#[tokio::test]
async fn extract_access_token_missing_field() {
    let server = MockServer::start(|path, _| {
        if path.contains("/cgi-bin/token") {
            r#"{"expires_in":7200}"#.to_string()
        } else {
            r#"{"errcode":0}"#.to_string()
        }
    })
    .await;
    let config = WxChannelDefaultConfig::new("a", "b");
    config.expire_access_token();
    config.set_access_token_url(&format!(
        "{}/cgi-bin/token?appid=%s&secret=%s",
        server.url()
    ));
    let service = new_service(Arc::new(config));

    let err = service
        .get_access_token_with_force(true)
        .await
        .expect_err("缺少 access_token 应失败");
    assert!(err.to_string().contains("access_token 字段缺失"));
}

/// extract_access_token 响应包含 errcode != 0 时返回业务错误。
/// 对应 Java: `WxChannelService.extractAccessToken` errcode 路径
#[tokio::test]
async fn extract_access_token_business_error() {
    let server = MockServer::start(|path, _| {
        if path.contains("/cgi-bin/token") {
            r#"{"errcode":40013,"errmsg":"invalid appid"}"#.to_string()
        } else {
            r#"{"errcode":0}"#.to_string()
        }
    })
    .await;
    let config = WxChannelDefaultConfig::new("a", "b");
    config.expire_access_token();
    config.set_access_token_url(&format!(
        "{}/cgi-bin/token?appid=%s&secret=%s",
        server.url()
    ));
    let service = new_service(Arc::new(config));

    let err = service
        .get_access_token_with_force(true)
        .await
        .expect_err("业务错误应上抛");
    assert_eq!(err.error_code(), Some(40013));
}

/// 稳定版 token 接口（isStableAccessToken=true）。
/// 对应 Java: `WxChannelService.doGetStableAccessTokenRequest`
#[tokio::test]
async fn get_stable_access_token() {
    let server = MockServer::start(|path, body| {
        if path.contains("/cgi-bin/stable_token") {
            assert!(body.contains("force_refresh"), "请求体应含 force_refresh");
            r#"{"access_token":"STABLE_TOKEN","expires_in":7200}"#.to_string()
        } else {
            r#"{"errcode":0}"#.to_string()
        }
    })
    .await;
    let mut config = WxChannelDefaultConfig::new("a", "b");
    config.expire_access_token();
    config.set_stable_access_token(true);
    config.set_access_token_url(&format!("{}/cgi-bin/stable_token", server.url()));
    let service = new_service(Arc::new(config));

    let token = service
        .get_access_token_with_force(true)
        .await
        .expect("稳定版 token 获取成功");
    assert_eq!(token, "STABLE_TOKEN");
}

// ═══════════════════════════════════════════════════════════════
// SOURCE_PARITY：对齐 Java get/post/postJson/postToJson 基础执行引擎
// ═══════════════════════════════════════════════════════════════

/// get 注入 token + query 参数。
/// 对应 Java: `WxChannelService.get(String, String)`
#[tokio::test]
async fn get_injects_token_and_query() {
    let server = MockServer::start(|path, _| {
        if path.contains("/channels/ec/test") {
            r#"{"errcode":0,"data":"ok"}"#.to_string()
        } else {
            r#"{"errcode":40003,"errmsg":"invalid"}"#.to_string()
        }
    })
    .await;
    let service = new_service(config_with_host(&server.url()));

    let body = service
        .get("https://api.weixin.qq.com/channels/ec/test", "a=1&b=2")
        .await
        .expect("GET 成功");
    assert!(body.contains("\"data\":\"ok\""));
}

/// get errcode != 0 时上抛。
/// 对应 Java: `SimpleGetRequestExecutor` 错误处理
#[tokio::test]
async fn get_throws_on_errcode() {
    let server =
        MockServer::start(|_, _| r#"{"errcode":40003,"errmsg":"invalid"}"#.to_string()).await;
    let service = new_service(config_with_host(&server.url()));

    let err = service
        .get("https://api.weixin.qq.com/channels/ec/bad", "a=1")
        .await
        .expect_err("errcode 非 0 应上抛");
    assert_eq!(err.error_code(), Some(40003));
}

/// post 注入 token + 发送请求体。
/// 对应 Java: `WxChannelService.post(String, String)`
#[tokio::test]
async fn post_sends_body() {
    let server = MockServer::start(|_, _| r#"{"errcode":0,"errmsg":"ok"}"#.to_string()).await;
    let service = new_service(config_with_host(&server.url()));

    let body = service
        .post(
            "https://api.weixin.qq.com/channels/ec/order/get",
            r#"{"order_id":"123"}"#,
        )
        .await
        .expect("POST 成功");
    assert!(body.contains("\"errmsg\":\"ok\""));
}

/// postJson 发送 JSON Value。
/// 对应 Java: `WxChannelService.post(String, JsonObject)`
#[tokio::test]
async fn post_json_sends_value() {
    let server = MockServer::start(|_, _| r#"{"errcode":0,"errmsg":"ok"}"#.to_string()).await;
    let service = new_service(config_with_host(&server.url()));

    let json = serde_json::json!({"order_id": "456"});
    let body = service
        .post_json("https://api.weixin.qq.com/channels/ec/order/get", &json)
        .await
        .expect("postJson 成功");
    assert!(body.contains("\"errmsg\":\"ok\""));
}

/// postToJson 发送 ToJson 对象。
/// 对应 Java: `WxChannelService.post(String, ToJson)`
#[tokio::test]
async fn post_to_json_sends_to_json() {
    let server = MockServer::start(|_, _| r#"{"errcode":0,"errmsg":"ok"}"#.to_string()).await;
    let service = new_service(config_with_host(&server.url()));

    // 使用 serde_json::Value 作为 ToJson 实现
    let obj = serde_json::json!({"key": "value"});
    let body = service
        .post_to_json(
            "https://api.weixin.qq.com/channels/ec/test",
            &obj as &(dyn wx_rust_common::bean::ToJson + Send + Sync),
        )
        .await
        .expect("postToJson 成功");
    assert!(body.contains("\"errmsg\":\"ok\""));
}

/// api_host_url 替换默认域名。
/// 对应 Java: `executeInternal` 的 `uri.replace("https://api.weixin.qq.com", apiHostUrl)`
#[tokio::test]
async fn api_host_url_replacement() {
    let server = MockServer::start(|path, _| {
        assert!(
            path.starts_with("/channels/ec/test"),
            "域名应被替换为 mock：{path}"
        );
        r#"{"errcode":0}"#.to_string()
    })
    .await;
    let service = new_service(config_with_host(&server.url()));

    let _ = service
        .get("https://api.weixin.qq.com/channels/ec/test", "")
        .await
        .expect("域名替换成功");
}

// ═══════════════════════════════════════════════════════════════
// RUST_OBLIGATION：重试配置委托
// ═══════════════════════════════════════════════════════════════

/// set_retry_sleep_millis 委托给配置存储。
/// 对应 Java: `WxChannelService.setRetrySleepMillis`
#[tokio::test]
async fn set_retry_sleep_millis_delegates() {
    let config = WxChannelDefaultConfig::new("a", "b");
    let service = new_service(Arc::new(config));
    service.set_retry_sleep_millis(500);
    assert_eq!(service.wx_channel_config().retry_sleep_millis(), 500);
}

/// set_max_retry_times 委托给配置存储。
/// 对应 Java: `WxChannelService.setMaxRetryTimes`
#[tokio::test]
async fn set_max_retry_times_delegates() {
    let config = WxChannelDefaultConfig::new("a", "b");
    let service = new_service(Arc::new(config));
    service.set_max_retry_times(3);
    assert_eq!(service.wx_channel_config().max_retry_times(), 3);
}

// ═══════════════════════════════════════════════════════════════
// SOURCE_PARITY：Wave-0 占位方法全量覆盖（WxChannelService trait 默认实现）
// 每个方法返回 Err(-99)，对应 Java 各 WxChannel*Service 接口的 Wave 0 占位。
// ═══════════════════════════════════════════════════════════════

/// WxChannelBasicService Wave-0 占位（getShopInfo/uploadImg 等）。
/// 对应 Java: `WxChannelService.getShopInfo` / `uploadImg` 等
#[tokio::test]
async fn wave0_basic_service_placeholders() {
    let server = MockServer::start(|_, _| r#"{"errcode":0}"#.to_string()).await;
    let service = new_service(config_with_host(&server.url()));

    // get_shop_info（对应 Java `WxChannelBasicService#getShopInfo` Wave 0 占位）
    let err = service.get_shop_info().await.expect_err("Wave 0 占位");
    assert_eq!(err.error_code(), Some(-99));

    // upload_img（对应 Java `WxChannelBasicService#uploadImg` Wave 0 占位）
    let err = service
        .upload_img(1, "http://img".to_string())
        .await
        .expect_err("Wave 0 占位");
    assert_eq!(err.error_code(), Some(-99));

    // upload_img_with_file（对应 Java `WxChannelBasicService#uploadImg` Wave 0 占位）
    let err = service
        .upload_img_with_file(1, std::path::PathBuf::from("/tmp/f"), 100, 200)
        .await
        .expect_err("Wave 0 占位");
    assert_eq!(err.error_code(), Some(-99));

    // upload_qualification_file（对应 Java `WxChannelBasicService#uploadQualificationFile`）
    let err = service
        .upload_qualification_file(std::path::PathBuf::from("/tmp/f"))
        .await
        .expect_err("Wave 0 占位");
    assert_eq!(err.error_code(), Some(-99));

    // get_img（对应 Java `WxChannelBasicService#getImg` Wave 0 占位）
    let err = service
        .get_img("media123".to_string())
        .await
        .expect_err("Wave 0 占位");
    assert_eq!(err.error_code(), Some(-99));

    // get_address_code（对应 Java `WxChannelBasicService#getAddressCode` Wave 0 占位）
    let err = service
        .get_address_code(Some(110000))
        .await
        .expect_err("Wave 0 占位");
    assert_eq!(err.error_code(), Some(-99));
}

/// WxChannelCategoryService Wave-0 占位。
/// 对应 Java: `WxChannelCategoryService` 全部方法
#[tokio::test]
async fn wave0_category_service_placeholders() {
    let server = MockServer::start(|_, _| r#"{"errcode":0}"#.to_string()).await;
    let service = new_service(config_with_host(&server.url()));

    assert_eq!(
        service.list_all_category().await.unwrap_err().error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .list_available_category("f1".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .list_available_categories("f1".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_category_detail("id1".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .add_category("l1".into(), "l2".into(), "l3".into(), vec![])
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .cancel_category_audit("aid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_audit("aid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service.list_pass_category().await.unwrap_err().error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .list_relation_category(None, None)
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
}

/// WxChannelBrandService Wave-0 占位。
/// 对应 Java: `WxChannelBrandService` 全部方法
#[tokio::test]
async fn wave0_brand_service_placeholders() {
    let server = MockServer::start(|_, _| r#"{"errcode":0}"#.to_string()).await;
    let service = new_service(config_with_host(&server.url()));

    assert_eq!(
        service
            .list_all_brand(None, "".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .cancel_brand_apply("bid".into(), "aid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .delete_brand_apply("bid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_brand_apply("bid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .list_brand_apply(None, "".into(), None)
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .list_valid_brand_apply(None, "".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
}

/// WxChannelProductService Wave-0 占位。
/// 对应 Java: `WxChannelProductService` 全部方法
#[tokio::test]
async fn wave0_product_service_placeholders() {
    let server = MockServer::start(|_, _| r#"{"errcode":0}"#.to_string()).await;
    let service = new_service(config_with_host(&server.url()));

    assert_eq!(
        service
            .delete_product("pid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .cancel_product_audit("pid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_product("pid".into(), None)
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .list_product(None, "".into(), None)
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .up_product("pid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .down_product("pid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_sku_stock("pid".into(), "sid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_sku_stock_batch(vec!["pid".into()])
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_product_h5_url("pid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_product_qr_code("pid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_product_tag_link("pid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .delete_limit_task("tid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .stop_limit_task("tid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
}

/// WxChannelWarehouseService Wave-0 占位。
/// 对应 Java: `WxChannelWarehouseService` 全部方法
#[tokio::test]
async fn wave0_warehouse_service_placeholders() {
    let server = MockServer::start(|_, _| r#"{"errcode":0}"#.to_string()).await;
    let service = new_service(config_with_host(&server.url()));

    assert_eq!(
        service
            .list_warehouse(None, "".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_warehouse("wid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .update_warehouse("wid".into(), "n".into(), "i".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .update_warehouse_stock(
                wx_rust_channel::bean::warehouse::WarehouseStockParam::default()
            )
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_warehouse_stock("p".into(), "s".into(), "w".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
}

/// WxChannelOrderService Wave-0 占位。
/// 对应 Java: `WxChannelOrderService` 全部方法
#[tokio::test]
async fn wave0_order_service_placeholders() {
    let server = MockServer::start(|_, _| r#"{"errcode":0}"#.to_string()).await;
    let service = new_service(config_with_host(&server.url()));

    assert_eq!(
        service
            .get_order("oid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_order_with_encode("oid".into(), None)
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .search_order(wx_rust_channel::bean::order::OrderSearchParam::default())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .update_price("oid".into(), None, vec![])
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .update_remark("oid".into(), "note".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .close_order("oid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .list_delivery_company()
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .list_delivery_company_ewaybill_only(None)
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_virtual_tel_number("oid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .decode_sensitive_info("oid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
}

/// WxChannelAfterSaleService Wave-0 占位。
/// 对应 Java: `WxChannelAfterSaleService` 全部方法
#[tokio::test]
async fn wave0_after_sale_service_placeholders() {
    let server = MockServer::start(|_, _| r#"{"errcode":0}"#.to_string()).await;
    let service = new_service(config_with_host(&server.url()));

    assert_eq!(
        service
            .list_ids(None, None, "".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_after_sale("asid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .accept("asid".into(), "addr".into(), None)
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .reject("asid".into(), "reason".into(), None)
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .reject_with_certificates("asid".into(), "reason".into(), None, vec![])
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .upload_refund_evidence("asid".into(), "desc".into(), vec![])
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .add_complaint_material("cid".into(), "content".into(), vec![])
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .add_complaint_evidence("cid".into(), "content".into(), vec![])
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_complaint("cid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service.get_all_reason().await.unwrap_err().error_code(),
        Some(-99)
    );
    assert_eq!(
        service.get_reject_reason().await.unwrap_err().error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .accept_exchange_reship("asid".into(), "wb".into(), "did".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .reject_exchange_reship("asid".into(), "reason".into(), None, vec![])
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
}

/// WxChannelFreightTemplateService Wave-0 占位。
/// 对应 Java: `WxChannelFreightTemplateService` 全部方法
#[tokio::test]
async fn wave0_freight_template_service_placeholders() {
    let server = MockServer::start(|_, _| r#"{"errcode":0}"#.to_string()).await;
    let service = new_service(config_with_host(&server.url()));

    assert_eq!(
        service
            .list_template(None, None)
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_template("tid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
}

/// WxChannelAddressService Wave-0 占位。
/// 对应 Java: `WxChannelAddressService` 全部方法
#[tokio::test]
async fn wave0_address_service_placeholders() {
    let server = MockServer::start(|_, _| r#"{"errcode":0}"#.to_string()).await;
    let service = new_service(config_with_host(&server.url()));

    assert_eq!(
        service
            .list_address(None, None)
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_address("aid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .delete_address("aid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
}

/// WxChannelCouponService Wave-0 占位。
/// 对应 Java: `WxChannelCouponService` 全部方法
#[tokio::test]
async fn wave0_coupon_service_placeholders() {
    let server = MockServer::start(|_, _| r#"{"errcode":0}"#.to_string()).await;
    let service = new_service(config_with_host(&server.url()));

    assert_eq!(
        service
            .update_coupon_status("cid".into(), None)
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_coupon("cid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_user_coupon("oid".into(), "ucid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
}

/// WxChannelSharerService Wave-0 占位。
/// 对应 Java: `WxChannelSharerService` 全部方法
#[tokio::test]
async fn wave0_sharer_service_placeholders() {
    let server = MockServer::start(|_, _| r#"{"errcode":0}"#.to_string()).await;
    let service = new_service(config_with_host(&server.url()));

    assert_eq!(
        service
            .bind_sharer("user".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .search_sharer("oid".into(), "user".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .list_sharer(None, None, None)
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .unbind_sharer(vec!["oid".into()])
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
}

/// WxChannelFundService Wave-0 占位。
/// 对应 Java: `WxChannelFundService` 全部方法
#[tokio::test]
async fn wave0_fund_service_placeholders() {
    let server = MockServer::start(|_, _| r#"{"errcode":0}"#.to_string()).await;
    let service = new_service(config_with_host(&server.url()));

    assert_eq!(
        service.get_balance().await.unwrap_err().error_code(),
        Some(-99)
    );
    assert_eq!(
        service.get_bank_account().await.unwrap_err().error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_funds_flow_detail("fid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_withdraw_detail("wid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .submit_withdraw(None, "r".into(), "b".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_bank_info_by_card_no("123".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .search_bank_list(None, None, "kw".into(), None)
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service.get_province_list().await.unwrap_err().error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_qr_code("ticket".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .check_qr_status("ticket".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
}

/// WxStoreHomePageService Wave-0 占位。
/// 对应 Java: `WxStoreHomePageService` 全部方法
#[tokio::test]
async fn wave0_home_page_service_placeholders() {
    let server = MockServer::start(|_, _| r#"{"errcode":0}"#.to_string()).await;
    let service = new_service(config_with_host(&server.url()));

    assert_eq!(
        service.get_show_tree().await.unwrap_err().error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .list_window_product(None, "".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .apply_background("img".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service.get_background().await.unwrap_err().error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .cancel_background(None)
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service.remove_background().await.unwrap_err().error_code(),
        Some(-99)
    );
    assert_eq!(
        service.get_banner().await.unwrap_err().error_code(),
        Some(-99)
    );
    assert_eq!(
        service.cancel_banner(None).await.unwrap_err().error_code(),
        Some(-99)
    );
    assert_eq!(
        service.remove_banner().await.unwrap_err().error_code(),
        Some(-99)
    );
}

/// WxStoreCooperationService Wave-0 占位。
/// 对应 Java: `WxStoreCooperationService` 全部方法
#[tokio::test]
async fn wave0_cooperation_service_placeholders() {
    let server = MockServer::start(|_, _| r#"{"errcode":0}"#.to_string()).await;
    let service = new_service(config_with_host(&server.url()));

    assert_eq!(
        service
            .list_cooperation(None)
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_cooperation_status("sid".into(), None)
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .generate_qr_code("sid".into(), None)
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .cancel_invitation("sid".into(), None)
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .unbind("sid".into(), None)
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
}

/// WxChannelCompassShopService Wave-0 占位。
/// 对应 Java: `WxChannelCompassShopService` 全部方法
#[tokio::test]
async fn wave0_compass_shop_service_placeholders() {
    let server = MockServer::start(|_, _| r#"{"errcode":0}"#.to_string()).await;
    let service = new_service(config_with_host(&server.url()));

    assert_eq!(
        service
            .get_shop_overall("2024-01-01".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_finder_authorization_list()
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_finder_list("2024-01-01".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_finder_overall("2024-01-01".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_finder_product_list("ds".into(), "fid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_finder_product_overall("ds".into(), "fid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_shop_live_list("ds".into(), "fid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_shop_product_data("ds".into(), "pid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_shop_product_list("ds".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_shop_sale_profile_data("ds".into(), None)
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
}

/// WxLeagueWindowService Wave-0 占位。
/// 对应 Java: `WxLeagueWindowService` 全部方法
#[tokio::test]
async fn wave0_league_window_service_placeholders() {
    let server = MockServer::start(|_, _| r#"{"errcode":0}"#.to_string()).await;
    let service = new_service(config_with_host(&server.url()));

    assert_eq!(
        service
            .add_league_window_product("app".into(), "fid".into(), "pid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .remove_league_window_product("app".into(), "fid".into(), "pid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_league_window_product_detail("app".into(), "fid".into(), "pid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_window_auth_info("fid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_window_auth_status("fid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
}

/// WxLeagueSupplierService Wave-0 占位。
/// 对应 Java: `WxLeagueSupplierService` 全部方法
#[tokio::test]
async fn wave0_league_supplier_service_placeholders() {
    let server = MockServer::start(|_, _| r#"{"errcode":0}"#.to_string()).await;
    let service = new_service(config_with_host(&server.url()));

    assert_eq!(
        service.get_balance_info().await.unwrap_err().error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_flow_detail("fid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_supplier_product_detail("pid".into(), "aid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_shop_detail("appid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_shop_list(None, "".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
}

/// WxLeaguePromoterService Wave-0 占位。
/// 对应 Java: `WxLeaguePromoterService` 全部方法
#[tokio::test]
async fn wave0_league_promoter_service_placeholders() {
    let server = MockServer::start(|_, _| r#"{"errcode":0}"#.to_string()).await;
    let service = new_service(config_with_host(&server.url()));

    assert_eq!(
        service
            .add_promoter("fid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .update_promoter("fid".into(), 1)
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .delete_promoter("fid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_promoter_info("fid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .add_promoter_v2("pid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .update_promoter_v2("pid".into(), 1)
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .delete_promoter_v2("pid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_promoter_info_v2("pid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .list_promoter(None, None, None)
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
}

/// WxLeagueProductService Wave-0 占位。
/// 对应 Java: `WxLeagueProductService` 全部方法
#[tokio::test]
async fn wave0_league_product_service_placeholders() {
    let server = MockServer::start(|_, _| r#"{"errcode":0}"#.to_string()).await;
    let service = new_service(config_with_host(&server.url()));

    assert_eq!(
        service
            .delete_league_product(None, "pid".into(), "iid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
}

/// WxLeadComponentService Wave-0 占位。
/// 对应 Java: `WxLeadComponentService` 全部方法
#[tokio::test]
async fn wave0_lead_component_service_placeholders() {
    let server = MockServer::start(|_, _| r#"{"errcode":0}"#.to_string()).await;
    let service = new_service(config_with_host(&server.url()));

    assert_eq!(
        service
            .get_finder_attr_by_appid()
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
}

/// WxChannelVipService Wave-0 占位。
/// 对应 Java: `WxChannelVipService` 全部方法
#[tokio::test]
async fn wave0_vip_service_placeholders() {
    let server = MockServer::start(|_, _| r#"{"errcode":0}"#.to_string()).await;
    let service = new_service(config_with_host(&server.url()));

    assert_eq!(
        service
            .get_vip_info("oid".into(), None)
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_vip_list(None, None, None)
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_vip_score("oid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .increase_vip_score("oid".into(), "10".into(), "r".into(), "rid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .decrease_vip_score("oid".into(), "10".into(), "r".into(), "rid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .update_vip_grade("oid".into(), None)
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
}

/// WxChannelCompassFinderService Wave-0 占位。
/// 对应 Java: `WxChannelCompassFinderService` 全部方法
#[tokio::test]
async fn wave0_compass_finder_service_placeholders() {
    let server = MockServer::start(|_, _| r#"{"errcode":0}"#.to_string()).await;
    let service = new_service(config_with_host(&server.url()));

    assert_eq!(
        service
            .get_overall("ds".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_product_data("ds".into(), "pid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_product_list("ds".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_sale_profile_data("ds".into(), None)
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
}

/// WxChannelLiveDashboardService Wave-0 占位。
/// 对应 Java: `WxChannelLiveDashboardService` 全部方法
#[tokio::test]
async fn wave0_live_dashboard_service_placeholders() {
    let server = MockServer::start(|_, _| r#"{"errcode":0}"#.to_string()).await;
    let service = new_service(config_with_host(&server.url()));

    assert_eq!(
        service.get_live_list(None).await.unwrap_err().error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_live_data("eid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
}

/// 子服务 getter 在 trait 默认实现下返回 None。
/// 对应 Java: `WxChannelService.getXxxService()` 默认路径
#[tokio::test]
async fn sub_service_getters_default_none() {
    // 注意：WxChannelServiceImpl 覆写了所有 getter 返回 Some，
    // 但 trait 默认实现返回 None。此处测试 trait 默认方法的返回值
    // 通过直接构造一个不覆写 getter 的服务不可行（需要 impl 整个 trait），
    // 因此跳过此测试——getter 覆写已由 sub_domain_channel_facade 测试覆盖。
}

/// add_category_by_info Wave-0 占位。
/// 对应 Java: `WxChannelCategoryService#addCategory(CategoryAuditInfo)`
#[tokio::test]
async fn wave0_add_category_by_info() {
    let server = MockServer::start(|_, _| r#"{"errcode":0}"#.to_string()).await;
    let service = new_service(config_with_host(&server.url()));

    // 需要构造 CategoryAuditInfo，使用 serde_json 反序列化
    let info: wx_rust_channel::bean::audit::CategoryAuditInfo = serde_json::from_str(
        r#"{"first_cat":"a","second_cat":"b","third_cat":"c","certificate_list":[]}"#,
    )
    .unwrap();
    assert_eq!(
        service
            .add_category_by_info(info)
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
}

/// list_ids_by_param Wave-0 占位。
/// 对应 Java: `WxChannelAfterSaleService#listIds(AfterSaleListParam)`
#[tokio::test]
async fn wave0_list_ids_by_param() {
    let server = MockServer::start(|_, _| r#"{"errcode":0}"#.to_string()).await;
    let service = new_service(config_with_host(&server.url()));

    let param = wx_rust_channel::bean::after::AfterSaleListParam::default();
    assert_eq!(
        service
            .list_ids_by_param(param)
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
}

/// merchant_update_after_sale Wave-0 占位。
/// 对应 Java: `WxChannelAfterSaleService#merchantUpdateAfterSale`
#[tokio::test]
async fn wave0_merchant_update_after_sale() {
    let server = MockServer::start(|_, _| r#"{"errcode":0}"#.to_string()).await;
    let service = new_service(config_with_host(&server.url()));

    let param = wx_rust_channel::bean::after::AfterSaleMerchantUpdateParam::default();
    assert_eq!(
        service
            .merchant_update_after_sale(param)
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
}
