//! 对应 Java `cn.binarywang.wx.miniapp.bean.code.WxMaCodeAuditStatus.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaCodeAuditStatus {
    #[serde(rename = "auditId", default)]
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
    pub submit_audit_time: String,
}

impl WxMaCodeAuditStatus {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxMaCodeAuditStatus 解析失败: {e}"))
    }
}
