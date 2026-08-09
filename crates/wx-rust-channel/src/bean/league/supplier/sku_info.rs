//! 对应 Java `me.chanjar.weixin.channel.bean.league.supplier.SkuInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::league::*;

#[allow(unused_imports)]
use crate::bean::base::AttrInfo;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SkuInfo {
    #[serde(rename = "sku_id", default)]
    pub sku_id: String,
    #[serde(rename = "thumb_img", default)]
    pub thumb_img: String,
    #[serde(rename = "sale_price", default)]
    pub sale_price: i32,
    #[serde(rename = "stock_num", default)]
    pub stock_num: i32,
    #[serde(rename = "sku_attrs", default)]
    pub sku_attrs: Vec<AttrInfo>,
}
