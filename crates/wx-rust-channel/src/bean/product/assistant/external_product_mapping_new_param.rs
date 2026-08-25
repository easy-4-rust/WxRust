//! 对应 Java `me.chanjar.weixin.channel.bean.product.assistant.ExternalProductMappingNewParam.java`。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExternalProductMappingNewParam {
    /// 外部商品 ID
    #[serde(rename = "out_product_id", default)]
    pub out_product_id: String,
}
