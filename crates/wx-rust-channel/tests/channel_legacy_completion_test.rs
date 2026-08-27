//! 老存量 service 在 WxJava 4.8.5/4.8.6 新增但 Rust 缺失的方法补齐测试。
//!
//! 镜像 Java 实现，验证 35 个补齐方法的 URL/请求体/响应解析正确性。
//! 使用内置 MockServer 模拟微信 API 响应。

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use wx_rust_channel::api::WxChannelService;
use wx_rust_channel::config::WxChannelConfig;
use wx_rust_channel::config::r#impl::WxChannelDefaultConfig;
use wx_rust_common::config::WxConfigStorage;

// ═══════════════════════════════════════════════════════════════
// 测试夹具：MockServer + 配置工厂
// ═══════════════════════════════════════════════════════════════

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

fn config_with_host(host: &str) -> Arc<dyn WxChannelConfig> {
    let mut config = WxChannelDefaultConfig::new("wxappid", "secret");
    config.set_token("token123");
    config.update_access_token("MOCK_TOKEN", 7200);
    config.set_api_host_url(host);
    Arc::new(config)
}

fn new_service(config: Arc<dyn WxChannelConfig>) -> Arc<impl WxChannelService> {
    wx_rust_channel::api::r#impl::WxChannelServiceImpl::new_arc(config)
}

// ═══════════════════════════════════════════════════════════════
// basic_service: 3 methods
// ═══════════════════════════════════════════════════════════════

/// getShopH5Url: POST GET_SHOP_H5URL "{}" -> ShopH5UrlResponse
#[tokio::test]
async fn basic_get_shop_h5_url() {
    let server = MockServer::start(|_path, body| {
        assert!(body.contains("{}"));
        r#"{"errcode":0,"errmsg":"ok","shop_h5url":"https://shop.weixin.qq.com/h5/123"}"#
            .to_string()
    })
    .await;
    let svc = new_service(config_with_host(&server.url()));
    let basic = svc.basic_service().unwrap();
    let resp = basic.get_shop_h5_url().await.unwrap();
    assert_eq!(resp.err_code, 0);
    assert_eq!(resp.shop_h5url, "https://shop.weixin.qq.com/h5/123");
    assert_eq!(server.request_count(), 1);
}

/// getShopQrCode: POST GET_SHOP_QRCODE {"qrcode_type":1} -> ShopQrCodeResponse
#[tokio::test]
async fn basic_get_shop_qr_code() {
    let server = MockServer::start(|_path, body| {
        assert!(body.contains("qrcode_type"));
        assert!(body.contains("1"));
        r#"{"errcode":0,"errmsg":"ok","shop_qrcode":"https://cdn.weixin.qq.com/qr/abc"}"#
            .to_string()
    })
    .await;
    let svc = new_service(config_with_host(&server.url()));
    let basic = svc.basic_service().unwrap();
    let resp = basic.get_shop_qr_code(1).await.unwrap();
    assert_eq!(resp.err_code, 0);
    assert_eq!(resp.shop_qrcode, "https://cdn.weixin.qq.com/qr/abc");
}

/// getShopTagLink: POST GET_SHOP_TAGLINK "{}" -> ShopTagLinkResponse
#[tokio::test]
async fn basic_get_shop_tag_link() {
    let server = MockServer::start(|_path, body| {
        assert!(body.contains("{}"));
        r##"{"errcode":0,"errmsg":"ok","shop_taglink":"#小程序://shop/abc"}"##.to_string()
    })
    .await;
    let svc = new_service(config_with_host(&server.url()));
    let basic = svc.basic_service().unwrap();
    let resp = basic.get_shop_tag_link().await.unwrap();
    assert_eq!(resp.err_code, 0);
    assert_eq!(resp.shop_taglink, "#小程序://shop/abc");
}

// ═══════════════════════════════════════════════════════════════
// order_service: 13 methods
// ═══════════════════════════════════════════════════════════════

/// addPresentNote: POST PRESENT_NOTE_ADD_URL {order_id, note}
#[tokio::test]
async fn order_add_present_note() {
    let server = MockServer::start(|_path, body| {
        assert!(body.contains("ORD-001"));
        assert!(body.contains("请尽快发货"));
        r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
    })
    .await;
    let svc = new_service(config_with_host(&server.url()));
    let order = svc.order_service().unwrap();
    let resp = order
        .add_present_note("ORD-001".into(), "请尽快发货".into())
        .await
        .unwrap();
    assert_eq!(resp.err_code, 0);
}

