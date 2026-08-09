//! 对应 Java `cn.binarywang.wx.miniapp.bean.code.WxMaCodeSubmitAuditUgcDeclare.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaCodeSubmitAuditUgcDeclare {
    #[serde(rename = "scene", default)]
    pub scene: Vec<i32>,
    #[serde(rename = "other_scene_desc", default)]
    pub other_scene_desc: String,
    #[serde(rename = "method", default)]
    pub method: Vec<i32>,
    #[serde(rename = "has_audit_team", default)]
    pub has_audit_team: i32,
    #[serde(rename = "audit_desc", default)]
    pub audit_desc: String,
}
