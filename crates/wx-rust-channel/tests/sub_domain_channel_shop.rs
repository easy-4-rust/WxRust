#![allow(clippy::field_reassign_with_default)]
//! 视频号小店 shop 域子服务集成测试（H2a 批次）。
//!
//! 镜像 Java `WxChannelProductServiceImplTest` / `WxChannelOrderServiceImplTest` /
//! `WxChannelAfterSaleServiceImplTest` / `WxChannelCategoryServiceImplTest` /
//! `WxChannelBrandServiceImplTest` / `WxChannelCouponServiceImplTest` /
//! `WxChannelWarehouseServiceImplTest` / `WxChannelFreightTemplateServiceImplTest` /
//! `WxChannelAddressServiceImplTest` / `WxChannelSharerServiceImplTest` /
//! `WxChannelBasicServiceImplTest` 的 HTTP 语义，经 MockServer 验证。
//!
//! 覆盖：basic/category/brand/product/warehouse/order/after_sale/freight_template/
//! address/coupon/sharer 共 11 域 15 个测试函数，每个测试断言请求路径、
//! 请求体关键字段（serde_json 解析 last_body，键以 bean serde rename +
//! Java impl 手拼 JSON 为准：`product_id` 裸数字、空值跳过等）与响应解析值
//! （响应键 `errcode`/`errmsg` 统一，errcode != 0 由执行引擎上抛）。

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use wx_rust_channel::api::r#impl::{
    WxChannelAddressServiceImpl, WxChannelAfterSaleServiceImpl, WxChannelBasicServiceImpl,
    WxChannelBrandServiceImpl, WxChannelCategoryServiceImpl, WxChannelCouponServiceImpl,
    WxChannelFreightTemplateServiceImpl, WxChannelOrderServiceImpl, WxChannelProductServiceImpl,
    WxChannelServiceImpl, WxChannelSharerServiceImpl, WxChannelWarehouseServiceImpl,
};
use wx_rust_channel::api::{
    WxChannelAddressService, WxChannelAfterSaleService, WxChannelBasicService,
    WxChannelBrandService, WxChannelCategoryService, WxChannelCouponService,
    WxChannelFreightTemplateService, WxChannelOrderService, WxChannelProductService,
    WxChannelService, WxChannelSharerService, WxChannelWarehouseService,
};
use wx_rust_channel::bean::address::{AddressDetail, AddressIdParam};
use wx_rust_channel::bean::after::{
    AfterSaleIdParam, AfterSaleMerchantUpdateParam, RefundEvidenceParam,
};
use wx_rust_channel::bean::audit::{AuditApplyResponse, CategoryAuditInfo};
use wx_rust_channel::bean::base::{AddressInfo, WxChannelBaseResponse};
use wx_rust_channel::bean::brand::Brand;
use wx_rust_channel::bean::coupon::CouponParam;
use wx_rust_channel::bean::delivery::DeliveryInfo;
use wx_rust_channel::bean::freight::FreightTemplate;
use wx_rust_channel::bean::order::{
    ChangeOrderInfo, OrderListParam, OrderSearchCondition, OrderSearchParam,
};
use wx_rust_channel::bean::product::{SkuStockBatchParam, SpuUpdateInfo};
use wx_rust_channel::bean::warehouse::{StockGetParam, WarehouseParam};
use wx_rust_channel::config::WxChannelConfig;
use wx_rust_channel::config::r#impl::WxChannelDefaultConfig;
use wx_rust_common::config::WxConfigStorage;

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
fn config_with_host(host: &str) -> Arc<dyn WxChannelConfig> {
    let mut config = WxChannelDefaultConfig::new("wxappid", "secret");
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
fn new_service(config: Arc<dyn WxChannelConfig>) -> Arc<WxChannelServiceImpl> {
    WxChannelServiceImpl::new_arc(config)
}

/// 子服务注入弱引用（`Arc<WxChannelServiceImpl>` → `Weak<dyn WxChannelService>`，
/// 对应 Java 子服务构造器 `new WxChannelXxxServiceImpl(this)` 的循环引用）。
fn weak_service(service: &Arc<WxChannelServiceImpl>) -> std::sync::Weak<dyn WxChannelService> {
    let weak: std::sync::Weak<WxChannelServiceImpl> = Arc::downgrade(service);
    weak
}

// ---- product 商品域（镜像 Java WxChannelProductServiceImplTest.testAddProduct / testUpProduct） ----

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
    let product_service = WxChannelProductServiceImpl::new(weak_service(&service));

    // Java testAddProduct：addProduct(SpuUpdateInfo) → POST SPU_ADD_URL
    let mut info = SpuUpdateInfo::default();
    info.product_id = "1001".to_string();
    info.title = "测试商品".to_string();
    info.sub_title = "子标题".to_string();
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
    let product_service = WxChannelProductServiceImpl::new(weak_service(&service));

    // Java testGetProduct：getProduct("10000029995861", 3) → `{"product_id":10000029995861,"data_type":3}`
    let response = product_service
        .get_product("10000029995861".to_string(), Some(3))
        .await
        .expect("获取商品成功");
    assert_eq!(response.err_code, 0);
    assert_eq!(response.product.title, "商品");
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
    let product_service = WxChannelProductServiceImpl::new(weak_service(&service));

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
        product_ids: vec!["123".to_string()],
    };
    let response = product_service
        .get_sku_stock_batch(param.product_ids)
        .await
        .expect("批量获取库存成功");
    assert_eq!(response.err_code, 0);
    let body = last_body_json(&server);
    assert_eq!(body["product_id"][0], "123");
}

