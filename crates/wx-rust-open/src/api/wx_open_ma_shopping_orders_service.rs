//! 购物订单服务接口。
//!
//! 对应 Java `me.chanjar.weixin.open.api.WxOpenMaShoppingOrdersService`。
//!
//! URL 常量见 [`crate::enums::url_ma_domain`]（`ma_orders_*_url`，
//! api_host 前缀模式）。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::CombinedShippingInfo;
use crate::bean::CombinedShoppingInfo;
use crate::bean::ShippingInfo;
use crate::bean::ShoppingInfo;
use crate::bean::ShoppingInfoVerifyUpload;
use crate::bean::WxOpenResult;
use crate::bean::WxOpenShoppingInfoVerifyUploadResult;
use crate::bean::WxOpenShoppingOrdersConfirmResult;

/// 微信第三方平台 购物订单服务（对应 Java
/// `WxOpenMaShoppingOrdersService`）。
#[async_trait]
pub trait WxOpenMaShoppingOrdersService: Send + Sync {
    /// 上传购物详情（对应 Java `upload(ShoppingInfo info)`）。
    async fn upload_shopping_info(
        &self,
        info: &ShoppingInfo,
    ) -> Result<WxOpenResult, WxErrorException>;

    /// 上传物流信息（对应 Java `upload(ShippingInfo info)`）。
    async fn upload_shipping_info(
        &self,
        info: &ShippingInfo,
    ) -> Result<WxOpenResult, WxErrorException>;

    /// 上传合单购物详情（对应 Java `upload(CombinedShoppingInfo info)`）。
    async fn upload_combined_shopping_info(
        &self,
        info: &CombinedShoppingInfo,
    ) -> Result<WxOpenResult, WxErrorException>;

    /// 上传合单物流信息（对应 Java `upload(CombinedShippingInfo info)`）。
    async fn upload_combined_shipping_info(
        &self,
        info: &CombinedShippingInfo,
    ) -> Result<WxOpenResult, WxErrorException>;

    /// 开通购物订单产品权限（对应 Java
    /// `openShoppingOrderProductPermission()`，POST 空数据包）。
    async fn open_shopping_order_product_permission(
        &self,
    ) -> Result<WxOpenResult, WxErrorException>;

    /// 提交购物订单接入审核（对应 Java `confirmProductPermission()`，
    /// POST 空数据包）。
    async fn confirm_product_permission(
        &self,
    ) -> Result<WxOpenShoppingOrdersConfirmResult, WxErrorException>;

    /// 验证购物订单上传结果（对应 Java
    /// `verifyUploadResult(ShoppingInfoVerifyUpload info)`）。
    async fn verify_upload_result(
        &self,
        info: &ShoppingInfoVerifyUpload,
    ) -> Result<WxOpenShoppingInfoVerifyUploadResult, WxErrorException>;
}
