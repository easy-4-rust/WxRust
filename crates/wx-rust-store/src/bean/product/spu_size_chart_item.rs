//! 对应 Java `com.binarywang.wxjava.store.bean.product.SpuSizeChartItem.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 SpuSizeChartItem；对应 Java com.binarywang.wxjava.store.bean.product.SpuSizeChartItem.java。
pub struct SpuSizeChartItem {
    #[serde(rename = "name", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "unit", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "is_range", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<bool>,
    #[serde(rename = "value_list", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_list: Option<Vec<ValueRange>>,
}

/// 微信小店 ValueRange 数据类型；对应 Java com.binarywang.wxjava.store.bean.product.SpuSizeChartItem.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ValueRange {
    #[serde(rename = "key", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "value", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "left", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left: Option<String>,
    #[serde(rename = "right", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right: Option<String>,
}
