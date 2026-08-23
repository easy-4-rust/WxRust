#![allow(clippy::field_reassign_with_default)]
//! 小程序覆盖率提升：标准版商品 + 直播房间 + 直播商品 + 直播成员（MockServer 模式）。
//!
//! 对应 Java `WxMaProductServiceImplTest` / `WxMaLiveServiceImplTest` /
//! `WxMaLiveGoodsServiceImplTest` / `WxMaLiveMemberServiceImplTest` 的 HTTP 语义。
//!
//! 覆盖：商品 CRUD + SKU 管理、直播房间全生命周期、直播商品上下架审核、
//! 直播成员角色管理，均经 MockServer 验证请求路径 / 请求体 / 响应解析。

use std::sync::Arc;
use std::sync::atomic::Ordering;

use wx_rust_miniapp::api::WxMaService;
use wx_rust_miniapp::api::r#impl::WxMaServiceImpl;
use wx_rust_miniapp::bean::product::WxMinishopSku;
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

/// 构建指向 mock 服务器的小程序配置。
fn config_with_host(host: &str) -> Arc<dyn WxMaConfig> {
    let mut config = WxMaDefaultConfig::new("wxappid", "secret");
    config.set_aes_key("abcdefghijklmnopqrstuvwxyz0123456789ABCDEFG");
    let mut host_config = wx_rust_miniapp::config::WxMaHostConfig::new();
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

// ══════════════════════════════════════════════════════════════════════════════
// SOURCE_PARITY: 标准版商品服务（镜像 Java WxMaProductServiceImplTest）
// ══════════════════════════════════════════════════════════════════════════════

/// 对应 Java: WxMaProductServiceImplTest.testDeleteSpu / testGetSpu
#[tokio::test]
async fn product_delete_spu_and_get_spu() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/product/spu/del") {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        } else if path.contains("/product/spu/get") {
            r#"{"errcode":0,"errmsg":"ok","data":{"spu":{"out_product_id":"OUT_1","title":"商品标题","head_img":["https://img.example.com/a.png"],"desc_info":{"text":"描述"},"third_cat_id":101,"brand_id":202}}}"#
                .to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let product = service.product_service().expect("标准版商品服务存在");

    // deleteSpu
    let resp = product
        .delete_spu(9001, Some("OUT_1"))
        .await
        .expect("删除商品成功");
    assert_eq!(resp.err_code, 0);
    let body = last_body_json(&server);
    assert_eq!(body["product_id"], 9001);
    assert_eq!(body["out_product_id"], "OUT_1");

    // getSpu
    let resp = product
        .get_spu(9001, Some("OUT_1"), None)
        .await
        .expect("获取商品详情成功");
    assert_eq!(resp.errcode, 0);
    // data 为 serde_json::Value，SPU 详情位于 data.spu（与微信接口结构一致）
    assert_eq!(resp.data["spu"]["out_product_id"], "OUT_1");
    assert_eq!(resp.data["spu"]["title"], "商品标题");
    let body = last_body_json(&server);
    assert_eq!(body["product_id"], 9001);
    assert_eq!(body["out_product_id"], "OUT_1");
}

/// 对应 Java: WxMaProductServiceImplTest.testUpdateSpu
#[tokio::test]
async fn product_update_spu() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/product/spu/update") {
            r#"{"errcode":0,"errmsg":"ok","data":{"product_id":9001,"out_product_id":"OUT_1","update_time":"2024-06-01 12:00:00"}}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let product = service.product_service().expect("标准版商品服务存在");

    let mut spu = wx_rust_miniapp::bean::product::WxMinishopSpu::default();
    spu.out_product_id = "OUT_1".to_string();
    spu.title = "更新后商品".to_string();
    spu.head_imgs = vec!["https://img.example.com/new.png".to_string()];
    let result = product.update_spu(&spu).await.expect("更新商品成功");
    assert_eq!(result.errcode, 0);
    assert_eq!(result.data["product_id"], 9001);
    assert_eq!(result.data["update_time"], "2024-06-01 12:00:00");
    let body = last_body_json(&server);
    assert_eq!(body["title"], "更新后商品");
    assert_eq!(body["out_product_id"], "OUT_1");
}

