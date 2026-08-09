#![allow(clippy::field_reassign_with_default)]
//! 小程序电商服务组（G3）子域集成测试（镜像 Java
//! `WxMaShopAccountServiceImplTest` / `WxMaShopCatServiceImplTest` /
//! `WxMaShopRegisterServiceImplTest` / `WxMaShopAfterSaleServiceImplTest` /
//! `WxMaImmediateDeliveryServiceImplTest` 及 `WxMaShopSpuServiceImpl` /
//! `WxMaShopOrderServiceImpl` / `WxMaProductServiceImpl` /
//! `WxMaProductOrderServiceImpl` / `WxMaOrderShippingServiceImpl` /
//! `WxMaCustomserviceWorkServiceImpl` 的 HTTP 语义，经 MockServer 验证）。
//!
//! 覆盖：shop 交易组件（商家入驻/类目/接入申请/商品/订单/售后 6 域）+ 标准版
//! product/productOrder + 发货信息 + 即时配送 + 微信客服，共 12 个测试函数，
//! 每个测试断言请求路径、请求体关键字段（serde_json 解析 last_body）与
//! 响应解析值（响应键以 bean serde rename 为准：`errcode`/`errmsg`/`data`）。

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use sha1::Digest as _;

use wx_rust_miniapp::api::WxMaService;
use wx_rust_miniapp::api::r#impl::WxMaServiceImpl;
use wx_rust_miniapp::bean::customservice::WxMaCustomserviceResult;
use wx_rust_miniapp::bean::delivery::{
    AddOrderRequest, AddOrderResponse, BindAccountResponse, Cargo, GetOrderRequest,
    GetOrderResponse, Goods, GoodsDetail, OrderInfo as DeliveryOrderInfo, Receiver, Sender,
    Shop as DeliveryShop,
};
use wx_rust_miniapp::bean::product::{WxMinishopResult, WxMinishopSpu, WxMinishopSpuListResponse};
use wx_rust_miniapp::bean::shop::WxMaShopSpuInfo;
use wx_rust_miniapp::bean::shop::request::shipping::{
    ContactBean, OrderKeyBean, PayerBean, ShippingListBean,
};
use wx_rust_miniapp::bean::shop::request::{
    ProductInfosBean, WxMaOrderShippingInfoUploadRequest, WxMaShopAccountUpdateInfoRequest,
    WxMaShopAfterSaleAddRequest, WxMaShopAfterSaleUpdateRequest, WxMaShopRegisterApplySceneRequest,
    WxMaShopRegisterFinishAccessInfoRequest, WxMaShopSpuPageRequest,
};
use wx_rust_miniapp::bean::shop::response::{
    WxMaOrderShippingInfoBaseResponse, WxMaShopAccountGetInfoResponse, WxMaShopAddSpuResponse,
    WxMaShopAfterSaleAddResponse, WxMaShopBaseResponse, WxMaShopCatGetResponse,
    WxMaShopGetOrderListResponse, WxMaShopGetOrderResponse, WxMaShopRegisterCheckResponse,
};
use wx_rust_miniapp::config::r#impl::WxMaDefaultConfig;
use wx_rust_miniapp::config::{WxMaConfig, WxMaHostConfig};

/// 极简 mock HTTP 服务器：按请求路径返回固定响应，记录最近一次请求体。
struct MockServer {
    addr: std::net::SocketAddr,
    requests: Arc<AtomicUsize>,
    last_body: Arc<std::sync::Mutex<String>>,
    stop: Arc<std::sync::atomic::AtomicBool>,
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
        let stop = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let handler = Arc::new(handler);

