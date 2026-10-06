//! 对应 Java `com.binarywang.wxjava.store.bean.product.SpuSizeChart.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::SpuSizeChartItem;

/// 微信小店 SpuSizeChart 数据类型；对应 Java com.binarywang.wxjava.store.bean.product.SpuSizeChart.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SpuSizeChart {
    #[serde(rename = "enable", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    #[serde(rename = "specification_list", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specification_list: Option<Vec<SpuSizeChartItem>>,
}
