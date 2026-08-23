//! 覆盖提升：`WxChannelService` trait 默认实现（`wx_channel_service.rs`）
//! Wave-0 占位方法补齐（与 coverage_boost_channel_service_mock.rs 互补，
//! 不重复其用例）。
//!
//! 覆盖既有用例未触达的占位方法：
//! - Brand：addBrandApply/updateBrandApply；
//! - Product：addProduct/updateProduct（SpuUpdateInfo/SpuInfo 两种重载）、
//!   updateProductAuditFree、updateStock、addLimitTask/listLimitTask；
//! - Warehouse：createWarehouse、addWarehouseArea/deleteWarehouseArea、
//!   setWarehousePriority/getWarehousePriority；
//! - Order：getOrders、updateAddress、updateDelivery、
//!   acceptAddressModify/rejectAddressModify、deliveryOrder、
//!   uploadFreshInspect；
//! - FreightTemplate：addTemplate/updateTemplate；
//! - Address：addAddress/updateAddress；
//! - Coupon：createCoupon/updateCoupon/getCouponList/getUserCouponList；
//! - Sharer：listSharerOrder；
//! - Fund：listFundsFlow、listWithdraw、setBankAccount、searchCityList、
//!   searchBranchList；
//! - HomePage：addTreeProduct/delTreeProduct/getTreeProductList/setShowTree、
//!   reorderWindowProduct/hideWindowProduct/topWindowProduct、applyBanner；
//! - LeagueWindow：listProduct；
//! - LeagueSupplier：getFlowList/getProductList/getCommissionOrder(List)；
//! - LeagueProduct：batchAddProduct/updateProduct/getProductDetail/listProduct；
//! - LeadComponent：getLeadsInfoByComponentId/getLeadsInfoByRequestId/
//!   getLeadsRequestId/getLeadsComponentPromoteRecord/getLeadsComponentId；
//! - FinderLive：getFinderLiveDataList/getFinderLiveLeadsData；
//! - Assistant：addWindowProduct/getWindowProduct/getWindowProductList/
//!   offWindowProduct。
//!
//! 测试三层：
//! - SOURCE_PARITY: Wave-0 占位方法返回 Err(-99)（对应 Java 接口签名冻结）
//! - RUST_OBLIGATION: 占位方法不产生网络副作用（请求计数为 0）
//! - VALUE_ADD: 按 Java `WxChannel*Service` 子域分组批量断言

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use wx_rust_channel::api::WxChannelService;
use wx_rust_channel::bean::address::AddressDetail;
use wx_rust_channel::bean::base::AddressInfo;
use wx_rust_channel::bean::brand::Brand;
use wx_rust_channel::bean::coupon::{CouponListParam, CouponParam, UserCouponListParam};
use wx_rust_channel::bean::delivery::{DeliveryInfo, PackageAuditInfo};
use wx_rust_channel::bean::freight::FreightTemplate;
use wx_rust_channel::bean::fund::{AccountInfo, FundsListParam};
use wx_rust_channel::bean::home::banner::BannerInfo;
use wx_rust_channel::bean::home::tree::{TreeProductEditInfo, TreeProductListInfo, TreeShowInfo};
use wx_rust_channel::bean::league::product::{
    BatchAddParam, ProductDetailParam, ProductListParam, ProductUpdateParam,
};
use wx_rust_channel::bean::league::supplier::{CommissionOrderListParam, FlowListParam};
use wx_rust_channel::bean::league::window::ProductSearchParam;
use wx_rust_channel::bean::limit::LimitTaskParam;
use wx_rust_channel::bean::order::{ChangeOrderInfo, DeliveryUpdateParam, OrderListParam};
use wx_rust_channel::bean::product::{SpuFastInfo, SpuInfo, SpuUpdateInfo};
use wx_rust_channel::bean::sharer::SharerOrderParam;
use wx_rust_channel::bean::warehouse::{
    PriorityLocationParam, WarehouseLocation, WarehouseParam, WarehouseStockParam,
};
use wx_rust_channel::config::WxChannelConfig;
use wx_rust_channel::config::r#impl::WxChannelDefaultConfig;
use wx_rust_common::config::WxConfigStorage;

// ═══════════════════════════════════════════════════════════════
// 测试夹具：MockServer + 配置工厂（与 coverage_boost_channel_service_mock.rs 同一模式）
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

