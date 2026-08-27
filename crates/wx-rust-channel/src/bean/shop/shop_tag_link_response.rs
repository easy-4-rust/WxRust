//! 对应 Java `me.chanjar.weixin.channel.bean.shop.ShopTagLinkResponse.java`。

#[allow(unused_imports)]
use super::*;

/// 店铺口令响应。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ShopTagLinkResponse {
    /// 错误码（继承自 WxChannelBaseResponse）。
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息（继承自 WxChannelBaseResponse）。
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 店铺微信口令。
    #[serde(rename = "shop_taglink", default)]
    pub shop_taglink: String,
}