        let requests_clone = requests.clone();
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
                let last_body_clone = last_body_clone.clone();
                tokio::spawn(async move {
                    use tokio::io::{AsyncReadExt, AsyncWriteExt};
                    let mut buf = [0u8; 16384];
                    let n = socket.read(&mut buf).await.unwrap_or(0);
                    let request = String::from_utf8_lossy(&buf[..n]).to_string();
                    // 记录请求体（POST 场景；GET 无请求体时置空）
                    if let Some(idx) = request.find("\r\n\r\n") {
                        let body = request[idx + 4..].to_string();
                        *last_body_clone.lock().unwrap() = body;
                    }
                    let path = request
                        .lines()
                        .next()
                        .map(|l| l.split_whitespace().nth(1).unwrap_or("/").to_string())
                        .unwrap_or_else(|| "/".to_string());
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
}

impl Drop for MockServer {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
    }
}

/// 构建指向 mock 服务器的小程序配置（`WxMaDefaultConfig` + `set_host_config`）。
fn config_with_host(host: &str) -> Arc<dyn WxMaConfig> {
    let mut config = WxMaDefaultConfig::new("wxappid", "secret");
    config
        .set_token("token123")
        .set_aes_key("abcdefghijklmnopqrstuvwxyz0123456789ABCDEFG");
    let mut host_config = WxMaHostConfig::new();
    host_config.api_host = host.to_string();
    config.set_host_config(host_config);
    Arc::new(config)
}

/// 通用路由 handler：token 请求 + 各子域响应。
fn dispatch(
    handler: impl Fn(&str) -> String + Send + Sync + 'static,
) -> impl Fn(&str) -> String + Send + Sync + 'static {
    move |path: &str| {
        if path.contains("/cgi-bin/token") || path.contains("/stable_token") {
            return r#"{"access_token":"MOCK_TOKEN","expires_in":7200}"#.to_string();
        }
        handler(path)
    }
}

/// 解析最近一次请求体为 JSON。
fn last_body_json(server: &MockServer) -> serde_json::Value {
    serde_json::from_str(&server.last_body()).expect("请求体 JSON")
}

// ---- shop_account 商家入驻（镜像 Java WxMaShopAccountServiceImplTest.testGetInfo / testUpdateInfo） ----

#[tokio::test]
async fn shop_account_get_info_and_update_info() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/shop/account/get_info") {
            // Java 响应：errcode/errmsg/data（data 为 {brand_id, brand_wording}）
            r#"{"errcode":0,"errmsg":"ok","data":{"brand_id":100,"brand_wording":"品牌商"}}"#
                .to_string()
        } else if path.contains("/shop/account/get_category_list") {
            r#"{"errcode":0,"errmsg":"ok","data":{"category_list":[]}}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let account_service = service.shop_account_service().expect("商家入驻服务存在");

    // Java testGetInfo：getInfo() 解析 data 中的 brand_id/brand_wording
    let info: WxMaShopAccountGetInfoResponse =
        account_service.get_info().await.expect("获取商家信息成功");
    assert_eq!(info.err_code, 0);
    assert_eq!(info.data.brand_id, 100);
    assert_eq!(info.data.brand_wording, "品牌商");

    // Java testUpdateInfo：updateInfo(request)，request 含 serviceAgentPhone/serviceAgentPath
    let mut request = WxMaShopAccountUpdateInfoRequest::default();
    request.service_agent_phone = "020-888888".to_string();
    request.service_agent_path = "https://www.web.com".to_string();
    let response: WxMaShopBaseResponse = account_service
        .update_info(&request)
        .await
        .expect("更新商家信息成功");
    assert_eq!(response.err_code, 0);
    let body = last_body_json(&server);
    assert_eq!(body["service_agent_phone"], "020-888888");
    assert_eq!(body["service_agent_path"], "https://www.web.com");
}

#[tokio::test]
async fn shop_account_get_info_errcode_nonzero_errors() {
    // Java 语义：响应 errcode != 0 时（WxMaShopAccountServiceImpl 校验
    // `jsonObject.get(ERR_CODE).getAsInt() != 0`）抛 WxErrorException；
    // Rust 由执行引擎 handle_response 校验 errcode 上抛。
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/shop/account/get_info") {
            r#"{"errcode":40001,"errmsg":"invalid credential"}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let account_service = service.shop_account_service().expect("商家入驻服务存在");

    let err = account_service
        .get_info()
        .await
        .expect_err("errcode!=0 应报错");
    assert_eq!(err.error_code(), Some(40001));
}

// ---- shop_cat 商品类目（镜像 Java WxMaShopCatServiceImplTest.testGetCat） ----

#[tokio::test]
async fn shop_cat_get_cat() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/shop/cat/get") {
            // Java 响应：third_cat_list 类目列表
            r#"{"errcode":0,"errmsg":"ok","third_cat_list":[{"third_cat_id":101,"third_cat_name":"测试类目","first_cat_id":1,"first_cat_name":"一级类目","second_cat_id":2,"second_cat_name":"二级类目","qualification":"","qualification_type":0,"product_qualification":"","product_qualification_type":0}]}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let cat_service = service.shop_cat_service().expect("商品类目服务存在");

    let response: WxMaShopCatGetResponse = cat_service.get_cat().await.expect("获取类目成功");
    assert_eq!(response.err_code, 0);
    assert_eq!(response.third_cat_list.len(), 1);
    assert_eq!(response.third_cat_list[0].third_cat_id, 101);
    assert_eq!(response.third_cat_list[0].third_cat_name, "测试类目");
    assert_eq!(response.third_cat_list[0].first_cat_name, "一级类目");
    assert_eq!(response.third_cat_list[0].second_cat_name, "二级类目");
}

