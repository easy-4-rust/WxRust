//! 对应 Java `me.chanjar.weixin.channel.bean.after.AfterSaleRejectReason.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AfterSaleRejectReason {
    #[serde(rename = "reject_reason_type", default)]
    pub reject_reason_type: i32,
    #[serde(rename = "reject_reason_type_text", default)]
    pub reject_reason_type_text: String,
    #[serde(rename = "reject_reason", default)]
    pub reject_reason: String,
    #[serde(rename = "reject_scene", default)]
    pub reject_scene: i32,
}
