#![allow(clippy::field_reassign_with_default)]
//! 视频号小店 shop 域子服务集成测试（H2a 批次）。
//!
//! 镜像 Java `WxStoreProductServiceImplTest` / `WxStoreOrderServiceImplTest` /
//! `WxStoreAfterSaleServiceImplTest` / `WxStoreCategoryServiceImplTest` /
//! `WxStoreBrandServiceImplTest` / `WxStoreCouponServiceImplTest` /
//! `WxStoreWarehouseServiceImplTest` / `WxStoreFreightTemplateServiceImplTest` /
//! `WxStoreAddressServiceImplTest` / `WxStoreSharerServiceImplTest` /
//! `WxStoreBasicServiceImplTest` 的 HTTP 语义，经 MockServer 验证。
//!
//! 覆盖：basic/category/brand/product/warehouse/order/after_sale/freight_template/
//! address/coupon/sharer 共 11 域 15 个测试函数，每个测试断言请求路径、
//! 请求体关键字段（serde_json 解析 last_body，键以 bean serde rename +
//! Java impl 手拼 JSON 为准：`product_id` 裸数字、空值跳过等）与响应解析值
//! （响应键 `errcode`/`errmsg` 统一，errcode != 0 由执行引擎上抛）。

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use wx_rust_common::config::WxConfigStorage;
use wx_rust_store::api::r#impl::{
    WxStoreAddressServiceImpl, WxStoreAfterSaleServiceImpl, WxStoreBasicServiceImpl,
    WxStoreBrandServiceImpl, WxStoreCategoryServiceImpl, WxStoreCouponServiceImpl,
    WxStoreFreightTemplateServiceImpl, WxStoreOrderServiceImpl, WxStoreProductServiceImpl,
    WxStoreServiceImpl, WxStoreSharerServiceImpl, WxStoreWarehouseServiceImpl,
};
use wx_rust_store::api::{
    WxStoreAddressService, WxStoreAfterSaleService, WxStoreBasicService, WxStoreBrandService,
    WxStoreCategoryService, WxStoreCouponService, WxStoreFreightTemplateService,
    WxStoreOrderService, WxStoreProductService, WxStoreService, WxStoreSharerService,
    WxStoreWarehouseService,
};
use wx_rust_store::bean::address::{AddressDetail, AddressIdParam};
use wx_rust_store::bean::after::{
    AfterSaleIdParam, AfterSaleMerchantUpdateParam, RefundEvidenceParam,
};
use wx_rust_store::bean::audit::{AuditApplyResponse, CategoryAuditInfo};
use wx_rust_store::bean::base::{AddressInfo, WxStoreBaseResponse};
use wx_rust_store::bean::brand::Brand;
use wx_rust_store::bean::coupon::CouponParam;
use wx_rust_store::bean::delivery::DeliveryInfo;
use wx_rust_store::bean::freight::FreightTemplate;
use wx_rust_store::bean::order::{
    ChangeOrderInfo, OrderListParam, OrderSearchCondition, OrderSearchParam,
};
use wx_rust_store::bean::product::{SkuStockBatchParam, SpuUpdateInfo};
use wx_rust_store::bean::warehouse::{StockGetParam, WarehouseParam};
use wx_rust_store::config::WxStoreConfig;
use wx_rust_store::config::r#impl::WxStoreDefaultConfig;

#[tokio::test]
async fn separate_accounts_refresh_and_send_distinct_tokens() {
    let first = MockServer::start(|path| {
        if path.contains("/token") {
            r#"{"access_token":"first_token","expires_in":7200}"#.into()
        } else {
            r#"{"errcode":0,"shop_info":{}}"#.into()
        }
    })
    .await;
    let second = MockServer::start(|path| {
        if path.contains("/token") {
            r#"{"access_token":"second_token","expires_in":7200}"#.into()
        } else {
            r#"{"errcode":0,"shop_info":{}}"#.into()
        }
    })
    .await;
    let a = WxStoreDefaultConfig::new("first_app", "first_secret");
    a.set_api_host_url(&first.url(""));
    a.set_access_token_url(&first.url("/token?appid=first_app"));
    let b = WxStoreDefaultConfig::new("second_app", "second_secret");
    b.set_api_host_url(&second.url(""));
    b.set_access_token_url(&second.url("/token?appid=second_app"));
    let a = new_service(Arc::new(a));
    let b = new_service(Arc::new(b));
    let (at, bt) = tokio::join!(a.get_access_token(), b.get_access_token());
    assert_eq!(at.unwrap(), "first_token");
    assert_eq!(bt.unwrap(), "second_token");
    assert!(first.last_path().contains("first_app"));
    assert!(second.last_path().contains("second_app"));
    let (ar, br) = tokio::join!(
        a.post(
            "https://api.weixin.qq.com/channels/ec/basics/info/get",
            "{}"
        ),
        b.post(
            "https://api.weixin.qq.com/channels/ec/basics/info/get",
            "{}"
        )
    );
    ar.unwrap();
    br.unwrap();
    assert!(first.last_path().contains("first_token"));
    assert!(second.last_path().contains("second_token"));
    assert_eq!(first.request_count(), 2);
    assert_eq!(second.request_count(), 2);
}

#[tokio::test]
async fn product_facade_gift_stock_uses_real_endpoint() {
    let server = MockServer::start(|_| r#"{"errcode":0,"errmsg":"ok"}"#.into()).await;
    let service = new_service(config_with_host(&server.url("")));
    service
        .product_service()
        .unwrap()
        .update_gift_stock("gift1".into(), "sku1".into(), 1, 7)
        .await
        .unwrap();
    assert!(server.last_path().contains("gift"));
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).unwrap();
    assert_eq!(
        body,
        serde_json::json!({"product_id":"gift1","sku_id":"sku1","diff_type":1,"num":7})
    );
}

/// 极简 mock HTTP 服务器：按请求路径返回固定响应，记录最近一次请求体与请求路径。
struct MockServer {
    addr: std::net::SocketAddr,
    requests: Arc<AtomicUsize>,
    last_body: Arc<std::sync::Mutex<String>>,
    last_path: Arc<std::sync::Mutex<String>>,
    stop: Arc<AtomicBool>,
}

impl MockServer {
    /// 启动服务器（`handler(path) -> body`）。
    async fn start<F>(handler: F) -> Self
    where
        F: Fn(&str) -> String + Send + Sync + 'static,
    {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("绑定端口");
        let addr = listener.local_addr().expect("获取地址");
        let requests = Arc::new(AtomicUsize::new(0));
        let last_body = Arc::new(std::sync::Mutex::new(String::new()));
        let last_path = Arc::new(std::sync::Mutex::new(String::new()));
        let stop = Arc::new(AtomicBool::new(false));
        let handler = Arc::new(handler);

        let requests_clone = requests.clone();
        let last_body_clone = last_body.clone();
        let last_path_clone = last_path.clone();
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
                let last_body_clone = last_body_clone.clone();
                let last_path_clone = last_path_clone.clone();
                tokio::spawn(async move {
                    use tokio::io::{AsyncReadExt, AsyncWriteExt};
                    let mut buf = [0u8; 16384];
                    let n = socket.read(&mut buf).await.unwrap_or(0);
                    let request = String::from_utf8_lossy(&buf[..n]).to_string();
                    if let Some(idx) = request.find("\r\n\r\n") {
                        let body = request[idx + 4..].to_string();
                        *last_body_clone.lock().unwrap() = body;
                    }
                    let path = request
                        .lines()
                        .next()
                        .map(|l| l.split_whitespace().nth(1).unwrap_or("/").to_string())
                        .unwrap_or_else(|| "/".to_string());
                    *last_path_clone.lock().unwrap() = path.clone();
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
            requests,
            last_body,
            last_path,
            stop,
        }
    }

    fn url(&self, path: &str) -> String {
        format!("http://{}{}", self.addr, path)
    }

    #[allow(dead_code)]
    fn request_count(&self) -> usize {
        self.requests.load(Ordering::SeqCst)
    }

    fn last_body(&self) -> String {
        self.last_body.lock().unwrap().clone()
    }

    fn last_path(&self) -> String {
        self.last_path.lock().unwrap().clone()
    }
}

impl Drop for MockServer {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
    }
}

/// 构建指向 mock 服务器的视频号小店配置：
/// 预置 access_token（免 token 请求）+ `api_host_url` 指向 mock 服务器
/// （对应 Java `setApiHostUrl`，执行引擎替换 `https://api.weixin.qq.com` 前缀）。
fn config_with_host(host: &str) -> Arc<dyn WxStoreConfig> {
    let mut config = WxStoreDefaultConfig::new("wxappid", "secret");
    config.set_token("token123");
    config.update_access_token("MOCK_TOKEN", 7200);
    config.set_api_host_url(host);
    Arc::new(config)
}

/// 解析最近一次请求体为 JSON。
fn last_body_json(server: &MockServer) -> serde_json::Value {
    serde_json::from_str(&server.last_body()).expect("请求体 JSON")
}

/// 构建门面服务（配置指向 mock 服务器）。
fn new_service(config: Arc<dyn WxStoreConfig>) -> Arc<WxStoreServiceImpl> {
    WxStoreServiceImpl::new_arc(config)
}

/// 子服务注入弱引用（`Arc<WxStoreServiceImpl>` → `Weak<dyn WxStoreService>`，
/// 对应 Java 子服务构造器 `new WxStoreXxxServiceImpl(this)` 的循环引用）。
fn weak_service(service: &Arc<WxStoreServiceImpl>) -> std::sync::Weak<dyn WxStoreService> {
    let weak: std::sync::Weak<WxStoreServiceImpl> = Arc::downgrade(service);
    weak
}

// ---- product 商品域（镜像 Java WxStoreProductServiceImplTest.testAddProduct / testUpProduct） ----