// ---- shop_register 申请接入（镜像 Java WxMaShopRegisterServiceImplTest 全部 4 个用例） ----

#[tokio::test]
async fn shop_register_check_apply_and_scene() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/shop/register/check") {
            // Java testRegisterCheck：registerCheck() → data 为接入状态
            r#"{"errcode":0,"errmsg":"ok","data":{"status":1}}"#.to_string()
        } else {
            // apply / finish_access_info / apply_scene（注意 /apply_scene 含
            // /apply 子串，响应相同故统一走 else）均返回基础响应
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let register_service = service.shop_register_service().expect("申请接入服务存在");

    // Java testRegisterApply：registerApply() POST 空对象
    let response: WxMaShopBaseResponse = register_service
        .register_apply()
        .await
        .expect("接入申请成功");
    assert_eq!(response.err_code, 0);
    assert_eq!(last_body_json(&server), serde_json::json!({}));

    // Java testRegisterCheck：registerCheck() → data.status
    let check: WxMaShopRegisterCheckResponse = register_service
        .register_check()
        .await
        .expect("接入状态成功");
    assert_eq!(check.err_code, 0);
    assert_eq!(check.data["status"], 1);

    // Java testRegisterFinishAccessInfo：accessInfoItem=6L
    let mut finish = WxMaShopRegisterFinishAccessInfoRequest::default();
    finish.access_info_item = 6;
    let response: WxMaShopBaseResponse = register_service
        .register_finish_access_info(&finish)
        .await
        .expect("完成接入任务成功");
    assert_eq!(response.err_code, 0);
    let body = last_body_json(&server);
    assert_eq!(body["access_info_item"], 6);

    // Java testRegisterApplyScene：sceneGroupId=1L
    let mut scene = WxMaShopRegisterApplySceneRequest::default();
    scene.scene_group_id = 1;
    let response: WxMaShopBaseResponse = register_service
        .register_apply_scene(&scene)
        .await
        .expect("场景接入申请成功");
    assert_eq!(response.err_code, 0);
    let body = last_body_json(&server);
    assert_eq!(body["scene_group_id"], 1);
}

// ---- shop_spu 商品（镜像 Java WxMaShopSpuServiceImpl.addSpu / listingSpu / delistingSpu） ----

#[tokio::test]
async fn shop_spu_add_listing_and_delisting() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/shop/spu/add") {
            // Java 响应：data 含 product_id/out_product_id/create_time
            r#"{"errcode":0,"errmsg":"ok","data":{"product_id":"123","out_product_id":"OUT_123","create_time":"2024-01-01 10:00:00"}}"#.to_string()
        } else {
            // listing / delisting 均返回基础响应
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let spu_service = service.shop_spu_service().expect("商品服务存在");

    // addSpu(WxMaShopSpuInfo)：请求体序列化 product 字段
    let mut spu_info = WxMaShopSpuInfo::default();
    spu_info.out_product_id = "OUT_123".to_string();
    spu_info.title = "测试商品".to_string();
    spu_info.path = "pages/goods/detail?id=1".to_string();
    spu_info.head_img = vec!["https://img.example.com/head.png".to_string()];
    spu_info.third_cat_id = 101;
    spu_info.brand_id = 202;
    let response: WxMaShopAddSpuResponse =
        spu_service.add_spu(&spu_info).await.expect("添加商品成功");
    assert_eq!(response.err_code, 0);
    assert_eq!(response.data.product_id, "123");
    assert_eq!(response.data.out_product_id, "OUT_123");
    let body = last_body_json(&server);
    assert_eq!(body["title"], "测试商品");
    assert_eq!(body["out_product_id"], "OUT_123");
    assert_eq!(body["path"], "pages/goods/detail?id=1");
    assert_eq!(body["head_img"][0], "https://img.example.com/head.png");
    assert_eq!(body["third_cat_id"], 101);

    // listingSpu(productId, outProductId)：{"product_id","out_product_id"}
    let response: WxMaShopBaseResponse = spu_service
        .listing_spu(123, Some("OUT_123"))
        .await
        .expect("上架成功");
    assert_eq!(response.err_code, 0);
    let body = last_body_json(&server);
    assert_eq!(body["product_id"], 123);
    assert_eq!(body["out_product_id"], "OUT_123");

    // delistingSpu(productId, outProductId)
    let response: WxMaShopBaseResponse = spu_service
        .delisting_spu(123, Some("OUT_123"))
        .await
        .expect("下架成功");
    assert_eq!(response.err_code, 0);
    let body = last_body_json(&server);
    assert_eq!(body["product_id"], 123);
    assert_eq!(body["out_product_id"], "OUT_123");
}

