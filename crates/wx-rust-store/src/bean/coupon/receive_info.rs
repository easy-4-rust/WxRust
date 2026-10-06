//! 对应 Java `com.binarywang.wxjava.store.bean.coupon.ReceiveInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 ReceiveInfo；对应 Java com.binarywang.wxjava.store.bean.coupon.ReceiveInfo.java。
pub struct ReceiveInfo {
    #[serde(rename = "end_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    #[serde(rename = "limit_num_one_person", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_num_one_person: Option<i32>,
    #[serde(rename = "start_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    #[serde(rename = "total_num", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_num: Option<i32>,
}