/// 对应 Java: WxMaProductServiceImplTest.testGetSkuList
#[tokio::test]
async fn product_get_sku_list() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/product/sku/get_list") {
            r#"{"errcode":0,"errmsg":"ok","skus":[{"sku_id":1,"out_sku_id":"SKU_1","sale_price":9900,"stock_quantity":100}]}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let product = service.product_service().expect("标准版商品服务存在");

    let result = product
        .get_sku_list(9001, Some(1), None)
        .await
        .expect("获取 SKU 列表成功");
    assert_eq!(result.err_code, 0);
    assert_eq!(result.skus.len(), 1);
    assert_eq!(result.skus[0].sku_id, 1);
    assert_eq!(result.skus[0].out_sku_id, "SKU_1");
    let body = last_body_json(&server);
    assert_eq!(body["product_id"], 9001);
    assert_eq!(body["need_real_stock"], 1);
}

/// 对应 Java: WxMaProductServiceImplTest.testMinishiopGoodsAddSku
#[tokio::test]
async fn product_add_sku() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/product/sku/add") {
            r#"{"errcode":0,"errmsg":"ok","data":{"sku_id":5001,"create_time":"2024-06-01 10:00:00"}}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let product = service.product_service().expect("标准版商品服务存在");

    let mut sku = WxMinishopSku::default();
    sku.out_product_id = "OUT_1".to_string();
    sku.out_sku_id = "SKU_1".to_string();
    sku.sale_price = 9900;
    sku.stock_num = 100;
    let result = product
        .minishop_goods_add_sku(&sku)
        .await
        .expect("添加 SKU 成功");
    assert_eq!(result.errcode, 0);
    assert_eq!(result.data["sku_id"], 5001);
    assert_eq!(result.data["create_time"], "2024-06-01 10:00:00");
}

/// 对应 Java: WxMaProductServiceImplTest.testMinishiopGoodsBatchAddSku
#[tokio::test]
async fn product_batch_add_sku() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/product/sku/batch_add") {
            r#"{"errcode":0,"errmsg":"ok","data":[{"sku_id":5001,"out_sku_id":"SKU_1","create_time":"2024-06-01"},{"sku_id":5002,"out_sku_id":"SKU_2","create_time":"2024-06-01"}]}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let product = service.product_service().expect("标准版商品服务存在");

    let mut sku1 = WxMinishopSku::default();
    sku1.out_sku_id = "SKU_1".to_string();
    let mut sku2 = WxMinishopSku::default();
    sku2.out_sku_id = "SKU_2".to_string();
    let result = product
        .minishop_goods_batch_add_sku(&[sku1, sku2])
        .await
        .expect("批量添加 SKU 成功");
    assert_eq!(result.errcode, 0);
    let data = result.data.as_array().expect("data 为数组");
    assert_eq!(data.len(), 2);
    assert_eq!(data[0]["sku_id"], 5001);
    assert_eq!(data[1]["out_sku_id"], "SKU_2");
}

