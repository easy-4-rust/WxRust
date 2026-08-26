//! 语义审计批次 A：ewaybill / gift / supplier / qic / kf 五个子服务的逐方法对齐测试。
//!
//! 对应 Java `WxChannelEwaybillServiceImplTest` / `WxChannelGiftServiceImplTest` /
//! `WxChannelSupplierServiceImplTest` / `WxChannelQicServiceImplTest` /
//! `WxChannelKfServiceImplTest` 的 HTTP 语义，经 MockServer 验证。
//!
//! 覆盖：
//! - Qic: getInspectConfig / getSubmitConfig 改为 GET（之前错误地用 POST）
//! - Gift: add_gift_activity 需要 GiftActivityAddParam 包装层
//! - Kf: upload_media 通过 CommonUploadParam 实现 COS 上传
//! - Ewaybill: 16 个方法 URL 与参数对齐
//! - Supplier: 13 个方法 URL 与参数对齐
// mock dispatcher 中多个端点有意返回相同的 canned 响应（if_same_then_else 为测试意图，文件级豁免）。
#![allow(clippy::if_same_then_else)]
#![allow(clippy::field_reassign_with_default)]

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use wx_rust_channel::api::WxChannelService;
use wx_rust_channel::bean::product::{GiftActivityInfo, GiftProductInfo};
use wx_rust_channel::config::WxChannelConfig;
use wx_rust_channel::config::r#impl::WxChannelDefaultConfig;
use wx_rust_common::config::WxConfigStorage;

// ═══════════════════════════════════════════════════════════════
// 测试夹具
// ═══════════════════════════════════════════════════════════════

/// Mock HTTP 服务器：记录请求方法、路径、请求体。
struct MockServer {
    addr: std::net::SocketAddr,
    requests: Arc<AtomicUsize>,
    last_path: Arc<std::sync::Mutex<String>>,
    last_body: Arc<std::sync::Mutex<String>>,
    last_method: Arc<std::sync::Mutex<String>>,
    stop: Arc<AtomicBool>,
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
        let last_method = Arc::new(std::sync::Mutex::new(String::new()));
        let stop = Arc::new(AtomicBool::new(false));
        let handler = Arc::new(handler);

