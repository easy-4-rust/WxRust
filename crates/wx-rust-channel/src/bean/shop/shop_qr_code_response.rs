//! 对应 Java `me.chanjar.weixin.channel.bean.shop.ShopQrCodeResponse.java`。

#[allow(unused_imports)]
use super::*;

/// 店铺二维码响应。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ShopQrCodeResponse {
    /// 错误码（继承自 WxChannelBaseResponse）。
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息（继承自 WxChannelBaseResponse）。
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 店铺二维码链接。
    #[serde(rename = "shop_qrcode", default)]
    pub shop_qrcode: String,
}
