#![allow(clippy::field_reassign_with_default)]
//! 小程序覆盖率提升：云开发 + 客服 + 安全 + 标准版订单 + 订单管理 +
//! 快递退货 + 员工关系（MockServer 模式）。
//!
//! 对应 Java 各 `WxMa*ServiceImplTest` 的 HTTP 语义，经 MockServer 验证请求路径 /
//! 请求体线格式与响应解析。推广服务因所有方法均需独立 request struct，已在已有
//! 测试中覆盖 add_role/send_msg，此处不再重复。

use std::sync::Arc;
use std::sync::atomic::Ordering;

use wx_rust_miniapp::api::WxMaService;
use wx_rust_miniapp::api::r#impl::WxMaServiceImpl;
use wx_rust_miniapp::config::WxMaConfig;
use wx_rust_miniapp::config::r#impl::WxMaDefaultConfig;

/// 极简 mock HTTP 服务器：按请求路径返回固定响应，记录最近一次请求行与请求体。
struct MockServer {
    addr: std::net::SocketAddr,
    last_request_line: Arc<std::sync::Mutex<String>>,
    last_body: Arc<std::sync::Mutex<String>>,
    stop: Arc<std::sync::atomic::AtomicBool>,
}

impl MockServer {
    async fn start<F>(handler: F) -> Self
    where
        F: Fn(&str) -> String + Send + Sync + 'static,
    {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("绑定端口");
        let addr = listener.local_addr().expect("获取地址");
        let last_request_line = Arc::new(std::sync::Mutex::new(String::new()));
        let last_body = Arc::new(std::sync::Mutex::new(String::new()));
        let stop = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let handler = Arc::new(handler);

        let last_request_line_clone = last_request_line.clone();
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
                let handler = handler.clone();
                let last_request_line_clone = last_request_line_clone.clone();
                let last_body_clone = last_body_clone.clone();
                tokio::spawn(async move {
                    use tokio::io::{AsyncReadExt, AsyncWriteExt};
                    let mut buf = [0u8; 65536];
                    let n = socket.read(&mut buf).await.unwrap_or(0);
                    let request = String::from_utf8_lossy(&buf[..n]).to_string();
                    if let Some(line) = request.lines().next() {
                        *last_request_line_clone.lock().unwrap() = line.to_string();
                    }
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
                    let _ = socket.write_all(&response.into_bytes()).await;
                });
            }
        });

        Self {
            addr,
            last_request_line,
            last_body,
            stop,
        }
    }

    fn url(&self, path: &str) -> String {
        format!("http://{}{}", self.addr, path)
    }

    fn last_request_line(&self) -> String {
        self.last_request_line.lock().unwrap().clone()
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

/// 构建指向 mock 服务器的小程序配置（可指定云环境 ID）。
fn config_with_host_and_cloud_env(host: &str, cloud_env: &str) -> Arc<dyn WxMaConfig> {
    let mut config = WxMaDefaultConfig::new("wxappid", "secret");
    config.set_aes_key("abcdefghijklmnopqrstuvwxyz0123456789ABCDEFG");
    config.set_cloud_env(cloud_env);
    let mut host_config = wx_rust_miniapp::config::WxMaHostConfig::new();
    host_config.api_host = host.to_string();
    config.set_host_config(host_config);
    Arc::new(config)
}

/// 构建指向 mock 服务器的小程序配置。
fn config_with_host(host: &str) -> Arc<dyn WxMaConfig> {
    config_with_host_and_cloud_env(host, "")
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

// ══════════════════════════════════════════════════════════════════════════════
// SOURCE_PARITY: 云开发服务补充（镜像 Java WxMaCloudServiceImplTest）
// ══════════════════════════════════════════════════════════════════════════════

/// 对应 Java: WxMaCloudServiceImplTest.testAddSingle / testDelete / testUpdate / testQuery
#[tokio::test]
async fn cloud_crud_operations() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/tcb/databaseadd") {
            r#"{"errcode":0,"id_list":["doc-1"]}"#.to_string()
        } else if path.contains("/tcb/databasedelete") {
            r#"{"errcode":0,"deleted":1}"#.to_string()
        } else if path.contains("/tcb/databaseupdate") {
            r#"{"errcode":0,"matched":1,"modified":1}"#.to_string()
        } else if path.contains("/tcb/databasequery") {
            r#"{"errcode":0,"data":["{\"name\":\"item1\"}"],"pager":{"Offset":0,"Limit":10,"Total":1}}"#.to_string()
        } else if path.contains("/tcb/databaseaggregate") {
            r#"{"errcode":0,"data":["{\"total\":5}"]}"#.to_string()
        } else {
            r#"{"errcode":0}"#.to_string()
        }
    }))
    .await;
    let service =
        WxMaServiceImpl::new_arc(config_with_host_and_cloud_env(&server.url(""), "env-1"));
    let cloud = service.cloud_service().expect("云开发服务存在");

    // addSingle(collection, obj: &serde_json::Value)
    let id = cloud
        .add_single("users", &serde_json::json!({"name":"张三","age":25}))
        .await
        .expect("单条添加成功");
    assert_eq!(id, "doc-1");
    let body = last_body_json(&server);
    assert_eq!(body["env"], "env-1");

    // delete(collection, where_json)
    let deleted = cloud
        .delete("users", r#"{"name":"张三"}"#)
        .await
        .expect("删除成功");
    assert_eq!(deleted, 1);
    assert!(server.last_request_line().contains("/tcb/databasedelete"));

    // update(collection, where_json, update_json) → WxCloudDatabaseUpdateResult
    let result = cloud
        .update("users", r#"{"name":"张三"}"#, r#"{"$set":{"age":26}}"#)
        .await
        .expect("更新成功");
    assert_eq!(result.modified, 1);
    assert!(server.last_request_line().contains("/tcb/databaseupdate"));

    // query(collection, where_json, order_by, skip, limit)
    let query_result = cloud
        .query("users", r#"{"age":{"$gt":20}}"#, None, None, None)
        .await
        .expect("查询成功");
    assert!(query_result.pager.total >= 0);
    assert!(server.last_request_line().contains("/tcb/databasequery"));

    // aggregate
    let result = cloud
        .database_aggregate(
            "db.collection(\"users\").aggregate().group({_id:null,total:$.sum(1)}).end()",
        )
        .await
        .expect("聚合查询成功");
    // databaseAggregate 返回响应中的 `data` 数组（每项为序列化后的聚合文档）
    assert!(result.is_array());
    assert!(result[0].as_str().unwrap_or_default().contains("total"));
    assert!(
        server
            .last_request_line()
            .contains("/tcb/databaseaggregate")
    );
}

/// 对应 Java: WxMaCloudServiceImplTest.testUpdateIndex / testDatabaseMigrateImport / testDatabaseMigrateExport / testDatabaseMigrateQueryInfo
#[tokio::test]
async fn cloud_update_index_and_migrate() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/tcb/updateindex") {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        } else if path.contains("/tcb/databasemigrateimport") {
            r#"{"errcode":0,"job_id":10001}"#.to_string()
        } else if path.contains("/tcb/databasemigrateexport") {
            r#"{"errcode":0,"job_id":12345}"#.to_string()
        } else if path.contains("/tcb/databasemigratequeryinfo") {
            r#"{"errcode":0,"status":"success","record_success":100,"record_fail":0,"file_url":"https://cos/export.json"}"#.to_string()
        } else {
            r#"{"errcode":0}"#.to_string()
        }
    }))
    .await;
    let service =
        WxMaServiceImpl::new_arc(config_with_host_and_cloud_env(&server.url(""), "env-1"));
    let cloud = service.cloud_service().expect("云开发服务存在");

    // databaseMigrateImport
    let job_id = cloud
        .database_migrate_import("collection-1", "https://cos/import.json", 1, true, 1)
        .await
        .expect("导入成功");
    assert_eq!(job_id, 10001);
    assert!(
        server
            .last_request_line()
            .contains("/tcb/databasemigrateimport")
    );

    // databaseMigrateExport(file_path, file_type, query) → i64 job_id
    let job_id = cloud
        .database_migrate_export("export.json", 1, "db.collection(\"users\").get()")
        .await
        .expect("导出成功");
    assert_eq!(job_id, 12345);
    assert!(
        server
            .last_request_line()
            .contains("/tcb/databasemigrateexport")
    );

    // databaseMigrateQueryInfo(job_id: i64)
    let info = cloud
        .database_migrate_query_info(12345)
        .await
        .expect("查询任务成功");
    assert_eq!(info.status, "success");
    assert_eq!(info.record_success, 100);
    assert!(
        server
            .last_request_line()
            .contains("/tcb/databasemigratequeryinfo")
    );
}

