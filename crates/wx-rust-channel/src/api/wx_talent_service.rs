//! WxTalentService（对应 Java `me.chanjar.weixin.channel.api.WxTalentService`）。

use wx_rust_common::error::WxErrorException;

use crate::bean::talent::{
    TalentOrderDetailParam, TalentOrderDetailResponse, TalentOrderListParam,
    TalentOrderListResponse, TalentWindowProductDetailParam, TalentWindowProductDetailResponse,
    TalentWindowProductListParam, TalentWindowProductListResponse,
};

/// 带货助手服务（对应 Java `WxTalentService`）。
///
/// 真实实现见 `crate::api::r#impl::wx_talent_service_impl` 的
/// `WxTalentServiceImpl`（Java `WxTalentServiceImpl`）。
#[async_trait::async_trait]
pub trait WxTalentService: Send + Sync {
    /// 获取佣金单列表（对应 Java `WxTalentService#getOrderList(TalentOrderListParam)`）。
    async fn get_order_list(
        &self,
        param: TalentOrderListParam,
    ) -> Result<TalentOrderListResponse, WxErrorException>;

    /// 获取佣金单详情（对应 Java `WxTalentService#getOrderDetail(TalentOrderDetailParam)`）。
    async fn get_order_detail(
        &self,
        param: TalentOrderDetailParam,
    ) -> Result<TalentOrderDetailResponse, WxErrorException>;

    /// 获取达人橱窗商品列表（对应 Java `WxTalentService#getWindowProductList(TalentWindowProductListParam)`）。
    async fn get_window_product_list(
        &self,
        param: TalentWindowProductListParam,
    ) -> Result<TalentWindowProductListResponse, WxErrorException>;

    /// 获取达人橱窗商品详情（对应 Java `WxTalentService#getWindowProductDetail(TalentWindowProductDetailParam)`）。
    async fn get_window_product_detail(
        &self,
        param: TalentWindowProductDetailParam,
    ) -> Result<TalentWindowProductDetailResponse, WxErrorException>;
}
