//! 对应 Java `me.chanjar.weixin.cp.bean.oa.doc.WxCpDocAuthInfo.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::oa::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpDocAuthInfo {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "access_rule", default)]
    pub access_rule: AccessRule,
    #[serde(rename = "secure_setting", default)]
    pub secure_setting: SecureSetting,
    #[serde(rename = "doc_member_list", default)]
    pub doc_member_list: Vec<DocMember>,
    #[serde(rename = "co_auth_list", default)]
    pub co_auth_list: Vec<CoAuthInfo>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AccessRule {
    #[serde(rename = "enable_corp_internal", default)]
    pub enable_corp_internal: bool,
    #[serde(rename = "corp_internal_auth", default)]
    pub corp_internal_auth: i32,
    #[serde(rename = "enable_corp_external", default)]
    pub enable_corp_external: bool,
    #[serde(rename = "corp_external_auth", default)]
    pub corp_external_auth: i32,
    #[serde(rename = "corp_internal_approve_only_by_admin", default)]
    pub corp_internal_approve_only_by_admin: bool,
    #[serde(rename = "corp_external_approve_only_by_admin", default)]
    pub corp_external_approve_only_by_admin: bool,
    #[serde(rename = "ban_share_external", default)]
    pub ban_share_external: bool,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SecureSetting {
    #[serde(rename = "enable_readonly_copy", default)]
    pub enable_readonly_copy: bool,
    #[serde(rename = "enable_readonly_comment", default)]
    pub enable_readonly_comment: bool,
    #[serde(rename = "watermark", default)]
    pub watermark: crate::bean::oa::doc::wx_cp_doc_auth_info::Watermark,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Watermark {
    #[serde(rename = "margin_type", default)]
    pub margin_type: i32,
    #[serde(rename = "show_visitor_name", default)]
    pub show_visitor_name: bool,
    #[serde(rename = "show_text", default)]
    pub show_text: bool,
    #[serde(rename = "text", default)]
    pub text: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DocMember {
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "userid", default)]
    pub user_id: String,
    #[serde(rename = "tmp_external_userid", default)]
    pub tmp_external_user_id: String,
    #[serde(rename = "auth", default)]
    pub auth: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CoAuthInfo {
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "departmentid", default)]
    pub department_id: i64,
    #[serde(rename = "auth", default)]
    pub auth: i32,
}

impl WxCpDocAuthInfo {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpDocAuthInfo 解析失败: {e}"))
    }
}

impl WxCpDocAuthInfo {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpDocAuthInfo 序列化失败: {e}"))
    }
}
