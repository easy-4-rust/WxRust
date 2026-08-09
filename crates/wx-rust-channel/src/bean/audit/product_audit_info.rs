//! 对应 Java `me.chanjar.weixin.channel.bean.audit.ProductAuditInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProductAuditInfo {
    #[serde(rename = "audit_id", default)]
    pub audit_id: String,
    #[serde(rename = "submit_time", default)]
    pub submit_time: String,
    #[serde(rename = "audit_time", default)]
    pub audit_time: String,
    #[serde(rename = "reject_reason", default)]
    pub reject_reason: String,
    #[serde(rename = "func_type", default)]
    pub func_type: i32,
}