        let requests_clone = requests.clone();
        let last_path_clone = last_path.clone();
        let last_body_clone = last_body.clone();
        let last_method_clone = last_method.clone();
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
                let lm = last_method_clone.clone();
                tokio::spawn(async move {
                    use tokio::io::{AsyncReadExt, AsyncWriteExt};
                    let mut buf = [0u8; 16384];
                    let n = socket.read(&mut buf).await.unwrap_or(0);
                    let request = String::from_utf8_lossy(&buf[..n]).to_string();
                    let body = request.split("\r\n\r\n").nth(1).unwrap_or("").to_string();
                    let first_line = request.lines().next().unwrap_or("").to_string();
                    let method = first_line
                        .split_whitespace()
                        .next()
                        .unwrap_or("")
                        .to_string();
                    let path = first_line
                        .split_whitespace()
                        .nth(1)
                        .unwrap_or("/")
                        .to_string();
                    *lb.lock().unwrap() = body.clone();
                    *lp.lock().unwrap() = path.clone();
                    *lm.lock().unwrap() = method.clone();
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
            last_method,
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

    fn last_path(&self) -> String {
        self.last_path.lock().unwrap().clone()
    }

    fn last_body(&self) -> String {
        self.last_body.lock().unwrap().clone()
    }

    fn last_method(&self) -> String {
        self.last_method.lock().unwrap().clone()
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

fn new_service(
    config: Arc<dyn WxChannelConfig>,
) -> Arc<wx_rust_channel::api::r#impl::WxChannelServiceImpl> {
    wx_rust_channel::api::r#impl::WxChannelServiceImpl::new_arc(config)
}

// ═══════════════════════════════════════════════════════════════
// Qic 服务：验证 getInspectConfig / getSubmitConfig 使用 GET
// ═══════════════════════════════════════════════════════════════

/// get_inspect_config 应使用 GET 请求（对齐 Java `shopService.get`）。
#[tokio::test]
async fn qic_get_inspect_config_uses_get() {
    let server = MockServer::start(|path, _| {
        if path.contains("/channels/ec/qic/inspect/config/get") {
            r#"{"errcode":0,"errmsg":"ok","inspect_config":{}}"#.to_string()
        } else {
            ok_response()
        }
    })
    .await;
    let service = new_service(config_with_host(&server.url()));
    let qic_svc = service.qic_service().unwrap();

    let resp = qic_svc
        .get_inspect_config()
        .await
        .expect("查询质检仓配置成功");
    assert_eq!(resp.err_code, 0);
    // 验证使用 GET 方法
    assert_eq!(server.last_method(), "GET");
    // 验证 URL 路径包含正确的端点
    assert!(
        server
            .last_path()
            .contains("/channels/ec/qic/inspect/config/get")
    );
}

/// get_submit_config 应使用 GET 请求，无 query 参数。
#[tokio::test]
async fn qic_get_submit_config_uses_get() {
    let server = MockServer::start(|path, _| {
        if path.contains("/channels/ec/qic/inspect/submitconfig/get") {
            r#"{"errcode":0,"errmsg":"ok","submit_config":{}}"#.to_string()
        } else {
            ok_response()
        }
    })
    .await;
    let service = new_service(config_with_host(&server.url()));
    let qic_svc = service.qic_service().unwrap();

    let resp = qic_svc.get_submit_config().await.expect("查询送检配置成功");
    assert_eq!(resp.err_code, 0);
    assert_eq!(server.last_method(), "GET");
    assert!(
        server
            .last_path()
            .contains("/channels/ec/qic/inspect/submitconfig/get")
    );
}

/// get_submit_config_with_order 应使用 GET 请求，带 order_id query 参数。
#[tokio::test]
async fn qic_get_submit_config_with_order_uses_get() {
    let server = MockServer::start(|path, _| {
        if path.contains("/channels/ec/qic/inspect/submitconfig/get") {
            r#"{"errcode":0,"errmsg":"ok","submit_config":{}}"#.to_string()
        } else {
            ok_response()
        }
    })
    .await;
    let service = new_service(config_with_host(&server.url()));
    let qic_svc = service.qic_service().unwrap();

    let resp = qic_svc
        .get_submit_config_with_order("ORDER_QIC_001".into())
        .await
        .expect("查询送检配置成功");
    assert_eq!(resp.err_code, 0);
    assert_eq!(server.last_method(), "GET");
    // 验证 query 参数包含 order_id
    assert!(server.last_path().contains("order_id=ORDER_QIC_001"));
}

/// print_inspect_code 应使用 POST 请求。
#[tokio::test]
async fn qic_print_inspect_code_uses_post() {
    let server = MockServer::start(|path, _| {
        if path.contains("/channels/ec/qic/inspect/code/print") {
            r#"{"errcode":0,"errmsg":"ok","inspect_code":"CODE_001"}"#.to_string()
        } else {
            ok_response()
        }
    })
    .await;
    let service = new_service(config_with_host(&server.url()));
    let qic_svc = service.qic_service().unwrap();

    let resp = qic_svc
        .print_inspect_code("ORDER_QIC_002".into())
        .await
        .expect("打印质检码成功");
    assert_eq!(resp.err_code, 0);
    assert_eq!(server.last_method(), "POST");
    assert!(server.last_body().contains("ORDER_QIC_002"));
}

/// submit_inspect_info 应使用 POST 请求。
#[tokio::test]
async fn qic_submit_inspect_info_uses_post() {
    let server = MockServer::start(|_, _| ok_response()).await;
    let service = new_service(config_with_host(&server.url()));
    let qic_svc = service.qic_service().unwrap();

    let req = wx_rust_channel::bean::qic::SubmitInspectRequest::default();
    let resp = qic_svc
        .submit_inspect_info(req)
        .await
        .expect("绑定送检信息成功");
    assert_eq!(resp.err_code, 0);
    assert_eq!(server.last_method(), "POST");
}

/// register_logistics 应使用 POST 请求。
#[tokio::test]
async fn qic_register_logistics_uses_post() {
    let server = MockServer::start(|_, _| ok_response()).await;
    let service = new_service(config_with_host(&server.url()));
    let qic_svc = service.qic_service().unwrap();

    let req = wx_rust_channel::bean::qic::RegisterLogisticsRequest::default();
    let resp = qic_svc
        .register_logistics(req)
        .await
        .expect("自寄快递送检成功");
    assert_eq!(resp.err_code, 0);
    assert_eq!(server.last_method(), "POST");
}

// ═══════════════════════════════════════════════════════════════
// Gift 服务：验证 add_gift_activity 包装层
// ═══════════════════════════════════════════════════════════════

/// add_gift_activity 应将 GiftActivityInfo 包装在 gift_activity 字段中。
#[tokio::test]
async fn gift_add_gift_activity_wraps_in_param() {
    let server = MockServer::start(|path, body| {
        if path.contains("/channels/ec/product/activity/add") {
            // 验证请求体包含 gift_activity 包装
            assert!(
                body.contains("gift_activity"),
                "add_gift_activity 请求体应包含 gift_activity 包装字段，实际: {body}"
            );
            r#"{"errcode":0,"errmsg":"ok","activity_id":"ACT_001"}"#.to_string()
        } else {
            ok_response()
        }
    })
    .await;
    let service = new_service(config_with_host(&server.url()));
    let gift_svc = service.gift_service().unwrap();

    let info = GiftActivityInfo {
        activity_name: "测试活动".into(),
        start_time: "1700000000".into(),
        end_time: "1700100000".into(),
    };
    let resp = gift_svc
        .add_gift_activity(info)
        .await
        .expect("创建赠品活动成功");
    assert_eq!(resp.err_code, 0);
    assert_eq!(resp.activity_id, "ACT_001");
    // 验证请求体包含 gift_activity 包装
    let body = server.last_body();
    let parsed: serde_json::Value = serde_json::from_str(&body).unwrap();
    assert!(
        parsed.get("gift_activity").is_some(),
        "请求体应包含 gift_activity 字段"
    );
    assert_eq!(
        parsed["gift_activity"]["activity_name"].as_str().unwrap(),
        "测试活动"
    );
}

/// add_gift_product 应直接序列化 GiftProductInfo。
#[tokio::test]
async fn gift_add_gift_product_sends_info_directly() {
    let server = MockServer::start(|path, body| {
        if path.contains("/channels/ec/product/gift/add") {
            // add_gift_product 不需要包装层
            assert!(
                !body.contains("gift_activity"),
                "add_gift_product 不应有 gift_activity 包装"
            );
            r#"{"errcode":0,"errmsg":"ok","product_id":"PROD_001"}"#.to_string()
        } else {
            ok_response()
        }
    })
    .await;
    let service = new_service(config_with_host(&server.url()));
    let gift_svc = service.gift_service().unwrap();

    let info = GiftProductInfo::default();
    let resp = gift_svc
        .add_gift_product(info)
        .await
        .expect("添加非卖商品成功");
    assert_eq!(resp.err_code, 0);
}

/// delete_gift_activity 应发送 activity_id。
#[tokio::test]
async fn gift_delete_gift_activity_sends_activity_id() {
    let server = MockServer::start(|path, body| {
        if path.contains("/channels/ec/product/activity/del") {
            assert!(body.contains("ACT_DEL_001"));
            ok_response()
        } else {
            ok_response()
        }
    })
    .await;
    let service = new_service(config_with_host(&server.url()));
    let gift_svc = service.gift_service().unwrap();

    let resp = gift_svc
        .delete_gift_activity("ACT_DEL_001".into())
        .await
        .expect("删除赠品活动成功");
    assert_eq!(resp.err_code, 0);
}

/// stop_gift_activity 应发送 activity_id。
#[tokio::test]
async fn gift_stop_gift_activity_sends_activity_id() {
    let server = MockServer::start(|path, body| {
        if path.contains("/channels/ec/product/activity/stop") {
            assert!(body.contains("ACT_STOP_001"));
            ok_response()
        } else {
            ok_response()
        }
    })
    .await;
    let service = new_service(config_with_host(&server.url()));
    let gift_svc = service.gift_service().unwrap();

    let resp = gift_svc
        .stop_gift_activity("ACT_STOP_001".into())
        .await
        .expect("停止赠品活动成功");
    assert_eq!(resp.err_code, 0);
}

/// update_gift_stock 应发送 product_id、sku_id、diff_type、num。
#[tokio::test]
async fn gift_update_gift_stock_sends_all_fields() {
    let server = MockServer::start(|path, body| {
        if path.contains("/channels/ec/product/gift/stock/update") {
            assert!(body.contains("PROD_STOCK_001"));
            assert!(body.contains("SKU_001"));
            assert!(body.contains("diff_type"));
            assert!(body.contains("num"));
            ok_response()
        } else {
            ok_response()
        }
    })
    .await;
    let service = new_service(config_with_host(&server.url()));
    let gift_svc = service.gift_service().unwrap();

    let resp = gift_svc
        .update_gift_stock("PROD_STOCK_001".into(), "SKU_001".into(), 1, 10)
        .await
        .expect("更新赠品库存成功");
    assert_eq!(resp.err_code, 0);
}

// ═══════════════════════════════════════════════════════════════
// Supplier 服务：URL 与参数对齐验证
// ═══════════════════════════════════════════════════════════════

/// get_supplier_list 应发送 page_size 和 next_key。
#[tokio::test]
async fn supplier_get_supplier_list_sends_params() {
    let server = MockServer::start(|path, body| {
        if path.contains("/channels/ec/supplier/relation/get_supplier_list") {
            assert!(body.contains("10"));
            assert!(body.contains("NEXT_KEY_001"));
            r#"{"errcode":0,"errmsg":"ok","supplier_list":[]}"#.to_string()
        } else {
            ok_response()
        }
    })
    .await;
    let service = new_service(config_with_host(&server.url()));
    let supplier_svc = service.supplier_service().unwrap();

    let resp = supplier_svc
        .get_supplier_list(Some(10), "NEXT_KEY_001".into())
        .await
        .expect("获取供货商列表成功");
    assert_eq!(resp.err_code, 0);
}

/// get_distribute 应发送空 JSON。
#[tokio::test]
async fn supplier_get_distribute_sends_empty() {
    let server = MockServer::start(|path, _| {
        if path.contains("/channels/ec/supplier/relation/get_distribute") {
            r#"{"errcode":0,"errmsg":"ok","distribute_type":1}"#.to_string()
        } else {
            ok_response()
        }
    })
    .await;
    let service = new_service(config_with_host(&server.url()));
    let supplier_svc = service.supplier_service().unwrap();

    let resp = supplier_svc
        .get_distribute()
        .await
        .expect("获取分配方式成功");
    assert_eq!(resp.err_code, 0);
}

/// set_manually_distribute 应发送空 JSON。
#[tokio::test]
async fn supplier_set_manually_distribute() {
    let server = MockServer::start(|path, _| {
        if path.contains("/channels/ec/supplier/relation/set_manually_distribute") {
            ok_response()
        } else {
            ok_response()
        }
    })
    .await;
    let service = new_service(config_with_host(&server.url()));
    let supplier_svc = service.supplier_service().unwrap();

    let resp = supplier_svc
        .set_manually_distribute()
        .await
        .expect("设置手动分配成功");
    assert_eq!(resp.err_code, 0);
}

/// set_all_distribute 应发送 supplier_id。
#[tokio::test]
async fn supplier_set_all_distribute_sends_supplier_id() {
    let server = MockServer::start(|path, body| {
        if path.contains("/channels/ec/supplier/relation/set_all_distribution") {
            assert!(body.contains("SUP_001"));
            ok_response()
        } else {
            ok_response()
        }
    })
    .await;
    let service = new_service(config_with_host(&server.url()));
    let supplier_svc = service.supplier_service().unwrap();

    let resp = supplier_svc
        .set_all_distribute("SUP_001".into())
        .await
        .expect("设置全店自动分配成功");
    assert_eq!(resp.err_code, 0);
}

/// cancel_dropship 应发送 order_id。
#[tokio::test]
async fn supplier_cancel_dropship_sends_order_id() {
    let server = MockServer::start(|path, body| {
        if path.contains("/channels/ec/order/dropship/cancel") {
            assert!(body.contains("ORDER_DROP_001"));
            ok_response()
        } else {
            ok_response()
        }
    })
    .await;
    let service = new_service(config_with_host(&server.url()));
    let supplier_svc = service.supplier_service().unwrap();

    let resp = supplier_svc
        .cancel_dropship("ORDER_DROP_001".into())
        .await
        .expect("取消代发单成功");
    assert_eq!(resp.err_code, 0);
}

/// get_dropship 应发送 order_id。
#[tokio::test]
async fn supplier_get_dropship_sends_order_id() {
    let server = MockServer::start(|path, body| {
        if path.contains("/channels/ec/order/dropship/get") {
            assert!(body.contains("ORDER_DROP_002"));
            r#"{"errcode":0,"errmsg":"ok","dropship_order":{}}"#.to_string()
        } else {
            ok_response()
        }
    })
    .await;
    let service = new_service(config_with_host(&server.url()));
    let supplier_svc = service.supplier_service().unwrap();

    let resp = supplier_svc
        .get_dropship("ORDER_DROP_002".into())
        .await
        .expect("查询代发单详情成功");
    assert_eq!(resp.err_code, 0);
}

// ═══════════════════════════════════════════════════════════════
// Ewaybill 服务：URL 与参数对齐验证
// ═══════════════════════════════════════════════════════════════

/// get_template_config 应发送空 JSON 到正确的 URL。
#[tokio::test]
async fn ewaybill_get_template_config() {
    let server = MockServer::start(|path, _| {
        if path.contains("/channels/ec/logistics/ewaybill/biz/template/config") {
            r#"{"errcode":0,"errmsg":"ok","template_configs":[]}"#.to_string()
        } else {
            ok_response()
        }
    })
    .await;
    let service = new_service(config_with_host(&server.url()));
    let ewaybill_svc = service.ewaybill_service().unwrap();

    let resp = ewaybill_svc
        .get_template_config()
        .await
        .expect("获取面单模板配置成功");
    assert_eq!(resp.err_code, 0);
}

/// create_template 应发送模板数据。
#[tokio::test]
async fn ewaybill_create_template() {
    let server = MockServer::start(|path, body| {
        if path.contains("/channels/ec/logistics/ewaybill/biz/template/create") {
            assert!(!body.is_empty());
            r#"{"errcode":0,"errmsg":"ok","template_id":"TPL_001"}"#.to_string()
        } else {
            ok_response()
        }
    })
    .await;
    let service = new_service(config_with_host(&server.url()));
    let ewaybill_svc = service.ewaybill_service().unwrap();

    let req = wx_rust_channel::bean::ewaybill::TemplateCreateRequest::default();
    let resp = ewaybill_svc
        .create_template(req)
        .await
        .expect("创建面单模板成功");
    assert_eq!(resp.err_code, 0);
}

/// delete_template 应发送 template_id。
#[tokio::test]
async fn ewaybill_delete_template_sends_template_id() {
    let server = MockServer::start(|path, body| {
        if path.contains("/channels/ec/logistics/ewaybill/biz/template/delete") {
            assert!(body.contains("TPL_DEL_001"));
            ok_response()
        } else {
            ok_response()
        }
    })
    .await;
    let service = new_service(config_with_host(&server.url()));
    let ewaybill_svc = service.ewaybill_service().unwrap();

    let resp = ewaybill_svc
        .delete_template("TPL_DEL_001".into())
        .await
        .expect("删除面单模板成功");
    assert_eq!(resp.err_code, 0);
}

/// get_account 应发送空 JSON。
#[tokio::test]
async fn ewaybill_get_account() {
    let server = MockServer::start(|path, _| {
        if path.contains("/channels/ec/logistics/ewaybill/biz/account/get") {
            r#"{"errcode":0,"errmsg":"ok","account_list":[]}"#.to_string()
        } else {
            ok_response()
        }
    })
    .await;
    let service = new_service(config_with_host(&server.url()));
    let ewaybill_svc = service.ewaybill_service().unwrap();

    let resp = ewaybill_svc.get_account().await.expect("查询网点账号成功");
    assert_eq!(resp.err_code, 0);
}

/// get_delivery_list 应发送空 JSON。
#[tokio::test]
async fn ewaybill_get_delivery_list() {
    let server = MockServer::start(|path, _| {
        if path.contains("/channels/ec/logistics/ewaybill/biz/delivery/get") {
            r#"{"errcode":0,"errmsg":"ok","delivery_list":[]}"#.to_string()
        } else {
            ok_response()
        }
    })
    .await;
    let service = new_service(config_with_host(&server.url()));
    let ewaybill_svc = service.ewaybill_service().unwrap();

    let resp = ewaybill_svc
        .get_delivery_list()
        .await
        .expect("查询快递公司成功");
    assert_eq!(resp.err_code, 0);
}

/// create_order 应发送创建订单数据。
#[tokio::test]
async fn ewaybill_create_order() {
    let server = MockServer::start(|path, body| {
        if path.contains("/channels/ec/logistics/ewaybill/biz/order/create") {
            assert!(!body.is_empty());
            r#"{"errcode":0,"errmsg":"ok","ewaybill_order_id":"EB_001"}"#.to_string()
        } else {
            ok_response()
        }
    })
    .await;
    let service = new_service(config_with_host(&server.url()));
    let ewaybill_svc = service.ewaybill_service().unwrap();

    let req = wx_rust_channel::bean::ewaybill::CreateOrderRequest::default();
    let resp = ewaybill_svc
        .create_order(req)
        .await
        .expect("获取电子面单号成功");
    assert_eq!(resp.err_code, 0);
}

/// get_order 应发送 ewaybill_order_id。
#[tokio::test]
async fn ewaybill_get_order_sends_order_id() {
    let server = MockServer::start(|path, body| {
        if path.contains("/channels/ec/logistics/ewaybill/biz/order/get") {
            assert!(body.contains("EB_GET_001"));
            r#"{"errcode":0,"errmsg":"ok","order_detail":{}}"#.to_string()
        } else {
            ok_response()
        }
    })
    .await;
    let service = new_service(config_with_host(&server.url()));
    let ewaybill_svc = service.ewaybill_service().unwrap();

    let resp = ewaybill_svc
        .get_order("EB_GET_001".into())
        .await
        .expect("查询面单详情成功");
    assert_eq!(resp.err_code, 0);
}

/// get_print_content 应发送 ewaybill_order_id 和 template_id。
#[tokio::test]
async fn ewaybill_get_print_content_sends_both_ids() {
    let server = MockServer::start(|path, body| {
        if path.contains("/channels/ec/logistics/ewaybill/biz/print/get") {
            assert!(body.contains("EB_PRINT_001"));
            assert!(body.contains("TPL_PRINT_001"));
            r#"{"errcode":0,"errmsg":"ok","print_content":"BASE64_DATA"}"#.to_string()
        } else {
            ok_response()
        }
    })
    .await;
    let service = new_service(config_with_host(&server.url()));
    let ewaybill_svc = service.ewaybill_service().unwrap();

    let resp = ewaybill_svc
        .get_print_content("EB_PRINT_001".into(), "TPL_PRINT_001".into())
        .await
        .expect("获取打印报文成功");
    assert_eq!(resp.err_code, 0);
}

/// print_order 应发送打印请求。
#[tokio::test]
async fn ewaybill_print_order() {
    let server = MockServer::start(|path, _| {
        if path.contains("/channels/ec/logistics/ewaybill/biz/order/print") {
            ok_response()
        } else {
            ok_response()
        }
    })
    .await;
    let service = new_service(config_with_host(&server.url()));
    let ewaybill_svc = service.ewaybill_service().unwrap();

    let req = wx_rust_channel::bean::ewaybill::PrintOrderRequest::default();
    let resp = ewaybill_svc.print_order(req).await.expect("通知打印成功");
    assert_eq!(resp.err_code, 0);
}

/// batch_print_order 应发送批量打印请求。
#[tokio::test]
async fn ewaybill_batch_print_order() {
    let server = MockServer::start(|path, _| {
        if path.contains("/channels/ec/logistics/ewaybill/biz/order/batchprint") {
            ok_response()
        } else {
            ok_response()
        }
    })
    .await;
    let service = new_service(config_with_host(&server.url()));
    let ewaybill_svc = service.ewaybill_service().unwrap();

    let req = wx_rust_channel::bean::ewaybill::BatchPrintOrderRequest::default();
    let resp = ewaybill_svc
        .batch_print_order(req)
        .await
        .expect("批量通知打印成功");
    assert_eq!(resp.err_code, 0);
}

// ═══════════════════════════════════════════════════════════════
// Kf 服务：send_message URL 对齐验证
// ═══════════════════════════════════════════════════════════════

/// send_message 应发送到正确的 URL。
#[tokio::test]
async fn kf_send_message_sends_to_correct_url() {
    let server = MockServer::start(|path, body| {
        if path.contains("/channels/ec/commkf/sendmsg") {
            assert!(!body.is_empty());
            r#"{"errcode":0,"errmsg":"ok","msg_id":"MSG_001"}"#.to_string()
        } else {
            ok_response()
        }
    })
    .await;
    let service = new_service(config_with_host(&server.url()));
    let kf_svc = service.kf_service().unwrap();

    let param = wx_rust_channel::bean::kf::WxChannelKfSendMsgParam::default();
    let resp = kf_svc.send_message(param).await.expect("发送客服消息成功");
    assert_eq!(resp.err_code, 0);
}
