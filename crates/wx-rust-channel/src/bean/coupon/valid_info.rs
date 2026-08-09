//! 对应 Java `me.chanjar.weixin.channel.bean.coupon.ValidInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ValidInfo {
    #[serde(rename = "valid_type", default)]
    pub valid_type: i32,
    #[serde(rename = "valid_day_num", default)]
    pub valid_day_num: i32,
    #[serde(rename = "start_time", default)]
    pub start_time: i64,
    #[serde(rename = "end_time", default)]
    pub end_time: i64,
}
