//! 对应 Java `me.chanjar.weixin.channel.bean.after.RefundInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RefundInfo {
    #[serde(rename = "amount", default)]
    pub amount: i32,
    #[serde(rename = "refund_reason", default)]
    pub refund_reason: i32,
}