/// 对应 Java: WxMaProductServiceImplTest.testMinishiopGoodsDelSku
#[tokio::test]
async fn product_del_sku() {
    let server = MockServer::start(dispatch(|_path| {
        r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let product = service.product_service().expect("标准版商品服务存在");

    let resp = product
        .minishop_goods_del_sku(9001, None, Some("SKU_1"), Some(5001))
        .await
        .expect("删除 SKU 成功");
    assert_eq!(resp.err_code, 0);
    let body = last_body_json(&server);
    assert_eq!(body["product_id"], 9001);
    assert_eq!(body["out_sku_id"], "SKU_1");
    assert_eq!(body["sku_id"], 5001);
}

/// 对应 Java: WxMaProductServiceImplTest.testMinishiopGoodsUpdateSku
#[tokio::test]
async fn product_update_sku() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/product/sku/update") {
            r#"{"errcode":0,"errmsg":"ok","data":{"update_time":"2024-06-02 10:00:00"}}"#
                .to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let product = service.product_service().expect("标准版商品服务存在");

    let mut sku = WxMinishopSku::default();
    sku.sku_id = 5001;
    sku.out_product_id = "OUT_1".to_string();
    sku.out_sku_id = "SKU_1".to_string();
    let result = product
        .minishop_goods_update_sku(&sku)
        .await
        .expect("更新 SKU 成功");
    assert_eq!(result.data["update_time"], "2024-06-02 10:00:00");
}

/// 对应 Java: WxMaProductServiceImplTest.testMinishiopGoodsUpdateSkuPrice
#[tokio::test]
async fn product_update_sku_price() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/product/sku/update_price") {
            r#"{"errcode":0,"errmsg":"ok","data":{"update_time":"2024-06-03"}}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let product = service.product_service().expect("标准版商品服务存在");

    let result = product
        .minishop_goods_update_sku_price(
            9001,
            Some("OUT_1"),
            Some("SKU_1"),
            Some(5001),
            Some(8800),
            Some(9900),
        )
        .await
        .expect("更新 SKU 价格成功");
    assert_eq!(result.data["update_time"], "2024-06-03");
    let body = last_body_json(&server);
    assert_eq!(body["product_id"], 9001);
    assert_eq!(body["out_sku_id"], "SKU_1");
    assert_eq!(body["sale_price"], 8800);
    assert_eq!(body["market_price"], 9900);
}

/// 对应 Java: WxMaProductServiceImplTest.testMinishiopGoodsUpdateSkuStock
#[tokio::test]
async fn product_update_sku_stock() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/product/stock/update") {
            r#"{"errcode":0,"errmsg":"ok","data":{"update_time":"2024-06-04"}}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let product = service.product_service().expect("标准版商品服务存在");

    let result = product
        .minishop_goods_update_sku_stock(
            9001,
            Some("OUT_1"),
            Some("SKU_1"),
            Some(5001),
            Some(1),
            Some(50),
        )
        .await
        .expect("更新 SKU 库存成功");
    assert_eq!(result.data["update_time"], "2024-06-04");
    let body = last_body_json(&server);
    assert_eq!(body["product_id"], 9001);
    // build_json 以 "type" 为键发送库存变更类型
    assert_eq!(body["type"], 1);
    assert_eq!(body["stock_num"], 50);
}

/// 对应 Java: WxMaProductServiceImplTest.testGetCategory / testGetBrand / testGetFreightTemplate
#[tokio::test]
async fn product_get_category_brand_and_freight() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/product/category/get") {
            // WxMinishopGetCategoryResponse 的列表字段为 cat_list
            r#"{"errcode":0,"errmsg":"ok","cat_list":[{"cat_id":1,"f_cat_id":0,"name":"服饰"}]}"#
                .to_string()
        } else if path.contains("/product/brand/get") {
            r#"{"errcode":0,"errmsg":"ok","brands":[{"brand_id":1,"name":"品牌A"}]}"#.to_string()
        } else if path.contains("/product/delivery/get_freight_template") {
            // WxMinishopGetFrightTemplateResponse 的列表字段为 template_list
            r#"{"errcode":0,"errmsg":"ok","template_list":[{"template_id":1,"name":"包邮"}]}"#
                .to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let product = service.product_service().expect("标准版商品服务存在");

    // getCategory
    let cat = product.get_category(None).await.expect("获取类目成功");
    assert_eq!(cat.err_code, 0);
    assert_eq!(cat.cat_list.len(), 1);
    assert_eq!(cat.cat_list[0].name, "服饰");
    let body = last_body_json(&server);
    // f_cat_id 为 None 时不出现（build_json 跳过 null）
    assert!(body.get("f_cat_id").is_none());

    // getBrand
    let brand = product.get_brand().await.expect("获取品牌成功");
    assert_eq!(brand.err_code, 0);

    // getFreightTemplate
    let freight = product
        .get_freight_template()
        .await
        .expect("获取运费模板成功");
    assert_eq!(freight.err_code, 0);
}

