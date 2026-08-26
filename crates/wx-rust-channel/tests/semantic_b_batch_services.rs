//! 语义审计批次 B：favorite / limited_discount / product_assistant / product_stock / talent
//! 5 个 service 的逐方法 URL + 字段名 + 响应解析审计验证。
//!
//! 每个测试断言：
//! 1. 请求路径包含正确的 API endpoint（与 Java `WxChannelApiUrlConstants` 逐字符对齐）
//! 2. 请求体包含正确的 JSON 字段名（与 Java bean `@SerializedName` / `@JsonProperty` 对齐）
//! 3. 响应能正确反序列化到目标类型

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use wx_rust_channel::api::WxChannelService;
use wx_rust_channel::api::r#impl::{
    WxChannelFavoriteServiceImpl, WxChannelLimitedDiscountServiceImpl,
    WxChannelProductAssistantServiceImpl, WxChannelProductStockServiceImpl, WxChannelServiceImpl,
    WxTalentServiceImpl,
};
use wx_rust_channel::api::{
    WxChannelFavoriteService, WxChannelLimitedDiscountService, WxChannelProductAssistantService,
    WxChannelProductStockService, WxTalentService,
};
use wx_rust_channel::config::WxChannelConfig;
use wx_rust_channel::config::r#impl::WxChannelDefaultConfig;
use wx_rust_common::config::WxConfigStorage;

// ═══════════════════════════════════════════════════════════════
// 测试夹具（与 coverage_boost_sub_services.rs 相同模式）
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
        F: Fn(&str, &str) -> String + Send + Sync + 'static,
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
                let lp = last_path_clone.clone();
                let lb = last_body_clone.clone();
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
                    *lb.lock().unwrap() = body.clone();
                    *lp.lock().unwrap() = path.clone();
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
            last_path,
            last_body,
            stop,
        }
    }

    fn url(&self) -> String {
        format!("http://{}", self.addr)
    }

    #[allow(dead_code)]
    fn request_count(&self) -> usize {
        self.requests.load(Ordering::SeqCst)
    }

    #[allow(dead_code)]
    fn last_path(&self) -> String {
        self.last_path.lock().unwrap().clone()
    }

    #[allow(dead_code)]
    fn last_body(&self) -> String {
        self.last_body.lock().unwrap().clone()
    }
}

impl Drop for MockServer {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
    }
}

fn ok_response() -> String {
    r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
}

fn config_with_host(host: &str) -> Arc<dyn WxChannelConfig> {
    let mut config = WxChannelDefaultConfig::new("wxappid", "secret");
    config.set_token("token123");
    config.update_access_token("MOCK_TOKEN", 7200);
    config.set_api_host_url(host);
    Arc::new(config)
}

fn new_service(config: Arc<dyn WxChannelConfig>) -> Arc<WxChannelServiceImpl> {
    WxChannelServiceImpl::new_arc(config)
}

/// 从主服务获取 Weak<dyn WxChannelService> 用于构造子服务。
fn weak_from_service(svc: &Arc<WxChannelServiceImpl>) -> std::sync::Weak<dyn WxChannelService> {
    let dyn_arc: Arc<dyn WxChannelService> = svc.clone();
    Arc::downgrade(&dyn_arc)
}

// ═══════════════════════════════════════════════════════════════
// 1. WxChannelFavoriteService（1 方法）
// ═══════════════════════════════════════════════════════════════

/// get_favorite_count：POST /channels/ec/favorites/count/get
/// 对应 Java: `WxChannelFavoriteServiceImpl.getFavoriteCount`
#[tokio::test]
async fn favorite_get_count() {
    let server = MockServer::start(|path, body| {
        assert!(
            path.contains("/channels/ec/favorites/count/get"),
            "URL 应包含 /channels/ec/favorites/count/get，实际: {path}"
        );
        assert!(body.contains("{}"), "收藏计数请求体应为 {{}}，实际: {body}");
        r#"{"errcode":0,"errmsg":"ok","favorite_count":42}"#.to_string()
    })
    .await;
    let service = new_service(config_with_host(&server.url()));
    let fav_svc = WxChannelFavoriteServiceImpl::new(weak_from_service(&service));

    let resp = fav_svc
        .get_favorite_count()
        .await
        .expect("获取收藏人数成功");
    assert_eq!(resp.err_code, 0);
    assert_eq!(resp.favorite_count, 42);
}

