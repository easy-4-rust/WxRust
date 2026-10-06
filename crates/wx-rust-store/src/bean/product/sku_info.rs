//! 对应 Java `com.binarywang.wxjava.store.bean.product.SkuInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::SkuDeliverInfo;

#[allow(unused_imports)]
use crate::bean::base::AttrInfo;

/// 微信小店 SkuInfo 数据类型；对应 Java com.binarywang.wxjava.store.bean.product.SkuInfo.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SkuInfo {
    #[serde(rename = "out_product_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_product_id: Option<String>,
    #[serde(rename = "out_sku_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_sku_id: Option<String>,
    #[serde(rename = "thumb_img", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_img: Option<String>,
    #[serde(rename = "sale_price", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sale_price: Option<i32>,
    #[serde(rename = "market_price", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_price: Option<i32>,
    #[serde(rename = "stock_num", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stock_num: Option<i32>,
    #[serde(rename = "sku_code", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku_code: Option<String>,
    #[serde(rename = "sku_attrs", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<Vec<AttrInfo>>,
    #[serde(rename = "sku_deliver_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku_deliver_info: Option<SkuDeliverInfo>,
    #[serde(rename = "sku_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku_id: Option<String>,
    #[serde(rename = "bar_code", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bar_code: Option<String>,
}
