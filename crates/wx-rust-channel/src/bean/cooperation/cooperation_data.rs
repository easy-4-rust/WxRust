//! 对应 Java `me.chanjar.weixin.channel.bean.cooperation.CooperationData.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CooperationData {
    #[serde(rename = "sharer_id", default)]
    pub sharer_id: String,
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "sharer_name", default)]
    pub sharer_name: String,
    #[serde(rename = "sharer_type", default)]
    pub sharer_type: i32,
    #[serde(rename = "bind_time", default)]
    pub bind_time: i64,
    #[serde(rename = "reject_time", default)]
    pub reject_time: i64,
    #[serde(rename = "cancel_time", default)]
    pub cancel_time: i64,
}