// ═══════════════════════════════════════════════════════════════
// 2. WxChannelLimitedDiscountService（5 方法）
// ═══════════════════════════════════════════════════════════════

/// add_limit_task：POST /channels/ec/product/limiteddiscounttask/add
/// 对应 Java: `WxChannelLimitedDiscountServiceImpl.addLimitTask`
#[tokio::test]
async fn limited_discount_add_task() {
    let server = MockServer::start(|path, _| {
        assert!(
            path.contains("/channels/ec/product/limiteddiscounttask/add"),
            "URL 应包含 limiteddiscounttask/add，实际: {path}"
        );
        r#"{"errcode":0,"errmsg":"ok","task_id":"TASK001"}"#.to_string()
    })
    .await;
    let service = new_service(config_with_host(&server.url()));
    let svc = WxChannelLimitedDiscountServiceImpl::new(weak_from_service(&service));

    let param = wx_rust_channel::bean::limit::LimitTaskParam::default();
    let resp = svc
        .add_limit_task(param)
        .await
        .expect("添加限时抢购任务成功");
    assert_eq!(resp.err_code, 0);
}

/// list_limit_task：POST /channels/ec/product/limiteddiscounttask/list/get
/// 对应 Java: `WxChannelLimitedDiscountServiceImpl.listLimitTask`
#[tokio::test]
async fn limited_discount_list_task() {
    let server = MockServer::start(|path, body| {
        assert!(
            path.contains("/channels/ec/product/limiteddiscounttask/list/get"),
            "URL 应包含 limiteddiscounttask/list/get，实际: {path}"
        );
        assert!(
            body.contains("page_size"),
            "列表请求体应含 page_size，实际: {body}"
        );
        r#"{"errcode":0,"errmsg":"ok","task_list":[]}"#.to_string()
    })
    .await;
    let service = new_service(config_with_host(&server.url()));
    let svc = WxChannelLimitedDiscountServiceImpl::new(weak_from_service(&service));

    let resp = svc
        .list_limit_task(Some(10), "".into(), Some(1))
        .await
        .expect("获取限时抢购列表成功");
    assert_eq!(resp.err_code, 0);
}

/// stop_limit_task：POST /channels/ec/product/limiteddiscounttask/stop
/// 对应 Java: `WxChannelLimitedDiscountServiceImpl.stopLimitTask`
#[tokio::test]
async fn limited_discount_stop_task() {
    let server = MockServer::start(|path, body| {
        assert!(
            path.contains("/channels/ec/product/limiteddiscounttask/stop"),
            "URL 应包含 limiteddiscounttask/stop，实际: {path}"
        );
        assert!(
            body.contains("TASK001"),
            "停止请求体应含 task_id，实际: {body}"
        );
        ok_response()
    })
    .await;
    let service = new_service(config_with_host(&server.url()));
    let svc = WxChannelLimitedDiscountServiceImpl::new(weak_from_service(&service));

    let resp = svc
        .stop_limit_task("TASK001".into())
        .await
        .expect("停止限时抢购任务成功");
    assert_eq!(resp.err_code, 0);
}

