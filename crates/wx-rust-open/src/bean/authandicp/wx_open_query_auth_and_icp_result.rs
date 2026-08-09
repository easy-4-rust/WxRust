//! 对应 Java `me.chanjar.weixin.open.bean.authandicp.WxOpenQueryAuthAndIcpResult.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxOpenQueryAuthAndIcpResult {
    #[serde(rename = "errcode", default)]
    pub errcode: String,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "procedure_status", default)]
    pub procedure_status: i32,
    #[serde(rename = "orderid", default)]
    pub order_id: i32,
    #[serde(rename = "refill_reason", default)]
    pub refill_reason: String,
    #[serde(rename = "fail_reason", default)]
    pub fail_reason: String,
    #[serde(rename = "icp_audit", default)]
    pub icp_audit: IcpAudit,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IcpAudit {
    #[serde(rename = "hints", default)]
    pub hints: Vec<Hint>,
    #[serde(rename = "audit_data", default)]
    pub audit_data: AuditData,
    #[serde(rename = "sms_verify_status", default)]
    pub sms_verify_status: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AuditData {
    #[serde(rename = "key_name", default)]
    pub key_name: String,
    #[serde(rename = "error", default)]
    pub error: String,
    #[serde(rename = "suggest", default)]
    pub suggest: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Hint {
    #[serde(rename = "errcode", default)]
    pub errcode: String,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "err_field", default)]
    pub err_field: String,
}
