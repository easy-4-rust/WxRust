//! 对应 Java `com.binarywang.wxjava.store.bean.audit.AuditApplyResponse.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 AuditApplyResponse；对应 Java com.binarywang.wxjava.store.bean.audit.AuditApplyResponse.java。
pub struct AuditApplyResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "audit_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_id: Option<String>,
}