#[tokio::test]
async fn product_add_and_listing() {
    let server = MockServer::start(|path| {
        if path.contains("/channels/ec/product/add") {
            r#"{"errcode":0,"errmsg":"ok","data":{"product_id":"1001"}}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    })
    .await;
    let service = new_service(config_with_host(&server.url("")));
    let product_service = WxStoreProductServiceImpl::new(weak_service(&service));

    // Java testAddProduct：addProduct(SpuUpdateInfo) → POST SPU_ADD_URL
    let mut info = SpuUpdateInfo::default();
    info.product_id = Some("1001".to_string());
    info.title = Some("测试商品".to_string());
    info.sub_title = Some("子标题".to_string());
    let response = product_service
        .add_product(info)
        .await
        .expect("添加商品成功");
    assert_eq!(response.err_code, 0);
    let body = last_body_json(&server);
    assert_eq!(body["product_id"], "1001");
    assert_eq!(body["title"], "测试商品");

    // Java testUpProduct：upProduct(productId) → `{"product_id":1001}`（裸数字）
    let response = product_service
        .up_product("1001".to_string())
        .await
        .expect("上架商品成功");
    assert_eq!(response.err_code, 0);
    let body = last_body_json(&server);
    assert_eq!(body["product_id"], 1001);
    assert!(body["data_type"].is_null(), "上架请求不应携带 data_type");
}

#[tokio::test]
async fn product_get_detail_stock_and_list() {
    let server = MockServer::start(|path| {
        if path.contains("/channels/ec/product/get") {
            r#"{"errcode":0,"errmsg":"ok","product":{"product_id":"10000029995861","title":"商品"}}"#.to_string()
        } else if path.contains("/channels/ec/product/stock/get") {
            r#"{"errcode":0,"errmsg":"ok","data":{"stock_num":5,"total_stock_num":10}}"#.to_string()
        } else if path.contains("/channels/ec/product/list/get") {
            r#"{"errcode":0,"errmsg":"ok","total_num":1,"next_key":"","spu_list":[{"product_id":"1001"}]}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    })
    .await;
    let service = new_service(config_with_host(&server.url("")));
    let product_service = WxStoreProductServiceImpl::new(weak_service(&service));

    // Java testGetProduct：getProduct("10000029995861", 3) → `{"product_id":10000029995861,"data_type":3}`
    let response = product_service
        .get_product("10000029995861".to_string(), Some(3))
        .await
        .expect("获取商品成功");
    assert_eq!(response.err_code, 0);
    assert_eq!(
        response.product.clone().unwrap().title.clone().unwrap(),
        "商品"
    );
    let body = last_body_json(&server);
    assert_eq!(body["product_id"], 10000029995861_i64);
    assert_eq!(body["data_type"], 3);

    // Java testListProduct：listProduct(10, null, null) → 空值跳过 `{"page_size":10}`
    let _response = product_service
        .list_product(Some(10), String::new(), None)
        .await
        .expect("获取商品列表成功");
    let body = last_body_json(&server);
    assert_eq!(body["page_size"], 10);
    assert!(body.get("next_key").is_none(), "next_key 为空应跳过");
    assert!(body.get("status").is_none(), "status 为空应跳过");

    // Java testGetSkuStock：`{"product_id":"..","sku_id":".."}`（字符串）
    let response = product_service
        .get_sku_stock("10000076089602".to_string(), "1918289111".to_string())
        .await
        .expect("获取库存成功");
    assert_eq!(response.err_code, 0);
    let body = last_body_json(&server);
    assert_eq!(body["product_id"], "10000076089602");
    assert_eq!(body["sku_id"], "1918289111");
}

#[tokio::test]
async fn product_stock_update_batch_and_limit_task() {
    let server = MockServer::start(|path| {
        if path.contains("/channels/ec/product/stock/update") {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        } else if path.contains("/channels/ec/product/stock/batch_get") {
            r#"{"errcode":0,"errmsg":"ok","data":{"stock_list":[]}}"#.to_string()
        } else if path.contains("/channels/ec/product/limit_task/add") {
            r#"{"errcode":0,"errmsg":"ok","task_id":"task1"}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    })
    .await;
    let service = new_service(config_with_host(&server.url("")));
    let product_service = WxStoreProductServiceImpl::new(weak_service(&service));

    // Java testUpdateStock：updateStock(pid, skuId, 1, 10)
    let response = product_service
        .update_stock("1001".to_string(), "sku1".to_string(), Some(1), Some(10))
        .await
        .expect("更新库存成功");
    assert_eq!(response.err_code, 0);
    let body = last_body_json(&server);
    assert_eq!(body["product_id"], "1001");
    assert_eq!(body["sku_id"], "sku1");
    assert_eq!(body["diff_type"], 1);
    assert_eq!(body["num"], 10);

    // Java testGetSkuStockBatch：getSkuStockBatch(["123"]) → key `product_id`
    let param = SkuStockBatchParam {
        product_ids: vec!["123".to_string()].into(),
    };
    let response = product_service
        .get_sku_stock_batch(param.product_ids.clone().unwrap())
        .await
        .expect("批量获取库存成功");
    assert_eq!(response.err_code, 0);
    let body = last_body_json(&server);
    assert_eq!(body["product_id"][0], "123");
}

// ---- order 订单域（镜像 Java WxStoreOrderServiceImplTest.testGetOrder / testGetOrders） ----

#[tokio::test]
async fn order_get_and_list() {
    let server = MockServer::start(|path| {
        if path.contains("/channels/ec/order/get") {
            r#"{"errcode":0,"errmsg":"ok","order":{"order_id":"order123","status":10}}"#.to_string()
        } else if path.contains("/channels/ec/order/list/get") {
            r#"{"errcode":0,"errmsg":"ok","order_id_list":["order123"],"next_key":"","has_more":false}"#.to_string()
        } else if path.contains("/channels/ec/order/search") {
            r#"{"errcode":0,"errmsg":"ok","order_id_list":[],"next_key":""}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    })
    .await;
    let service = new_service(config_with_host(&server.url("")));
    let order_service = WxStoreOrderServiceImpl::new(weak_service(&service));

    // Java testGetOrder：getOrder(orderId) → `{"order_id":".."}`（encode_sensitive_info 空值跳过）
    let response = order_service
        .get_order("order123".to_string())
        .await
        .expect("获取订单成功");
    assert_eq!(response.err_code, 0);
    assert_eq!(
        response.order.clone().unwrap().order_id.clone().unwrap(),
        "order123"
    );
    let body = last_body_json(&server);
    assert_eq!(body["order_id"], "order123");
    assert!(body.get("encode_sensitive_info").is_none());

    // Java testGetOrder（带敏感信息参数）
    let _response = order_service
        .get_order_with_encode("order123".to_string(), Some(true))
        .await
        .expect("获取订单详情成功");
    let body = last_body_json(&server);
    assert_eq!(body["encode_sensitive_info"], true);

    // Java testGetOrders：getOrders(OrderListParam)
    let mut param = OrderListParam::default();
    param.page_size = Some(10);
    param.status = Some(10);
    let response = order_service
        .get_orders(param)
        .await
        .expect("获取订单列表成功");
    assert_eq!(response.ids.clone().unwrap(), vec!["order123"]);
    let body = last_body_json(&server);
    assert_eq!(body["page_size"], 10);
    assert_eq!(body["status"], 10);

    // Java testSearchOrder：searchOrder(OrderSearchParam)
    let mut search = OrderSearchParam::default();
    search.page_size = Some(5);
    search.status = Some(10);
    let mut condition = OrderSearchCondition::default();
    condition.order_id = Some("order123".to_string());
    search.search_condition = Some(condition);
    let response = order_service
        .search_order(search)
        .await
        .expect("搜索订单成功");
    assert_eq!(response.err_code, 0);
    let body = last_body_json(&server);
    assert_eq!(body["search_condition"]["order_id"], "order123");
}

#[tokio::test]
async fn order_price_delivery_and_delivery_company() {
    let server = MockServer::start(|path| {
        if path.contains("/channels/ec/order/price/update")
            || path.contains("/channels/ec/order/delivery/send")
        {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        } else if path.contains("/channels/ec/order/deliverycompany/get") {
            r#"{"errcode":0,"errmsg":"ok","delivery_company_list":[{"delivery_id":"d1","delivery_name":"顺丰"}]}"#.to_string()
        } else if path.contains("/channels/ec/order/deliverycompany/new_get") {
            r#"{"errcode":0,"errmsg":"ok","delivery_company_list":[]}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    })
    .await;
    let service = new_service(config_with_host(&server.url("")));
    let order_service = WxStoreOrderServiceImpl::new(weak_service(&service));

    // Java testUpdatePrice：updatePrice(orderId, 100, list) →
    // `{"order_id":"..","change_express":true,"express_fee":100,"change_order_infos":[...]}`
    let mut change = ChangeOrderInfo::default();
    change.product_id = Some("sku1".to_string());
    change.change_price = Some("99".to_string());
    let response = order_service
        .update_price("o1".to_string(), Some(100), vec![change])
        .await
        .expect("改价成功");
    assert_eq!(response.err_code, 0);
    let body = last_body_json(&server);
    assert_eq!(body["order_id"], "o1");
    assert_eq!(body["change_express"], true);
    assert_eq!(body["express_fee"], 100);
    assert_eq!(body["change_order_infos"][0]["change_price"], "99");

    // Java testDeliveryOrder：deliveryOrder(orderId, deliveryList)
    let mut delivery = DeliveryInfo::default();
    delivery.waybill_id = Some("waybill1".to_string());
    delivery.delivery_id = Some("d1".to_string());
    let response = order_service
        .delivery_order("o1".to_string(), vec![delivery])
        .await
        .expect("发货成功");
    assert_eq!(response.err_code, 0);
    let body = last_body_json(&server);
    assert_eq!(body["order_id"], "o1");
    assert_eq!(body["delivery_list"][0]["waybill_id"], "waybill1");
    assert_eq!(body["delivery_list"][0]["delivery_id"], "d1");

    // Java testListDeliveryCompany：POST "{}"
    let response = order_service
        .list_delivery_company()
        .await
        .expect("获取快递公司列表成功");
    assert_eq!(response.err_code, 0);
    assert_eq!(server.last_body(), "{}");

    // Java listDeliveryCompany(Boolean)：`{"ewaybill_only":true}`
    let _response = order_service
        .list_delivery_company_ewaybill_only(Some(true))
        .await
        .expect("获取快递公司列表成功");
    let body = last_body_json(&server);
    assert_eq!(body["ewaybill_only"], true);
}

// ---- after_sale 售后域（镜像 Java WxStoreAfterSaleServiceImplTest.testListIds / testAccept） ----

#[tokio::test]
async fn after_sale_list_accept_reject_and_reason() {
    let server = MockServer::start(|path| {
        if path.contains("/channels/ec/aftersale/getaftersalelist") {
            r#"{"errcode":0,"errmsg":"ok","after_sale_order_id_list":["as1"],"next_key":""}"#.to_string()
        } else if path.contains("/channels/ec/aftersale/getaftersaleorder") {
            r#"{"errcode":0,"errmsg":"ok","after_sale_order":{"after_sale_order_id":"as1","status":"1"}}"#.to_string()
        } else if path.contains("/channels/ec/aftersale/acceptapply")
            || path.contains("/channels/ec/aftersale/rejectapply")
        {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        } else if path.contains("/channels/ec/aftersale/reason/get") {
            r#"{"errcode":0,"errmsg":"ok","reason_list":[{"reason_type":1,"reason_text":"质量问题"}]}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    })
    .await;
    let service = new_service(config_with_host(&server.url("")));
    let after_sale_service = WxStoreAfterSaleServiceImpl::new(weak_service(&service));

    // Java testListIds：listIds(begin, end, null) → 空值跳过
    let response = after_sale_service
        .list_ids(Some(1690000000), Some(1690000100), String::new())
        .await
        .expect("获取售后单列表成功");
    assert_eq!(response.ids.clone().unwrap(), vec!["as1"]);
    let body = last_body_json(&server);
    assert_eq!(body["begin_create_time"], 1690000000_i64);
    assert_eq!(body["end_create_time"], 1690000100_i64);
    assert!(body.get("next_key").is_none());

    // Java testGet：get(afterSaleOrderId) → AfterSaleIdParam
    let param = AfterSaleIdParam {
        after_sale_order_id: Some("as1".to_string()),
    };
    let response = after_sale_service
        .get_after_sale(param.after_sale_order_id.clone().unwrap())
        .await
        .expect("获取售后单成功");
    assert_eq!(response.err_code, 0);
    assert_eq!(
        response
            .info
            .clone()
            .unwrap()
            .after_sale_order_id
            .clone()
            .unwrap(),
        "as1"
    );

    // Java testAccept：accept(as1, addr1, null) → address_id 有值、accept_type 跳过
    let response = after_sale_service
        .accept("as1".to_string(), "addr1".to_string(), None)
        .await
        .expect("同意售后成功");
    assert_eq!(response.err_code, 0);
    let body = last_body_json(&server);
    assert_eq!(body["after_sale_order_id"], "as1");
    assert_eq!(body["address_id"], "addr1");
    assert!(body.get("accept_type").is_none(), "accept_type 为空应跳过");

    // Java testReject：reject(as1, "拒绝原因", 1) → 无 reject_certificates
    let response = after_sale_service
        .reject("as1".to_string(), "拒绝原因".to_string(), Some(1))
        .await
        .expect("拒绝售后成功");
    assert_eq!(response.err_code, 0);
    let body = last_body_json(&server);
    assert_eq!(body["reject_reason"], "拒绝原因");
    assert_eq!(body["reject_reason_type"], 1);
    assert!(body.get("reject_certificates").is_none());

    // Java testGetAllReason：POST "{}"
    let response = after_sale_service
        .get_all_reason()
        .await
        .expect("获取售后原因成功");
    assert_eq!(
        response.reason_list.clone().unwrap()[0]
            .reason_text
            .clone()
            .unwrap(),
        "质量问题"
    );
    assert_eq!(server.last_body(), "{}");
}

#[tokio::test]
async fn after_sale_evidence_exchange_and_merchant_update() {
    let server = MockServer::start(|_path| r#"{"errcode":0,"errmsg":"ok"}"#.to_string()).await;
    let service = new_service(config_with_host(&server.url("")));
    let after_sale_service = WxStoreAfterSaleServiceImpl::new(weak_service(&service));

    // Java testUploadRefundEvidence：uploadRefundEvidence → RefundEvidenceParam
    let param = RefundEvidenceParam {
        after_sale_order_id: Some("as1".to_string()),
        desc: Some("退款凭证".to_string()),
        certificates: vec!["m1".to_string()].into(),
    };
    let response = after_sale_service
        .upload_refund_evidence(
            param.after_sale_order_id.clone().unwrap(),
            param.desc.clone().unwrap(),
            param.certificates.clone().unwrap(),
        )
        .await
        .expect("上传退款凭证成功");
    assert_eq!(response.err_code, 0);
    let body = last_body_json(&server);
    assert_eq!(body["desc"], "退款凭证");
    assert_eq!(body["refund_certificates"][0], "m1");

    // Java testAcceptExchangeReship：after_sale_order_id/waybill_id/delivery_id
    let response = after_sale_service
        .accept_exchange_reship("as1".to_string(), "waybill1".to_string(), "d1".to_string())
        .await
        .expect("换货发货成功");
    assert_eq!(response.err_code, 0);
    let body = last_body_json(&server);
    assert_eq!(body["waybill_id"], "waybill1");
    assert_eq!(body["delivery_id"], "d1");

    // Java testMerchantUpdateAfterSale：merchant_update_after_sale 参数原样序列化
    let param = AfterSaleMerchantUpdateParam {
        after_sale_order_id: Some("as1".to_string()),
        merchant_update_desc: Some("协商退款".to_string()),
        amount: Some(100),
        merchant_update_type: Some(2),
        ..Default::default()
    };
    let response = after_sale_service
        .merchant_update_after_sale(param)
        .await
        .expect("商家协商成功");
    assert_eq!(response.err_code, 0);
    let body = last_body_json(&server);
    assert_eq!(body["merchant_update_desc"], "协商退款");
    assert_eq!(body["merchant_update_type"], 2);
}

// ---- category 类目域（镜像 Java WxStoreCategoryServiceImplTest.testListAllCategory 等） ----

#[tokio::test]
async fn category_list_detail_and_add() {
    let server = MockServer::start(|path| {
        if path.contains("/shop/ec/category/all") {
            r#"{"errcode":0,"errmsg":"ok","cats":[{"cat_id":1,"name":"一级类目"}]}"#.to_string()
        } else if path.contains("/channels/ec/category/availablesoncategories/get") {
            r#"{"errcode":0,"errmsg":"ok","cat_list":[{"cat_id":"101","name":"子类目"}]}"#
                .to_string()
        } else if path.contains("/channels/ec/category/detail") {
            r#"{"errcode":0,"errmsg":"ok","info":{"cat_id":123,"name":"测试类目"}}"#.to_string()
        } else if path.contains("/channels/ec/category/add") {
            r#"{"errcode":0,"errmsg":"ok","audit_id":"audit1"}"#.to_string()
        } else if path.contains("/channels/ec/category/list/get") {
            r#"{"errcode":0,"errmsg":"ok","cat_list":[]}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    })
    .await;
    let service = new_service(config_with_host(&server.url("")));
    let category_service = WxStoreCategoryServiceImpl::new(weak_service(&service));

    // Java testListAllCategory：GET LIST_ALL_CATEGORY_URL
    let response = category_service
        .list_all_category()
        .await
        .expect("获取所有类目成功");
    assert_eq!(response.err_code, 0);
    assert_eq!(
        response.list.clone().unwrap().len(),
        1,
        "cats 应解析出 1 个类目（含资质信息）"
    );

    // Java testListAvailableCategories：listAvailableCategories("0") → `{"f_cat_id":0}`
    let response = category_service
        .list_available_categories("0".to_string())
        .await
        .expect("获取可用类目成功");
    assert_eq!(response.err_code, 0);
    assert_eq!(
        response.categories.clone().unwrap()[0].id.clone().unwrap(),
        "101"
    );
    let body = last_body_json(&server);
    assert_eq!(body["f_cat_id"], 0);

    // Java testGetCategoryDetail：getCategoryDetail("123") → `{"cat_id":123}`
    let response = category_service
        .get_category_detail("123".to_string())
        .await
        .expect("获取类目详情成功");
    assert_eq!(response.err_code, 0);
    let body = last_body_json(&server);
    assert_eq!(body["cat_id"], 123);

    // Java testAddCategory(CategoryAuditInfo)：category_info 包裹
    let mut info = CategoryAuditInfo::default();
    info.level1 = Some(1);
    info.level2 = Some(2);
    info.level3 = Some(3);
    info.certificates = vec!["m1".to_string()].into();
    let response: AuditApplyResponse = category_service
        .add_category_by_info(info)
        .await
        .expect("添加类目成功");
    assert_eq!(response.err_code, 0);
    assert_eq!(response.audit_id.clone().unwrap(), "audit1");
    let body = last_body_json(&server);
    assert_eq!(body["category_info"]["level1"], 1);
    assert_eq!(body["category_info"]["level2"], 2);
    assert_eq!(body["category_info"]["level3"], 3);
    assert_eq!(body["category_info"]["certificate"][0], "m1");
}

// ---- brand 品牌域（镜像 Java WxStoreBrandServiceImplTest.testAddBrandApply 等） ----

#[tokio::test]
async fn brand_apply_flow() {
    let server = MockServer::start(|path| {
        if path.contains("/shop/ec/brand/all") {
            r#"{"errcode":0,"errmsg":"ok","brands":[{"brand_id":"b1","ch_name":"测试品牌"}],"next_key":""}"#.to_string()
        } else if path.contains("/shop/ec/brand/add") {
            r#"{"errcode":0,"errmsg":"ok","audit_id":"audit1"}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    })
    .await;
    let service = new_service(config_with_host(&server.url("")));
    let brand_service = WxStoreBrandServiceImpl::new(weak_service(&service));

    // Java testListAllBrand：listAllBrand(10, null) → `{"page_size":10}`（next_key 空跳过）
    let response = brand_service
        .list_all_brand(Some(10), String::new())
        .await
        .expect("获取品牌库成功");
    assert_eq!(response.err_code, 0);
    assert_eq!(
        response.brands.clone().unwrap()[0]
            .brand_id
            .clone()
            .unwrap(),
        "b1"
    );
    let body = last_body_json(&server);
    assert_eq!(body["page_size"], 10);
    assert!(body.get("next_key").is_none());

    // Java testAddBrandApply：addBrandApply(brand) → `{"brand":{...}}`
    let mut brand = Brand::default();
    brand.brand_id = Some("b1".to_string());
    brand.ch_name = Some("测试品牌".to_string());
    let response: AuditApplyResponse = brand_service
        .add_brand_apply(brand)
        .await
        .expect("新增品牌成功");
    assert_eq!(response.audit_id.clone().unwrap(), "audit1");
    let body = last_body_json(&server);
    assert_eq!(body["brand"]["brand_id"], "b1");
    assert_eq!(body["brand"]["ch_name"], "测试品牌");

    // Java testCancelBrandApply：`{"brand_id":"b1","audit_id":"audit1"}`
    let response = brand_service
        .cancel_brand_apply("b1".to_string(), "audit1".to_string())
        .await
        .expect("撤回品牌审核成功");
    assert_eq!(response.err_code, 0);
    let body = last_body_json(&server);
    assert_eq!(body["brand_id"], "b1");
    assert_eq!(body["audit_id"], "audit1");
}

// ---- coupon 优惠券域（镜像 Java WxStoreCouponServiceImplTest.testCreateCoupon 等） ----

#[tokio::test]
async fn coupon_create_status_and_get() {
    let server = MockServer::start(|path| {
        if path.contains("/channels/ec/coupon/create") {
            r#"{"errcode":0,"errmsg":"ok","data":{"coupon_id":"c1"}}"#.to_string()
        } else if path.contains("/channels/ec/coupon/update_status") {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        } else if path.contains("/channels/ec/coupon/get") {
            r#"{"errcode":0,"errmsg":"ok","data":{"coupon_id":"c1","name":"满减券","status":2}}"#
                .to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    })
    .await;
    let service = new_service(config_with_host(&server.url("")));
    let coupon_service = WxStoreCouponServiceImpl::new(weak_service(&service));

    // Java testCreateCoupon：createCoupon(CouponParam)
    let mut coupon = CouponParam::default();
    coupon.name = Some("满减券".to_string());
    coupon.r#type = Some(1);
    let response = coupon_service
        .create_coupon(coupon)
        .await
        .expect("创建优惠券成功");
    assert_eq!(response.err_code, 0);
    assert_eq!(
        response.data.clone().unwrap().coupon_id.clone().unwrap(),
        "c1"
    );
    let body = last_body_json(&server);
    assert_eq!(body["name"], "满减券");
    assert_eq!(body["type"], 1);

    // Java testUpdateCouponStatus：updateCouponStatus("c1", 2)
    let response = coupon_service
        .update_coupon_status("c1".to_string(), Some(2))
        .await
        .expect("更新优惠券状态成功");
    assert_eq!(response.err_code, 0);
    let body = last_body_json(&server);
    assert_eq!(body["coupon_id"], "c1");
    assert_eq!(body["status"], 2);

    // Java testGetCoupon：getCoupon("c1") → `{"coupon_id":"c1"}`
    let response = coupon_service
        .get_coupon("c1".to_string())
        .await
        .expect("获取优惠券成功");
    assert_eq!(response.err_code, 0);
    let body = last_body_json(&server);
    assert_eq!(body["coupon_id"], "c1");
}

// ---- warehouse 仓库域（镜像 Java WxStoreWarehouseServiceImplTest.testCreateWarehouse 等） ----

#[tokio::test]
async fn warehouse_crud_and_stock() {
    let server = MockServer::start(|path| {
        if path.contains("/channels/ec/warehouse/create") {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        } else if path.contains("/channels/ec/warehouse/list/get") {
            r#"{"errcode":0,"errmsg":"ok","data":{"out_warehouse_ids":["w1"],"next_key":""}}"#
                .to_string()
        } else if path.contains("/channels/ec/warehouse/get") {
            r#"{"errcode":0,"errmsg":"ok","warehouse":{"out_warehouse_id":"w1","name":"华东仓"}}"#
                .to_string()
        } else if path.contains("/channels/ec/warehouse/stock/get") {
            r#"{"errcode":0,"errmsg":"ok","data":{"num":100}}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    })
    .await;
    let service = new_service(config_with_host(&server.url("")));
    let warehouse_service = WxStoreWarehouseServiceImpl::new(weak_service(&service));

    // Java testCreateWarehouse：createWarehouse(WarehouseParam)
    let param = WarehouseParam {
        out_warehouse_id: Some("w1".to_string()),
        name: Some("华东仓".to_string()),
        intro: Some("覆盖江浙沪".to_string()),
        ..Default::default()
    };
    let response = warehouse_service
        .create_warehouse(param)
        .await
        .expect("创建仓库成功");
    assert_eq!(response.err_code, 0);
    let body = last_body_json(&server);
    assert_eq!(body["out_warehouse_id"], "w1");
    assert_eq!(body["name"], "华东仓");

    // Java testListWarehouse：listWarehouse(10, null) → `{"page_size":10}`
    let response = warehouse_service
        .list_warehouse(Some(10), String::new())
        .await
        .expect("获取仓库列表成功");
    assert_eq!(response.ids.clone().unwrap(), vec!["w1"]);
    let body = last_body_json(&server);
    assert_eq!(body["page_size"], 10);
    assert!(body.get("next_key").is_none());

    // Java testGetWarehouse：`{"out_warehouse_id":"w1"}`
    let response = warehouse_service
        .get_warehouse("w1".to_string())
        .await
        .expect("获取仓库成功");
    assert_eq!(response.err_code, 0);
    let body = last_body_json(&server);
    assert_eq!(body["out_warehouse_id"], "w1");

    // Java testGetWarehouseStock：StockGetParam
    let param = StockGetParam {
        product_id: Some("p1".to_string()),
        sku_id: Some("s1".to_string()),
        out_warehouse_id: Some("w1".to_string()),
    };
    let response = warehouse_service
        .get_warehouse_stock(
            param.product_id.clone().unwrap(),
            param.sku_id.clone().unwrap(),
            param.out_warehouse_id.clone().unwrap(),
        )
        .await
        .expect("获取仓库库存成功");
    assert_eq!(response.err_code, 0);
    let body = last_body_json(&server);
    assert_eq!(body["product_id"], "p1");
    assert_eq!(body["sku_id"], "s1");
    assert_eq!(body["out_warehouse_id"], "w1");
}

// ---- freight_template 运费模板域（镜像 Java WxStoreFreightTemplateServiceImplTest 等） ----

#[tokio::test]
async fn freight_template_add_and_list() {
    let server = MockServer::start(|path| {
        if path.contains("/channels/ec/merchant/getfreighttemplatelist") {
            r#"{"errcode":0,"errmsg":"ok","template_id_list":["t1"]}"#.to_string()
        } else if path.contains("/channels/ec/merchant/getfreighttemplatedetail") {
            r#"{"errcode":0,"errmsg":"ok","freight_template":{"template_id":"t1","name":"模板A"}}"#
                .to_string()
        } else if path.contains("/channels/ec/merchant/addfreighttemplate") {
            r#"{"errcode":0,"errmsg":"ok","template_id":"t1"}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    })
    .await;
    let service = new_service(config_with_host(&server.url("")));
    let freight_service = WxStoreFreightTemplateServiceImpl::new(weak_service(&service));

    // Java testListTemplate：listTemplate(0, 10)
    let response = freight_service
        .list_template(Some(0), Some(10))
        .await
        .expect("获取运费模板列表成功");
    assert_eq!(response.ids.clone().unwrap(), vec!["t1"]);
    let body = last_body_json(&server);
    assert_eq!(body["offset"], 0);
    assert_eq!(body["limit"], 10);

    // Java testGetTemplate：`{"template_id": "t1"}`
    let response = freight_service
        .get_template("t1".to_string())
        .await
        .expect("获取运费模板成功");
    assert_eq!(response.err_code, 0);
    let body = last_body_json(&server);
    assert_eq!(body["template_id"], "t1");

    // Java testAddTemplate：addTemplate(FreightTemplate) → `{"freight_template":{...}}`
    let mut template = FreightTemplate::default();
    template.name = Some("模板A".to_string());
    template.valuation_type = Some("1".to_string());
    let response = freight_service
        .add_template(template)
        .await
        .expect("添加运费模板成功");
    assert_eq!(response.err_code, 0);
    assert_eq!(response.template_id.clone().unwrap(), "t1");
    let body = last_body_json(&server);
    assert_eq!(body["freight_template"]["name"], "模板A");
    assert_eq!(body["freight_template"]["valuation_type"], "1");
}

// ---- address 地址域（镜像 Java WxStoreAddressServiceImplTest.testAddAddress 等） ----

#[tokio::test]
async fn address_crud() {
    let server = MockServer::start(|path| {
        if path.contains("/channels/ec/merchant/address/list") {
            r#"{"errcode":0,"errmsg":"ok","address_id_list":["addr1"]}"#.to_string()
        } else if path.contains("/channels/ec/merchant/address/add") {
            r#"{"errcode":0,"errmsg":"ok","address_id":"addr1"}"#.to_string()
        } else if path.contains("/channels/ec/merchant/address/get") {
            r#"{"errcode":0,"errmsg":"ok","address_detail":{"address_id":"addr1","name":"张三"}}"#
                .to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    })
    .await;
    let service = new_service(config_with_host(&server.url("")));
    let address_service = WxStoreAddressServiceImpl::new(weak_service(&service));

    // Java testListAddress：listAddress(0, 10)
    let response = address_service
        .list_address(Some(0), Some(10))
        .await
        .expect("获取地址列表成功");
    assert_eq!(response.ids.clone().unwrap(), vec!["addr1"]);
    let body = last_body_json(&server);
    assert_eq!(body["offset"], 0);
    assert_eq!(body["limit"], 10);

    // Java testAddAddress：addAddress(AddressDetail) → `{"address_detail":{...}}`
    let mut detail = AddressDetail::default();
    detail.name = Some("张三".to_string());
    let mut address_info = AddressInfo::default();
    address_info.user_name = Some("张三".to_string());
    address_info.tel_number = Some("13800000000".to_string());
    detail.address_info = Some(address_info);
    let response = address_service
        .add_address(detail)
        .await
        .expect("添加地址成功");
    assert_eq!(response.address_id.clone().unwrap(), "addr1");
    let body = last_body_json(&server);
    assert_eq!(body["address_detail"]["name"], "张三");
    assert_eq!(
        body["address_detail"]["address_info"]["tel_number"],
        "13800000000"
    );

    // Java testGetAddress：AddressIdParam
    let param = AddressIdParam {
        address_id: Some("addr1".to_string()),
    };
    let response = address_service
        .get_address(param.address_id.clone().unwrap())
        .await
        .expect("获取地址成功");
    assert_eq!(response.err_code, 0);

    // Java testDeleteAddress：`{"address_id":"addr1"}`
    let response = address_service
        .delete_address("addr1".to_string())
        .await
        .expect("删除地址成功");
    assert_eq!(response.err_code, 0);
    let body = last_body_json(&server);
    assert_eq!(body["address_id"], "addr1");
}

// ---- sharer 分享员域（镜像 Java WxStoreSharerServiceImplTest.testBindSharer 等） ----

#[tokio::test]
async fn sharer_bind_search_and_unbind() {
    let server = MockServer::start(|path| {
        if path.contains("/channels/ec/sharer/bind") {
            r#"{"errcode":0,"errmsg":"ok","qrcode_img_base64":"BASE64"}"#.to_string()
        } else if path.contains("/channels/ec/sharer/search_sharer") {
            r#"{"errcode":0,"errmsg":"ok","sharer_info":{"openid":"o1","nickname":"分享员"}}"#
                .to_string()
        } else if path.contains("/channels/ec/sharer/get_sharer_list") {
            r#"{"errcode":0,"errmsg":"ok","sharer_info_list":[{"openid":"o1"}],"total_num":1}"#
                .to_string()
        } else if path.contains("/channels/ec/sharer/get_sharer_order_list") {
            r#"{"errcode":0,"errmsg":"ok","order_id_list":["order1"],"total_num":1}"#.to_string()
        } else if path.contains("/channels/ec/sharer/unbind") {
            r#"{"errcode":0,"errmsg":"ok","success_openid":["o1"],"fail_openid":[]}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    })
    .await;
    let service = new_service(config_with_host(&server.url("")));
    let sharer_service = WxStoreSharerServiceImpl::new(weak_service(&service));

    // Java testBindSharer：bindSharer(username) → `{"username":".."}`
    let response = sharer_service
        .bind_sharer("wxid_test".to_string())
        .await
        .expect("邀请分享员成功");
    assert_eq!(response.err_code, 0);
    let body = last_body_json(&server);
    assert_eq!(body["username"], "wxid_test");

    // Java testSearchSharer：searchSharer(openid, null) → 空值跳过
    let response = sharer_service
        .search_sharer("o1".to_string(), String::new())
        .await
        .expect("查询分享员成功");
    assert_eq!(response.err_code, 0);
    let body = last_body_json(&server);
    assert_eq!(body["openid"], "o1");
    assert!(body.get("username").is_none());

    // Java testListSharer：listSharer(1, 10, 1)
    let response = sharer_service
        .list_sharer(Some(1), Some(10), Some(1))
        .await
        .expect("获取分享员列表成功");
    assert_eq!(response.err_code, 0);
    assert_eq!(
        response.list.clone().unwrap()[0].openid.clone().unwrap(),
        "o1"
    );
    let body = last_body_json(&server);
    assert_eq!(body["page"], 1);
    assert_eq!(body["page_size"], 10);
    assert_eq!(body["sharer_type"], 1);

    // Java testUnbindSharer：unbindSharer(["o1","o2"]) → key `openid_list`
    let response = sharer_service
        .unbind_sharer(vec!["o1".to_string(), "o2".to_string()])
        .await
        .expect("解绑分享员成功");
    assert_eq!(response.err_code, 0);
    assert_eq!(response.success_list.clone().unwrap(), vec!["o1"]);
    let body = last_body_json(&server);
    assert_eq!(body["openid_list"][0], "o1");
    assert_eq!(body["openid_list"][1], "o2");
}

// ---- basic 基础域（镜像 Java WxStoreBasicServiceImplTest.testGetShopInfo / testUploadImg） ----

#[tokio::test]
async fn basic_shop_info_and_upload_img() {
    let server = MockServer::start(|path| {
        if path.contains("/channels/ec/basics/info/get") {
            r#"{"errcode":0,"errmsg":"ok","info":{"nickname":"测试小店"}}"#.to_string()
        } else if path.contains("/shop/ec/basics/img/upload") {
            r#"{"errcode":0,"errmsg":"ok","pic_file":{"media_id":"media1","img_url":"https://img.example.com/1.jpg"}}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    })
    .await;
    let service = new_service(config_with_host(&server.url("")));
    let basic_service = WxStoreBasicServiceImpl::new(weak_service(&service));

    // Java testGetShopInfo：GET GET_SHOP_INFO
    let response = basic_service
        .get_shop_info()
        .await
        .expect("获取店铺信息成功");
    assert_eq!(response.err_code, 0);
    assert_eq!(
        response.info.clone().unwrap().nickname.clone().unwrap(),
        "测试小店"
    );

    // Java testUploadImg：uploadImg(1, imgUrl) →
    // POST `IMG_UPLOAD_URL?upload_type=1&resp_type=1`，请求体 `{"img_url":".."}`
    let response = basic_service
        .upload_img(1, "https://img.example.com/1.jpg".to_string())
        .await
        .expect("上传图片成功");
    assert_eq!(response.media_id.clone().unwrap(), "media1");
    assert_eq!(
        response.url.clone().unwrap(),
        "https://img.example.com/1.jpg"
    );
    let path = server.last_path();
    assert!(
        path.contains("upload_type=1"),
        "上传类型应为 1，实际: {path}"
    );
    assert!(
        path.contains("resp_type=1"),
        "resp_type 应为 1，实际: {path}"
    );
    let body = last_body_json(&server);
    assert_eq!(body["img_url"], "https://img.example.com/1.jpg");
}

#[tokio::test]
async fn basic_get_address_code_with_null() {
    let server = MockServer::start(|path| {
        if path.contains("/channels/ec/basics/addresscode/get") {
            r#"{"errcode":0,"errmsg":"ok","next_level_addrs":[{"code":440000,"name":"广东省"}]}"#
                .to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    })
    .await;
    let service = new_service(config_with_host(&server.url("")));
    let basic_service = WxStoreBasicServiceImpl::new(weak_service(&service));

    // Java testGetAddressCode：getAddressCode(null) → `{"addr_code": null}`（Java 手拼保留 null）
    let response = basic_service
        .get_address_code(None)
        .await
        .expect("获取地址编码成功");
    assert_eq!(response.err_code, 0);
    assert_eq!(response.list.clone().unwrap()[0].code, Some(440000));
    assert_eq!(server.last_body(), r#"{"addr_code": null}"#);

    // getAddressCode(440000) → `{"addr_code": 440000}`
    let _response = basic_service
        .get_address_code(Some(440000))
        .await
        .expect("获取地址编码成功");
    assert_eq!(server.last_body(), r#"{"addr_code": 440000}"#);
}

// ---- 错误语义（执行引擎 errcode != 0 上抛；Java `WxError.fromJson` 语义） ----

#[tokio::test]
async fn errcode_nonzero_throws() {
    let server =
        MockServer::start(|_path| r#"{"errcode":40001,"errmsg":"invalid credential"}"#.to_string())
            .await;
    let service = new_service(config_with_host(&server.url("")));
    let product_service = WxStoreProductServiceImpl::new(weak_service(&service));

    let err = product_service
        .add_product(SpuUpdateInfo::default())
        .await
        .expect_err("errcode!=0 应报错");
    assert_eq!(err.error_code(), Some(40001));
}

#[tokio::test]
async fn close_order_returns_internal_error() {
    // Java `WxStoreOrderServiceImpl.closeOrder`：暂不支持，返回内部错误
    // （err_code=-99，err_msg="内部错误"），不发请求。
    let server = MockServer::start(|_path| r#"{"errcode":0,"errmsg":"ok"}"#.to_string()).await;
    let service = new_service(config_with_host(&server.url("")));
    let order_service = WxStoreOrderServiceImpl::new(weak_service(&service));

    let response: WxStoreBaseResponse = order_service
        .close_order("order1".to_string())
        .await
        .expect("closeOrder 不抛异常（Java 返回内部错误对象）");
    assert_eq!(response.err_code, -99);
    assert_eq!(response.err_msg, "内部错误");
    assert_eq!(server.request_count(), 0, "closeOrder 不应发起请求");
}

#[tokio::test]
async fn media_and_qualification_uploads_use_multipart_and_public_facade() {
    let server = MockServer::start(|path| {
        if path.contains("cos") {
            r#"{"errcode":0,"cos_url":"https://example.invalid/uploaded"}"#.into()
        } else {
            r#"{"errcode":0,"media_id":"qualification_media"}"#.into()
        }
    })
    .await;
    let service = new_service(config_with_host(&server.url("")));
    let uploaded = service
        .kf_service()
        .unwrap()
        .upload_media(
            "openid".into(),
            "image".into(),
            "fixture.txt".into(),
            b"store-contract-upload".to_vec(),
        )
        .await
        .unwrap();
    assert_eq!(uploaded, "https://example.invalid/uploaded");
    assert!(server.last_path().contains("access_token=MOCK_TOKEN"));
    assert!(server.last_body().contains("openid"));
    assert!(server.last_body().contains("store-contract-upload"));
    let file =
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/upload.txt");
    let result = service
        .basic_service()
        .unwrap()
        .upload_qualification_file(file)
        .await
        .unwrap();
    assert_eq!(result.err_code, 0);
    assert!(server.last_body().contains("store-contract-upload"));
    assert!(server.last_path().contains("access_token=MOCK_TOKEN"));
}

// GENERATED_UPSTREAM_CONTRACTS
#[tokio::test]
async fn contract_wxstoreaddressservicelist_address() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreAddressServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_address_service::WxStoreAddressService;
    let error = service
        .list_address(Default::default(), Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/merchant/address/list"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreaddressserviceget_address() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreAddressServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_address_service::WxStoreAddressService;
    let error = service
        .get_address("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/merchant/address/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreaddressserviceadd_address() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreAddressServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_address_service::WxStoreAddressService;
    let error = service
        .add_address(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/merchant/address/add"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreaddressserviceupdate_address_detail() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreAddressServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_address_service::WxStoreAddressService;
    let error = service
        .update_address_detail(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/merchant/address/update"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreaddressservicedelete_address() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreAddressServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_address_service::WxStoreAddressService;
    let error = service
        .delete_address("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/merchant/address/delete"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreaftersaleservicelist_ids() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreAfterSaleServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_after_sale_service::WxStoreAfterSaleService;
    let error = service
        .list_ids(Default::default(), Default::default(), "17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/aftersale/getaftersalelist"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreaftersaleserviceget_after_sale() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreAfterSaleServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_after_sale_service::WxStoreAfterSaleService;
    let error = service
        .get_after_sale("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/aftersale/getaftersaleorder"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreaftersaleserviceaccept() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreAfterSaleServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_after_sale_service::WxStoreAfterSaleService;
    let error = service
        .accept("17".into(), "17".into(), Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/aftersale/acceptapply"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreaftersaleserviceupload_refund_evidence() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreAfterSaleServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_after_sale_service::WxStoreAfterSaleService;
    let error = service
        .upload_refund_evidence("17".into(), "17".into(), Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/aftersale/uploadrefundcertificate"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreaftersaleserviceadd_complaint_material() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreAfterSaleServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_after_sale_service::WxStoreAfterSaleService;
    let error = service
        .add_complaint_material("17".into(), "17".into(), Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/aftersale/addcomplaintmaterial"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreaftersaleserviceadd_complaint_evidence() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreAfterSaleServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_after_sale_service::WxStoreAfterSaleService;
    let error = service
        .add_complaint_evidence("17".into(), "17".into(), Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/aftersale/addcomplaintproof"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreaftersaleserviceget_complaint() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreAfterSaleServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_after_sale_service::WxStoreAfterSaleService;
    let error = service
        .get_complaint("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/aftersale/getcomplaintorder"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreaftersaleserviceget_all_reason() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreAfterSaleServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_after_sale_service::WxStoreAfterSaleService;
    let error = service
        .get_all_reason()
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/aftersale/reason/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreaftersaleserviceget_reject_reason() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreAfterSaleServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_after_sale_service::WxStoreAfterSaleService;
    let error = service
        .get_reject_reason()
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/aftersale/rejectreason/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreaftersaleserviceaccept_exchange_reship() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreAfterSaleServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_after_sale_service::WxStoreAfterSaleService;
    let error = service
        .accept_exchange_reship("17".into(), "17".into(), "17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/aftersale/acceptexchangereship"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreaftersaleservicereject_exchange_reship() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreAfterSaleServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_after_sale_service::WxStoreAfterSaleService;
    let error = service
        .reject_exchange_reship(
            "17".into(),
            "17".into(),
            Default::default(),
            Default::default(),
        )
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/aftersale/rejectexchangereship"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreaftersaleservicemerchant_update_after_sale() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreAfterSaleServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_after_sale_service::WxStoreAfterSaleService;
    let error = service
        .merchant_update_after_sale(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/aftersale/merchantupdateaftersale"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreaftersaleservicelist_guarantee_order() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreAfterSaleServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_after_sale_service::WxStoreAfterSaleService;
    let error = service
        .list_guarantee_order(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/aftersale/searchguaranteeorder"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreaftersaleserviceget_guarantee_order() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreAfterSaleServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_after_sale_service::WxStoreAfterSaleService;
    let error = service
        .get_guarantee_order("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/aftersale/getguaranteeorder"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreaftersaleserviceaccept_guarantee() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreAfterSaleServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_after_sale_service::WxStoreAfterSaleService;
    let error = service
        .accept_guarantee("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/aftersale/merchantacceptguarantee"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreaftersaleservicemodify_guarantee() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreAfterSaleServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_after_sale_service::WxStoreAfterSaleService;
    let error = service
        .modify_guarantee(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/aftersale/merchantmodifyguarantee"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreaftersaleserviceproof_guarantee() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreAfterSaleServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_after_sale_service::WxStoreAfterSaleService;
    let error = service
        .proof_guarantee(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/aftersale/merchantproofguarantee"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreaftersaleservicerefuse_guarantee() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreAfterSaleServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_after_sale_service::WxStoreAfterSaleService;
    let error = service
        .refuse_guarantee(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/aftersale/merchantrefuseguarantee"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorebasicserviceget_shop_info() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreBasicServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_basic_service::WxStoreBasicService;
    let error = service.get_shop_info().await.expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/basics/info/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorebasicserviceget_shop_h5_url() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreBasicServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_basic_service::WxStoreBasicService;
    let error = service
        .get_shop_h5_url()
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/basics/shop/h5url/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorebasicserviceget_shop_qr_code() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreBasicServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_basic_service::WxStoreBasicService;
    let error = service
        .get_shop_qr_code(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/basics/shop/qrcode/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorebasicserviceget_shop_tag_link() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreBasicServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_basic_service::WxStoreBasicService;
    let error = service
        .get_shop_tag_link()
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/basics/shop/taglink/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorebasicserviceupload_img() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreBasicServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_basic_service::WxStoreBasicService;
    let error = service
        .upload_img(Default::default(), "17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server.last_path().starts_with("/shop/ec/basics/img/upload"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorebasicserviceget_img() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreBasicServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_basic_service::WxStoreBasicService;
    let response = service.get_img("17".into()).await.unwrap();
    assert_eq!(response.err_code, 45009);
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/basics/media/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorebasicserviceget_address_code() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreBasicServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_basic_service::WxStoreBasicService;
    let error = service
        .get_address_code(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/basics/addresscode/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorebrandservicelist_all_brand() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreBrandServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_brand_service::WxStoreBrandService;
    let error = service
        .list_all_brand(Default::default(), "17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server.last_path().starts_with("/shop/ec/brand/all"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorebrandserviceadd_brand_apply() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreBrandServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_brand_service::WxStoreBrandService;
    let error = service
        .add_brand_apply(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server.last_path().starts_with("/shop/ec/brand/add"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorebrandserviceupdate_brand_apply() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreBrandServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_brand_service::WxStoreBrandService;
    let error = service
        .update_brand_apply(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server.last_path().starts_with("/channels/ec/brand/update"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorebrandservicecancel_brand_apply() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreBrandServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_brand_service::WxStoreBrandService;
    let error = service
        .cancel_brand_apply("17".into(), "17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/shop/ec/brand/audit/cancel"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorebrandservicedelete_brand_apply() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreBrandServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_brand_service::WxStoreBrandService;
    let error = service
        .delete_brand_apply("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server.last_path().starts_with("/channels/ec/brand/delete"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorebrandserviceget_brand_apply() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreBrandServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_brand_service::WxStoreBrandService;
    let error = service
        .get_brand_apply("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server.last_path().starts_with("/channels/ec/brand/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorebrandservicelist_brand_apply() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreBrandServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_brand_service::WxStoreBrandService;
    let error = service
        .list_brand_apply(Default::default(), "17".into(), Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/brand/list/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorebrandservicelist_valid_brand_apply() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreBrandServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_brand_service::WxStoreBrandService;
    let error = service
        .list_valid_brand_apply(Default::default(), "17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/brand/valid/list/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorecategoryservicelist_all_category() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreCategoryServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_category_service::WxStoreCategoryService;
    let error = service
        .list_all_category()
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server.last_path().starts_with("/shop/ec/category/all"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorecategoryservicelist_available_categories() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreCategoryServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_category_service::WxStoreCategoryService;
    let error = service
        .list_available_categories("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/category/availablesoncategories/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorecategoryserviceget_category_detail() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreCategoryServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_category_service::WxStoreCategoryService;
    let error = service
        .get_category_detail("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server.last_path().starts_with("/shop/ec/category/detail"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorecategoryserviceadd_category() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreCategoryServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_category_service::WxStoreCategoryService;
    let error = service
        .add_category("17".into(), "17".into(), "17".into(), Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server.last_path().starts_with("/channels/ec/category/add"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorecategoryservicecancel_category_audit() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreCategoryServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_category_service::WxStoreCategoryService;
    let error = service
        .cancel_category_audit("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/shop/ec/category/audit/cancel"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorecategoryserviceget_audit() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreCategoryServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_category_service::WxStoreCategoryService;
    let error = service
        .get_audit("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/category/audit/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorecategoryservicelist_pass_category() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreCategoryServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_category_service::WxStoreCategoryService;
    let error = service
        .list_pass_category()
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/category/list/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorecategoryservicelist_relation_category() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreCategoryServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_category_service::WxStoreCategoryService;
    let error = service
        .list_relation_category(Default::default(), Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/shop/ec/category/get_category_relation_list"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorecompassshopserviceget_shop_overall() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreCompassShopServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_compass_shop_service::WxStoreCompassShopService;
    let error = service
        .get_shop_overall("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/compass/shop/overall/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorecompassshopserviceget_finder_authorization_list() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreCompassShopServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_compass_shop_service::WxStoreCompassShopService;
    let error = service
        .get_finder_authorization_list()
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/compass/shop/finder/authorization/list/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorecompassshopserviceget_finder_list() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreCompassShopServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_compass_shop_service::WxStoreCompassShopService;
    let error = service
        .get_finder_list("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/compass/shop/finder/list/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorecompassshopserviceget_finder_overall() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreCompassShopServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_compass_shop_service::WxStoreCompassShopService;
    let error = service
        .get_finder_overall("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/compass/shop/finder/overall/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorecompassshopserviceget_finder_product_list() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreCompassShopServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_compass_shop_service::WxStoreCompassShopService;
    let error = service
        .get_finder_product_list("17".into(), "17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/compass/shop/finder/product/list/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorecompassshopserviceget_finder_product_overall() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreCompassShopServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_compass_shop_service::WxStoreCompassShopService;
    let error = service
        .get_finder_product_overall("17".into(), "17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/compass/shop/finder/product/overall/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorecompassshopserviceget_shop_live_list() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreCompassShopServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_compass_shop_service::WxStoreCompassShopService;
    let error = service
        .get_shop_live_list("17".into(), "17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/compass/shop/live/list/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorecompassshopserviceget_shop_product_data() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreCompassShopServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_compass_shop_service::WxStoreCompassShopService;
    let error = service
        .get_shop_product_data("17".into(), "17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/compass/shop/product/data/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorecompassshopserviceget_shop_product_list() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreCompassShopServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_compass_shop_service::WxStoreCompassShopService;
    let error = service
        .get_shop_product_list("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/compass/shop/product/list/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorecompassshopserviceget_shop_sale_profile_data() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreCompassShopServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_compass_shop_service::WxStoreCompassShopService;
    let error = service
        .get_shop_sale_profile_data("17".into(), Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/compass/shop/sale/profile/data/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorecooperationservicelist_cooperation() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreCooperationServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_cooperation_service::WxStoreCooperationService;
    let error = service
        .list_cooperation(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/cooperation/list"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorecooperationserviceget_cooperation_status() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreCooperationServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_cooperation_service::WxStoreCooperationService;
    let error = service
        .get_cooperation_status("17".into(), Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/cooperation/invitation/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorecooperationservicegenerate_qr_code() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreCooperationServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_cooperation_service::WxStoreCooperationService;
    let error = service
        .generate_qr_code("17".into(), Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/cooperation/invitation/qrcode/generate"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorecooperationservicecancel_invitation() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreCooperationServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_cooperation_service::WxStoreCooperationService;
    let error = service
        .cancel_invitation("17".into(), Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/cooperation/invitation/cancel"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorecooperationserviceunbind() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreCooperationServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_cooperation_service::WxStoreCooperationService;
    let error = service
        .unbind("17".into(), Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/cooperation/unbind"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorecouponservicecreate_coupon() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreCouponServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_coupon_service::WxStoreCouponService;
    let error = service
        .create_coupon(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server.last_path().starts_with("/channels/ec/coupon/create"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorecouponserviceupdate_coupon() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreCouponServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_coupon_service::WxStoreCouponService;
    let error = service
        .update_coupon(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server.last_path().starts_with("/channels/ec/coupon/update"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorecouponserviceupdate_coupon_status() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreCouponServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_coupon_service::WxStoreCouponService;
    let error = service
        .update_coupon_status("17".into(), Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/coupon/update_status"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorecouponserviceget_coupon() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreCouponServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_coupon_service::WxStoreCouponService;
    let error = service
        .get_coupon("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server.last_path().starts_with("/channels/ec/coupon/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorecouponserviceget_coupon_list() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreCouponServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_coupon_service::WxStoreCouponService;
    let error = service
        .get_coupon_list(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/coupon/get_list"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorecouponserviceget_user_coupon() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreCouponServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_coupon_service::WxStoreCouponService;
    let error = service
        .get_user_coupon("17".into(), "17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/coupon/get_user_coupon"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorecouponserviceget_user_coupon_list() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreCouponServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_coupon_service::WxStoreCouponService;
    let error = service
        .get_user_coupon_list(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/coupon/get_user_coupon_list"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreewaybillserviceget_template_config() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreEwaybillServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_ewaybill_service::WxStoreEwaybillService;
    let error = service
        .get_template_config()
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/logistics/ewaybill/biz/template/config"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreewaybillservicecreate_template() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreEwaybillServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_ewaybill_service::WxStoreEwaybillService;
    let error = service
        .create_template(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/logistics/ewaybill/biz/template/create"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreewaybillservicedelete_template() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreEwaybillServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_ewaybill_service::WxStoreEwaybillService;
    let error = service
        .delete_template("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/logistics/ewaybill/biz/template/delete"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreewaybillserviceupdate_template() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreEwaybillServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_ewaybill_service::WxStoreEwaybillService;
    let error = service
        .update_template(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/logistics/ewaybill/biz/template/update"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreewaybillserviceget_template() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreEwaybillServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_ewaybill_service::WxStoreEwaybillService;
    let error = service
        .get_template("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/logistics/ewaybill/biz/template/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreewaybillserviceget_template_by_id() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreEwaybillServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_ewaybill_service::WxStoreEwaybillService;
    let error = service
        .get_template_by_id("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/logistics/ewaybill/biz/template/getbyid"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreewaybillserviceget_account() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreEwaybillServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_ewaybill_service::WxStoreEwaybillService;
    let error = service.get_account().await.expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/logistics/ewaybill/biz/account/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreewaybillserviceget_delivery_list() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreEwaybillServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_ewaybill_service::WxStoreEwaybillService;
    let error = service
        .get_delivery_list()
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/logistics/ewaybill/biz/delivery/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreewaybillservicepre_create_order() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreEwaybillServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_ewaybill_service::WxStoreEwaybillService;
    let error = service
        .pre_create_order(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/logistics/ewaybill/biz/order/precreate"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreewaybillservicecreate_order() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreEwaybillServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_ewaybill_service::WxStoreEwaybillService;
    let error = service
        .create_order(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/logistics/ewaybill/biz/order/create"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreewaybillserviceadd_sub_order() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreEwaybillServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_ewaybill_service::WxStoreEwaybillService;
    let error = service
        .add_sub_order(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/logistics/ewaybill/biz/order/addsuborder"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreewaybillservicecancel_order() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreEwaybillServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_ewaybill_service::WxStoreEwaybillService;
    let error = service
        .cancel_order(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/logistics/ewaybill/biz/order/cancel"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreewaybillserviceget_order() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreEwaybillServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_ewaybill_service::WxStoreEwaybillService;
    let error = service
        .get_order("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/logistics/ewaybill/biz/order/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreewaybillserviceget_print_content() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreEwaybillServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_ewaybill_service::WxStoreEwaybillService;
    let error = service
        .get_print_content("17".into(), "17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/logistics/ewaybill/biz/print/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreewaybillserviceprint_order() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreEwaybillServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_ewaybill_service::WxStoreEwaybillService;
    let error = service
        .print_order(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/logistics/ewaybill/biz/order/print"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreewaybillservicebatch_print_order() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreEwaybillServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_ewaybill_service::WxStoreEwaybillService;
    let error = service
        .batch_print_order(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/logistics/ewaybill/biz/order/batchprint"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorefavoriteserviceget_favorite_count() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreFavoriteServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_favorite_service::WxStoreFavoriteService;
    let error = service
        .get_favorite_count()
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/favorites/count/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorefreighttemplateservicelist_template() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreFreightTemplateServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_freight_template_service::WxStoreFreightTemplateService;
    let error = service
        .list_template(Default::default(), Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/merchant/getfreighttemplatelist"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorefreighttemplateserviceget_template() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreFreightTemplateServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_freight_template_service::WxStoreFreightTemplateService;
    let error = service
        .get_template("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/merchant/getfreighttemplatedetail"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorefreighttemplateserviceadd_template() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreFreightTemplateServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_freight_template_service::WxStoreFreightTemplateService;
    let error = service
        .add_template(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/merchant/addfreighttemplate"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorefreighttemplateserviceupdate_template() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreFreightTemplateServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_freight_template_service::WxStoreFreightTemplateService;
    let error = service
        .update_template(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/merchant/updatefreighttemplate"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorefundserviceget_balance() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreFundServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_fund_service::WxStoreFundService;
    let error = service.get_balance().await.expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/funds/getbalance"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorefundserviceget_bank_account() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreFundServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_fund_service::WxStoreFundService;
    let error = service
        .get_bank_account()
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/funds/getbankacct"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorefundserviceget_funds_flow_detail() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreFundServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_fund_service::WxStoreFundService;
    let error = service
        .get_funds_flow_detail("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/funds/getfundsflowdetail"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorefundservicelist_funds_flow() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreFundServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_fund_service::WxStoreFundService;
    let error = service
        .list_funds_flow(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/funds/getfundsflowlist"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorefundserviceget_withdraw_detail() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreFundServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_fund_service::WxStoreFundService;
    let error = service
        .get_withdraw_detail("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/funds/getwithdrawdetail"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorefundservicelist_withdraw() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreFundServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_fund_service::WxStoreFundService;
    let error = service
        .list_withdraw(
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
        )
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/funds/getwithdrawlist"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorefundserviceset_bank_account() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreFundServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_fund_service::WxStoreFundService;
    let error = service
        .set_bank_account(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/funds/setbankacct"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorefundservicesubmit_withdraw() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreFundServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_fund_service::WxStoreFundService;
    let error = service
        .submit_withdraw(Default::default(), "17".into(), "17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/funds/submitwithdraw"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorefundserviceget_bank_info_by_card_no() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreFundServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_fund_service::WxStoreFundService;
    let error = service
        .get_bank_info_by_card_no("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server.last_path().starts_with("/shop/funds/getbankbynum"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorefundservicesearch_bank_list() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreFundServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_fund_service::WxStoreFundService;
    let error = service
        .search_bank_list(
            Default::default(),
            Default::default(),
            "17".into(),
            Default::default(),
        )
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server.last_path().starts_with("/shop/funds/getbanklist"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorefundservicesearch_city_list() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreFundServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_fund_service::WxStoreFundService;
    let error = service
        .search_city_list("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server.last_path().starts_with("/shop/funds/getcity"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorefundserviceget_province_list() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreFundServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_fund_service::WxStoreFundService;
    let error = service
        .get_province_list()
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server.last_path().starts_with("/shop/funds/getprovince"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorefundservicesearch_branch_list() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreFundServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_fund_service::WxStoreFundService;
    let error = service
        .search_branch_list(
            "17".into(),
            "17".into(),
            Default::default(),
            Default::default(),
        )
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server.last_path().starts_with("/shop/funds/getsubbranch"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorefundserviceget_qr_code() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreFundServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_fund_service::WxStoreFundService;
    let error = service
        .get_qr_code("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server.last_path().starts_with("/shop/funds/qrcode/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorefundservicecheck_qr_status() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreFundServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_fund_service::WxStoreFundService;
    let error = service
        .check_qr_status("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server.last_path().starts_with("/shop/funds/qrcode/check"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoregiftserviceadd_gift_product() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreGiftServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_gift_service::WxStoreGiftService;
    let error = service
        .add_gift_product(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/product/gift/add"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoregiftserviceupdate_gift_product() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreGiftServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_gift_service::WxStoreGiftService;
    let error = service
        .update_gift_product(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/product/gift/update"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoregiftserviceset_product_as_gift() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreGiftServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_gift_service::WxStoreGiftService;
    let error = service
        .set_product_as_gift("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/product/gift/onsale/set"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoregiftserviceget_gift_product() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreGiftServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_gift_service::WxStoreGiftService;
    let error = service
        .get_gift_product("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/product/gift/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoregiftservicelist_gift_product() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreGiftServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_gift_service::WxStoreGiftService;
    let error = service
        .list_gift_product(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/product/gift/list/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoregiftserviceupdate_gift_stock() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreGiftServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_gift_service::WxStoreGiftService;
    let error = service
        .update_gift_stock(
            "17".into(),
            "17".into(),
            Default::default(),
            Default::default(),
        )
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/product/gift/stock/update"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoregiftserviceadd_gift_activity() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreGiftServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_gift_service::WxStoreGiftService;
    let error = service
        .add_gift_activity(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/product/activity/add"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoregiftservicedelete_gift_activity() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreGiftServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_gift_service::WxStoreGiftService;
    let error = service
        .delete_gift_activity("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/product/activity/del"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoregiftservicestop_gift_activity() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreGiftServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_gift_service::WxStoreGiftService;
    let error = service
        .stop_gift_activity("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/product/activity/stop"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorehomepageserviceadd_tree_product() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreHomePageServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_home_page_service::WxStoreHomePageService;
    let error = service
        .add_tree_product(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/store/classification/tree/product/add"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorehomepageservicedel_tree_product() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreHomePageServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_home_page_service::WxStoreHomePageService;
    let error = service
        .del_tree_product(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/store/classification/tree/product/del"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorehomepageserviceget_tree_product_list() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreHomePageServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_home_page_service::WxStoreHomePageService;
    let error = service
        .get_tree_product_list(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/store/classification/tree/product/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorehomepageserviceset_show_tree() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreHomePageServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_home_page_service::WxStoreHomePageService;
    let error = service
        .set_show_tree(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/store/classification/tree/set"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorehomepageserviceget_show_tree() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreHomePageServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_home_page_service::WxStoreHomePageService;
    let error = service.get_show_tree().await.expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/store/classification/tree/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorehomepageservicelist_window_product() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreHomePageServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_home_page_service::WxStoreHomePageService;
    let error = service
        .list_window_product(Default::default(), "17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/store/window/product/list/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorehomepageservicereorder_window_product() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreHomePageServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_home_page_service::WxStoreHomePageService;
    let error = service
        .reorder_window_product("17".into(), Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/store/window/product/reorder"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorehomepageservicehide_window_product() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreHomePageServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_home_page_service::WxStoreHomePageService;
    let error = service
        .hide_window_product("17".into(), Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/store/window/product/hide"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorehomepageservicetop_window_product() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreHomePageServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_home_page_service::WxStoreHomePageService;
    let error = service
        .top_window_product("17".into(), Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/store/window/product/settop"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorehomepageserviceapply_background() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreHomePageServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_home_page_service::WxStoreHomePageService;
    let error = service
        .apply_background("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/basics/homepage/background/apply/submit"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorehomepageserviceget_background() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreHomePageServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_home_page_service::WxStoreHomePageService;
    let error = service
        .get_background()
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/basics/homepage/background/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorehomepageservicecancel_background() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreHomePageServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_home_page_service::WxStoreHomePageService;
    let error = service
        .cancel_background(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/basics/homepage/background/apply/cancel"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorehomepageserviceremove_background() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreHomePageServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_home_page_service::WxStoreHomePageService;
    let error = service
        .remove_background()
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/basics/homepage/background/remove"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorehomepageserviceapply_banner() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreHomePageServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_home_page_service::WxStoreHomePageService;
    let error = service
        .apply_banner(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/basics/homepage/banner/apply/submit"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorehomepageserviceget_banner() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreHomePageServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_home_page_service::WxStoreHomePageService;
    let error = service.get_banner().await.expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/basics/homepage/banner/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorehomepageservicecancel_banner() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreHomePageServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_home_page_service::WxStoreHomePageService;
    let error = service
        .cancel_banner(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/basics/homepage/banner/apply/cancel"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorehomepageserviceremove_banner() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreHomePageServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_home_page_service::WxStoreHomePageService;
    let error = service.remove_banner().await.expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/basics/homepage/banner/remove"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorekfservicesend_message() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreKfServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_kf_service::WxStoreKfService;
    let error = service
        .send_message(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/commkf/sendmsg"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorelimiteddiscountserviceadd_limit_task() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreLimitedDiscountServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_limited_discount_service::WxStoreLimitedDiscountService;
    let error = service
        .add_limit_task(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/product/limiteddiscounttask/add"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorelimiteddiscountservicelist_limit_task() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreLimitedDiscountServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_limited_discount_service::WxStoreLimitedDiscountService;
    let error = service
        .list_limit_task(Default::default(), "17".into(), Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/product/limiteddiscounttask/list/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorelimiteddiscountservicestop_limit_task() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreLimitedDiscountServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_limited_discount_service::WxStoreLimitedDiscountService;
    let error = service
        .stop_limit_task("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/product/limiteddiscounttask/stop"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorelimiteddiscountservicedelete_limit_task() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreLimitedDiscountServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_limited_discount_service::WxStoreLimitedDiscountService;
    let error = service
        .delete_limit_task("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/product/limiteddiscounttask/delete"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorelimiteddiscountserviceupdate_limit_task() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreLimitedDiscountServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_limited_discount_service::WxStoreLimitedDiscountService;
    let error = service
        .update_limit_task(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/product/limiteddiscounttask/update"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreorderserviceget_order() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreOrderServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_order_service::WxStoreOrderService;
    let error = service
        .get_order("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server.last_path().starts_with("/channels/ec/order/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreorderserviceget_orders() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreOrderServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_order_service::WxStoreOrderService;
    let error = service
        .get_orders(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/order/list/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreorderservicesearch_order() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreOrderServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_order_service::WxStoreOrderService;
    let error = service
        .search_order(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server.last_path().starts_with("/channels/ec/order/search"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreorderserviceupdate_price() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreOrderServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_order_service::WxStoreOrderService;
    let error = service
        .update_price("17".into(), Default::default(), Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/order/price/update"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreorderserviceupdate_remark() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreOrderServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_order_service::WxStoreOrderService;
    let error = service
        .update_remark("17".into(), "17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/order/merchantnotes/update"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreorderserviceupdate_order_address() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreOrderServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_order_service::WxStoreOrderService;
    let error = service
        .update_order_address("17".into(), Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/order/address/update"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreorderserviceupdate_delivery() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreOrderServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_order_service::WxStoreOrderService;
    let error = service
        .update_delivery(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/order/deliveryinfo/update"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreorderserviceaccept_address_modify() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreOrderServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_order_service::WxStoreOrderService;
    let error = service
        .accept_address_modify("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/order/addressmodify/accept"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreorderservicereject_address_modify() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreOrderServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_order_service::WxStoreOrderService;
    let error = service
        .reject_address_modify("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/order/addressmodify/reject"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreorderservicelist_delivery_company() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreOrderServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_order_service::WxStoreOrderService;
    let error = service
        .list_delivery_company()
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/order/deliverycompanylist/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreorderservicedelivery_order() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreOrderServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_order_service::WxStoreOrderService;
    let error = service
        .delivery_order("17".into(), Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/order/delivery/send"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreorderserviceupload_fresh_inspect() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreOrderServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_order_service::WxStoreOrderService;
    let error = service
        .upload_fresh_inspect("17".into(), Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/order/freshinspect/submit"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreorderserviceget_virtual_tel_number() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreOrderServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_order_service::WxStoreOrderService;
    let error = service
        .get_virtual_tel_number("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/order/virtualtelnumber/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreorderservicedecode_sensitive_info() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreOrderServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_order_service::WxStoreOrderService;
    let error = service
        .decode_sensitive_info("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/order/sensitiveinfo/decode"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreorderserviceadd_present_note() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreOrderServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_order_service::WxStoreOrderService;
    let error = service
        .add_present_note("17".into(), "17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/order/presentnote/add"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreorderserviceget_present_sub_orders() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreOrderServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_order_service::WxStoreOrderService;
    let error = service
        .get_present_sub_orders("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/order/presentsuborder/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreorderserviceget_pre_shipment_change_sku() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreOrderServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_order_service::WxStoreOrderService;
    let error = service
        .get_pre_shipment_change_sku("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/order/preshipmentchangesku/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreorderserviceapprove_pre_shipment_change_sku() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreOrderServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_order_service::WxStoreOrderService;
    let error = service
        .approve_pre_shipment_change_sku("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/order/preshipmentchangesku/approve"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreorderservicereject_pre_shipment_change_sku() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreOrderServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_order_service::WxStoreOrderService;
    let error = service
        .reject_pre_shipment_change_sku("17".into(), "17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/order/preshipmentchangesku/reject"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreorderserviceapply_real_number() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreOrderServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_order_service::WxStoreOrderService;
    let error = service
        .apply_real_number("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/order/realnumber/apply"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreorderserviceget_real_number_view_audit() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreOrderServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_order_service::WxStoreOrderService;
    let error = service
        .get_real_number_view_audit("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/order/realnumberviewaudit/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreorderserviceapply_virtual_number_again() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreOrderServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_order_service::WxStoreOrderService;
    let error = service
        .apply_virtual_number_again("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/order/virtualnumber/applyagain"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreorderservicedelay_virtual_number() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreOrderServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_order_service::WxStoreOrderService;
    let error = service
        .delay_virtual_number("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/order/virtualnumber/delay"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreorderserviceadd_private_phone() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreOrderServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_order_service::WxStoreOrderService;
    let error = service
        .add_private_phone("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/merchant/privatenumber/addphone"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreorderservicesend_private_phone_verify_code() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreOrderServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_order_service::WxStoreOrderService;
    let error = service
        .send_private_phone_verify_code("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/merchant/privatenumber/sendverifycode"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreorderserviceget_private_phone() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreOrderServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_order_service::WxStoreOrderService;
    let error = service
        .get_private_phone()
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/merchant/privatenumber/getphone"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreorderservicecompensation_delivery() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreOrderServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_order_service::WxStoreOrderService;
    let error = service
        .compensation_delivery(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/order/delivery/compensation"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreproductassistantservicecategory_pre_check() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreProductAssistantServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_product_assistant_service::WxStoreProductAssistantService;
    let error = service
        .category_pre_check(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/product/categoryprecheck"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreproductassistantserviceget_product_brand_recommend() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreProductAssistantServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_product_assistant_service::WxStoreProductAssistantService;
    let error = service
        .get_product_brand_recommend(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/product/productbrandrecommend"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreproductassistantserviceexternal_product_mapping() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreProductAssistantServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_product_assistant_service::WxStoreProductAssistantService;
    let error = service
        .external_product_mapping(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/product/externalproductmapping"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreproductassistantserviceexternal_product_mapping_new() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreProductAssistantServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_product_assistant_service::WxStoreProductAssistantService;
    let error = service
        .external_product_mapping_new(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/product/externalproductmappingnew"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreproductassistantservicebegin_timing_sale() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreProductAssistantServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_product_assistant_service::WxStoreProductAssistantService;
    let error = service
        .begin_timing_sale(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/product/begintimingsale"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreproductassistantservicecancel_timing_sale() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreProductAssistantServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_product_assistant_service::WxStoreProductAssistantService;
    let error = service
        .cancel_timing_sale(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/product/canceltimingsale"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreproductserviceadd_product() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreProductServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_product_service::WxStoreProductService;
    let error = service
        .add_product(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server.last_path().starts_with("/channels/ec/product/add"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreproductserviceupdate_product() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreProductServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_product_service::WxStoreProductService;
    let error = service
        .update_product(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/product/update"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreproductserviceupdate_product_audit_free() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreProductServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_product_service::WxStoreProductService;
    let error = service
        .update_product_audit_free(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/product/auditfree"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreproductservicedelete_product() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreProductServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_product_service::WxStoreProductService;
    let error = service
        .delete_product("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/product/delete"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreproductservicecancel_product_audit() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreProductServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_product_service::WxStoreProductService;
    let error = service
        .cancel_product_audit("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/product/audit/cancel"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreproductserviceget_product() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreProductServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_product_service::WxStoreProductService;
    let error = service
        .get_product("17".into(), Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server.last_path().starts_with("/channels/ec/product/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreproductservicelist_product() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreProductServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_product_service::WxStoreProductService;
    let error = service
        .list_product(Default::default(), "17".into(), Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/product/list/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreproductserviceup_product() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreProductServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_product_service::WxStoreProductService;
    let error = service
        .up_product("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/product/listing"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreproductservicedown_product() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreProductServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_product_service::WxStoreProductService;
    let error = service
        .down_product("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/product/delisting"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreproductserviceget_product_h5_url() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreProductServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_product_service::WxStoreProductService;
    let error = service
        .get_product_h5_url("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/product/h5url/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreproductserviceget_product_qr_code() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreProductServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_product_service::WxStoreProductService;
    let error = service
        .get_product_qr_code("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/product/qrcode/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreproductserviceget_product_tag_link() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreProductServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_product_service::WxStoreProductService;
    let error = service
        .get_product_tag_link("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/product/taglink/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreproductserviceget_product_scheme() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreProductServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_product_service::WxStoreProductService;
    let error = service
        .get_product_scheme(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/product/scheme/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreproductserviceclassify_product_category() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreProductServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_product_service::WxStoreProductService;
    let error = service
        .classify_product_category(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/product/category/classify"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreproductservicebegin_timing_sale() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreProductServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_product_service::WxStoreProductService;
    let error = service
        .begin_timing_sale(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/product/begintimingsale"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreproductservicecancel_timing_sale() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreProductServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_product_service::WxStoreProductService;
    let error = service
        .cancel_timing_sale("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/product/canceltimingsale"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreproductserviceexternal_product_mapping() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreProductServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_product_service::WxStoreProductService;
    let error = service
        .external_product_mapping(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/product/externalproductmapping"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreproductservicecategory_pre_check() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreProductServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_product_service::WxStoreProductService;
    let error = service
        .category_pre_check(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/product/categoryprecheck"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreproductserviceget_product_audit_strategy() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreProductServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_product_service::WxStoreProductService;
    let error = service
        .get_product_audit_strategy()
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/product/auditstrategy/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreproductserviceset_product_audit_strategy() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreProductServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_product_service::WxStoreProductService;
    let error = service
        .set_product_audit_strategy(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/product/auditstrategy/set"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreproductserviceget_product_audit_quota() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreProductServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_product_service::WxStoreProductService;
    let error = service
        .get_product_audit_quota()
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/product/getauditquota"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreproductserviceexternal_product_mapping_new() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreProductServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_product_service::WxStoreProductService;
    let error = service
        .external_product_mapping_new(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/product/externalproductmappingnew"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreproductserviceproduct_brand_recommend() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreProductServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_product_service::WxStoreProductService;
    let error = service
        .product_brand_recommend(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/product/productbrandrecommend"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreproductserviceadd_product_third_party_source() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreProductServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_product_service::WxStoreProductService;
    let error = service
        .add_product_third_party_source(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/product/addproductthirdpartysource"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreproductserviceget_stock_flow() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreProductServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_product_service::WxStoreProductService;
    let error = service
        .get_stock_flow(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/product/stock/getflow"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreproductserviceadd_gift_product() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreProductServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_product_service::WxStoreProductService;
    let error = service
        .add_gift_product(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/product/gift/add"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreproductserviceupdate_gift_product() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreProductServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_product_service::WxStoreProductService;
    let error = service
        .update_gift_product(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/product/gift/update"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreproductservicelist_gift_product() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreProductServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_product_service::WxStoreProductService;
    let error = service
        .list_gift_product(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/product/gift/list/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreproductserviceupdate_gift_stock() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreProductServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_product_service::WxStoreProductService;
    let error = service
        .update_gift_stock(
            "17".into(),
            "17".into(),
            Default::default(),
            Default::default(),
        )
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/product/gift/stock/update"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreproductserviceadd_gift_activity() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreProductServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_product_service::WxStoreProductService;
    let error = service
        .add_gift_activity(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/product/activity/add"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreproductserviceadd_limit_task() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreProductServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_product_service::WxStoreProductService;
    let error = service
        .add_limit_task(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/product/limiteddiscounttask/add"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreproductservicelist_limit_task() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreProductServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_product_service::WxStoreProductService;
    let error = service
        .list_limit_task(Default::default(), "17".into(), Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/product/limiteddiscounttask/list/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreproductstockserviceupdate_stock() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreProductStockServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_product_stock_service::WxStoreProductStockService;
    let error = service
        .update_stock(
            "17".into(),
            "17".into(),
            Default::default(),
            Default::default(),
        )
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/product/stock/update"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreproductstockserviceget_sku_stock() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreProductStockServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_product_stock_service::WxStoreProductStockService;
    let error = service
        .get_sku_stock("17".into(), "17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/product/stock/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreproductstockserviceget_sku_stock_batch() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreProductStockServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_product_stock_service::WxStoreProductStockService;
    let error = service
        .get_sku_stock_batch(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/product/stock/batchget"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreproductstockserviceget_stock_flow() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreProductStockServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_product_stock_service::WxStoreProductStockService;
    let error = service
        .get_stock_flow(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/product/stock/getflow"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreqicserviceget_inspect_config() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreQicServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_qic_service::WxStoreQicService;
    let error = service
        .get_inspect_config()
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/qic/inspect/config/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreqicserviceget_submit_config() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreQicServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_qic_service::WxStoreQicService;
    let error = service
        .get_submit_config()
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/qic/inspect/submitconfig/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreqicserviceprint_inspect_code() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreQicServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_qic_service::WxStoreQicService;
    let error = service
        .print_inspect_code("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/qic/inspect/code/print"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreqicservicesubmit_inspect_info() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreQicServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_qic_service::WxStoreQicService;
    let error = service
        .submit_inspect_info(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/qic/inspect/submit"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoreqicserviceregister_logistics() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreQicServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_qic_service::WxStoreQicService;
    let error = service
        .register_logistics(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/qic/inspect/register_logistics"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoresharerservicebind_sharer() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreSharerServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_sharer_service::WxStoreSharerService;
    let error = service
        .bind_sharer("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server.last_path().starts_with("/channels/ec/sharer/bind"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoresharerservicesearch_sharer() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreSharerServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_sharer_service::WxStoreSharerService;
    let error = service
        .search_sharer("17".into(), "17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/sharer/search_sharer"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoresharerservicelist_sharer() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreSharerServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_sharer_service::WxStoreSharerService;
    let error = service
        .list_sharer(Default::default(), Default::default(), Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/sharer/get_sharer_list"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoresharerservicelist_sharer_order() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreSharerServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_sharer_service::WxStoreSharerService;
    let error = service
        .list_sharer_order(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/sharer/get_sharer_order_list"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoresharerserviceunbind_sharer() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreSharerServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_sharer_service::WxStoreSharerService;
    let error = service
        .unbind_sharer(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server.last_path().starts_with("/channels/ec/sharer/unbind"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoresupplierserviceget_distribute() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreSupplierServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_supplier_service::WxStoreSupplierService;
    let error = service
        .get_distribute()
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/supplier/relation/get_distribute"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoresupplierserviceset_manually_distribute() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreSupplierServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_supplier_service::WxStoreSupplierService;
    let error = service
        .set_manually_distribute()
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/supplier/relation/set_manually_distribute"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoresupplierserviceset_all_distribute() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreSupplierServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_supplier_service::WxStoreSupplierService;
    let error = service
        .set_all_distribute("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/supplier/relation/set_all_distribution"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoresupplierserviceset_product_distribute() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreSupplierServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_supplier_service::WxStoreSupplierService;
    let error = service
        .set_product_distribute(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/supplier/relation/set_product_distribute"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoresupplierserviceget_product_default_distribute() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreSupplierServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_supplier_service::WxStoreSupplierService;
    let error = service
        .get_product_default_distribute("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/supplier/relation/get_product_default_distribute"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoresupplierserviceget_product_list() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreSupplierServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_supplier_service::WxStoreSupplierService;
    let error = service
        .get_product_list("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/supplier/relation/get_product_list"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoresupplierserviceassign_order() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreSupplierServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_supplier_service::WxStoreSupplierService;
    let error = service
        .assign_order(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/order/dropship/assign"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoresupplierservicecancel_dropship() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreSupplierServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_supplier_service::WxStoreSupplierService;
    let error = service
        .cancel_dropship("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/order/dropship/cancel"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoresupplierserviceget_dropship() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreSupplierServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_supplier_service::WxStoreSupplierService;
    let error = service
        .get_dropship("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/order/dropship/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoresupplierservicelist_dropship() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreSupplierServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_supplier_service::WxStoreSupplierService;
    let error = service
        .list_dropship(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/order/dropship/list"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstoresupplierservicesearch_dropship() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreSupplierServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_supplier_service::WxStoreSupplierService;
    let error = service
        .search_dropship(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/order/dropship/search"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorevipserviceget_vip_info() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreVipServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_vip_service::WxStoreVipService;
    let error = service
        .get_vip_info("17".into(), Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/vip/user/info/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorevipserviceget_vip_list() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreVipServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_vip_service::WxStoreVipService;
    let error = service
        .get_vip_list(Default::default(), Default::default(), Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/vip/user/list/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorevipserviceget_vip_score() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreVipServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_vip_service::WxStoreVipService;
    let error = service
        .get_vip_score("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/vip/user/score/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorevipserviceincrease_vip_score() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreVipServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_vip_service::WxStoreVipService;
    let error = service
        .increase_vip_score("17".into(), "17".into(), "17".into(), "17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/vip/user/score/increase"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorevipservicedecrease_vip_score() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreVipServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_vip_service::WxStoreVipService;
    let error = service
        .decrease_vip_score("17".into(), "17".into(), "17".into(), "17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/vip/user/score/decrease"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorevipserviceupdate_vip_grade() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxStoreVipServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_vip_service::WxStoreVipService;
    let error = service
        .update_vip_grade("17".into(), Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/vip/user/grade/update"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorewarehouseservicecreate_warehouse() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreWarehouseServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_warehouse_service::WxStoreWarehouseService;
    let error = service
        .create_warehouse(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/warehouse/create"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorewarehouseservicelist_warehouse() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreWarehouseServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_warehouse_service::WxStoreWarehouseService;
    let error = service
        .list_warehouse(Default::default(), "17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/warehouse/list/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorewarehouseserviceget_warehouse() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreWarehouseServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_warehouse_service::WxStoreWarehouseService;
    let error = service
        .get_warehouse("17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server.last_path().starts_with("/channels/ec/warehouse/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorewarehouseserviceupdate_warehouse() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreWarehouseServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_warehouse_service::WxStoreWarehouseService;
    let error = service
        .update_warehouse("17".into(), "17".into(), "17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/warehouse/detail/update"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorewarehouseserviceadd_warehouse_area() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreWarehouseServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_warehouse_service::WxStoreWarehouseService;
    let error = service
        .add_warehouse_area("17".into(), Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/warehouse/coverlocations/add"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorewarehouseservicedelete_warehouse_area() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreWarehouseServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_warehouse_service::WxStoreWarehouseService;
    let error = service
        .delete_warehouse_area("17".into(), Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/warehouse/coverlocations/del"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorewarehouseserviceset_warehouse_priority() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreWarehouseServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_warehouse_service::WxStoreWarehouseService;
    let error = service
        .set_warehouse_priority(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/warehouse/address/prioritysort/set"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorewarehouseserviceget_warehouse_priority() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreWarehouseServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_warehouse_service::WxStoreWarehouseService;
    let error = service
        .get_warehouse_priority(
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
        )
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/warehouse/address/prioritysort/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorewarehouseserviceupdate_warehouse_stock() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreWarehouseServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_warehouse_service::WxStoreWarehouseService;
    let error = service
        .update_warehouse_stock(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/warehouse/stock/update"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxstorewarehouseserviceget_warehouse_stock() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service =
        wx_rust_store::api::r#impl::WxStoreWarehouseServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_store_warehouse_service::WxStoreWarehouseService;
    let error = service
        .get_warehouse_stock("17".into(), "17".into(), "17".into())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/warehouse/stock/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxtalentserviceget_order_list() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxTalentServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_talent_service::WxTalentService;
    let error = service
        .get_order_list(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/talent/get_order_list"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxtalentserviceget_order_detail() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxTalentServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_talent_service::WxTalentService;
    let error = service
        .get_order_detail(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/talent/get_order_detail"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxtalentserviceget_window_product_list() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxTalentServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_talent_service::WxTalentService;
    let error = service
        .get_window_product_list(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/talent/window/product/list/get"),
        "{}",
        server.last_path()
    );
}
#[tokio::test]
async fn contract_wxtalentserviceget_window_product_detail() {
    let server =
        MockServer::start(|_| r#"{"errcode":45009,"errmsg":"upstream-contract"}"#.into()).await;
    let facade = new_service(config_with_host(&server.url("")));
    let service = wx_rust_store::api::r#impl::WxTalentServiceImpl::new(weak_service(&facade));
    use wx_rust_store::api::wx_talent_service::WxTalentService;
    let error = service
        .get_window_product_detail(Default::default())
        .await
        .expect_err("必须保留业务错误");
    assert_eq!(error.error_code(), Some(45009));
    assert!(
        server
            .last_path()
            .starts_with("/channels/ec/talent/window/product/get"),
        "{}",
        server.last_path()
    );
}
