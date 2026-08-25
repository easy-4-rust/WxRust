//! 对应 Java `me.chanjar.weixin.channel.bean.product.assistant.ProductBrandRecommendParam.java`。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProductBrandRecommendParam {
    /// 商品名称
    #[serde(rename = "product_name", default)]
    pub product_name: String,
}
