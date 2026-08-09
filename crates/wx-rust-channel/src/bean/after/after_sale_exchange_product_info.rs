//! 对应 Java `me.chanjar.weixin.channel.bean.after.AfterSaleExchangeProductInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AfterSaleExchangeProductInfo {
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    #[serde(rename = "old_sku_id", default)]
    pub old_sku_id: String,
    #[serde(rename = "new_sku_id", default)]
    pub new_sku_id: String,
    #[serde(rename = "product_cnt", default)]
    pub product_cnt: String,
    #[serde(rename = "old_sku_price", default)]
    pub old_sku_price: i32,
    #[serde(rename = "new_sku_price", default)]
    pub new_sku_price: i32,
}