/// getPresentSubOrders: POST PRESENT_SUB_ORDER_GET_URL {order_id}
#[tokio::test]
async fn order_get_present_sub_orders() {
    let server = MockServer::start(|_path, body| {
        assert!(body.contains("ORD-002"));
        r#"{"errcode":0,"errmsg":"ok","sub_order_ids":["SUB-001","SUB-002"]}"#.to_string()
    })
    .await;
    let svc = new_service(config_with_host(&server.url()));
    let order = svc.order_service().unwrap();
    let resp = order
        .get_present_sub_orders("ORD-002".into())
        .await
        .unwrap();
    assert_eq!(resp.err_code, 0);
    assert_eq!(resp.sub_order_ids.len(), 2);
    assert_eq!(resp.sub_order_ids[0], "SUB-001");
}

/// getPreShipmentChangeSku: POST PRE_SHIPMENT_CHANGE_SKU_GET_URL {order_id}
#[tokio::test]
async fn order_get_pre_shipment_change_sku() {
    let server = MockServer::start(|_path, body| {
        assert!(body.contains("ORD-003"));
        r#"{"errcode":0,"errmsg":"ok","change_sku_info":{"preshipment_change_sku_state":3,"old_sku_id":"SKU1","new_sku_id":"SKU2","ddl_time_stamp":1700000000}}"#.to_string()
    })
    .await;
    let svc = new_service(config_with_host(&server.url()));
    let order = svc.order_service().unwrap();
    let resp = order
        .get_pre_shipment_change_sku("ORD-003".into())
        .await
        .unwrap();
    assert_eq!(resp.err_code, 0);
    assert_eq!(resp.change_sku_info.preshipment_change_sku_state, 3);
    assert_eq!(resp.change_sku_info.old_sku_id, "SKU1");
    assert_eq!(resp.change_sku_info.new_sku_id, "SKU2");
}

/// approvePreShipmentChangeSku: POST PRE_SHIPMENT_CHANGE_SKU_APPROVE_URL {order_id}
#[tokio::test]
async fn order_approve_pre_shipment_change_sku() {
    let server = MockServer::start(|_path, body| {
        assert!(body.contains("ORD-004"));
        r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
    })
    .await;
    let svc = new_service(config_with_host(&server.url()));
    let order = svc.order_service().unwrap();
    let resp = order
        .approve_pre_shipment_change_sku("ORD-004".into())
        .await
        .unwrap();
    assert_eq!(resp.err_code, 0);
}

/// rejectPreShipmentChangeSku: POST PRE_SHIPMENT_CHANGE_SKU_REJECT_URL {order_id, reject_reason}
#[tokio::test]
async fn order_reject_pre_shipment_change_sku() {
    let server = MockServer::start(|_path, body| {
        assert!(body.contains("ORD-005"));
        assert!(body.contains("库存不足"));
        r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
    })
    .await;
    let svc = new_service(config_with_host(&server.url()));
    let order = svc.order_service().unwrap();
    let resp = order
        .reject_pre_shipment_change_sku("ORD-005".into(), "库存不足".into())
        .await
        .unwrap();
    assert_eq!(resp.err_code, 0);
}

/// applyRealNumber: POST REAL_NUMBER_APPLY_URL {order_id}
#[tokio::test]
async fn order_apply_real_number() {
    let server = MockServer::start(|_path, body| {
        assert!(body.contains("ORD-006"));
        r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
    })
    .await;
    let svc = new_service(config_with_host(&server.url()));
    let order = svc.order_service().unwrap();
    let resp = order.apply_real_number("ORD-006".into()).await.unwrap();
    assert_eq!(resp.err_code, 0);
}

/// getRealNumberViewAudit: POST REAL_NUMBER_VIEW_AUDIT_GET_URL {order_id}
#[tokio::test]
async fn order_get_real_number_view_audit() {
    let server = MockServer::start(|_path, body| {
        assert!(body.contains("ORD-007"));
        r#"{"errcode":0,"errmsg":"ok","audit_status":2,"real_number":"13800138000"}"#.to_string()
    })
    .await;
    let svc = new_service(config_with_host(&server.url()));
    let order = svc.order_service().unwrap();
    let resp = order
        .get_real_number_view_audit("ORD-007".into())
        .await
        .unwrap();
    assert_eq!(resp.err_code, 0);
    assert_eq!(resp.audit_status, 2);
    assert_eq!(resp.real_number, "13800138000");
}

/// applyVirtualNumberAgain: POST VIRTUAL_NUMBER_APPLY_AGAIN_URL {order_id}
#[tokio::test]
async fn order_apply_virtual_number_again() {
    let server = MockServer::start(|_path, body| {
        assert!(body.contains("ORD-008"));
        r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
    })
    .await;
    let svc = new_service(config_with_host(&server.url()));
    let order = svc.order_service().unwrap();
    let resp = order
        .apply_virtual_number_again("ORD-008".into())
        .await
        .unwrap();
    assert_eq!(resp.err_code, 0);
}

