//! 对应 Java `me.chanjar.weixin.channel.bean.product.ProductSchemeParam.java`。

#[allow(unused_imports)]
use super::*;

/// 获取商品移动应用跳转 scheme 码请求参数。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProductSchemeParam {
    /// 商品 ID。
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    /// 来源 appid。
    #[serde(rename = "from_appid", default)]
    pub from_appid: String,
    /// 过期时间。
    #[serde(rename = "expire", default)]
    pub expire: i32,
    /// 扩展信息。
    #[serde(rename = "ext_info", default)]
    pub ext_info: String,
}
