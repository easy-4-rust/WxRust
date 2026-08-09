//! 对应 Java `me.chanjar.weixin.open.bean.icp.WxOpenIcpEntranceInfoResult.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxOpenIcpEntranceInfoResult {
    #[serde(rename = "errcode", default)]
    pub errcode: String,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "info", default)]
    pub info: Info,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Info {
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "is_canceling", default)]
    pub canceling: bool,
    #[serde(rename = "audit_data", default)]
    pub audit_data: Vec<AuditData>,
    #[serde(rename = "available", default)]
    pub available: i32,
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
