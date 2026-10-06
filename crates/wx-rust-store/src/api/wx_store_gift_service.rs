//! WxStoreGiftService（对应 Java `com.binarywang.wxjava.store.api.WxStoreGiftService`）。

use wx_rust_common::error::WxErrorException;

use crate::bean::base::WxStoreBaseResponse;
use crate::bean::product::{
    GiftActivityAddResponse, GiftActivityInfo, GiftProductAddResponse, GiftProductGetResponse,
    GiftProductInfo, GiftProductListParam, GiftProductListResponse,
};

/// 赠品与买赠活动服务（对应 Java `WxStoreGiftService`）。
///
/// 真实实现见 `crate::api::r#impl::wx_store_gift_service_impl` 的
/// `WxStoreGiftServiceImpl`（Java `WxStoreGiftServiceImpl`）。
#[async_trait::async_trait]
pub trait WxStoreGiftService: Send + Sync {
    /// 添加非卖商品（对应 Java `WxStoreGiftService#addGiftProduct(GiftProductInfo)`）。
    async fn add_gift_product(
        &self,
        info: GiftProductInfo,
    ) -> Result<GiftProductAddResponse, WxErrorException>;

    /// 更新非卖商品（对应 Java `WxStoreGiftService#updateGiftProduct(GiftProductInfo)`）。
    async fn update_gift_product(
        &self,
        info: GiftProductInfo,
    ) -> Result<WxStoreBaseResponse, WxErrorException>;

    /// 在售商品转赠品（对应 Java `WxStoreGiftService#setProductAsGift(String)`）。
    async fn set_product_as_gift(
        &self,
        product_id: String,
    ) -> Result<WxStoreBaseResponse, WxErrorException>;

    /// 获取赠品（对应 Java `WxStoreGiftService#getGiftProduct(String)`）。
    async fn get_gift_product(
        &self,
        product_id: String,
    ) -> Result<GiftProductGetResponse, WxErrorException>;

    /// 获取赠品列表（对应 Java `WxStoreGiftService#listGiftProduct(GiftProductListParam)`）。
    async fn list_gift_product(
        &self,
        param: GiftProductListParam,
    ) -> Result<GiftProductListResponse, WxErrorException>;

    /// 更新赠品库存（对应 Java `WxStoreGiftService#updateGiftStock(String, String, Integer, Integer)`）。
    async fn update_gift_stock(
        &self,
        product_id: String,
        sku_id: String,
        diff_type: i32,
        num: i32,
    ) -> Result<WxStoreBaseResponse, WxErrorException>;

    /// 创建赠品活动（对应 Java `WxStoreGiftService#addGiftActivity(GiftActivityInfo)`）。
    async fn add_gift_activity(
        &self,
        info: GiftActivityInfo,
    ) -> Result<GiftActivityAddResponse, WxErrorException>;

    /// 删除赠品活动（对应 Java `WxStoreGiftService#deleteGiftActivity(String)`）。
    async fn delete_gift_activity(
        &self,
        activity_id: String,
    ) -> Result<WxStoreBaseResponse, WxErrorException>;

    /// 停止赠品活动（对应 Java `WxStoreGiftService#stopGiftActivity(String)`）。
    async fn stop_gift_activity(
        &self,
        activity_id: String,
    ) -> Result<WxStoreBaseResponse, WxErrorException>;
}
