//! WxLeagueSupplierService（对应 Java `me.chanjar.weixin.channel.api.WxLeagueSupplierService`）。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::league::supplier::{
    CommissionOrderListParam, CommissionOrderListResponse, CommissionOrderResponse,
    CoopProductListResponse, CoopProductResponse, FlowListParam, ShopDetailResponse,
    ShopListResponse, SupplierBalanceResponse, SupplierFlowDetailResponse,
    SupplierFlowListResponse,
};

/// 优选联盟 团长数据服务（对应 Java `WxLeagueSupplierService`）。
///
/// 真实实现见 `crate::api::r#impl::h2b_impls::wx_league_supplier_service_impl` 的
/// `WxLeagueSupplierServiceImpl`（Java `WxLeagueSupplierServiceImpl`）。
#[async_trait]
pub trait WxLeagueSupplierService: Send + Sync {
    /// 获取团长账户余额（对应 Java `WxLeagueSupplierService#getBalanceInfo`）。
    async fn get_balance_info(&self) -> Result<SupplierBalanceResponse, WxErrorException>;

    /// 获取资金流水详情（对应 Java `WxLeagueSupplierService#getFlowDetail(String)`）。
    async fn get_flow_detail(
        &self,
        flow_id: String,
    ) -> Result<SupplierFlowDetailResponse, WxErrorException>;

    /// 获取团长资金流水列表（对应 Java `WxLeagueSupplierService#getFlowList(FlowListParam)`）。
    async fn get_flow_list(
        &self,
        param: FlowListParam,
    ) -> Result<SupplierFlowListResponse, WxErrorException>;

    /// 获取合作商品详情（对应 Java `WxLeagueSupplierService#getProductDetail(String, String)`；
    /// `app_id`：团长商品所属小店 appid）。
    async fn get_supplier_product_detail(
        &self,
        product_id: String,
        app_id: String,
    ) -> Result<CoopProductResponse, WxErrorException>;

    /// 获取合作商品列表（对应 Java `WxLeagueSupplierService#getProductList(String, Integer, String)`；
    /// `page_size` 单页商品数（不超过 30））。
    async fn get_supplier_product_list(
        &self,
        appid: String,
        page_size: Option<i32>,
        next_key: String,
    ) -> Result<CoopProductListResponse, WxErrorException>;

    /// 获取佣金单详情（对应 Java `WxLeagueSupplierService#getCommissionOrder(String, String)`）。
    async fn get_commission_order(
        &self,
        order_id: String,
        sku_id: String,
    ) -> Result<CommissionOrderResponse, WxErrorException>;

    /// 获取佣金单列表（对应 Java `WxLeagueSupplierService#getCommissionOrderList`）。
    async fn get_commission_order_list(
        &self,
        param: CommissionOrderListParam,
    ) -> Result<CommissionOrderListResponse, WxErrorException>;

    /// 获取合作小店详情（对应 Java `WxLeagueSupplierService#getShopDetail(String)`）。
    async fn get_shop_detail(&self, appid: String) -> Result<ShopDetailResponse, WxErrorException>;

    /// 获取合作小店列表（对应 Java `WxLeagueSupplierService#getShopList(Integer, String)`；
    /// `page_size` 单页小店数（不超过 30））。
    async fn get_shop_list(
        &self,
        page_size: Option<i32>,
        next_key: String,
    ) -> Result<ShopListResponse, WxErrorException>;
}