/// Wave-0 占位方法统一断言：Err(-99) 且不产生网络请求。
async fn assert_placeholder<F, Fut>(server: &MockServer, invoke: F)
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<(), wx_rust_common::error::WxErrorException>>,
{
    let _ = invoke().await;
    assert_eq!(server.request_count(), 0, "Wave-0 占位方法不应产生网络请求");
}

// ═══════════════════════════════════════════════════════════════
// SOURCE_PARITY：WxChannelBrandService 补齐（add/update）
// ═══════════════════════════════════════════════════════════════

/// 品牌资质新增/修改占位。
/// 对应 Java: `WxChannelBrandService#addBrandApply` / `#updateBrandApply`
#[tokio::test]
async fn wave0_brand_apply_add_and_update() {
    let server = MockServer::start(|_, _| r#"{"errcode":0}"#.to_string()).await;
    let service = new_service(config_with_host(&server.url()));

    let brand = Brand::default();
    assert_eq!(
        service
            .add_brand_apply(brand.clone())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .update_brand_apply(brand)
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(server.request_count(), 0);
}

// ═══════════════════════════════════════════════════════════════
// SOURCE_PARITY：WxChannelProductService 补齐（增改/免审/库存/限时抢购）
// ═══════════════════════════════════════════════════════════════

/// 商品增改四种重载 + 免审更新 + 库存 + 限时抢购任务。
/// 对应 Java: `WxChannelProductService#addProduct/updateProduct/
/// updateProductAuditFree/updateStock/addLimitTask/listLimitTask`
#[tokio::test]
async fn wave0_product_add_update_stock_and_limit_task() {
    let server = MockServer::start(|_, _| r#"{"errcode":0}"#.to_string()).await;
    let service = new_service(config_with_host(&server.url()));

    assert_eq!(
        service
            .add_product(SpuUpdateInfo::default())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .update_product(SpuUpdateInfo::default())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .add_product_with_spu_info(SpuInfo::default())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .update_product_with_spu_info(SpuInfo::default())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .update_product_audit_free(SpuFastInfo::default())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .update_stock("pid".into(), "sid".into(), Some(1), Some(10))
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .add_limit_task(LimitTaskParam::default())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .list_limit_task(None, "".into(), None)
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(server.request_count(), 0);
}

// ═══════════════════════════════════════════════════════════════
// SOURCE_PARITY：WxChannelWarehouseService 补齐（建仓/覆盖区域/优先级）
// ═══════════════════════════════════════════════════════════════

/// 仓库创建、覆盖区域增删、优先级设置与查询。
/// 对应 Java: `WxChannelWarehouseService#createWarehouse/addWarehouseArea/
/// deleteWarehouseArea/setWarehousePriority/getWarehousePriority`
#[tokio::test]
async fn wave0_warehouse_create_area_and_priority() {
    let server = MockServer::start(|_, _| r#"{"errcode":0}"#.to_string()).await;
    let service = new_service(config_with_host(&server.url()));

    assert_eq!(
        service
            .create_warehouse(WarehouseParam::default())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .add_warehouse_area("wid".into(), vec![WarehouseLocation::default()])
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .delete_warehouse_area("wid".into(), vec![WarehouseLocation::default()])
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .set_warehouse_priority(PriorityLocationParam::default())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_warehouse_priority(Some(110000), None, None, None)
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .update_warehouse_stock(WarehouseStockParam::default())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(server.request_count(), 0);
}

// ═══════════════════════════════════════════════════════════════
// SOURCE_PARITY：WxChannelOrderService 补齐（列表/地址/物流/发货）
// ═══════════════════════════════════════════════════════════════

/// 订单列表、改地址、改物流、同意/拒绝改址、发货、生鲜质检。
/// 对应 Java: `WxChannelOrderService#getOrders/updateAddress/updateDelivery/
/// acceptAddressModify/rejectAddressModify/deliveryOrder/uploadFreshInspect`
#[tokio::test]
async fn wave0_order_list_address_delivery_and_ship() {
    let server = MockServer::start(|_, _| r#"{"errcode":0}"#.to_string()).await;
    let service = new_service(config_with_host(&server.url()));

    assert_eq!(
        service
            .get_orders(OrderListParam::default())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .update_order_address("oid".into(), AddressInfo::default())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .update_delivery(DeliveryUpdateParam::default())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .accept_address_modify("oid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .reject_address_modify("oid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .delivery_order("oid".into(), vec![DeliveryInfo::default()])
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .upload_fresh_inspect("oid".into(), vec![PackageAuditInfo::default()])
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .update_price("oid".into(), None, vec![ChangeOrderInfo::default()])
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(server.request_count(), 0);
}

// ═══════════════════════════════════════════════════════════════
// SOURCE_PARITY：FreightTemplate/Address/Coupon 补齐
// ═══════════════════════════════════════════════════════════════

/// 运费模板增改、地址增改、优惠券增改与列表查询。
/// 对应 Java: `addTemplate/updateTemplate/addAddress/updateAddress/
/// createCoupon/updateCoupon/getCouponList/getUserCouponList`
#[tokio::test]
async fn wave0_freight_address_and_coupon_placeholders() {
    let server = MockServer::start(|_, _| r#"{"errcode":0}"#.to_string()).await;
    let service = new_service(config_with_host(&server.url()));

    assert_eq!(
        service
            .add_template(FreightTemplate::default())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .update_template(FreightTemplate::default())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .add_address(AddressDetail::default())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .update_address_detail(AddressDetail::default())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .create_coupon(CouponParam::default())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .update_coupon(CouponParam::default())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_coupon_list(CouponListParam::default())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_user_coupon_list(UserCouponListParam::default())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(server.request_count(), 0);
}

// ═══════════════════════════════════════════════════════════════
// SOURCE_PARITY：Sharer/Fund 补齐
// ═══════════════════════════════════════════════════════════════

/// 分享员订单列表 + 资金流水/提现列表/结算账户/城市支行查询。
/// 对应 Java: `listSharerOrder/listFundsFlow/listWithdraw/setBankAccount/
/// searchCityList/searchBranchList`
#[tokio::test]
async fn wave0_sharer_order_and_fund_placeholders() {
    let server = MockServer::start(|_, _| r#"{"errcode":0}"#.to_string()).await;
    let service = new_service(config_with_host(&server.url()));

    assert_eq!(
        service
            .list_sharer_order(SharerOrderParam::default())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .list_funds_flow(FundsListParam::default())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .list_withdraw(Some(1), Some(10), Some(0), Some(0))
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .set_bank_account(AccountInfo::default())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .search_city_list("110000".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .search_branch_list("bank".into(), "110100".into(), None, None)
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(server.request_count(), 0);
}

// ═══════════════════════════════════════════════════════════════
// SOURCE_PARITY：WxStoreHomePageService 补齐（分类树/橱窗/横幅）
// ═══════════════════════════════════════════════════════════════

/// 分类树增删查设、橱窗排序/隐藏/置顶、横幅申请。
/// 对应 Java: `addTreeProduct/delTreeProduct/getTreeProductList/setShowTree/
/// reorderWindowProduct/hideWindowProduct/topWindowProduct/applyBanner`
#[tokio::test]
async fn wave0_home_page_tree_window_and_banner() {
    let server = MockServer::start(|_, _| r#"{"errcode":0}"#.to_string()).await;
    let service = new_service(config_with_host(&server.url()));

    assert_eq!(
        service
            .add_tree_product(TreeProductEditInfo::default())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .del_tree_product(TreeProductEditInfo::default())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_tree_product_list(TreeProductListInfo::default())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .set_show_tree(TreeShowInfo::default())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .reorder_window_product("pid".into(), Some(1))
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .hide_window_product("pid".into(), Some(1))
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .top_window_product("pid".into(), Some(1))
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .apply_banner(BannerInfo::default())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(server.request_count(), 0);
}

// ═══════════════════════════════════════════════════════════════
// SOURCE_PARITY：League 三个子服务补齐（橱窗/团长/商品）
// ═══════════════════════════════════════════════════════════════

/// 联盟橱窗列表、团长流水/商品/佣金单、联盟商品四方法。
/// 对应 Java: `WxLeagueWindowService#listProduct`、
/// `WxLeagueSupplierService#getFlowList/getProductList/
/// getCommissionOrder/getCommissionOrderList`、
/// `WxLeagueProductService#batchAddProduct/updateProduct/
/// getProductDetail/listProduct`
#[tokio::test]
async fn wave0_league_window_supplier_and_product() {
    let server = MockServer::start(|_, _| r#"{"errcode":0}"#.to_string()).await;
    let service = new_service(config_with_host(&server.url()));

    assert_eq!(
        service
            .list_league_window_product(ProductSearchParam::default())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_flow_list(FlowListParam::default())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_supplier_product_list("appid".into(), None, "".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_commission_order("oid".into(), "sid".into())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_commission_order_list(CommissionOrderListParam::default())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .batch_add_product(BatchAddParam::default())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .update_league_product(ProductUpdateParam::default())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_product_detail(ProductDetailParam::default())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .list_league_product(ProductListParam::default())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(server.request_count(), 0);
}

// ═══════════════════════════════════════════════════════════════
// SOURCE_PARITY：WxLeadComponentService + WxFinderLiveService 补齐
// ═══════════════════════════════════════════════════════════════

/// 留资组件五个查询 + 直播数据两个查询。
/// 对应 Java: `getLeadsInfoByComponentId/getLeadsInfoByRequestId/
/// getLeadsRequestId/getLeadsComponentPromoteRecord/getLeadsComponentId/
/// getFinderLiveDataList/getFinderLiveLeadsData`
#[tokio::test]
async fn wave0_lead_component_and_finder_live() {
    let server = MockServer::start(|_, _| r#"{"errcode":0}"#.to_string()).await;
    let service = new_service(config_with_host(&server.url()));

    use wx_rust_channel::bean::lead::component::request::{
        GetFinderLiveDataListRequest, GetFinderLiveLeadsDataRequest, GetLeadInfoByComponentRequest,
        GetLeadsComponentIdRequest, GetLeadsComponentPromoteRecordRequest,
        GetLeadsInfoByRequestIdRequest, GetLeadsRequestIdRequest,
    };

    assert_eq!(
        service
            .get_leads_info_by_component_id(GetLeadInfoByComponentRequest::default())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_leads_info_by_request_id(GetLeadsInfoByRequestIdRequest::default())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_leads_request_id(GetLeadsRequestIdRequest::default())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_leads_component_promote_record(GetLeadsComponentPromoteRecordRequest::default())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_leads_component_id(GetLeadsComponentIdRequest::default())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_finder_live_data_list(GetFinderLiveDataListRequest::default())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_finder_live_leads_data(GetFinderLiveLeadsDataRequest::default())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(server.request_count(), 0);
}

// ═══════════════════════════════════════════════════════════════
// SOURCE_PARITY：WxAssistantService 橱窗四方法补齐
// ═══════════════════════════════════════════════════════════════

/// 视频号助手橱窗上下架与查询。
/// 对应 Java: `WxAssistantService#addWindowProduct/getWindowProduct/
/// getWindowProductList/offWindowProduct`
#[tokio::test]
async fn wave0_assistant_window_products() {
    let server = MockServer::start(|_, _| r#"{"errcode":0}"#.to_string()).await;
    let service = new_service(config_with_host(&server.url()));

    use wx_rust_channel::bean::window::request::{
        AddWindowProductRequest, GetWindowProductListRequest, WindowProductRequest,
    };

    assert_eq!(
        service
            .add_window_product(AddWindowProductRequest::default())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_window_product(WindowProductRequest::default())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .get_window_product_list(GetWindowProductListRequest::default())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(
        service
            .off_window_product(WindowProductRequest::default())
            .await
            .unwrap_err()
            .error_code(),
        Some(-99)
    );
    assert_eq!(server.request_count(), 0);
}

// ═══════════════════════════════════════════════════════════════
// RUST_OBLIGATION：占位方法统一无网络副作用验证（helper 自身行为）
// ═══════════════════════════════════════════════════════════════

/// assert_placeholder 辅助：占位方法调用后请求计数保持 0。
/// 对应 Java: Wave-0 占位方法体仅 `throw new UnsupportedOperationException`
#[tokio::test]
async fn wave0_placeholder_no_network_side_effect() {
    let server = MockServer::start(|_, _| r#"{"errcode":0}"#.to_string()).await;
    let service = new_service(config_with_host(&server.url()));

    assert_placeholder(&server, || async {
        service.get_shop_info().await.map(|_| ())
    })
    .await;
    let err = service.get_shop_info().await.expect_err("Wave 0 占位");
    assert_eq!(err.error_code(), Some(-99));
    assert_eq!(server.request_count(), 0);
}
