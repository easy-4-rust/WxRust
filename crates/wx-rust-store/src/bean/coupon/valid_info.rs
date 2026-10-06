//! 对应 Java `com.binarywang.wxjava.store.bean.coupon.ValidInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 ValidInfo；对应 Java com.binarywang.wxjava.store.bean.coupon.ValidInfo.java。
pub struct ValidInfo {
    #[serde(rename = "valid_type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_type: Option<i32>,
    #[serde(rename = "valid_day_num", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_day_num: Option<i32>,
    #[serde(rename = "start_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    #[serde(rename = "end_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
}
