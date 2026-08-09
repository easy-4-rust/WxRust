//! 对应 Java `me.chanjar.weixin.channel.bean.product.SpuSizeChartItem.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SpuSizeChartItem {
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "unit", default)]
    pub unit: String,
    #[serde(rename = "is_range", default)]
    pub range: bool,
    #[serde(rename = "value_list", default)]
    pub value_list: Vec<ValueRange>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ValueRange {
    #[serde(rename = "key", default)]
    pub key: String,
    #[serde(rename = "value", default)]
    pub value: String,
    #[serde(rename = "left", default)]
    pub left: String,
    #[serde(rename = "right", default)]
    pub right: String,
}