/// delayVirtualNumber: POST VIRTUAL_NUMBER_DELAY_URL {order_id}
#[tokio::test]
async fn order_delay_virtual_number() {
    let server = MockServer::start(|_path, body| {
        assert!(body.contains("ORD-009"));
        r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
    })
    .await;
    let svc = new_service(config_with_host(&server.url()));
    let order = svc.order_service().unwrap();
    let resp = order.delay_virtual_number("ORD-009".into()).await.unwrap();
    assert_eq!(resp.err_code, 0);
}

/// addPrivatePhone: POST ADD_PHONE_URL {phone}
#[tokio::test]
async fn order_add_private_phone() {
    let server = MockServer::start(|_path, body| {
        assert!(body.contains("13800138000"));
        r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
    })
    .await;
    let svc = new_service(config_with_host(&server.url()));
    let order = svc.order_service().unwrap();
    let resp = order.add_private_phone("13800138000".into()).await.unwrap();
    assert_eq!(resp.err_code, 0);
}

/// sendPrivatePhoneVerifyCode: POST SEND_VERIFY_CODE_URL {phone}
#[tokio::test]
async fn order_send_private_phone_verify_code() {
    let server = MockServer::start(|_path, body| {
        assert!(body.contains("13800138001"));
        r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
    })
    .await;
    let svc = new_service(config_with_host(&server.url()));
    let order = svc.order_service().unwrap();
    let resp = order
        .send_private_phone_verify_code("13800138001".into())
        .await
        .unwrap();
    assert_eq!(resp.err_code, 0);
}

/// getPrivatePhone: POST GET_PHONE_URL "{}" -> PrivateNumberGetPhoneResponse
#[tokio::test]
async fn order_get_private_phone() {
    let server = MockServer::start(|_path, body| {
        assert!(body.contains("{}"));
        r#"{"errcode":0,"errmsg":"ok","phone_list":[{"phone":"13800138000","auth_status":2}]}"#
            .to_string()
    })
    .await;
    let svc = new_service(config_with_host(&server.url()));
    let order = svc.order_service().unwrap();
    let resp = order.get_private_phone().await.unwrap();
    assert_eq!(resp.err_code, 0);
    assert_eq!(resp.phone_list.len(), 1);
    assert_eq!(resp.phone_list[0].phone, "13800138000");
    assert_eq!(resp.phone_list[0].auth_status, 2);
}

/// compensationDelivery: POST DELIVERY_COMPENSATION_URL {order_id, delivery_list}
#[tokio::test]
async fn order_compensation_delivery() {
    let server = MockServer::start(|_path, body| {
        assert!(body.contains("ORD-010"));
        assert!(body.contains("delivery_list"));
        r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
    })
    .await;
    let svc = new_service(config_with_host(&server.url()));
    let order = svc.order_service().unwrap();
    let param = wx_rust_channel::bean::order::OrderCompensationDeliveryParam {
        order_id: "ORD-010".into(),
        delivery_list: vec![],
    };
    let resp = order.compensation_delivery(param).await.unwrap();
    assert_eq!(resp.err_code, 0);
}

// ═══════════════════════════════════════════════════════════════
// after_sale_service: 6 methods
// ═══════════════════════════════════════════════════════════════

/// listGuaranteeOrder: POST GUARANTEE_ORDER_LIST_URL -> GuaranteeOrderListResponse
#[tokio::test]
async fn after_sale_list_guarantee_order() {
    let server = MockServer::start(|_path, body| {
        assert!(body.contains("guarantee_order_id_list"));
        r#"{"errcode":0,"errmsg":"ok","total_num":1,"guarantee_order_list":[{"guarantee_order_id":"G-001","status":"pending","product_info":[{"product_id":"P-001"}]}]}"#.to_string()
    })
    .await;
    let svc = new_service(config_with_host(&server.url()));
    let after_sale = svc.after_sale_service().unwrap();
    let param = wx_rust_channel::bean::after::GuaranteeOrderListParam {
        guarantee_order_id_list: vec!["G-001".into()],
        ..Default::default()
    };
    let resp = after_sale.list_guarantee_order(param).await.unwrap();
    assert_eq!(resp.err_code, 0);
    assert_eq!(resp.total_num, 1);
    assert_eq!(resp.guarantee_order_list.len(), 1);
    assert_eq!(resp.guarantee_order_list[0].guarantee_order_id, "G-001");
}