// ══════════════════════════════════════════════════════════════════════════════
// SOURCE_PARITY: 客服服务补充（镜像 Java WxMaKefuServiceImplTest）
// ══════════════════════════════════════════════════════════════════════════════

/// 对应 Java: WxMaKefuServiceImplTest.testKfAccountUpdate / testKfAccountDel / testKfSessionClose / testKfSessionList
#[tokio::test]
async fn kefu_account_and_session_ops() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/customservice/kfaccount/update")
            || path.contains("/customservice/kfaccount/del")
            || path.contains("/customservice/kfsession/close")
        {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        } else if path.contains("/customservice/kfsession/getsessionlist") {
            r#"{"errcode":0,"sessionlist":[{"openid":"o1","createtime":1700000000}]}"#.to_string()
        } else {
            r#"{"errcode":0}"#.to_string()
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let kefu = service.kefu_service().expect("客服服务存在");

    // kfAccountUpdate
    let request = wx_rust_miniapp::bean::kefu::WxMaKfAccountRequest {
        kf_account: "test@kfaccount".to_string(),
        kf_nick: "新昵称".to_string(),
        kf_pwd: "newpwd".to_string(),
    };
    assert!(
        kefu.kf_account_update(&request)
            .await
            .expect("更新客服成功")
    );
    let body = last_body_json(&server);
    assert_eq!(body["kf_account"], "test@kfaccount");
    assert_eq!(body["kf_nick"], "新昵称");
    assert!(
        server
            .last_request_line()
            .contains("/customservice/kfaccount/update")
    );

    // kfAccountDel
    assert!(
        kefu.kf_account_del("test@kfaccount")
            .await
            .expect("删除客服成功")
    );
    assert!(
        server
            .last_request_line()
            .contains("/customservice/kfaccount/del")
    );

    // kfSessionClose
    assert!(
        kefu.kf_session_close("o1", "test@kfaccount")
            .await
            .expect("关闭会话成功")
    );
    let body = last_body_json(&server);
    assert_eq!(body["kf_account"], "test@kfaccount");
    assert_eq!(body["openid"], "o1");
    assert!(
        server
            .last_request_line()
            .contains("/customservice/kfsession/close")
    );

    // kfSessionList
    let sessions = kefu
        .kf_session_list("test@kfaccount")
        .await
        .expect("获取会话列表成功");
    assert_eq!(sessions.session_list.len(), 1);
    assert_eq!(sessions.session_list[0].openid, "o1");
    assert!(
        server
            .last_request_line()
            .contains("/customservice/kfsession/getsessionlist")
    );
}