// ---- order 订单域（镜像 Java WxChannelOrderServiceImplTest.testGetOrder / testGetOrders） ----

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
    let order_service = WxChannelOrderServiceImpl::new(weak_service(&service));

    // Java testGetOrder：getOrder(orderId) → `{"order_id":".."}`（encode_sensitive_info 空值跳过）
    let response = order_service
        .get_order("order123".to_string())
        .await
        .expect("获取订单成功");
    assert_eq!(response.err_code, 0);
    assert_eq!(response.order.order_id, "order123");
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
    param.page_size = 10;
    param.status = 10;
    let response = order_service
        .get_orders(param)
        .await
        .expect("获取订单列表成功");
    assert_eq!(response.ids, vec!["order123"]);
    let body = last_body_json(&server);
    assert_eq!(body["page_size"], 10);
    assert_eq!(body["status"], 10);

    // Java testSearchOrder：searchOrder(OrderSearchParam)
    let mut search = OrderSearchParam::default();
    search.page_size = 5;
    search.status = 10;
    let mut condition = OrderSearchCondition::default();
    condition.order_id = "order123".to_string();
    search.search_condition = condition;
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
    let order_service = WxChannelOrderServiceImpl::new(weak_service(&service));

    // Java testUpdatePrice：updatePrice(orderId, 100, list) →
    // `{"order_id":"..","change_express":true,"express_fee":100,"change_order_infos":[...]}`
    let mut change = ChangeOrderInfo::default();
    change.product_id = "sku1".to_string();
    change.change_price = "99".to_string();
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
    delivery.waybill_id = "waybill1".to_string();
    delivery.delivery_id = "d1".to_string();
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

// ---- after_sale 售后域（镜像 Java WxChannelAfterSaleServiceImplTest.testListIds / testAccept） ----

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
    let after_sale_service = WxChannelAfterSaleServiceImpl::new(weak_service(&service));

    // Java testListIds：listIds(begin, end, null) → 空值跳过
    let response = after_sale_service
        .list_ids(Some(1690000000), Some(1690000100), String::new())
        .await
        .expect("获取售后单列表成功");
    assert_eq!(response.ids, vec!["as1"]);
    let body = last_body_json(&server);
    assert_eq!(body["begin_create_time"], 1690000000_i64);
    assert_eq!(body["end_create_time"], 1690000100_i64);
    assert!(body.get("next_key").is_none());

    // Java testGet：get(afterSaleOrderId) → AfterSaleIdParam
    let param = AfterSaleIdParam {
        after_sale_order_id: "as1".to_string(),
    };
    let response = after_sale_service
        .get_after_sale(param.after_sale_order_id)
        .await
        .expect("获取售后单成功");
    assert_eq!(response.err_code, 0);
    assert_eq!(response.info.after_sale_order_id, "as1");

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
    assert_eq!(response.reason_list[0].reason_text, "质量问题");
    assert_eq!(server.last_body(), "{}");
}

