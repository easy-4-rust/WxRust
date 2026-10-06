//! 对应 Java `com.binarywang.wxjava.store.bean.cooperation.CooperationData.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 CooperationData；对应 Java com.binarywang.wxjava.store.bean.cooperation.CooperationData.java。
pub struct CooperationData {
    #[serde(rename = "sharer_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharer_id: Option<String>,
    #[serde(rename = "status", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "sharer_name", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharer_name: Option<String>,
    #[serde(rename = "sharer_type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharer_type: Option<i32>,
    #[serde(rename = "bind_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind_time: Option<i64>,
    #[serde(rename = "reject_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject_time: Option<i64>,
    #[serde(rename = "cancel_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_time: Option<i64>,
}