// ---- shop_order 订单（镜像 Java WxMaShopOrderServiceImpl.getOrderList / getOrder） ----

#[tokio::test]
async fn shop_order_get_order_list_and_detail() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/shop/order/get_list") {
            // 注意：/get_list 含 /get 子串，必须先匹配
            r#"{"errcode":0,"errmsg":"ok","total_num":1,"order":{"order_id":1001,"out_order_id":"OUT_ORDER_1","status":10,"path":"pages/order/detail"}}"#.to_string()
        } else if path.contains("/shop/order/get") {
            r#"{"errcode":0,"errmsg":"ok","order":{"order_id":1001,"out_order_id":"OUT_ORDER_1","status":10,"path":"pages/order/detail"}}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let order_service = service.shop_order_service().expect("订单服务存在");

    // Java getOrderList：page/page_size 默认 1/10；desc=true → 1；时间戳按
    // FastDateFormat("yyyy-MM-dd HH:mm:ss") 格式化（Rust 以 UTC 格式化）
    let start = 1_700_000_000_000i64;
    let end = 1_700_086_400_000i64;
    let list: WxMaShopGetOrderListResponse = order_service
        .get_order_list(Some(2), Some(20), true, Some(start), Some(end))
        .await
        .expect("订单列表成功");
    assert_eq!(list.err_code, 0);
    assert_eq!(list.total_num, 1);
    assert_eq!(list.order.order_id, 1001);
    assert_eq!(list.order.status, 10);
    let body = last_body_json(&server);
    assert_eq!(body["page"], 2);
    assert_eq!(body["page_size"], 20);
    assert_eq!(body["desc"], 1);
    let expected_start = chrono::DateTime::from_timestamp_millis(start)
        .expect("合法时间戳")
        .format("%Y-%m-%d %H:%M:%S")
        .to_string();
    let expected_end = chrono::DateTime::from_timestamp_millis(end)
        .expect("合法时间戳")
        .format("%Y-%m-%d %H:%M:%S")
        .to_string();
    assert_eq!(body["start_create_time"], expected_start);
    assert_eq!(body["end_create_time"], expected_end);

    // Java getOrder(orderId, outOrderId, openid)：{"order_id","out_order_id","openid"}
    let detail: WxMaShopGetOrderResponse = order_service
        .get_order(
            Some(1001),
            Some("OUT_ORDER_1"),
            Some("oTVP50O53a7jgmawAmxKukNlq3XI"),
        )
        .await
        .expect("订单详情成功");
    assert_eq!(detail.err_code, 0);
    assert_eq!(detail.order.order_id, 1001);
    let body = last_body_json(&server);
    assert_eq!(body["order_id"], 1001);
    assert_eq!(body["out_order_id"], "OUT_ORDER_1");
    assert_eq!(body["openid"], "oTVP50O53a7jgmawAmxKukNlq3XI");
}

// ---- shop_after_sale 售后（镜像 Java WxMaShopAfterSaleServiceImplTest.testAdd / testUpdate） ----

