//! 对应 Java `com.github.binarywang.wxpay.bean.applyment.ApplymentStateQueryResult.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApplymentStateQueryResult {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "business_code"
    )]
    pub business_code: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "applyment_id"
    )]
    pub applyment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sub_mchid")]
    pub sub_mchid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sign_url")]
    pub sign_url: Option<String>,
    #[serde(default, rename = "applyment_state")]
    pub applyment_state: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "applyment_state_msg"
    )]
    pub applyment_state_msg: Option<String>,
    #[serde(default, rename = "audit_detail")]
    pub audit_detail: Vec<AuditDetail>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AuditDetail {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "field")]
    pub field: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "field_name"
    )]
    pub field_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "reject_reason"
    )]
    pub reject_reason: Option<String>,
}
