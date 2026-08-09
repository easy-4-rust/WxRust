//! 对应 Java `me.chanjar.weixin.channel.bean.live.dashboard.LiveItem.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::live::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LiveItem {
    #[serde(rename = "export_id", default)]
    pub export_id: String,
    #[serde(rename = "create_time", default)]
    pub create_time: i64,
}