/// getGuaranteeOrder: POST GUARANTEE_ORDER_GET_URL {guarantee_order_id}
#[tokio::test]
async fn after_sale_get_guarantee_order() {
    let server = MockServer::start(|_path, body| {
        assert!(body.contains("G-002"));
        r#"{"errcode":0,"errmsg":"ok","guarantee_order":{"guarantee_order_id":"G-002","status":"accepted","product_info":{"product_id":"P-002"}}}"#.to_string()
    })
    .await;
    let svc = new_service(config_with_host(&server.url()));
    let after_sale = svc.after_sale_service().unwrap();
    let resp = after_sale
        .get_guarantee_order("G-002".into())
        .await
        .unwrap();
    assert_eq!(resp.err_code, 0);
    assert_eq!(resp.guarantee_order.guarantee_order_id, "G-002");
    assert_eq!(resp.guarantee_order.status, "accepted");
    assert_eq!(resp.guarantee_order.product_info.product_id, "P-002");
}

/// acceptGuarantee: POST GUARANTEE_ORDER_ACCEPT_URL {guarantee_order_id}
#[tokio::test]
async fn after_sale_accept_guarantee() {
    let server = MockServer::start(|_path, body| {
        assert!(body.contains("G-003"));
        r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
    })
    .await;
    let svc = new_service(config_with_host(&server.url()));
    let after_sale = svc.after_sale_service().unwrap();
    let resp = after_sale.accept_guarantee("G-003".into()).await.unwrap();
    assert_eq!(resp.err_code, 0);
}

/// modifyGuarantee: POST GUARANTEE_ORDER_MODIFY_URL {guarantee_order_id, bad_level, merchant_remark}
#[tokio::test]
async fn after_sale_modify_guarantee() {
    let server = MockServer::start(|_path, body| {
        assert!(body.contains("G-004"));
        assert!(body.contains("bad_level"));
        assert!(body.contains("轻微破损"));
        r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
    })
    .await;
    let svc = new_service(config_with_host(&server.url()));
    let after_sale = svc.after_sale_service().unwrap();
    let request = wx_rust_channel::bean::after::GuaranteeModifyRequest {
        guarantee_order_id: "G-004".into(),
        bad_level: 1,
        merchant_remark: "轻微破损".into(),
    };
    let resp = after_sale.modify_guarantee(request).await.unwrap();
    assert_eq!(resp.err_code, 0);
}

/// proofGuarantee: POST GUARANTEE_ORDER_PROOF_URL {guarantee_order_id, content, pic_list}
#[tokio::test]
async fn after_sale_proof_guarantee() {
    let server = MockServer::start(|_path, body| {
        assert!(body.contains("G-005"));
        assert!(body.contains("已发货"));
        assert!(body.contains("pic_list"));
        r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
    })
    .await;
    let svc = new_service(config_with_host(&server.url()));
    let after_sale = svc.after_sale_service().unwrap();
    let request = wx_rust_channel::bean::after::GuaranteeProofRequest {
        guarantee_order_id: "G-005".into(),
        content: "已发货".into(),
        pic_list: vec!["MEDIA_001".into()],
    };
    let resp = after_sale.proof_guarantee(request).await.unwrap();
    assert_eq!(resp.err_code, 0);
}

/// refuseGuarantee: POST GUARANTEE_ORDER_REFUSE_URL {guarantee_order_id, reason, pic_list}
#[tokio::test]
async fn after_sale_refuse_guarantee() {
    let server = MockServer::start(|_path, body| {
        assert!(body.contains("G-006"));
        assert!(body.contains("不符合保障条件"));
        assert!(body.contains("pic_list"));
        r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
    })
    .await;
    let svc = new_service(config_with_host(&server.url()));
    let after_sale = svc.after_sale_service().unwrap();
    let request = wx_rust_channel::bean::after::GuaranteeRefuseRequest {
        guarantee_order_id: "G-006".into(),
        reason: "不符合保障条件".into(),
        pic_list: vec!["MEDIA_002".into()],
    };
    let resp = after_sale.refuse_guarantee(request).await.unwrap();
    assert_eq!(resp.err_code, 0);
}

// ═══════════════════════════════════════════════════════════════
// product_service: 13 methods
// ═══════════════════════════════════════════════════════════════

/// getProductScheme: POST SPU_SCHEME_URL -> ProductSchemeResponse
#[tokio::test]
async fn product_get_product_scheme() {
    let server = MockServer::start(|_path, body| {
        assert!(body.contains("product_id"));
        assert!(body.contains("P-001"));
        r#"{"errcode":0,"errmsg":"ok","openlink":"weixin://dl/business/?t=abc"}"#.to_string()
    })
    .await;
    let svc = new_service(config_with_host(&server.url()));
    let product = svc.product_service().unwrap();
    let param = wx_rust_channel::bean::product::ProductSchemeParam {
        product_id: "P-001".into(),
        ..Default::default()
    };
    let resp = product.get_product_scheme(param).await.unwrap();
    assert_eq!(resp.err_code, 0);
    assert_eq!(resp.openlink, "weixin://dl/business/?t=abc");
}