#[tokio::test]
async fn shop_after_sale_add_and_update() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/shop/ecaftersale/add") {
            r#"{"errcode":0,"errmsg":"ok","aftersale_id":"318092069606883328X"}"#.to_string()
        } else {
            // aftersale/update 返回基础响应
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let after_sale_service = service.shop_after_sale_service().expect("售后服务存在");

    // Java testAdd：add(request)（Java builder 语义：outOrderId/outAftersaleId/
    // openid/type/status/finishAllAftersale/path/refund/productInfo）
    let mut product_info = ProductInfosBean::default();
    product_info.out_product_id = "19030".to_string();
    product_info.out_sku_id = "123266".to_string();
    product_info.product_cnt = 1;
    let mut request = WxMaShopAfterSaleAddRequest::default();
    request.out_order_id = "318070290792415232X".to_string();
    request.out_aftersale_id = "318092069606883328X".to_string();
    request.openid = "odIi15CuQ0IQviqsnUMy6CKNetrMX".to_string();
    request.r#type = 1;
    request.status = 1;
    request.finish_all_aftersale = 0;
    request.path = "/pages/aftersale.html?out_aftersale_id=318092069606883328X".to_string();
    request.refund = 100;
    request.product_info = product_info;
    let response: WxMaShopAfterSaleAddResponse = after_sale_service
        .add(&request)
        .await
        .expect("创建售后成功");
    assert_eq!(response.err_code, 0);
    assert_eq!(response.aftersale_id, "318092069606883328X");
    let body = last_body_json(&server);
    assert_eq!(body["out_order_id"], "318070290792415232X");
    assert_eq!(body["out_aftersale_id"], "318092069606883328X");
    assert_eq!(body["openid"], "odIi15CuQ0IQviqsnUMy6CKNetrMX");
    assert_eq!(body["type"], 1);
    assert_eq!(body["status"], 1);
    assert_eq!(body["finish_all_aftersale"], 0);
    assert_eq!(body["refund"], 100);
    assert_eq!(body["product_info"]["out_product_id"], "19030");
    assert_eq!(body["product_info"]["out_sku_id"], "123266");
    assert_eq!(body["product_info"]["product_cnt"], 1);

    // Java testUpdate：update(request)（outOrderId/openid/outAftersaleId/status/finishAllAftersale）
    let mut update = WxMaShopAfterSaleUpdateRequest::default();
    update.out_order_id = "xxxxx".to_string();
    update.openid = "oTVP50O53a7jgmawAmxKukNlq3XI".to_string();
    update.out_aftersale_id = "xxxxxx".to_string();
    update.status = 1;
    update.finish_all_aftersale = 0;
    let response: WxMaShopBaseResponse = after_sale_service
        .update(&update)
        .await
        .expect("更新售后成功");
    assert_eq!(response.err_code, 0);
    let body = last_body_json(&server);
    assert_eq!(body["out_order_id"], "xxxxx");
    assert_eq!(body["openid"], "oTVP50O53a7jgmawAmxKukNlq3XI");
    assert_eq!(body["out_aftersale_id"], "xxxxxx");
    assert_eq!(body["status"], 1);
    assert_eq!(body["finish_all_aftersale"], 0);
}

// ---- product 标准版商品（镜像 Java WxMaProductServiceImpl.addSpu / getSpuList） ----

#[tokio::test]
async fn product_add_spu_and_get_spu_list() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/product/spu/add") {
            // Java 响应：data 含 product_id/out_product_id/create_time
            r#"{"errcode":0,"errmsg":"ok","data":{"product_id":9001,"out_product_id":"OUT_P","create_time":"2024-06-01 12:00:00"}}"#.to_string()
        } else if path.contains("/product/spu/get_list") {
            r#"{"errcode":0,"errmsg":"ok","total_num":1,"spus":[{"product_id":"9001","out_product_id":"OUT_P","title":"标准版商品"}]}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let product_service = service.product_service().expect("标准版商品服务存在");

    // Java addSpu(WxMinishopSpu)：请求体序列化 spu 字段
    let mut spu = WxMinishopSpu::default();
    spu.out_product_id = "OUT_P".to_string();
    spu.title = "标准版商品".to_string();
    spu.head_imgs = vec!["https://img.example.com/a.png".to_string()];
    spu.desc_info.imgs = vec!["https://img.example.com/desc.png".to_string()];
    spu.brand_id = 101;
    let result: WxMinishopResult = product_service.add_spu(&spu).await.expect("添加商品成功");
    assert_eq!(result.errcode, 0);
    assert_eq!(result.data["product_id"], 9001);
    assert_eq!(result.data["out_product_id"], "OUT_P");
    let body = last_body_json(&server);
    assert_eq!(body["title"], "标准版商品");
    assert_eq!(body["out_product_id"], "OUT_P");
    assert_eq!(body["head_img"][0], "https://img.example.com/a.png");
    assert_eq!(
        body["desc_info"]["imgs"][0],
        "https://img.example.com/desc.png"
    );
    assert_eq!(body["brand_id"], 101);

    // Java getSpuList(WxMaShopSpuPageRequest)：分页参数序列化
    let mut page_request = WxMaShopSpuPageRequest::default();
    page_request.status = 5;
    page_request.page = 1;
    page_request.page_size = 10;
    let list: WxMinishopSpuListResponse = product_service
        .get_spu_list(&page_request)
        .await
        .expect("商品列表成功");
    assert_eq!(list.errcode, 0);
    assert_eq!(list.total_num, 1);
    assert_eq!(list.spus[0].title, "标准版商品");
    assert_eq!(list.spus[0].out_product_id, "OUT_P");
    let body = last_body_json(&server);
    assert_eq!(body["status"], 5);
    assert_eq!(body["page"], 1);
    assert_eq!(body["page_size"], 10);
}

