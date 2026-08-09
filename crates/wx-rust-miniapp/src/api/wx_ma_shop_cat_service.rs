//! 小程序交易组件-接入商品前必需接口（商品类目）。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaShopCatService`。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::shop::response::WxMaShopCatGetResponse;

/// 小程序交易组件-商品类目服务。
#[async_trait]
pub trait WxMaShopCatService: Send + Sync {
    /// 获取商品类目（对应 Java `getCat()`）。
    async fn get_cat(&self) -> Result<WxMaShopCatGetResponse, WxErrorException>;
}