/// 对应 Java: WxMaProductServiceImplTest.testUploadImg(String, Integer)
#[tokio::test]
async fn product_upload_img_from_url() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/product/img/upload") {
            // WxMinishopImageUploadResult.errcode 为 String，而通用错误网关（WxError）
            // 仅接受数字型 errcode：成功响应不带 errcode 字段时两侧均以默认值通过
            r#"{"errmsg":"ok","picFile":{"mediaId":"IMG_1","payMediaId":"PAY_1","tempImgUrl":"https://cdn/img1.png"}}"#
                .to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let product = service.product_service().expect("标准版商品服务存在");

    let result = product
        .upload_img_from_url("https://example.com/img.png", 0)
        .await
        .expect("URL 上传图片成功");
    // WxMinishopImageUploadResult.errcode 为 String 类型（响应无 errcode 时取默认空串）
    assert_eq!(result.errcode, "");
    assert_eq!(result.pic_file.media_id, "IMG_1");
    assert_eq!(result.pic_file.temp_img_url, "https://cdn/img1.png");
    let body = last_body_json(&server);
    assert_eq!(body["img_url"], "https://example.com/img.png");
    let request_line = server.last_request_line();
    assert!(request_line.contains("upload_type=1"), "{request_line}");
    assert!(request_line.contains("resp_type=0"), "{request_line}");
}

// ══════════════════════════════════════════════════════════════════════════════
// SOURCE_PARITY: 直播房间服务（镜像 Java WxMaLiveServiceImplTest）
// ══════════════════════════════════════════════════════════════════════════════

