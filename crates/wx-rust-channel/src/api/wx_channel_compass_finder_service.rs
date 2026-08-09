//! WxChannelCompassFinderService（对应 Java `me.chanjar.weixin.channel.api.WxChannelCompassFinderService`）。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::compass::finder::{
    OverallResponse, ProductDataResponse, ProductListResponse, SaleProfileDataResponse,
};

/// 视频号助手 罗盘达人版服务（对应 Java `WxChannelCompassFinderService`）。
///
/// 真实实现见 `crate::api::r#impl::h2b_impls::wx_channel_compass_finder_service_impl` 的
/// `WxChannelCompassFinderServiceImpl`（Java `WxChannelCompassFinderServiceImpl`）。
#[async_trait]
pub trait WxChannelCompassFinderService: Send + Sync {
    /// 获取电商概览数据（对应 Java `WxChannelCompassFinderService#getOverall`；
    /// `ds`：日期，格式 yyyyMMdd）。
    async fn get_overall(&self, ds: String) -> Result<OverallResponse, WxErrorException>;

    /// 获取带货商品数据（对应 Java `WxChannelCompassFinderService#getProductData`）。
    async fn get_product_data(
        &self,
        ds: String,
        product_id: String,
    ) -> Result<ProductDataResponse, WxErrorException>;

    /// 获取带货商品列表（对应 Java `WxChannelCompassFinderService#getProductList`）。
    async fn get_product_list(&self, ds: String) -> Result<ProductListResponse, WxErrorException>;

    /// 获取带货人群数据（对应 Java `WxChannelCompassFinderService#getSaleProfileData`；
    /// `type`：1 商品曝光用户 / 2 商品点击用户 / 3 购买用户 / 4 首购用户 / 5 复购用户 / 6 直播观看用户）。
    async fn get_sale_profile_data(
        &self,
        ds: String,
        r#type: Option<i32>,
    ) -> Result<SaleProfileDataResponse, WxErrorException>;
}