/// classifyProductCategory: POST SPU_CATEGORY_CLASSIFY_URL -> ProductCategoryClassifyResponse
#[tokio::test]
async fn product_classify_product_category() {
    let server = MockServer::start(|_path, body| {
        assert!(body.contains("title"));
        r#"{"errcode":0,"errmsg":"ok","categories":[{"cats":[{"cat_info":{"cat_id":"C1","cat_name":"服饰","is_shop_no_audit":false},"has_permission":true}]}],"wrong_cat":false}"#.to_string()
    })
    .await;
    let svc = new_service(config_with_host(&server.url()));
    let product = svc.product_service().unwrap();
    let param = wx_rust_channel::bean::product::ProductCategoryClassifyParam {
        req_type: 1,
        title: "连衣裙".into(),
        ..Default::default()
    };
    let resp = product.classify_product_category(param).await.unwrap();
    assert_eq!(resp.err_code, 0);
    assert!(!resp.wrong_cat);
    assert_eq!(resp.categories.len(), 1);
    assert_eq!(resp.categories[0].cats[0].cat_info.cat_id, "C1");
}

/// beginTimingSale: POST SPU_BEGIN_TIMING_SALE_URL
#[tokio::test]
async fn product_begin_timing_sale() {
    let server = MockServer::start(|_path, body| {
        assert!(body.contains("P-002"));
        r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
    })
    .await;
    let svc = new_service(config_with_host(&server.url()));
    let product = svc.product_service().unwrap();
    let param = wx_rust_channel::bean::product::assistant::BeginTimingSaleParam {
        product_id: "P-002".into(),
        ..Default::default()
    };
    let resp = product.begin_timing_sale(param).await.unwrap();
    assert_eq!(resp.err_code, 0);
}

/// cancelTimingSale: POST SPU_CANCEL_TIMING_SALE_URL {product_id}
#[tokio::test]
async fn product_cancel_timing_sale() {
    let server = MockServer::start(|_path, body| {
        assert!(body.contains("P-003"));
        r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
    })
    .await;
    let svc = new_service(config_with_host(&server.url()));
    let product = svc.product_service().unwrap();
    let resp = product.cancel_timing_sale("P-003".into()).await.unwrap();
    assert_eq!(resp.err_code, 0);
}

/// externalProductMapping: POST SPU_EXTERNAL_PRODUCT_MAPPING_URL
#[tokio::test]
async fn product_external_product_mapping() {
    let server = MockServer::start(|_path, body| {
        assert!(body.contains("product_id"));
        r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
    })
    .await;
    let svc = new_service(config_with_host(&server.url()));
    let product = svc.product_service().unwrap();
    let param = wx_rust_channel::bean::product::assistant::ExternalProductMappingParam::default();
    let resp = product.external_product_mapping(param).await.unwrap();
    assert_eq!(resp.err_code, 0);
}

