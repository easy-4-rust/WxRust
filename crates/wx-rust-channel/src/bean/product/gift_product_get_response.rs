//! 对应 Java `me.chanjar.weixin.channel.bean.product.GiftProductGetResponse.java`。

#[allow(unused_imports)]
use super::*;

#[allow(unused_imports)]
use crate::bean::base::WxChannelBaseResponse;

use super::gift_product_info::GiftProductInfo;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GiftProductGetResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 赠品信息
    #[serde(rename = "gift_product_info", default)]
    pub gift_product_info: GiftProductInfo,
}