/// delete_limit_task：POST /channels/ec/product/limiteddiscounttask/delete
/// 对应 Java: `WxChannelLimitedDiscountServiceImpl.deleteLimitTask`
#[tokio::test]
async fn limited_discount_delete_task() {
    let server = MockServer::start(|path, body| {
        assert!(
            path.contains("/channels/ec/product/limiteddiscounttask/delete"),
            "URL 应包含 limiteddiscounttask/delete，实际: {path}"
        );
        assert!(
            body.contains("TASK001"),
            "删除请求体应含 task_id，实际: {body}"
        );
        ok_response()
    })
    .await;
    let service = new_service(config_with_host(&server.url()));
    let svc = WxChannelLimitedDiscountServiceImpl::new(weak_from_service(&service));

    let resp = svc
        .delete_limit_task("TASK001".into())
        .await
        .expect("删除限时抢购任务成功");
    assert_eq!(resp.err_code, 0);
}

/// update_limit_task：POST /channels/ec/product/limiteddiscounttask/update
/// 对应 Java: `WxChannelLimitedDiscountServiceImpl.updateLimitTask`
#[tokio::test]
async fn limited_discount_update_task() {
    let server = MockServer::start(|path, _| {
        assert!(
            path.contains("/channels/ec/product/limiteddiscounttask/update"),
            "URL 应包含 limiteddiscounttask/update，实际: {path}"
        );
        r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
    })
    .await;
    let service = new_service(config_with_host(&server.url()));
    let svc = WxChannelLimitedDiscountServiceImpl::new(weak_from_service(&service));

    let param = wx_rust_channel::bean::limit::LimitTaskUpdateParam::default();
    let resp = svc
        .update_limit_task(param)
        .await
        .expect("更新限时抢购任务成功");
    assert_eq!(resp.err_code, 0);
}

// ═══════════════════════════════════════════════════════════════
// 3. WxChannelProductAssistantService（6 方法）
// ═══════════════════════════════════════════════════════════════

/// category_pre_check：POST /channels/ec/product/categoryprecheck
/// 对应 Java: `WxChannelProductAssistantServiceImpl.categoryPreCheck`
#[tokio::test]
async fn assistant_category_pre_check() {
    let server = MockServer::start(|path, _| {
        assert!(
            path.contains("/channels/ec/product/categoryprecheck"),
            "URL 应包含 categoryprecheck，实际: {path}"
        );
        r#"{"errcode":0,"errmsg":"ok","qualification_list":[]}"#.to_string()
    })
    .await;
    let service = new_service(config_with_host(&server.url()));
    let svc = WxChannelProductAssistantServiceImpl::new(weak_from_service(&service));

    let param = wx_rust_channel::bean::product::assistant::CategoryPreCheckParam::default();
    let resp = svc.category_pre_check(param).await.expect("发品前校验成功");
    assert_eq!(resp.err_code, 0);
}

/// get_product_brand_recommend：POST /channels/ec/product/productbrandrecommend
/// 对应 Java: `WxChannelProductAssistantServiceImpl.getProductBrandRecommend`
#[tokio::test]
async fn assistant_brand_recommend() {
    let server = MockServer::start(|path, _| {
        assert!(
            path.contains("/channels/ec/product/productbrandrecommend"),
            "URL 应包含 productbrandrecommend，实际: {path}"
        );
        r#"{"errcode":0,"errmsg":"ok","brand_list":[]}"#.to_string()
    })
    .await;
    let service = new_service(config_with_host(&server.url()));
    let svc = WxChannelProductAssistantServiceImpl::new(weak_from_service(&service));

    let param = wx_rust_channel::bean::product::assistant::ProductBrandRecommendParam::default();
    let resp = svc
        .get_product_brand_recommend(param)
        .await
        .expect("获取品牌推荐成功");
    assert_eq!(resp.err_code, 0);
}

/// external_product_mapping：POST /channels/ec/product/externalproductmapping
/// 对应 Java: `WxChannelProductAssistantServiceImpl.externalProductMapping`
#[tokio::test]
async fn assistant_external_mapping() {
    let server = MockServer::start(|path, _| {
        assert!(
            path.contains("/channels/ec/product/externalproductmapping"),
            "URL 应包含 externalproductmapping，实际: {path}"
        );
        r#"{"errcode":0,"errmsg":"ok","mapping_list":[]}"#.to_string()
    })
    .await;
    let service = new_service(config_with_host(&server.url()));
    let svc = WxChannelProductAssistantServiceImpl::new(weak_from_service(&service));

    let param = wx_rust_channel::bean::product::assistant::ExternalProductMappingParam::default();
    let resp = svc
        .external_product_mapping(param)
        .await
        .expect("获取商品属性映射成功");
    assert_eq!(resp.err_code, 0);
}

