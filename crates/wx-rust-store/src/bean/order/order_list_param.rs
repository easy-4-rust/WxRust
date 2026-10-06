//! 对应 Java `com.binarywang.wxjava.store.bean.order.OrderListParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use crate::bean::base::TimeRange;

/// 微信小店 OrderListParam 数据类型；对应 Java com.binarywang.wxjava.store.bean.order.OrderListParam.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderListParam {
    #[serde(rename = "page_size", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "next_key", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_key: Option<String>,
    #[serde(rename = "create_time_range", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time_range: Option<TimeRange>,
    #[serde(rename = "update_time_range", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time_range: Option<TimeRange>,
    #[serde(rename = "status", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "openid", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openid: Option<i32>,
}
