//! 对应 Java `me.chanjar.weixin.open.bean.WxOpenMaCodeTemplate.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxOpenMaCodeTemplate {
    #[serde(rename = "draftId", alias = "draft_id", default)]
    pub draft_id: i64,
    #[serde(rename = "templateId", alias = "template_id", default)]
    pub template_id: i64,
    #[serde(rename = "userVersion", alias = "user_version", default)]
    pub user_version: String,
    #[serde(rename = "userDesc", alias = "user_desc", default)]
    pub user_desc: String,
    #[serde(rename = "templateType", alias = "template_type", default)]
    pub template_type: i32,
    #[serde(rename = "createTime", alias = "create_time", default)]
    pub create_time: i64,
    #[serde(
        rename = "sourceMiniProgramAppid",
        alias = "source_miniprogram_appid",
        default
    )]
    pub source_mini_program_appid: String,
    #[serde(rename = "sourceMiniProgram", alias = "source_miniprogram", default)]
    pub source_mini_program: String,
    #[serde(rename = "auditScene", alias = "audit_scene", default)]
    pub audit_scene: i32,
    #[serde(rename = "auditStatus", alias = "audit_status", default)]
    pub audit_status: i32,
    #[serde(rename = "reason", default)]
    pub reason: String,
    #[serde(rename = "developer", default)]
    pub developer: String,
}