// ══════════════════════════════════════════════════════════════════════════════
// SOURCE_PARITY: 安全服务补充（镜像 Java WxMaSecurityServiceImplTest）
// ══════════════════════════════════════════════════════════════════════════════

/// 对应 Java: WxMaSecurityServiceImplTest.testCheckImageUrl / testMediaCheckAsync / testGetUserRiskRank
#[tokio::test]
async fn security_check_image_url_and_media_check() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/wxa/media_check_async") {
            r#"{"errcode":0,"errmsg":"ok","trace_id":"trace_100","result":{"suggest":"pass","label":"100"},"detail":[]}"#.to_string()
        } else if path.contains("/wxa/getuserriskrank") {
            r#"{"errcode":0,"errmsg":"ok","risk_rank":2,"unoin_id":0}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let security = service.security_service().expect("安全服务存在");

    // checkImageUrl：文件 URL 由 mock 服务器提供（下载离线进行），下载后
    // 以 multipart 字段 `media` 上传到 /wxa/img_sec_check
    let media_url = server.url("/media/test.png");
    assert!(
        security
            .check_image_url(&media_url)
            .await
            .expect("URL 图片检测成功")
    );
    assert!(server.last_body().contains("media"));
    assert!(server.last_request_line().contains("/wxa/img_sec_check"));

    // mediaCheckAsync(media_url, media_type)
    let result = security
        .media_check_async("https://img.example.com/test.png", 2)
        .await
        .expect("异步内容检测成功");
    assert_eq!(result.trace_id, "trace_100");
    assert_eq!(result.result.suggest, "pass");
    let body = last_body_json(&server);
    assert_eq!(body["media_url"], "https://img.example.com/test.png");
    assert_eq!(body["media_type"], 2);
    assert!(
        server
            .last_request_line()
            .contains("/wxa/media_check_async")
    );

    // getUserRiskRank
    let mut risk_request =
        wx_rust_miniapp::bean::safety::request::WxMaUserSafetyRiskRankRequest::default();
    risk_request.appid = "wxappid".to_string();
    risk_request.openid = "o1".to_string();
    risk_request.scene = 1;
    let risk = security
        .get_user_risk_rank(&risk_request)
        .await
        .expect("获取用户风险等级成功");
    assert_eq!(risk.risk_rank, 2);
    let body = last_body_json(&server);
    assert_eq!(body["appid"], "wxappid");
    assert_eq!(body["scene"], 1);
    assert!(server.last_request_line().contains("/wxa/getuserriskrank"));
}

