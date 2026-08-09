//! 对应 Java `me.chanjar.weixin.channel.bean.audit.AuditResult.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AuditResult {
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "reject_reason", default)]
    pub reject_reason: String,
}
