//! 对应 Java `me.chanjar.weixin.channel.bean.order.QualityInsepctInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct QualityInsepctInfo {
    #[serde(rename = "inspect_status", default)]
    pub inspect_status: i32,
}