#[tokio::test]
async fn after_sale_evidence_exchange_and_merchant_update() {
    let server = MockServer::start(|_path| r#"{"errcode":0,"errmsg":"ok"}"#.to_string()).await;
    let service = new_service(config_with_host(&server.url("")));
    let after_sale_service = WxChannelAfterSaleServiceImpl::new(weak_service(&service));

    // Java testUploadRefundEvidence：uploadRefundEvidence → RefundEvidenceParam
    let param = RefundEvidenceParam {
        after_sale_order_id: "as1".to_string(),
        desc: "退款凭证".to_string(),
        certificates: vec!["m1".to_string()],
    };
    let response = after_sale_service
        .upload_refund_evidence(param.after_sale_order_id, param.desc, param.certificates)
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
        after_sale_order_id: "as1".to_string(),
        merchant_update_desc: "协商退款".to_string(),
        amount: 100,
        merchant_update_type: 2,
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

// ---- category 类目域（镜像 Java WxChannelCategoryServiceImplTest.testListAllCategory 等） ----

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
    let category_service = WxChannelCategoryServiceImpl::new(weak_service(&service));

    // Java testListAllCategory：GET LIST_ALL_CATEGORY_URL
    let response = category_service
        .list_all_category()
        .await
        .expect("获取所有类目成功");
    assert_eq!(response.err_code, 0);
    assert_eq!(
        response.list.len(),
        1,
        "cats 应解析出 1 个类目（含资质信息）"
    );

    // Java testListAvailableCategories：listAvailableCategories("0") → `{"f_cat_id":0}`
    let response = category_service
        .list_available_categories("0".to_string())
        .await
        .expect("获取可用类目成功");
    assert_eq!(response.err_code, 0);
    assert_eq!(response.categories[0].id, "101");
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
    info.level1 = 1;
    info.level2 = 2;
    info.level3 = 3;
    info.certificates = vec!["m1".to_string()];
    let response: AuditApplyResponse = category_service
        .add_category_by_info(info)
        .await
        .expect("添加类目成功");
    assert_eq!(response.err_code, 0);
    assert_eq!(response.audit_id, "audit1");
    let body = last_body_json(&server);
    assert_eq!(body["category_info"]["level1"], 1);
    assert_eq!(body["category_info"]["level2"], 2);
    assert_eq!(body["category_info"]["level3"], 3);
    assert_eq!(body["category_info"]["certificate"][0], "m1");
}

// ---- brand 品牌域（镜像 Java WxChannelBrandServiceImplTest.testAddBrandApply 等） ----

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
    let brand_service = WxChannelBrandServiceImpl::new(weak_service(&service));

    // Java testListAllBrand：listAllBrand(10, null) → `{"page_size":10}`（next_key 空跳过）
    let response = brand_service
        .list_all_brand(Some(10), String::new())
        .await
        .expect("获取品牌库成功");
    assert_eq!(response.err_code, 0);
    assert_eq!(response.brands[0].brand_id, "b1");
    let body = last_body_json(&server);
    assert_eq!(body["page_size"], 10);
    assert!(body.get("next_key").is_none());

    // Java testAddBrandApply：addBrandApply(brand) → `{"brand":{...}}`
    let mut brand = Brand::default();
    brand.brand_id = "b1".to_string();
    brand.ch_name = "测试品牌".to_string();
    let response: AuditApplyResponse = brand_service
        .add_brand_apply(brand)
        .await
        .expect("新增品牌成功");
    assert_eq!(response.audit_id, "audit1");
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

// ---- coupon 优惠券域（镜像 Java WxChannelCouponServiceImplTest.testCreateCoupon 等） ----

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
    let coupon_service = WxChannelCouponServiceImpl::new(weak_service(&service));

    // Java testCreateCoupon：createCoupon(CouponParam)
    let mut coupon = CouponParam::default();
    coupon.name = "满减券".to_string();
    coupon.r#type = 1;
    let response = coupon_service
        .create_coupon(coupon)
        .await
        .expect("创建优惠券成功");
    assert_eq!(response.err_code, 0);
    assert_eq!(response.data.coupon_id, "c1");
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

// ---- warehouse 仓库域（镜像 Java WxChannelWarehouseServiceImplTest.testCreateWarehouse 等） ----

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
    let warehouse_service = WxChannelWarehouseServiceImpl::new(weak_service(&service));

    // Java testCreateWarehouse：createWarehouse(WarehouseParam)
    let param = WarehouseParam {
        out_warehouse_id: "w1".to_string(),
        name: "华东仓".to_string(),
        intro: "覆盖江浙沪".to_string(),
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
    assert_eq!(response.ids, vec!["w1"]);
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
        product_id: "p1".to_string(),
        sku_id: "s1".to_string(),
        out_warehouse_id: "w1".to_string(),
    };
    let response = warehouse_service
        .get_warehouse_stock(param.product_id, param.sku_id, param.out_warehouse_id)
        .await
        .expect("获取仓库库存成功");
    assert_eq!(response.err_code, 0);
    let body = last_body_json(&server);
    assert_eq!(body["product_id"], "p1");
    assert_eq!(body["sku_id"], "s1");
    assert_eq!(body["out_warehouse_id"], "w1");
}

// ---- freight_template 运费模板域（镜像 Java WxChannelFreightTemplateServiceImplTest 等） ----

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
    let freight_service = WxChannelFreightTemplateServiceImpl::new(weak_service(&service));

    // Java testListTemplate：listTemplate(0, 10)
    let response = freight_service
        .list_template(Some(0), Some(10))
        .await
        .expect("获取运费模板列表成功");
    assert_eq!(response.ids, vec!["t1"]);
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
    template.name = "模板A".to_string();
    template.valuation_type = "1".to_string();
    let response = freight_service
        .add_template(template)
        .await
        .expect("添加运费模板成功");
    assert_eq!(response.err_code, 0);
    assert_eq!(response.template_id, "t1");
    let body = last_body_json(&server);
    assert_eq!(body["freight_template"]["name"], "模板A");
    assert_eq!(body["freight_template"]["valuation_type"], "1");
}