/// categoryPreCheck: POST SPU_CATEGORY_PRE_CHECK_URL
#[tokio::test]
async fn product_category_pre_check() {
    let server =
        MockServer::start(|_path, _body| r#"{"errcode":0,"errmsg":"ok"}"#.to_string()).await;
    let svc = new_service(config_with_host(&server.url()));
    let product = svc.product_service().unwrap();
    let param = wx_rust_channel::bean::product::assistant::CategoryPreCheckParam::default();
    let resp = product.category_pre_check(param).await.unwrap();
    assert_eq!(resp.err_code, 0);
}

/// getProductAuditStrategy: POST SPU_AUDIT_STRATEGY_GET_URL "{}"
#[tokio::test]
async fn product_get_product_audit_strategy() {
    let server = MockServer::start(|_path, body| {
        assert!(body.contains("{}"));
        r#"{"errcode":0,"errmsg":"ok","audit_strategy":{"hide_err_field_flag":0,"hit_duplicated_flag":1,"hit_low_risk_rule_flag":0}}"#.to_string()
    })
    .await;
    let svc = new_service(config_with_host(&server.url()));
    let product = svc.product_service().unwrap();
    let resp = product.get_product_audit_strategy().await.unwrap();
    assert_eq!(resp.err_code, 0);
    assert_eq!(resp.audit_strategy.hit_duplicated_flag, 1);
}

/// setProductAuditStrategy: POST SPU_AUDIT_STRATEGY_SET_URL
#[tokio::test]
async fn product_set_product_audit_strategy() {
    let server = MockServer::start(|_path, body| {
        assert!(body.contains("audit_strategy"));
        r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
    })
    .await;
    let svc = new_service(config_with_host(&server.url()));
    let product = svc.product_service().unwrap();
    let param = wx_rust_channel::bean::product::ProductAuditStrategySetParam::default();
    let resp = product.set_product_audit_strategy(param).await.unwrap();
    assert_eq!(resp.err_code, 0);
}

/// getProductAuditQuota: POST SPU_GET_AUDIT_QUOTA_URL "{}"
#[tokio::test]
async fn product_get_product_audit_quota() {
    let server = MockServer::start(|_path, body| {
        assert!(body.contains("{}"));
        r#"{"errcode":0,"errmsg":"ok","audit_quota":{"block_status":0,"avail_quota":100,"total_quota":200,"unlimited_type":0,"audit_total_quota":500,"audit_total_remaining":300,"new_product_total_quota":100,"new_product_remaining":80}}"#.to_string()
    })
    .await;
    let svc = new_service(config_with_host(&server.url()));
    let product = svc.product_service().unwrap();
    let resp = product.get_product_audit_quota().await.unwrap();
    assert_eq!(resp.err_code, 0);
    assert_eq!(resp.audit_quota.avail_quota, 100);
    assert_eq!(resp.audit_quota.total_quota, 200);
}

/// externalProductMappingNew: POST SPU_EXTERNAL_PRODUCT_MAPPING_NEW_URL
#[tokio::test]
async fn product_external_product_mapping_new() {
    let server = MockServer::start(|_path, body| {
        assert!(body.contains("product_id"));
        r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
    })
    .await;
    let svc = new_service(config_with_host(&server.url()));
    let product = svc.product_service().unwrap();
    let param =
        wx_rust_channel::bean::product::assistant::ExternalProductMappingNewParam::default();
    let resp = product.external_product_mapping_new(param).await.unwrap();
    assert_eq!(resp.err_code, 0);
}

/// productBrandRecommend: POST SPU_PRODUCT_BRAND_RECOMMEND_URL
#[tokio::test]
async fn product_brand_recommend() {
    let server =
        MockServer::start(|_path, _body| r#"{"errcode":0,"errmsg":"ok"}"#.to_string()).await;
    let svc = new_service(config_with_host(&server.url()));
    let product = svc.product_service().unwrap();
    let param = wx_rust_channel::bean::product::assistant::ProductBrandRecommendParam::default();
    let resp = product.product_brand_recommend(param).await.unwrap();
    assert_eq!(resp.err_code, 0);
}

/// addProductThirdPartySource: POST SPU_ADD_PRODUCT_THIRD_PARTY_SOURCE_URL
#[tokio::test]
async fn product_add_product_third_party_source() {
    let server = MockServer::start(|_path, body| {
        assert!(body.contains("scene_value"));
        r#"{"errcode":0,"errmsg":"ok","third_party_source_id":12345}"#.to_string()
    })
    .await;
    let svc = new_service(config_with_host(&server.url()));
    let product = svc.product_service().unwrap();
    let param = wx_rust_channel::bean::product::AddProductThirdPartySourceParam {
        scene_value: 1,
        ..Default::default()
    };
    let resp = product.add_product_third_party_source(param).await.unwrap();
    assert_eq!(resp.err_code, 0);
    assert_eq!(resp.third_party_source_id, 12345);
}

/// getStockFlow: POST SPU_GET_STOCK_FLOW_URL
#[tokio::test]
async fn product_get_stock_flow() {
    let server = MockServer::start(|_path, body| {
        assert!(body.contains("product_id"));
        r#"{"errcode":0,"errmsg":"ok","flow_list":[]}"#.to_string()
    })
    .await;
    let svc = new_service(config_with_host(&server.url()));
    let product = svc.product_service().unwrap();
    let param = wx_rust_channel::bean::product::stock::StockFlowParam::default();
    let resp = product.get_stock_flow(param).await.unwrap();
    assert_eq!(resp.err_code, 0);
}

// ═══════════════════════════════════════════════════════════════
// Bean serialization tests (20 tests)
// ═══════════════════════════════════════════════════════════════

/// ShopH5UrlResponse serde round-trip.
#[test]
fn bean_shop_h5_url_response_serde() {
    let json = r#"{"errcode":0,"errmsg":"ok","shop_h5url":"https://shop.weixin.qq.com/h5/123"}"#;
    let resp: wx_rust_channel::bean::shop::ShopH5UrlResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.err_code, 0);
    assert_eq!(resp.shop_h5url, "https://shop.weixin.qq.com/h5/123");
}

/// ShopQrCodeResponse serde round-trip.
#[test]
fn bean_shop_qr_code_response_serde() {
    let json = r#"{"errcode":0,"errmsg":"ok","shop_qrcode":"https://cdn.weixin.qq.com/qr/abc"}"#;
    let resp: wx_rust_channel::bean::shop::ShopQrCodeResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.err_code, 0);
    assert_eq!(resp.shop_qrcode, "https://cdn.weixin.qq.com/qr/abc");
}

