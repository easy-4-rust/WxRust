//! 小程序交易组件-商家入驻服务。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaShopAccountService`。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::shop::request::WxMaShopAccountUpdateInfoRequest;
use crate::bean::shop::response::{
    WxMaShopAccountGetBrandListResponse, WxMaShopAccountGetCategoryListResponse,
    WxMaShopAccountGetInfoResponse, WxMaShopBaseResponse,
};

/// 小程序交易组件-商家入驻服务。
#[async_trait]
pub trait WxMaShopAccountService: Send + Sync {
    /// 获取商家类目列表（对应 Java `getCategoryList()`）。
    async fn get_category_list(
        &self,
    ) -> Result<WxMaShopAccountGetCategoryListResponse, WxErrorException>;

    /// 获取商家品牌列表（对应 Java `getBrandList()`）。
    async fn get_brand_list(&self)
    -> Result<WxMaShopAccountGetBrandListResponse, WxErrorException>;

    /// 更新商家信息（对应 Java `updateInfo(WxMaShopAccountUpdateInfoRequest)`）。
    async fn update_info(
        &self,
        request: &WxMaShopAccountUpdateInfoRequest,
    ) -> Result<WxMaShopBaseResponse, WxErrorException>;

    /// 获取商家信息（对应 Java `getInfo()`）。
    async fn get_info(&self) -> Result<WxMaShopAccountGetInfoResponse, WxErrorException>;
}