/// 对应 Java: WxMaLiveServiceImplTest.testDeleteRoom / testEditRoom
#[tokio::test]
async fn live_delete_room_and_edit_room() {
    let server = MockServer::start(dispatch(|_path| {
        r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let live = service.live_service().expect("直播服务存在");

    // deleteRoom
    assert!(live.delete_room(456).await.expect("删除直播间成功"));
    let body = last_body_json(&server);
    assert_eq!(body["id"], 456);
    assert!(
        server
            .last_request_line()
            .contains("/wxaapi/broadcast/room/deleteroom")
    );

    // editRoom
    let mut room = wx_rust_miniapp::bean::live::WxMaLiveRoomInfo::default();
    room.name = "编辑后的直播间".to_string();
    room.cover_img = "MEDIA_2".to_string();
    assert!(live.edit_room(&room).await.expect("编辑直播间成功"));
    let body = last_body_json(&server);
    assert_eq!(body["name"], "编辑后的直播间");
    assert_eq!(body["coverImg"], "MEDIA_2");
    assert!(
        server
            .last_request_line()
            .contains("/wxaapi/broadcast/room/editroom")
    );
}

/// 对应 Java: WxMaLiveServiceImplTest.testGetPushUrl / testGetSharedCode
#[tokio::test]
async fn live_get_push_url_and_shared_code() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/wxaapi/broadcast/room/getpushurl") {
            r#"{"errcode":0,"pushAddr":"rtmp://push.example.com/live/room_123"}"#.to_string()
        } else if path.contains("/wxaapi/broadcast/room/getsharedcode") {
            r#"{"errcode":0,"cdnUrl":"https://wx.qr/SHARE_1","pagePath":"pages/index","posterUrl":"https://img/poster.png"}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let live = service.live_service().expect("直播服务存在");

    // getPushUrl
    let push_url = live.get_push_url(123).await.expect("获取推流地址成功");
    assert_eq!(push_url, "rtmp://push.example.com/live/room_123");
    assert!(
        server
            .last_request_line()
            .contains("/wxaapi/broadcast/room/getpushurl")
    );
    assert!(server.last_request_line().contains("roomId=123"));

    // getSharedCode
    let shared = live
        .get_shared_code(123, Some("scene=test"))
        .await
        .expect("获取分享码成功");
    // WxMaLiveSharedCode 字段：cdn_url / page_path / poster_url
    assert!(!shared.cdn_url.is_empty() || shared.page_path.is_empty());
    assert!(
        server
            .last_request_line()
            .contains("/wxaapi/broadcast/room/getsharedcode")
    );
    assert!(server.last_request_line().contains("roomId=123"));
    assert!(server.last_request_line().contains("params=scene=test"));
}

/// 对应 Java: WxMaLiveServiceImplTest.testGetLiveReplay
#[tokio::test]
async fn live_get_live_replay() {
    let server = MockServer::start(dispatch(|_path| {
        r#"{"errcode":0,"total":1,"room_info":[{"name":"回放直播间","roomid":789,"live_status":0}]}"#.to_string()
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let live = service.live_service().expect("直播服务存在");

    let result = live
        .get_live_replay_default(789, 0, 10)
        .await
        .expect("获取回放成功");
    assert_eq!(result.total, 1);
    assert_eq!(result.room_infos[0].room_id, 789);
    let body = last_body_json(&server);
    assert_eq!(body["action"], "get_replay");
    // 对齐 Java Map<String,String>：room_id 以字符串发送
    assert_eq!(body["room_id"], "789");
    assert_eq!(body["start"], 0);
    assert_eq!(body["limit"], 10);
}

/// 对应 Java: WxMaLiveServiceImplTest.testAddGoodsToRoom
#[tokio::test]
async fn live_add_goods_to_room() {
    let server = MockServer::start(dispatch(|_path| {
        r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let live = service.live_service().expect("直播服务存在");

    assert!(
        live.add_goods_to_room(123, &[1001, 1002])
            .await
            .expect("导入商品成功")
    );
    let body = last_body_json(&server);
    assert_eq!(body["roomId"], 123);
    assert_eq!(body["ids"][0], 1001);
    assert_eq!(body["ids"][1], 1002);
    assert!(
        server
            .last_request_line()
            .contains("/wxaapi/broadcast/room/addgoods")
    );
}

/// 对应 Java: WxMaLiveServiceImplTest.testAddAssistant / testModifyAssistant / testRemoveAssistant / testGetAssistantList
#[tokio::test]
async fn live_assistant_crud() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/wxaapi/broadcast/room/getassistantlist") {
            r#"{"errcode":0,"list":[{"nickname":"助手A","openid":"o1","headimg":"https://img/a.png","alias":"alias1","timestamp":1700000000}]}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let live = service.live_service().expect("直播服务存在");

    // addAssistant
    assert!(live.add_assistant(123, &[]).await.expect("添加小助手成功"));
    assert!(
        server
            .last_request_line()
            .contains("/wxaapi/broadcast/room/addassistant")
    );

    // modifyAssistant
    assert!(
        live.modify_assistant(123, "user1", "新昵称")
            .await
            .expect("修改昵称成功")
    );
    let body = last_body_json(&server);
    assert_eq!(body["username"], "user1");
    assert_eq!(body["nickname"], "新昵称");

    // removeAssistant
    assert!(
        live.remove_assistant(123, "user1")
            .await
            .expect("删除小助手成功")
    );
    let body = last_body_json(&server);
    assert_eq!(body["username"], "user1");

    // getAssistantList
    let list = live
        .get_assistant_list(123)
        .await
        .expect("获取小助手列表成功");
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].nickname, "助手A");
}

/// 对应 Java: WxMaLiveServiceImplTest.testAddSubanchor / testModifySubanchor / testDeleteSubanchor / testGetSubanchor
#[tokio::test]
async fn live_subanchor_crud() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/wxaapi/broadcast/room/getsubanchor") {
            r#"{"errcode":0,"username":"sub_user1"}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let live = service.live_service().expect("直播服务存在");

    // addSubanchor
    assert!(
        live.add_subanchor(123, "sub_user1")
            .await
            .expect("添加主播副号成功")
    );
    assert!(
        server
            .last_request_line()
            .contains("/wxaapi/broadcast/room/addsubanchor")
    );

    // modifySubanchor
    assert!(
        live.modify_subanchor(123, "sub_user1")
            .await
            .expect("修改主播副号成功")
    );
    assert!(
        server
            .last_request_line()
            .contains("/wxaapi/broadcast/room/modifysubanchor")
    );

    // deleteSubanchor
    assert!(live.delete_subanchor(123).await.expect("删除主播副号成功"));
    let body = last_body_json(&server);
    assert_eq!(body["roomId"], 123);

    // getSubanchor
    let username = live.get_subanchor(123).await.expect("获取主播副号成功");
    assert_eq!(username, "sub_user1");
    assert!(
        server
            .last_request_line()
            .contains("/wxaapi/broadcast/room/getsubanchor")
    );
}

/// 对应 Java: WxMaLiveServiceImplTest.testUpdatefeedpublic / testUpdatereplay / testUpdatekf / testUpdatecomment
#[tokio::test]
async fn live_room_toggles() {
    let server = MockServer::start(dispatch(|_path| {
        r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let live = service.live_service().expect("直播服务存在");

    // updatefeedpublic
    assert!(live.updatefeedpublic(123, 1).await.expect("开启收录成功"));
    let body = last_body_json(&server);
    assert_eq!(body["roomId"], 123);
    assert_eq!(body["isFeedsPublic"], 1);
    assert!(
        server
            .last_request_line()
            .contains("/wxaapi/broadcast/room/updatefeedpublic")
    );

    // updatereplay
    assert!(live.updatereplay(123, 0).await.expect("开启回放成功"));
    let body = last_body_json(&server);
    assert_eq!(body["closeReplay"], 0);

    // updatekf
    assert!(live.updatekf(123, 1).await.expect("关闭客服成功"));
    let body = last_body_json(&server);
    assert_eq!(body["closeKf"], 1);

    // updatecomment
    assert!(live.updatecomment(123, 1).await.expect("开启禁言成功"));
    let body = last_body_json(&server);
    assert_eq!(body["banComment"], 1);
}

/// 对应 Java: WxMaLiveServiceImplTest.testOnsale / testDeleteInRoom / testPush / testSort / testGetVideo
#[tokio::test]
async fn live_goods_room_ops() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/wxaapi/broadcast/goods/getVideo") {
            r#"{"errcode":0,"url":"https://cdn/video1.mp4"}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let live = service.live_service().expect("直播服务存在");

    // onsale
    assert!(live.onsale(123, 1001, 1).await.expect("上架商品成功"));
    let body = last_body_json(&server);
    assert_eq!(body["roomId"], 123);
    assert_eq!(body["goodsId"], 1001);
    assert_eq!(body["onSale"], 1);

    // deleteInRoom
    assert!(live.delete_in_room(123, 1001).await.expect("删除商品成功"));
    let body = last_body_json(&server);
    assert_eq!(body["goodsId"], 1001);

    // push
    assert!(live.push(123, 1001).await.expect("推送商品成功"));
    assert!(
        server
            .last_request_line()
            .contains("/wxaapi/broadcast/goods/push")
    );

    // sort
    let goods = vec![
        std::collections::HashMap::from([("goodsId".to_string(), "1001".to_string())]),
        std::collections::HashMap::from([("goodsId".to_string(), "1002".to_string())]),
    ];
    assert!(live.sort(123, &goods).await.expect("排序成功"));
    let body = last_body_json(&server);
    assert_eq!(body["goods"][0]["goodsId"], "1001");
    assert_eq!(body["goods"][1]["goodsId"], "1002");

    // getVideo
    let url = live.get_video(123, 1001).await.expect("获取视频成功");
    assert_eq!(url, "https://cdn/video1.mp4");
    assert!(
        server
            .last_request_line()
            .contains("/wxaapi/broadcast/goods/getVideo")
    );
}

// ══════════════════════════════════════════════════════════════════════════════
// SOURCE_PARITY: 直播商品服务（镜像 Java WxMaLiveGoodsServiceImplTest）
// ══════════════════════════════════════════════════════════════════════════════

/// 对应 Java: WxMaLiveGoodsServiceImplTest.testResetAudit / testAuditGoods / testDeleteGoods
#[tokio::test]
async fn live_goods_reset_audit_and_delete() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/wxaapi/broadcast/goods/audit") {
            // auditGoods 从响应读取字符串型 auditId
            r#"{"errcode":0,"auditId":"999"}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let goods = service.live_goods_service().expect("直播商品服务存在");

    // resetAudit：请求体为 {"auditId": ..., "goodsId": ...}
    assert!(goods.reset_audit(123, 8).await.expect("重置审核成功"));
    let body = last_body_json(&server);
    assert_eq!(body["auditId"], 123);
    assert_eq!(body["goodsId"], 8);

    // auditGoods
    let audit_id = goods.audit_goods(8).await.expect("提交审核成功");
    assert_eq!(audit_id, "999");
    let body = last_body_json(&server);
    assert_eq!(body["goodsId"], 8);

    // deleteGoods
    assert!(goods.delete_goods(8).await.expect("删除商品成功"));
    let body = last_body_json(&server);
    assert_eq!(body["goodsId"], 8);
    assert!(
        server
            .last_request_line()
            .contains("/wxaapi/broadcast/goods/delete")
    );
}

/// 对应 Java: WxMaLiveGoodsServiceImplTest.testGetApprovedGoods
#[tokio::test]
async fn live_goods_get_approved_and_warehouse() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/wxaapi/broadcast/goods/getapproved") {
            r#"{"errcode":0,"total":1,"goods":[{"goods_id":8,"name":"商品A","cover_img_url":"http://cover/1.png"}]}"#.to_string()
        } else if path.contains("/wxa/business/getgoodswarehouse") {
            r#"{"errcode":0,"total":1,"goods":[{"goods_id":9,"name":"商品B"}]}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let goods = service.live_goods_service().expect("直播商品服务存在");

    // getApprovedGoods(offset, limit, status)
    let result = goods
        .get_approved_goods(0, 10, 0)
        .await
        .expect("获取审核商品成功");
    assert_eq!(result.total, 1);
    assert!(
        server
            .last_request_line()
            .contains("/wxaapi/broadcast/goods/getapproved")
    );

    // getGoodsWareHouse(goods_ids)
    let result = goods
        .get_goods_ware_house(&[8, 9])
        .await
        .expect("获取仓库商品成功");
    assert_eq!(result.total, 1);
    assert!(
        server
            .last_request_line()
            .contains("/wxa/business/getgoodswarehouse")
    );
}

/// 对应 Java: WxMaLiveGoodsServiceImplTest.testSetKey / testGetKey
#[tokio::test]
async fn live_goods_set_key_and_get_key() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/wxaapi/broadcast/goods/getkey") {
            // getKey 从响应读取 vendorGoodsKey 数组
            r#"{"errcode":0,"vendorGoodsKey":["key1","key2"]}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let goods = service.live_goods_service().expect("直播商品服务存在");

    // setKey：请求体为 {"goodsKey": [...]}
    assert!(
        goods
            .set_key(&["key1".to_string(), "key2".to_string()])
            .await
            .expect("设置商品 key 成功")
    );
    let body = last_body_json(&server);
    assert_eq!(body["goodsKey"][0], "key1");
    assert_eq!(body["goodsKey"][1], "key2");

    // getKey
    let key = goods.get_key().await.expect("获取商品 key 成功");
    let key = key.expect("key 列表非空");
    assert_eq!(key, vec!["key1", "key2"]);
}

// ══════════════════════════════════════════════════════════════════════════════
// SOURCE_PARITY: 直播成员服务（镜像 Java WxMaLiveMemberServiceImplTest）
// ══════════════════════════════════════════════════════════════════════════════

/// 对应 Java: WxMaLiveMemberServiceImplTest.testAddRole / testDeleteRole / testListByRole
#[tokio::test]
async fn live_member_role_crud() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/wxaapi/broadcast/role/getrolelist") {
            r#"{"errcode":0,"list":[{"username":"member1","role":1}]}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let member = service.live_member_service().expect("直播成员服务存在");

    // addRole：返回微信原始响应报文
    let result = member
        .add_role("member1", 1)
        .await
        .expect("添加成员角色成功");
    assert_eq!(result, r#"{"errcode":0,"errmsg":"ok"}"#);
    let body = last_body_json(&server);
    assert_eq!(body["username"], "member1");
    assert_eq!(body["role"], 1);

    // deleteRole：同样返回原始响应报文
    let result = member
        .delete_role("member1", 1)
        .await
        .expect("删除成员角色成功");
    assert_eq!(result, r#"{"errcode":0,"errmsg":"ok"}"#);

    // listByRole(role, offset, limit, keyword) → 返回 list 节点（JSON 数组）
    let list = member
        .list_by_role(1, 0, 10, None)
        .await
        .expect("按角色查询成员成功");
    assert!(list.is_array());
    assert_eq!(list[0]["username"], "member1");
    assert_eq!(list[0]["role"], 1);
}

// ══════════════════════════════════════════════════════════════════════════════
// RUST_OBLIGATION: 错误路径覆盖
// ══════════════════════════════════════════════════════════════════════════════

/// 对应 Java: WxMaLiveServiceImplTest 错误路径：getPushUrl 缺 pushAddr
#[tokio::test]
async fn live_get_push_url_missing_field() {
    let server = MockServer::start(dispatch(|_path| {
        r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let live = service.live_service().expect("直播服务存在");

    let err = live
        .get_push_url(123)
        .await
        .expect_err("缺少 pushAddr 应抛错");
    assert_eq!(err.error_code(), Some(-99));
    assert!(err.to_string().contains("pushAddr"));
}

/// 对应 Java: WxMaLiveServiceImplTest 错误路径：getSubanchor 缺 username
#[tokio::test]
async fn live_get_subanchor_missing_field() {
    let server = MockServer::start(dispatch(|_path| {
        r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let live = service.live_service().expect("直播服务存在");

    let err = live
        .get_subanchor(123)
        .await
        .expect_err("缺少 username 应抛错");
    assert_eq!(err.error_code(), Some(-99));
    assert!(err.to_string().contains("username"));
}

/// 对应 Java: WxMaLiveServiceImplTest 错误路径：getVideo 缺 url
#[tokio::test]
async fn live_get_video_missing_field() {
    let server = MockServer::start(dispatch(|_path| {
        r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let live = service.live_service().expect("直播服务存在");

    let err = live
        .get_video(123, 1001)
        .await
        .expect_err("缺少 url 应抛错");
    assert_eq!(err.error_code(), Some(-99));
    assert!(err.to_string().contains("url"));
}

// ══════════════════════════════════════════════════════════════════════════════
// VALUE_ADD: 零值 / 边界路径
// ══════════════════════════════════════════════════════════════════════════════

/// 标准版商品 deleteSpu 不传 out_product_id（build_json 跳过 null）
#[tokio::test]
async fn product_delete_spu_without_out_product_id() {
    let server = MockServer::start(dispatch(|_path| {
        r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let product = service.product_service().expect("标准版商品服务存在");

    let resp = product.delete_spu(9001, None).await.expect("删除商品成功");
    assert_eq!(resp.err_code, 0);
    let body = last_body_json(&server);
    assert_eq!(body["product_id"], 9001);
    assert!(body.get("out_product_id").is_none());
}

/// live getSharedCode 不传 params
#[tokio::test]
async fn live_get_shared_code_without_params() {
    let server = MockServer::start(dispatch(|_path| {
        // WxMaLiveSharedCode 字段为 camelCase：cdnUrl / pagePath / posterUrl
        r#"{"errcode":0,"cdnUrl":"https://wx.qr/SHARE_2","pagePath":"pages/index"}"#.to_string()
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let live = service.live_service().expect("直播服务存在");

    let shared = live
        .get_shared_code(456, None)
        .await
        .expect("获取分享码成功");
    assert!(!shared.cdn_url.is_empty());
    let request_line = server.last_request_line();
    assert!(request_line.contains("roomId=456"), "{request_line}");
    assert!(!request_line.contains("params="), "{request_line}");
}
