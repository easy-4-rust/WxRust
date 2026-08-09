//! WxChannelCompassShopService（对应 Java `me.chanjar.weixin.channel.api.WxChannelCompassShopService`）。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::compass::shop::{
    FinderAuthListResponse, FinderListResponse, FinderOverallResponse, FinderProductListResponse,
    FinderProductOverallResponse, ShopLiveListResponse, ShopOverallResponse,
    ShopProductDataResponse, ShopProductListResponse, ShopSaleProfileDataResponse,
};

/// 视频号/微信小店 罗盘商家版服务（对应 Java `WxChannelCompassShopService`）。
///
/// 真实实现见 `crate::api::r#impl::h2b_impls::wx_channel_compass_shop_service_impl` 的
/// `WxChannelCompassShopServiceImpl`（Java `WxChannelCompassShopServiceImpl`）。
#[async_trait]
pub trait WxChannelCompassShopService: Send + Sync {
    /// 获取电商概览数据（对应 Java `WxChannelCompassShopService#getShopOverall`；
    /// `ds`：日期，格式 yyyyMMdd）。
    async fn get_shop_overall(&self, ds: String) -> Result<ShopOverallResponse, WxErrorException>;

    /// 获取授权视频号列表（对应 Java `WxChannelCompassShopService#getFinderAuthorizationList`）。
    async fn get_finder_authorization_list(
        &self,
    ) -> Result<FinderAuthListResponse, WxErrorException>;

    /// 获取带货达人列表（对应 Java `WxChannelCompassShopService#getFinderList`）。
    async fn get_finder_list(&self, ds: String) -> Result<FinderListResponse, WxErrorException>;

    /// 获取带货数据概览（对应 Java `WxChannelCompassShopService#getFinderOverall`）。
    async fn get_finder_overall(
        &self,
        ds: String,
    ) -> Result<FinderOverallResponse, WxErrorException>;

    /// 获取带货达人商品列表（对应 Java `WxChannelCompassShopService#getFinderProductList`）。
    async fn get_finder_product_list(
        &self,
        ds: String,
        finder_id: String,
    ) -> Result<FinderProductListResponse, WxErrorException>;

    /// 获取带货达人详情（对应 Java `WxChannelCompassShopService#getFinderProductOverall`）。
    async fn get_finder_product_overall(
        &self,
        ds: String,
        finder_id: String,
    ) -> Result<FinderProductOverallResponse, WxErrorException>;

    /// 获取店铺开播列表（对应 Java `WxChannelCompassShopService#getShopLiveList`）。
    async fn get_shop_live_list(
        &self,
        ds: String,
        finder_id: String,
    ) -> Result<ShopLiveListResponse, WxErrorException>;

    /// 获取商品详细信息（对应 Java `WxChannelCompassShopService#getShopProductData`）。
    async fn get_shop_product_data(
        &self,
        ds: String,
        product_id: String,
    ) -> Result<ShopProductDataResponse, WxErrorException>;

    /// 获取商品列表（对应 Java `WxChannelCompassShopService#getShopProductList`）。
    async fn get_shop_product_list(
        &self,
        ds: String,
    ) -> Result<ShopProductListResponse, WxErrorException>;

    /// 获取店铺人群数据（对应 Java `WxChannelCompassShopService#getShopSaleProfileData`；
    /// `type`：1 商品曝光用户 / 2 商品点击用户 / 3 购买用户 / 4 首购用户 / 5 复购用户）。
    async fn get_shop_sale_profile_data(
        &self,
        ds: String,
        r#type: Option<i32>,
    ) -> Result<ShopSaleProfileDataResponse, WxErrorException>;
}
