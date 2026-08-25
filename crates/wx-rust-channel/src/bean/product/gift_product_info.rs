//! 对应 Java `me.chanjar.weixin.channel.bean.product.GiftProductInfo.java`。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GiftProductInfo {
    /// 赠品商品 ID
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    /// 赠品标题
    #[serde(rename = "title", default)]
    pub title: String,
    /// 赠品副标题
    #[serde(rename = "sub_title", default)]
    pub sub_title: String,
}
