//! 对应 Java `cn.binarywang.wx.miniapp.bean.product.WxMinishopPriceInfo.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMinishopPriceInfo {
    #[serde(rename = "product_price", default)]
    pub product_price: i32,
    #[serde(rename = "order_price", default)]
    pub order_price: i32,
    #[serde(rename = "freight", default)]
    pub freight: i32,
    #[serde(rename = "discounted_price", default)]
    pub discounted_price: i32,
    #[serde(rename = "is_discounted", default)]
    pub is_discounted: bool,
}
