//! 对应 Java `me.chanjar.weixin.channel.bean.live.dashboard.ConversionMetric.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::live::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ConversionMetric {
    #[serde(rename = "product_view_click_conversion_ratio", default)]
    pub product_view_click_conversion_ratio: ItemConversionMetric,
    #[serde(rename = "bubble_view_click_conversion_ratio", default)]
    pub bubble_view_click_conversion_ratio: ItemConversionMetric,
    #[serde(rename = "pay_conversion_ratio", default)]
    pub pay_conversion_ratio: ItemConversionMetric,
    #[serde(rename = "k_view_pay_conversion_ratio", default)]
    pub k_view_pay_conversion_ratio: ItemConversionMetric,
    #[serde(rename = "update_time", default)]
    pub update_time: i64,
    #[serde(rename = "product_list_click_conversion_ratio", default)]
    pub product_list_click_conversion_ratio: ItemConversionMetric,
    #[serde(rename = "shelftime", default)]
    pub shelftime: i64,
}
