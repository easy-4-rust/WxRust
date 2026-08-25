//! 对应 Java `me.chanjar.weixin.channel.bean.product.assistant.ExternalProductMappingParam.java`。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExternalProductMappingParam {
    /// 外部商品 ID
    #[serde(rename = "out_product_id", default)]
    pub out_product_id: String,
}
