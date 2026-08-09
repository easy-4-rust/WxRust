//! 对应 Java `me.chanjar.weixin.channel.bean.after.AfterSaleListParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AfterSaleListParam {
    #[serde(rename = "begin_create_time", default)]
    pub begin_create_time: i64,
    #[serde(rename = "end_create_time", default)]
    pub end_create_time: i64,
    #[serde(rename = "begin_update_time", default)]
    pub begin_update_time: i64,
    #[serde(rename = "end_update_time", default)]
    pub end_update_time: i64,
    #[serde(rename = "next_key", default)]
    pub next_key: String,
}
