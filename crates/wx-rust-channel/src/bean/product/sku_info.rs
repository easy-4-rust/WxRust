//! 对应 Java `me.chanjar.weixin.channel.bean.product.SkuInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[allow(unused_imports)]
use crate::bean::base::AttrInfo;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SkuInfo {
    #[serde(rename = "out_product_id", default)]
    pub out_product_id: String,
    #[serde(rename = "out_sku_id", default)]
    pub out_sku_id: String,
    #[serde(rename = "thumb_img", default)]
    pub thumb_img: String,
    #[serde(rename = "sale_price", default)]
    pub sale_price: i32,
    #[serde(rename = "market_price", default)]
    pub market_price: i32,
    #[serde(rename = "stock_num", default)]
    pub stock_num: i32,
    #[serde(rename = "sku_code", default)]
    pub sku_code: String,
    #[serde(rename = "sku_attrs", default)]
    pub attrs: Vec<AttrInfo>,
    #[serde(rename = "sku_deliver_info", default)]
    pub sku_deliver_info: SkuDeliverInfo,
    #[serde(rename = "sku_id", default)]
    pub sku_id: String,
    #[serde(rename = "bar_code", default)]
    pub bar_code: String,
}
