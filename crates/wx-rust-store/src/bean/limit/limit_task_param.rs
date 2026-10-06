//! 对应 Java `com.binarywang.wxjava.store.bean.limit.LimitTaskParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::LimitSku;

/// 微信小店 LimitTaskParam 数据类型；对应 Java com.binarywang.wxjava.store.bean.limit.LimitTaskParam.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LimitTaskParam {
    #[serde(rename = "product_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename = "start_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "end_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "limited_discount_skus", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skus: Option<Vec<LimitSku>>,
}