/// external_product_mapping_new：POST /channels/ec/product/externalproductmappingnew
/// 对应 Java: `WxChannelProductAssistantServiceImpl.externalProductMappingNew`
#[tokio::test]
async fn assistant_external_mapping_new() {
    let server = MockServer::start(|path, _| {
        assert!(
            path.contains("/channels/ec/product/externalproductmappingnew"),
            "URL 应包含 externalproductmappingnew，实际: {path}"
        );
        r#"{"errcode":0,"errmsg":"ok","mapping_list":[]}"#.to_string()
    })
    .await;
    let service = new_service(config_with_host(&server.url()));
    let svc = WxChannelProductAssistantServiceImpl::new(weak_from_service(&service));

    let param =
        wx_rust_channel::bean::product::assistant::ExternalProductMappingNewParam::default();
    let resp = svc
        .external_product_mapping_new(param)
        .await
        .expect("获取商品属性映射及推荐成功");
    assert_eq!(resp.err_code, 0);
}

/// begin_timing_sale：POST /channels/ec/product/begintimingsale
/// 对应 Java: `WxChannelProductAssistantServiceImpl.beginTimingSale`
#[tokio::test]
async fn assistant_begin_timing_sale() {
    let server = MockServer::start(|path, _| {
        assert!(
            path.contains("/channels/ec/product/begintimingsale"),
            "URL 应包含 begintimingsale，实际: {path}"
        );
        ok_response()
    })
    .await;
    let service = new_service(config_with_host(&server.url()));
    let svc = WxChannelProductAssistantServiceImpl::new(weak_from_service(&service));

    let param = wx_rust_channel::bean::product::assistant::BeginTimingSaleParam::default();
    let resp = svc
        .begin_timing_sale(param)
        .await
        .expect("定时开售改立即开售成功");
    assert_eq!(resp.err_code, 0);
}

/// cancel_timing_sale：POST /channels/ec/product/canceltimingsale
/// 对应 Java: `WxChannelProductAssistantServiceImpl.cancelTimingSale`
#[tokio::test]
async fn assistant_cancel_timing_sale() {
    let server = MockServer::start(|path, _| {
        assert!(
            path.contains("/channels/ec/product/canceltimingsale"),
            "URL 应包含 canceltimingsale，实际: {path}"
        );
        ok_response()
    })
    .await;
    let service = new_service(config_with_host(&server.url()));
    let svc = WxChannelProductAssistantServiceImpl::new(weak_from_service(&service));

    let param = wx_rust_channel::bean::product::assistant::CancelTimingSaleParam::default();
    let resp = svc
        .cancel_timing_sale(param)
        .await
        .expect("取消定时开售成功");
    assert_eq!(resp.err_code, 0);
}

// ═══════════════════════════════════════════════════════════════
// 4. WxChannelProductStockService（4 方法）
// ═══════════════════════════════════════════════════════════════

/// update_stock：POST /channels/ec/product/stock/update
/// 对应 Java: `WxChannelProductStockServiceImpl.updateStock`
#[tokio::test]
async fn stock_update() {
    let server = MockServer::start(|path, body| {
        assert!(
            path.contains("/channels/ec/product/stock/update"),
            "URL 应包含 /product/stock/update，实际: {path}"
        );
        assert!(
            body.contains("product_id") && body.contains("diff_type") && body.contains("num"),
            "请求体应含 product_id/diff_type/num，实际: {body}"
        );
        ok_response()
    })
    .await;
    let service = new_service(config_with_host(&server.url()));
    let svc = WxChannelProductStockServiceImpl::new(weak_from_service(&service));

    let resp = svc
        .update_stock("PID".into(), "SID".into(), 1, 10)
        .await
        .expect("更新库存成功");
    assert_eq!(resp.err_code, 0);
}