// ---- address 地址域（镜像 Java WxChannelAddressServiceImplTest.testAddAddress 等） ----

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
    let address_service = WxChannelAddressServiceImpl::new(weak_service(&service));

    // Java testListAddress：listAddress(0, 10)
    let response = address_service
        .list_address(Some(0), Some(10))
        .await
        .expect("获取地址列表成功");
    assert_eq!(response.ids, vec!["addr1"]);
    let body = last_body_json(&server);
    assert_eq!(body["offset"], 0);
    assert_eq!(body["limit"], 10);

    // Java testAddAddress：addAddress(AddressDetail) → `{"address_detail":{...}}`
    let mut detail = AddressDetail::default();
    detail.name = "张三".to_string();
    let mut address_info = AddressInfo::default();
    address_info.user_name = "张三".to_string();
    address_info.tel_number = "13800000000".to_string();
    detail.address_info = address_info;
    let response = address_service
        .add_address(detail)
        .await
        .expect("添加地址成功");
    assert_eq!(response.address_id, "addr1");
    let body = last_body_json(&server);
    assert_eq!(body["address_detail"]["name"], "张三");
    assert_eq!(
        body["address_detail"]["address_info"]["tel_number"],
        "13800000000"
    );

    // Java testGetAddress：AddressIdParam
    let param = AddressIdParam {
        address_id: "addr1".to_string(),
    };
    let response = address_service
        .get_address(param.address_id)
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

// ---- sharer 分享员域（镜像 Java WxChannelSharerServiceImplTest.testBindSharer 等） ----

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
    let sharer_service = WxChannelSharerServiceImpl::new(weak_service(&service));

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
    assert_eq!(response.list[0].openid, "o1");
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
    assert_eq!(response.success_list, vec!["o1"]);
    let body = last_body_json(&server);
    assert_eq!(body["openid_list"][0], "o1");
    assert_eq!(body["openid_list"][1], "o2");
}

// ---- basic 基础域（镜像 Java WxChannelBasicServiceImplTest.testGetShopInfo / testUploadImg） ----

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
    let basic_service = WxChannelBasicServiceImpl::new(weak_service(&service));

    // Java testGetShopInfo：GET GET_SHOP_INFO
    let response = basic_service
        .get_shop_info()
        .await
        .expect("获取店铺信息成功");
    assert_eq!(response.err_code, 0);
    assert_eq!(response.info.nickname, "测试小店");

    // Java testUploadImg：uploadImg(1, imgUrl) →
    // POST `IMG_UPLOAD_URL?upload_type=1&resp_type=1`，请求体 `{"img_url":".."}`
    let response = basic_service
        .upload_img(1, "https://img.example.com/1.jpg".to_string())
        .await
        .expect("上传图片成功");
    assert_eq!(response.media_id, "media1");
    assert_eq!(response.url, "https://img.example.com/1.jpg");
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
    let basic_service = WxChannelBasicServiceImpl::new(weak_service(&service));

    // Java testGetAddressCode：getAddressCode(null) → `{"addr_code": null}`（Java 手拼保留 null）
    let response = basic_service
        .get_address_code(None)
        .await
        .expect("获取地址编码成功");
    assert_eq!(response.err_code, 0);
    assert_eq!(response.list[0].code, 440000);
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
    let product_service = WxChannelProductServiceImpl::new(weak_service(&service));

    let err = product_service
        .add_product(SpuUpdateInfo::default())
        .await
        .expect_err("errcode!=0 应报错");
    assert_eq!(err.error_code(), Some(40001));
}

#[tokio::test]
async fn close_order_returns_internal_error() {
    // Java `WxChannelOrderServiceImpl.closeOrder`：暂不支持，返回内部错误
    // （err_code=-99，err_msg="内部错误"），不发请求。
    let server = MockServer::start(|_path| r#"{"errcode":0,"errmsg":"ok"}"#.to_string()).await;
    let service = new_service(config_with_host(&server.url("")));
    let order_service = WxChannelOrderServiceImpl::new(weak_service(&service));

    let response: WxChannelBaseResponse = order_service
        .close_order("order1".to_string())
        .await
        .expect("closeOrder 不抛异常（Java 返回内部错误对象）");
    assert_eq!(response.err_code, -99);
    assert_eq!(response.err_msg, "内部错误");
    assert_eq!(server.request_count(), 0, "closeOrder 不应发起请求");
}
