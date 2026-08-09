//! 对应 Java `me.chanjar.weixin.channel.bean.order.OrderListParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[allow(unused_imports)]
use crate::bean::base::StreamPageParam;
#[allow(unused_imports)]
use crate::bean::base::TimeRange;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderListParam {
    #[serde(rename = "page_size", default)]
    pub page_size: i32,
    #[serde(rename = "next_key", default)]
    pub next_key: String,
    #[serde(rename = "create_time_range", default)]
    pub create_time_range: TimeRange,
    #[serde(rename = "update_time_range", default)]
    pub update_time_range: TimeRange,
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "openid", default)]
    pub openid: i32,
}
