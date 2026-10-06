//! 对应 Java `com.binarywang.wxjava.store.bean.audit.ProductAuditInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 ProductAuditInfo；对应 Java com.binarywang.wxjava.store.bean.audit.ProductAuditInfo.java。
pub struct ProductAuditInfo {
    #[serde(rename = "audit_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_id: Option<String>,
    #[serde(rename = "submit_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time: Option<String>,
    #[serde(rename = "audit_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_time: Option<String>,
    #[serde(rename = "reject_reason", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject_reason: Option<String>,
    #[serde(rename = "func_type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub func_type: Option<i32>,
}