/// get_sku_stock：POST /channels/ec/product/stock/get（修复前为错误路径 /sku/stock/get）
/// 对应 Java: `WxChannelProductStockServiceImpl.getSkuStock`
#[tokio::test]
async fn stock_get_sku_stock() {
    let server = MockServer::start(|path, body| {
        // 验证修复后的 URL：/channels/ec/product/stock/get（不是 /product/sku/stock/get）
        assert!(
            path.contains("/channels/ec/product/stock/get"),
            "URL 应包含 /product/stock/get（非 /sku/stock/get），实际: {path}"
        );
        assert!(
            !path.contains("/sku/stock/"),
            "URL 不应包含 /sku/stock/（旧错误路径），实际: {path}"
        );
        assert!(
            body.contains("PID") && body.contains("SID"),
            "请求体应含 product_id 和 sku_id，实际: {body}"
        );
        r#"{"errcode":0,"errmsg":"ok","stock":100}"#.to_string()
    })
    .await;
    let service = new_service(config_with_host(&server.url()));
    let svc = WxChannelProductStockServiceImpl::new(weak_from_service(&service));

    let resp = svc
        .get_sku_stock("PID".into(), "SID".into())
        .await
        .expect("获取实时库存成功");
    assert_eq!(resp.err_code, 0);
}

/// get_sku_stock_batch：POST /channels/ec/product/stock/batchget
///   修复 1：URL 从硬编码 /sku/stock/batch/get 改为 /product/stock/batchget
///   修复 2：JSON 字段从 "product_ids"（复数）改为 "product_id"（单数，
///           与 Java SkuStockBatchParam @JsonProperty("product_id") 一致）
/// 对应 Java: `WxChannelProductStockServiceImpl.getSkuStockBatch`
#[tokio::test]
async fn stock_get_sku_stock_batch() {
    let server = MockServer::start(|path, body| {
        // 验证修复后的 URL：/channels/ec/product/stock/batchget
        assert!(
            path.contains("/channels/ec/product/stock/batchget"),
            "URL 应包含 /product/stock/batchget（非 /sku/stock/batch/get），实际: {path}"
        );
        assert!(
            !path.contains("/sku/stock/"),
            "URL 不应包含 /sku/stock/（旧错误路径），实际: {path}"
        );
        // 验证修复后的字段名：product_id（单数，与 Java @JsonProperty 一致）
        assert!(
            body.contains("\"product_id\""),
            "请求体字段应为 product_id（单数），实际: {body}"
        );
        assert!(
            !body.contains("\"product_ids\""),
            "请求体字段不应为 product_ids（复数），实际: {body}"
        );
        r#"{"errcode":0,"errmsg":"ok","stock_list":[]}"#.to_string()
    })
    .await;
    let service = new_service(config_with_host(&server.url()));
    let svc = WxChannelProductStockServiceImpl::new(weak_from_service(&service));

    let resp = svc
        .get_sku_stock_batch(vec!["PID1".into(), "PID2".into()])
        .await
        .expect("批量获取库存成功");
    assert_eq!(resp.err_code, 0);
}

/// get_stock_flow：POST /channels/ec/product/stock/getflow
/// 对应 Java: `WxChannelProductStockServiceImpl.getStockFlow`
#[tokio::test]
async fn stock_get_flow() {
    let server = MockServer::start(|path, _| {
        assert!(
            path.contains("/channels/ec/product/stock/getflow"),
            "URL 应包含 /product/stock/getflow，实际: {path}"
        );
        r#"{"errcode":0,"errmsg":"ok","flow_list":[]}"#.to_string()
    })
    .await;
    let service = new_service(config_with_host(&server.url()));
    let svc = WxChannelProductStockServiceImpl::new(weak_from_service(&service));

    let param = wx_rust_channel::bean::product::stock::StockFlowParam::default();
    let resp = svc.get_stock_flow(param).await.expect("获取库存流水成功");
    assert_eq!(resp.err_code, 0);
}