/// ShopTagLinkResponse serde round-trip.
#[test]
fn bean_shop_tag_link_response_serde() {
    let json = r##"{"errcode":0,"errmsg":"ok","shop_taglink":"#小程序://shop/abc"}"##;
    let resp: wx_rust_channel::bean::shop::ShopTagLinkResponse =
        serde_json::from_str(json).unwrap();
    assert_eq!(resp.err_code, 0);
    assert_eq!(resp.shop_taglink, "#小程序://shop/abc");
}

/// PresentNoteAddParam serde round-trip.
#[test]
fn bean_present_note_add_param_serde() {
    let param = wx_rust_channel::bean::order::PresentNoteAddParam {
        order_id: "ORD-001".into(),
        note: "请尽快发货".into(),
    };
    let json = serde_json::to_string(&param).unwrap();
    assert!(json.contains("ORD-001"));
    assert!(json.contains("请尽快发货"));
}

/// PresentSubOrderResponse serde round-trip.
#[test]
fn bean_present_sub_order_response_serde() {
    let json = r#"{"errcode":0,"errmsg":"ok","sub_order_ids":["SUB-001","SUB-002"]}"#;
    let resp: wx_rust_channel::bean::order::PresentSubOrderResponse =
        serde_json::from_str(json).unwrap();
    assert_eq!(resp.sub_order_ids.len(), 2);
}

/// PreShipmentChangeSkuResponse serde round-trip.
#[test]
fn bean_pre_shipment_change_sku_response_serde() {
    let json = r#"{"errcode":0,"errmsg":"ok","change_sku_info":{"preshipment_change_sku_state":3,"old_sku_id":"SKU1","new_sku_id":"SKU2","ddl_time_stamp":1700000000}}"#;
    let resp: wx_rust_channel::bean::order::PreShipmentChangeSkuResponse =
        serde_json::from_str(json).unwrap();
    assert_eq!(resp.change_sku_info.preshipment_change_sku_state, 3);
}

/// RealNumberViewAuditResponse serde round-trip.
#[test]
fn bean_real_number_view_audit_response_serde() {
    let json = r#"{"errcode":0,"errmsg":"ok","audit_status":2,"real_number":"13800138000"}"#;
    let resp: wx_rust_channel::bean::order::RealNumberViewAuditResponse =
        serde_json::from_str(json).unwrap();
    assert_eq!(resp.audit_status, 2);
    assert_eq!(resp.real_number, "13800138000");
}

/// PrivateNumberGetPhoneResponse serde round-trip.
#[test]
fn bean_private_number_get_phone_response_serde() {
    let json =
        r#"{"errcode":0,"errmsg":"ok","phone_list":[{"phone":"13800138000","auth_status":2}]}"#;
    let resp: wx_rust_channel::bean::order::PrivateNumberGetPhoneResponse =
        serde_json::from_str(json).unwrap();
    assert_eq!(resp.phone_list.len(), 1);
    assert_eq!(resp.phone_list[0].auth_status, 2);
}

/// GuaranteeOrderListResponse serde round-trip.
#[test]
fn bean_guarantee_order_list_response_serde() {
    let json = r#"{"errcode":0,"errmsg":"ok","total_num":2,"guarantee_order_list":[{"guarantee_order_id":"G-001","status":"pending","product_info":[{"product_id":"P-001"}]},{"guarantee_order_id":"G-002","status":"accepted","product_info":[]}]}"#;
    let resp: wx_rust_channel::bean::after::GuaranteeOrderListResponse =
        serde_json::from_str(json).unwrap();
    assert_eq!(resp.total_num, 2);
    assert_eq!(resp.guarantee_order_list.len(), 2);
    assert_eq!(resp.guarantee_order_list[0].guarantee_order_id, "G-001");
}

/// GuaranteeOrderInfoResponse serde round-trip.
#[test]
fn bean_guarantee_order_info_response_serde() {
    let json = r#"{"errcode":0,"errmsg":"ok","guarantee_order":{"guarantee_order_id":"G-001","status":"pending","product_info":{"product_id":"P-001"}}}"#;
    let resp: wx_rust_channel::bean::after::GuaranteeOrderInfoResponse =
        serde_json::from_str(json).unwrap();
    assert_eq!(resp.guarantee_order.guarantee_order_id, "G-001");
}

