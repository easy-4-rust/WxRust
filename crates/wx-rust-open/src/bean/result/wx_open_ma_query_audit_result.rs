//! 对应 Java `me.chanjar.weixin.open.bean.result.WxOpenMaQueryAuditResult.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxOpenMaQueryAuditResult {
    #[serde(rename = "errcode", default)]
    pub errcode: String,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "auditid", default)]
    pub audit_id: i64,
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "reason", default)]
    pub reason: String,
    #[serde(rename = "screenshot", default)]
    pub screen_shot: String,
    #[serde(rename = "user_version", default)]
    pub user_version: String,
    #[serde(rename = "user_desc", default)]
    pub user_desc: String,
    #[serde(rename = "submit_audit_time", default)]
    pub submit_audit_time: i64,
}