// ═══════════════════════════════════════════════════════════════
// 5. WxTalentService（4 方法）
// ═══════════════════════════════════════════════════════════════

/// get_order_list：POST /channels/ec/talent/get_order_list
/// 对应 Java: `WxTalentServiceImpl.getOrderList`
#[tokio::test]
async fn talent_get_order_list() {
    let server = MockServer::start(|path, _| {
        assert!(
            path.contains("/channels/ec/talent/get_order_list"),
            "URL 应包含 talent/get_order_list，实际: {path}"
        );
        r#"{"errcode":0,"errmsg":"ok","order_list":[],"total_count":0}"#.to_string()
    })
    .await;
    let service = new_service(config_with_host(&server.url()));
    let svc = WxTalentServiceImpl::new(weak_from_service(&service));

    let param = wx_rust_channel::bean::talent::TalentOrderListParam::default();
    let resp = svc.get_order_list(param).await.expect("获取佣金单列表成功");
    assert_eq!(resp.err_code, 0);
}

/// get_order_detail：POST /channels/ec/talent/get_order_detail
/// 对应 Java: `WxTalentServiceImpl.getOrderDetail`
#[tokio::test]
async fn talent_get_order_detail() {
    let server = MockServer::start(|path, _| {
        assert!(
            path.contains("/channels/ec/talent/get_order_detail"),
            "URL 应包含 talent/get_order_detail，实际: {path}"
        );
        r#"{"errcode":0,"errmsg":"ok","order":{}}"#.to_string()
    })
    .await;
    let service = new_service(config_with_host(&server.url()));
    let svc = WxTalentServiceImpl::new(weak_from_service(&service));

    let param = wx_rust_channel::bean::talent::TalentOrderDetailParam::default();
    let resp = svc
        .get_order_detail(param)
        .await
        .expect("获取佣金单详情成功");
    assert_eq!(resp.err_code, 0);
}

/// get_window_product_list：POST /channels/ec/talent/window/product/list/get
/// 对应 Java: `WxTalentServiceImpl.getWindowProductList`
#[tokio::test]
async fn talent_get_window_product_list() {
    let server = MockServer::start(|path, _| {
        assert!(
            path.contains("/channels/ec/talent/window/product/list/get"),
            "URL 应包含 talent/window/product/list/get，实际: {path}"
        );
        r#"{"errcode":0,"errmsg":"ok","product_list":[],"total_count":0}"#.to_string()
    })
    .await;
    let service = new_service(config_with_host(&server.url()));
    let svc = WxTalentServiceImpl::new(weak_from_service(&service));

    let param = wx_rust_channel::bean::talent::TalentWindowProductListParam::default();
    let resp = svc
        .get_window_product_list(param)
        .await
        .expect("获取橱窗商品列表成功");
    assert_eq!(resp.err_code, 0);
}

/// get_window_product_detail：POST /channels/ec/talent/window/product/get
/// 对应 Java: `WxTalentServiceImpl.getWindowProductDetail`
#[tokio::test]
async fn talent_get_window_product_detail() {
    let server = MockServer::start(|path, _| {
        assert!(
            path.contains("/channels/ec/talent/window/product/get"),
            "URL 应包含 talent/window/product/get，实际: {path}"
        );
        r#"{"errcode":0,"errmsg":"ok","product":{}}"#.to_string()
    })
    .await;
    let service = new_service(config_with_host(&server.url()));
    let svc = WxTalentServiceImpl::new(weak_from_service(&service));

    let param = wx_rust_channel::bean::talent::TalentWindowProductDetailParam::default();
    let resp = svc
        .get_window_product_detail(param)
        .await
        .expect("获取橱窗商品详情成功");
    assert_eq!(resp.err_code, 0);
}