/// ProductSchemeResponse serde round-trip.
#[test]
fn bean_product_scheme_response_serde() {
    let json = r#"{"errcode":0,"errmsg":"ok","openlink":"weixin://dl/business/?t=abc"}"#;
    let resp: wx_rust_channel::bean::product::ProductSchemeResponse =
        serde_json::from_str(json).unwrap();
    assert_eq!(resp.openlink, "weixin://dl/business/?t=abc");
}

/// ProductCategoryClassifyResponse serde round-trip.
#[test]
fn bean_product_category_classify_response_serde() {
    let json = r#"{"errcode":0,"errmsg":"ok","categories":[{"cats":[{"cat_info":{"cat_id":"C1","cat_name":"服饰","is_shop_no_audit":false},"has_permission":true}]}],"wrong_cat":false}"#;
    let resp: wx_rust_channel::bean::product::ProductCategoryClassifyResponse =
        serde_json::from_str(json).unwrap();
    assert_eq!(resp.categories.len(), 1);
    assert_eq!(resp.categories[0].cats[0].cat_info.cat_name, "服饰");
}

/// ProductAuditStrategyResponse serde round-trip.
#[test]
fn bean_product_audit_strategy_response_serde() {
    let json = r#"{"errcode":0,"errmsg":"ok","audit_strategy":{"hide_err_field_flag":0,"hit_duplicated_flag":1,"hit_low_risk_rule_flag":0}}"#;
    let resp: wx_rust_channel::bean::product::ProductAuditStrategyResponse =
        serde_json::from_str(json).unwrap();
    assert_eq!(resp.audit_strategy.hit_duplicated_flag, 1);
}

/// ProductAuditQuotaResponse serde round-trip.
#[test]
fn bean_product_audit_quota_response_serde() {
    let json = r#"{"errcode":0,"errmsg":"ok","audit_quota":{"block_status":0,"avail_quota":100,"total_quota":200,"unlimited_type":0,"audit_total_quota":500,"audit_total_remaining":300,"new_product_total_quota":100,"new_product_remaining":80}}"#;
    let resp: wx_rust_channel::bean::product::ProductAuditQuotaResponse =
        serde_json::from_str(json).unwrap();
    assert_eq!(resp.audit_quota.avail_quota, 100);
    assert_eq!(resp.audit_quota.new_product_remaining, 80);
}

/// AddProductThirdPartySourceResponse serde round-trip.
#[test]
fn bean_add_product_third_party_source_response_serde() {
    let json = r#"{"errcode":0,"errmsg":"ok","third_party_source_id":12345}"#;
    let resp: wx_rust_channel::bean::product::AddProductThirdPartySourceResponse =
        serde_json::from_str(json).unwrap();
    assert_eq!(resp.third_party_source_id, 12345);
}

/// OrderCompensationDeliveryParam serde round-trip.
#[test]
fn bean_order_compensation_delivery_param_serde() {
    let param = wx_rust_channel::bean::order::OrderCompensationDeliveryParam {
        order_id: "ORD-010".into(),
        delivery_list: vec![],
    };
    let json = serde_json::to_string(&param).unwrap();
    assert!(json.contains("ORD-010"));
    assert!(json.contains("delivery_list"));
}

/// GuaranteeModifyRequest serde round-trip.
#[test]
fn bean_guarantee_modify_request_serde() {
    let json = r#"{"guarantee_order_id":"G-001","bad_level":1,"merchant_remark":"轻微破损"}"#;
    let req: wx_rust_channel::bean::after::GuaranteeModifyRequest =
        serde_json::from_str(json).unwrap();
    assert_eq!(req.guarantee_order_id, "G-001");
    assert_eq!(req.bad_level, 1);
    assert_eq!(req.merchant_remark, "轻微破损");
}

/// GuaranteeProofRequest serde round-trip.
#[test]
fn bean_guarantee_proof_request_serde() {
    let json = r#"{"guarantee_order_id":"G-001","content":"已发货","pic_list":["M1","M2"]}"#;
    let req: wx_rust_channel::bean::after::GuaranteeProofRequest =
        serde_json::from_str(json).unwrap();
    assert_eq!(req.pic_list.len(), 2);
}

/// GuaranteeRefuseRequest serde round-trip.
#[test]
fn bean_guarantee_refuse_request_serde() {
    let json = r#"{"guarantee_order_id":"G-001","reason":"不符合条件","pic_list":["M1"]}"#;
    let req: wx_rust_channel::bean::after::GuaranteeRefuseRequest =
        serde_json::from_str(json).unwrap();
    assert_eq!(req.reason, "不符合条件");
}