// ══════════════════════════════════════════════════════════════════════════════
// SOURCE_PARITY: 标准版商品订单补充（镜像 Java WxMaProductOrderServiceImplTest）
// ══════════════════════════════════════════════════════════════════════════════

/// 对应 Java: WxMaProductOrderServiceImplTest.testChangeMerchantNotes
#[tokio::test]
async fn product_order_merchant_notes() {
    let server = MockServer::start(dispatch(|_path| {
        r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let order = service
        .product_order_service()
        .expect("标准版商品订单服务存在");

    // changeMerchantNotes(order_id, merchant_notes)
    order
        .change_merchant_notes(7001, "已催促发货")
        .await
        .expect("修改商家备注成功");
    let body = last_body_json(&server);
    assert_eq!(body["order_id"], 7001);
    assert_eq!(body["merchant_notes"], "已催促发货");
    assert!(
        server
            .last_request_line()
            .contains("/product/order/change_merchant_notes")
    );
}

/// 对应 Java: WxMaProductOrderServiceImplTest.testDeliverySend
#[tokio::test]
async fn product_order_delivery_send() {
    let server = MockServer::start(dispatch(|_path| {
        r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let order = service
        .product_order_service()
        .expect("标准版商品订单服务存在");

    // deliverySend
    let mut request = wx_rust_miniapp::bean::product::WxMiniOrderDeliveryRequest::default();
    request.order_id = 7001;
    let mut delivery = wx_rust_miniapp::bean::product::DeliveryListBean::default();
    delivery.delivery_id = "SF".to_string();
    delivery.waybill_id = "SF1234567890".to_string();
    delivery.is_all_product = true;
    request.delivery_list = vec![delivery];
    order.delivery_send(&request).await.expect("发货成功");
    let body = last_body_json(&server);
    assert_eq!(body["order_id"], 7001);
    assert_eq!(body["delivery_list"][0]["delivery_id"], "SF");
    assert_eq!(body["delivery_list"][0]["waybill_id"], "SF1234567890");
    assert!(
        server
            .last_request_line()
            .contains("/product/delivery/send")
    );
}

/// 对应 Java: WxMaProductOrderServiceImplTest.testGetAfterSaleOrder / testAfterSaleAccept / testAfterSaleReject
#[tokio::test]
async fn product_order_after_sale_ops() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/product/order/getaftersaleorder") {
            r#"{"errcode":0,"errmsg":"ok","after_sale_order":{"out_aftersale_id":"AS_1","status":"2"}}"#.to_string()
        } else if path.contains("/product/order/batchgetaftersaleorder") {
            r#"{"errcode":0,"errmsg":"ok","after_sale_order_list":[{"out_aftersale_id":"AS_1","status":"2"}]}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let order = service
        .product_order_service()
        .expect("标准版商品订单服务存在");

    // getAfterSaleOrder(after_sale_order_id)
    let after_sale = order
        .get_after_sale_order(7001)
        .await
        .expect("获取售后订单成功");
    assert_eq!(after_sale.err_code, 0);
    assert!(
        server
            .last_request_line()
            .contains("/product/order/getaftersaleorder")
    );
    let body = last_body_json(&server);
    assert_eq!(body["after_sale_order_id"], 7001);

    // batchGetAfterSaleOrder(after_sale_order_id_list)
    let after_sale_list = order
        .batch_get_after_sale_order(&[7001, 7002])
        .await
        .expect("批量获取售后订单成功");
    assert_eq!(after_sale_list.err_code, 0);
    assert!(
        server
            .last_request_line()
            .contains("/product/order/batchgetaftersaleorder")
    );

    // afterSaleAccept(order_id, address_id)
    order
        .after_sale_accept(7001, 100)
        .await
        .expect("同意退款成功");
    assert!(
        server
            .last_request_line()
            .contains("/product/order/acceptapply")
    );
    let body = last_body_json(&server);
    assert_eq!(body["order_id"], 7001);
    assert_eq!(body["address_id"], 100);

    // afterSaleReject(after_sale_order_id, reject_reason)
    // Java/Rust 以 `order_id` 键提交 afterSaleOrderId
    order
        .after_sale_reject(7001, "商品无问题")
        .await
        .expect("拒绝退款成功");
    let body = last_body_json(&server);
    assert_eq!(body["order_id"], 7001);
    assert_eq!(body["reject_reason"], "商品无问题");
    assert!(
        server
            .last_request_line()
            .contains("/product/order/rejectrefund")
    );
}

// ══════════════════════════════════════════════════════════════════════════════
// SOURCE_PARITY: 订单管理服务（镜像 Java WxMaOrderManagementServiceImplTest）
// ══════════════════════════════════════════════════════════════════════════════

/// 对应 Java: WxMaOrderManagementServiceImplTest.testGetOrderDetailPath / testUpdateOrderDetailPath
#[tokio::test]
async fn order_management_detail_path() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/wxa/sec/order/get_order_detail_path") {
            r#"{"errcode":0,"errmsg":"ok","order_detail_path":"pages/order/detail?id=1"}"#
                .to_string()
        } else if path.contains("/wxa/sec/order/update_order_detail_path") {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        } else {
            r#"{"errcode":0}"#.to_string()
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let order_mgmt = service
        .order_management_service()
        .expect("订单管理服务存在");

    // getOrderDetailPath()
    let result = order_mgmt
        .get_order_detail_path()
        .await
        .expect("获取订单详情路径成功");
    assert_eq!(result.err_code, 0);
    assert!(
        server
            .last_request_line()
            .contains("/wxa/sec/order/get_order_detail_path")
    );

    // updateOrderDetailPath(path)
    let result = order_mgmt
        .update_order_detail_path("pages/order/detail?id=1")
        .await
        .expect("更新订单状态成功");
    assert_eq!(result.err_code, 0);
    let body = last_body_json(&server);
    assert_eq!(body["path"], "pages/order/detail?id=1");
    assert!(
        server
            .last_request_line()
            .contains("/wxa/sec/order/update_order_detail_path")
    );
}

// ══════════════════════════════════════════════════════════════════════════════
// SOURCE_PARITY: 快递退货服务（镜像 Java WxMaExpressDeliveryReturnServiceImplTest）
// ══════════════════════════════════════════════════════════════════════════════

/// 对应 Java: WxMaExpressDeliveryReturnServiceImplTest.testGetDeliveryReturn / testUnbindDeliveryReturn
#[tokio::test]
async fn express_delivery_return_ops() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/express/delivery/return/get") {
            r#"{"errcode":0,"errmsg":"ok","return_id":"RETURN_1","delivery_id":"YUNDA","delivery_name":"韵达","status":"1"}"#.to_string()
        } else if path.contains("/cgi-bin/express/delivery/return/unbind") {
            r#"{"errcode":0,"errmsg":"ok","return_id":"RETURN_1"}"#.to_string()
        } else {
            r#"{"errcode":0}"#.to_string()
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let delivery_return = service
        .express_delivery_return_service()
        .expect("快递退货服务存在");

    // getDeliveryReturn(return_id)
    let result = delivery_return
        .get_delivery_return("RETURN_1")
        .await
        .expect("查询退货服务成功");
    assert_eq!(result.errcode, 0);
    assert_eq!(result.return_id, "RETURN_1");
    let body = last_body_json(&server);
    assert_eq!(body["return_id"], "RETURN_1");
    assert!(
        server
            .last_request_line()
            .contains("/cgi-bin/express/delivery/return/get")
    );

    // unbindDeliveryReturn(return_id)
    let result = delivery_return
        .unbind_delivery_return("RETURN_1")
        .await
        .expect("解绑退货服务成功");
    assert_eq!(result.errcode, 0);
    let body = last_body_json(&server);
    assert_eq!(body["return_id"], "RETURN_1");
    assert!(
        server
            .last_request_line()
            .contains("/cgi-bin/express/delivery/return/unbind")
    );
}

// ══════════════════════════════════════════════════════════════════════════════
// SOURCE_PARITY: 员工关系服务（镜像 Java WxMaEmployeeRelationServiceImplTest）
// ══════════════════════════════════════════════════════════════════════════════

/// 对应 Java: WxMaEmployeeRelationServiceImplTest.testUnbindEmployee / testSendEmployeeMsg
#[tokio::test]
async fn employee_relation_ops() {
    let server = MockServer::start(dispatch(|_path| {
        r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let employee = service
        .employee_relation_service()
        .expect("员工关系服务存在");

    // unbindEmployee
    let mut request = wx_rust_miniapp::bean::employee::WxMaUnbindEmployeeRequest::default();
    request.openid_list = vec!["o1".to_string()];
    employee
        .unbind_employee(&request)
        .await
        .expect("解绑员工成功");
    let body = last_body_json(&server);
    assert_eq!(body["openid_list"][0], "o1");
    assert!(
        server
            .last_request_line()
            .contains("/wxa/business/unbinduserb2cauthinfo")
    );

    // sendEmployeeMsg
    let mut msg = wx_rust_miniapp::bean::employee::WxMaSendEmployeeMsgRequest::default();
    msg.touser = "o1".to_string();
    msg.template_id = "TMPL_1".to_string();
    msg.page = "pages/index".to_string();
    msg.data = r#"{"content":{"value":"业务消息"}}"#.to_string();
    employee
        .send_employee_msg(&msg)
        .await
        .expect("发送员工消息成功");
    let body = last_body_json(&server);
    assert_eq!(body["touser"], "o1");
    assert_eq!(body["template_id"], "TMPL_1");
    assert!(
        server
            .last_request_line()
            .contains("/cgi-bin/message/wxopen/employeerelationmsg/send")
    );
}

// ══════════════════════════════════════════════════════════════════════════════
// SOURCE_PARITY: 物流助手补充（镜像 Java WxMaExpressServiceImplTest）
// ══════════════════════════════════════════════════════════════════════════════

/// 对应 Java: WxMaExpressServiceImplTest.testGetAllDelivery / testUpdatePrinter / testGetPrinter
#[tokio::test]
async fn express_delivery_and_printer() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/express/business/delivery/getall") {
            // getAllDelivery 解析 `data` 数组（Java WxMaExpressDelivery.fromJson）
            r#"{"errcode":0,"data":[{"delivery_id":"YUNDA","delivery_name":"韵达"}]}"#.to_string()
        } else if path.contains("/cgi-bin/express/business/printer/update") {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        } else if path.contains("/cgi-bin/express/business/printer/getall") {
            r#"{"errcode":0,"count":1,"openid":["o1"],"tagid_list":["tag1"]}"#.to_string()
        } else {
            r#"{"errcode":0}"#.to_string()
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let express = service.express_service().expect("物流服务存在");

    // getAllDelivery
    let deliveries = express.get_all_delivery().await.expect("获取快递公司成功");
    assert_eq!(deliveries.len(), 1);
    assert_eq!(deliveries[0].delivery_id, "YUNDA");
    assert!(
        server
            .last_request_line()
            .contains("/cgi-bin/express/business/delivery/getall")
    );

    // updatePrinter
    let printer = wx_rust_miniapp::bean::express::request::WxMaExpressPrinterUpdateRequest {
        openid: "o1".to_string(),
        update_type: "bind".to_string(),
        tagid_list: "tag1".to_string(),
    };
    express
        .update_printer(&printer)
        .await
        .expect("更新打印机成功");
    assert!(
        server
            .last_request_line()
            .contains("/cgi-bin/express/business/printer/update")
    );

    // getPrinter
    let printer = express.get_printer().await.expect("获取打印机列表成功");
    assert_eq!(printer.count, 1);
    assert!(
        server
            .last_request_line()
            .contains("/cgi-bin/express/business/printer/getall")
    );
}

/// 对应 Java: WxMaExpressServiceImplTest.testCancelOrder / testTestUpdateOrder
#[tokio::test]
async fn express_cancel_and_test_update() {
    let server = MockServer::start(dispatch(|_path| {
        r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let express = service.express_service().expect("物流服务存在");

    // cancelOrder
    let cancel = wx_rust_miniapp::bean::express::request::WxMaExpressGetOrderRequest {
        order_id: "ORD_1".to_string(),
        delivery_id: "TEST".to_string(),
        waybill_id: "WB_1".to_string(),
        openid: "o1".to_string(),
    };
    express.cancel_order(&cancel).await.expect("取消运单成功");
    assert!(
        server
            .last_request_line()
            .contains("/cgi-bin/express/business/order/cancel")
    );

    // testUpdateOrder
    let test = wx_rust_miniapp::bean::express::request::WxMaExpressTestUpdateOrderRequest {
        order_id: "ORD_1".to_string(),
        delivery_id: "TEST".to_string(),
        waybill_id: "WB_1".to_string(),
        biz_id: "biz1".to_string(),
        action_time: 1700000000,
        ..Default::default()
    };
    express
        .test_update_order(&test)
        .await
        .expect("测试更新订单成功");
    assert!(
        server
            .last_request_line()
            .contains("/cgi-bin/express/business/test_update_order")
    );
}

/// 对应 Java: WxMaExpressServiceImplTest.testGetPath
#[tokio::test]
async fn express_get_path() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/express/business/path/get") {
            r#"{"errcode":0,"path_item_list":[{"action_time":1700000000,"action_type":200001,"action_msg":"已揽收"}]}"#.to_string()
        } else {
            r#"{"errcode":0}"#.to_string()
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let express = service.express_service().expect("物流服务存在");

    // getPath
    let request = wx_rust_miniapp::bean::express::request::WxMaExpressGetOrderRequest {
        order_id: "ORD_1".to_string(),
        delivery_id: "TEST".to_string(),
        waybill_id: "WB_1".to_string(),
        openid: "o1".to_string(),
    };
    let path = express.get_path(&request).await.expect("获取运单轨迹成功");
    assert_eq!(path.path_item_list.len(), 1);
    assert_eq!(path.path_item_list[0].action_msg, "已揽收");
    assert!(
        server
            .last_request_line()
            .contains("/cgi-bin/express/business/path/get")
    );
}

// ══════════════════════════════════════════════════════════════════════════════
// RUST_OBLIGATION: 错误路径
// ══════════════════════════════════════════════════════════════════════════════

/// 安全服务 checkImageUrl 响应 errcode!=0 应抛错
#[tokio::test]
async fn security_check_image_url_error() {
    let server = MockServer::start(dispatch(|_path| {
        r#"{"errcode":40001,"errmsg":"invalid credential"}"#.to_string()
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let security = service.security_service().expect("安全服务存在");

    // 文件 URL 指向 mock 服务器：下载离线成功，上传 img_sec_check 得
    // errcode=40001 后抛错（token 自动刷新一次后仍 40001）
    let media_url = server.url("/media/test.png");
    let err = security
        .check_image_url(&media_url)
        .await
        .expect_err("errcode!=0 应抛错");
    assert_eq!(err.error_code(), Some(40001));
}
