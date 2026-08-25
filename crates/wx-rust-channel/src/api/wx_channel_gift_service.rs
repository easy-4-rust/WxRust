//! WxChannelGiftService（对应 Java `me.chanjar.weixin.channel.api.WxChannelGiftService`）。

use wx_rust_common::error::WxErrorException;

use crate::bean::base::WxChannelBaseResponse;
use crate::bean::product::{
    GiftActivityAddResponse, GiftActivityInfo, GiftProductAddResponse, GiftProductGetResponse,
    GiftProductInfo, GiftProductListParam, GiftProductListResponse,
};

/// 赠品与买赠活动服务（对应 Java `WxChannelGiftService`）。
///
/// 真实实现见 `crate::api::r#impl::wx_channel_gift_service_impl` 的
/// `WxChannelGiftServiceImpl`（Java `WxChannelGiftServiceImpl`）。
#[async_trait::async_trait]
pub trait WxChannelGiftService: Send + Sync {
    /// 添加非卖商品（对应 Java `WxChannelGiftService#addGiftProduct(GiftProductInfo)`）。
    async fn add_gift_product(
        &self,
        info: GiftProductInfo,
    ) -> Result<GiftProductAddResponse, WxErrorException>;

    /// 更新非卖商品（对应 Java `WxChannelGiftService#updateGiftProduct(GiftProductInfo)`）。
    async fn update_gift_product(
        &self,
        info: GiftProductInfo,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 在售商品转赠品（对应 Java `WxChannelGiftService#setProductAsGift(String)`）。
    async fn set_product_as_gift(
        &self,
        product_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 获取赠品（对应 Java `WxChannelGiftService#getGiftProduct(String)`）。
    async fn get_gift_product(
        &self,
        product_id: String,
    ) -> Result<GiftProductGetResponse, WxErrorException>;

    /// 获取赠品列表（对应 Java `WxChannelGiftService#listGiftProduct(GiftProductListParam)`）。
    async fn list_gift_product(
        &self,
        param: GiftProductListParam,
    ) -> Result<GiftProductListResponse, WxErrorException>;

    /// 更新赠品库存（对应 Java `WxChannelGiftService#updateGiftStock(String, String, Integer, Integer)`）。
    async fn update_gift_stock(
        &self,
        product_id: String,
        sku_id: String,
        diff_type: i32,
        num: i32,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 创建赠品活动（对应 Java `WxChannelGiftService#addGiftActivity(GiftActivityInfo)`）。
    async fn add_gift_activity(
        &self,
        info: GiftActivityInfo,
    ) -> Result<GiftActivityAddResponse, WxErrorException>;

    /// 删除赠品活动（对应 Java `WxChannelGiftService#deleteGiftActivity(String)`）。
    async fn delete_gift_activity(
        &self,
        activity_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 停止赠品活动（对应 Java `WxChannelGiftService#stopGiftActivity(String)`）。
    async fn stop_gift_activity(
        &self,
        activity_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;
}
