//! 对应 Java `me.chanjar.weixin.channel.bean.supplier.ProductDistributeRequest.java`。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProductDistributeRequest {
    /// 商品 ID
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    /// 供货商 ID
    #[serde(rename = "supplier_id", default)]
    pub supplier_id: String,
}