// ---- product_order 标准版商品订单（镜像 Java WxMaProductOrderServiceImpl.getOrderList / getOrderDetail） ----

#[tokio::test]
async fn product_order_get_order_list_and_detail() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/product/order/get_list") {
            // 注意：/get_list 含 /get 子串，必须先匹配
            r#"{"errcode":0,"errmsg":"ok","total_num":1,"orders":[{"order_id":7001,"status":10,"create_time":"2024-06-01 00:00:00","update_time":"2024-06-01 01:00:00"}]}"#.to_string()
        } else if path.contains("/product/order/get") {
            r#"{"errcode":0,"errmsg":"ok","order":{"order_id":7001,"status":10,"create_time":"2024-06-01 00:00:00"}}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let order_service = service.product_order_service().expect("商品订单服务存在");

    // Java getOrderList：8 个可选字段（时间串 + status/page/page_size/source）
    let list = order_service
        .get_order_list(
            Some("2024-06-01 00:00:00"),
            Some("2024-06-01 23:59:59"),
            None,
            None,
            Some(10),
            Some(1),
            Some(10),
            None,
        )
        .await
        .expect("订单列表成功");
    assert_eq!(list.err_code, 0);
    assert_eq!(list.total_num, 1);
    assert_eq!(list.orders[0].order_id, 7001);
    assert_eq!(list.orders[0].status, 10);
    let body = last_body_json(&server);
    assert_eq!(body["start_create_time"], "2024-06-01 00:00:00");
    assert_eq!(body["end_create_time"], "2024-06-01 23:59:59");
    assert_eq!(body["status"], 10);
    assert_eq!(body["page"], 1);
    assert_eq!(body["page_size"], 10);
    // 未传字段不出现（Java GsonHelper.buildJsonObject 跳过空值）
    assert!(body.get("source").is_none());

    // Java getOrderDetail(orderId)：{"order_id": orderId}
    let detail = order_service
        .get_order_detail(7001)
        .await
        .expect("订单详情成功");
    assert_eq!(detail.err_code, 0);
    assert_eq!(detail.order.order_id, 7001);
    let body = last_body_json(&server);
    assert_eq!(body["order_id"], 7001);
}

// ---- order_shipping 发货信息（镜像 Java WxMaOrderShippingServiceImpl.upload） ----

