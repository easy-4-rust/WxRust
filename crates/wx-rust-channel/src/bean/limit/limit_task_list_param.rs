//! 对应 Java `me.chanjar.weixin.channel.bean.limit.LimitTaskListParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[allow(unused_imports)]
use crate::bean::base::StreamPageParam;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LimitTaskListParam {
    #[serde(rename = "page_size", default)]
    pub page_size: i32,
    #[serde(rename = "next_key", default)]
    pub next_key: String,
    #[serde(rename = "status", default)]
    pub status: i32,
}
