//! 对应 Java `me.chanjar.weixin.channel.bean.shop.ShopH5UrlResponse.java`。

#[allow(unused_imports)]
use super::*;

/// 店铺 H5 链接响应。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ShopH5UrlResponse {
    /// 错误码（继承自 WxChannelBaseResponse）。
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息（继承自 WxChannelBaseResponse）。
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 店铺 H5 链接。
    #[serde(rename = "shop_h5url", default)]
    pub shop_h5url: String,
}