#[tokio::test]
async fn order_shipping_upload() {
    let server = MockServer::start(dispatch(|_path| {
        // upload_shipping_info 返回基础响应
        r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let shipping_service = service.order_shipping_service().expect("发货信息服务存在");

    // Java upload(WxMaOrderShippingInfoUploadRequest)：订单标识 + 物流列表 + 支付者
    let mut order_key = OrderKeyBean::default();
    order_key.order_number_type = 2;
    order_key.transaction_id = "4200001234".to_string();
    order_key.mch_id = "1900001".to_string();
    order_key.out_trade_no = "OUT_TRADE_1".to_string();
    let mut contact = ContactBean::default();
    contact.consignor_contact = "张三".to_string();
    contact.receiver_contact = "李四".to_string();
    let mut shipping_list = ShippingListBean::default();
    shipping_list.tracking_no = "SF1390000001".to_string();
    shipping_list.express_company = "SF".to_string();
    shipping_list.item_desc = "测试商品".to_string();
    shipping_list.contact = contact;
    let mut payer = PayerBean::default();
    payer.openid = "oTVP50O53a7jgmawAmxKukNlq3XI".to_string();

    let mut request = WxMaOrderShippingInfoUploadRequest::default();
    request.order_key = order_key;
    request.logistics_type = 1;
    request.delivery_mode = 1;
    request.is_all_delivered = true;
    request.shipping_list = vec![shipping_list];
    request.upload_time = "2024-06-01 10:00:00".to_string();
    request.payer = payer;

    let response: WxMaOrderShippingInfoBaseResponse = shipping_service
        .upload(&request)
        .await
        .expect("发货信息录入成功");
    assert_eq!(response.err_code, 0);
    let body = last_body_json(&server);
    assert_eq!(body["order_key"]["order_number_type"], 2);
    assert_eq!(body["order_key"]["transaction_id"], "4200001234");
    assert_eq!(body["order_key"]["mchid"], "1900001");
    assert_eq!(body["order_key"]["out_trade_no"], "OUT_TRADE_1");
    assert_eq!(body["logistics_type"], 1);
    assert_eq!(body["delivery_mode"], 1);
    assert_eq!(body["is_all_delivered"], true);
    assert_eq!(body["shipping_list"][0]["tracking_no"], "SF1390000001");
    assert_eq!(body["shipping_list"][0]["express_company"], "SF");
    assert_eq!(body["shipping_list"][0]["item_desc"], "测试商品");
    assert_eq!(
        body["shipping_list"][0]["contact"]["receiver_contact"],
        "李四"
    );
    assert_eq!(body["payer"]["openid"], "oTVP50O53a7jgmawAmxKukNlq3XI");
}

// ---- immediate_delivery 即时配送（镜像 Java WxMaImmediateDeliveryServiceImplTest
//      testGetBindAccount / testAddOrder / testGetOrder） ----

#[tokio::test]
async fn immediate_delivery_bind_account_add_order_and_get_order() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/express/local/business/order/add") {
            // 运力方响应：resultcode/resultmsg（parse 语义校验 resultcode==0）
            r#"{"resultcode":0,"resultmsg":"ok","waybill_id":"WB_20240601","order_status":101,"fee":"5","distance":"1200"}"#.to_string()
        } else if path.contains("/cgi-bin/express/local/business/order/get") {
            r#"{"resultcode":0,"resultmsg":"ok","order_status":102,"waybill_id":"WB_20240601","rider_name":"骑手小王","rider_phone":"16600000000"}"#.to_string()
        } else if path.contains("/cgi-bin/express/local/business/shop/get") {
            r#"{"resultcode":0,"resultmsg":"ok","shop_list":[{"delivery_id":"SFTC","shopid":"shopId","audit_result":"0"}]}"#.to_string()
        } else {
            r#"{"resultcode":0,"resultmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let delivery_service = service
        .immediate_delivery_service()
        .expect("即时配送服务存在");

    // Java testGetBindAccount：getBindAccount() → shop_list
    let bind: BindAccountResponse = delivery_service
        .get_bind_account()
        .await
        .expect("拉取已绑定账号成功");
    assert_eq!(bind.result_code, 0);
    assert_eq!(bind.shop_list.len(), 1);
    assert_eq!(bind.shop_list[0].delivery_id, "SFTC");
    assert_eq!(bind.shop_list[0].shop_id, "shopId");

    // Java testAddOrder：addOrder(request) 自动注入 delivery_sign
    // = SHA1(shopid + shop_order_id + appSecret)
    let mut request = AddOrderRequest::default();
    request.openid = "odIi15CuQ0IQviqsnUMy6CKNetrMX".to_string();
    request.shop_id = "shopId".to_string();
    request.app_secret = "secret".to_string();
    request.shop_no = "shopNo_1".to_string();
    request.delivery_id = "SFTC".to_string();
    request.shop_order_id = "order_001".to_string();
    let mut order_info = DeliveryOrderInfo::default();
    order_info.order_time = 1_700_000_000;
    request.order_info = order_info;
    let mut sender = Sender::default();
    sender.city = "上海市".to_string();
    sender.address = "测试路".to_string();
    sender.address_detail = "1 号".to_string();
    sender.name = "发件人".to_string();
    sender.phone = "16600008829".to_string();
    sender.lng = "121.281379".to_string();
    sender.lat = "31.049363".to_string();
    request.sender = sender;
    let mut receiver = Receiver::default();
    receiver.coordinate_type = 1;
    receiver.city = "北京市".to_string();
    receiver.address = "海淀区".to_string();
    receiver.name = "顺丰同城".to_string();
    receiver.phone = "16600008829".to_string();
    request.receiver = receiver;
    let mut cargo = Cargo::default();
    cargo.cargo_first_class = "电商".to_string();
    cargo.cargo_second_class = "线上商城".to_string();
    cargo.goods_height = "1".to_string();
    cargo.goods_length = "3".to_string();
    cargo.goods_value = "5".to_string();
    cargo.goods_weight = "1".to_string();
    cargo.goods_width = "2".to_string();
    let mut goods1 = Goods::default();
    goods1.good_count = 1;
    goods1.good_name = "水果".to_string();
    goods1.good_price = "10".to_string();
    let mut goods2 = Goods::default();
    goods2.good_count = 2;
    goods2.good_name = "蔬菜".to_string();
    goods2.good_price = "20".to_string();
    let mut goods_detail = GoodsDetail::default();
    goods_detail.goods = vec![goods1, goods2];
    cargo.goods_detail = goods_detail;
    request.cargo = cargo;
    let mut shop = DeliveryShop::default();
    shop.goods_count = 3;
    shop.goods_name = "商品".to_string();
    shop.img_url = "https://".to_string();
    shop.wxa_path = "pages/index/index".to_string();
    request.shop = shop;

    let response: AddOrderResponse = delivery_service
        .add_order(&request)
        .await
        .expect("下配送单成功");
    assert_eq!(response.result_code, 0);
    assert_eq!(response.waybill_id, "WB_20240601");
    assert_eq!(response.order_status, 101);
    let body = last_body_json(&server);
    assert_eq!(body["shopid"], "shopId");
    assert_eq!(body["appSecret"], "secret");
    assert_eq!(body["delivery_id"], "SFTC");
    assert_eq!(body["shop_order_id"], "order_001");
    let expected_sign = hex::encode(sha1::Sha1::digest(b"shopIdorder_001secret"));
    assert_eq!(body["delivery_sign"], expected_sign);
    assert_eq!(body["order_info"]["order_time"], 1_700_000_000);
    assert_eq!(body["sender"]["city"], "上海市");
    // Java BigDecimal → JSON 数字（对应 `assert_eq!(body["sender"]["lng"], 121.281379)`）
    assert_eq!(body["sender"]["lng"], 121.281379_f64);
    assert_eq!(body["receiver"]["coordinate_type"], 1);
    assert_eq!(body["cargo"]["cargo_first_class"], "电商");
    assert_eq!(body["cargo"]["goods_detail"]["goods"][0]["good_count"], 1);
    assert_eq!(body["shop"]["goods_count"], 3);
    assert_eq!(body["shop"]["wxa_path"], "pages/index/index");

    // Java testGetOrder：getOrder(request)（shopId/shopNo/appSecret/shopOrderId）
    let mut get_request = GetOrderRequest::default();
    get_request.shop_id = "shopId".to_string();
    get_request.shop_no = "shopNo_1".to_string();
    get_request.app_secret = "secret".to_string();
    get_request.shop_order_id = "order_001".to_string();
    let get_response: GetOrderResponse = delivery_service
        .get_order(&get_request)
        .await
        .expect("拉取配送单成功");
    assert_eq!(get_response.result_code, 0);
    assert_eq!(get_response.order_status, 102);
    assert_eq!(get_response.waybill_id, "WB_20240601");
    assert_eq!(get_response.rider_name, "骑手小王");
    let body = last_body_json(&server);
    assert_eq!(body["shopid"], "shopId");
    assert_eq!(body["shop_order_id"], "order_001");
    assert_eq!(body["delivery_sign"], expected_sign);
}

// ---- customservice_work 微信客服（镜像 Java WxMaCustomserviceWorkServiceImpl
//      getCustomservice / bindCustomservice / unbindCustomservice） ----

#[tokio::test]
async fn customservice_work_get_bind_and_unbind() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/customservice/work/get") {
            r#"{"errcode":0,"entityName":"测试客服","corpid":"corp_1","bindTime":1700000000}"#
                .to_string()
        } else {
            // bind / unbind（注意 /unbind 含 /bind 子串，响应相同）均返回绑定信息
            r#"{"errcode":0,"entityName":"测试客服","corpid":"corp_1","bindTime":1700000000}"#
                .to_string()
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let customservice_service = service
        .customservice_work_service()
        .expect("微信客服服务存在");

    // Java getCustomservice：GET 查询绑定情况（GET 请求无请求体）
    let result: WxMaCustomserviceResult = customservice_service
        .get_customservice()
        .await
        .expect("查询客服绑定成功");
    assert_eq!(result.err_code, 0);
    assert_eq!(result.entity_name, "测试客服");
    assert_eq!(result.corpid, "corp_1");
    assert_eq!(result.bind_time, 1_700_000_000);

    // Java bindCustomservice(corpid)：{"corpid": corpid}
    let result: WxMaCustomserviceResult = customservice_service
        .bind_customservice("corp_1")
        .await
        .expect("绑定客服成功");
    assert_eq!(result.err_code, 0);
    assert_eq!(result.corpid, "corp_1");
    let body = last_body_json(&server);
    assert_eq!(body["corpid"], "corp_1");

    // Java unbindCustomservice(corpid)：{"corpid": corpid}
    let result: WxMaCustomserviceResult = customservice_service
        .unbind_customservice("corp_1")
        .await
        .expect("解除绑定成功");
    assert_eq!(result.err_code, 0);
    let body = last_body_json(&server);
    assert_eq!(body["corpid"], "corp_1");
}
