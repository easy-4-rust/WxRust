//! 对应 Java `me.chanjar.weixin.channel.bean.coupon.ReceiveInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReceiveInfo {
    #[serde(rename = "end_time", default)]
    pub end_time: i64,
    #[serde(rename = "limit_num_one_person", default)]
    pub limit_num_one_person: i32,
    #[serde(rename = "start_time", default)]
    pub start_time: i64,
    #[serde(rename = "total_num", default)]
    pub total_num: i32,
}
