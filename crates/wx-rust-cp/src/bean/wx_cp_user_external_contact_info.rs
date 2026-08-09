//! 对应 Java `me.chanjar.weixin.cp.bean.WxCpUserExternalContactInfo.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpUserExternalContactInfo {
    #[serde(rename = "external_contact", default)]
    pub external_contact: ExternalContact,
    #[serde(rename = "follow_user", default)]
    pub followed_users: Vec<FollowedUser>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExternalContact {
    #[serde(rename = "external_userid", default)]
    pub external_user_id: String,
    #[serde(rename = "position", default)]
    pub position: String,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "avatar", default)]
    pub avatar: String,
    #[serde(rename = "corp_name", default)]
    pub corp_name: String,
    #[serde(rename = "corp_full_name", default)]
    pub corp_full_name: String,
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "gender", default)]
    pub gender: i32,
    #[serde(rename = "unionid", default)]
    pub union_id: String,
    #[serde(rename = "external_profile", default)]
    pub external_profile: crate::bean::wx_cp_user_external_contact_info::ExternalProfile,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExternalProfile {
    #[serde(rename = "external_attr", default)]
    pub external_attrs: Vec<crate::bean::wx_cp_user_external_contact_info::ExternalAttribute>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExternalAttribute {
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "text", default)]
    pub text: Text,
    #[serde(rename = "web", default)]
    pub web: Web,
    #[serde(rename = "miniprogram", default)]
    pub mini_program: MiniProgram,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Text {
    #[serde(rename = "value", default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Web {
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "url", default)]
    pub url: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MiniProgram {
    #[serde(rename = "pagepath", default)]
    pub page_path: String,
    #[serde(rename = "appid", default)]
    pub appid: String,
    #[serde(rename = "title", default)]
    pub title: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FollowedUser {
    #[serde(rename = "userid", default)]
    pub user_id: String,
    #[serde(rename = "remark", default)]
    pub remark: String,
    #[serde(rename = "description", default)]
    pub description: String,
    #[serde(rename = "createtime", default)]
    pub create_time: i64,
    #[serde(rename = "state", default)]
    pub state: String,
    #[serde(rename = "remark_company", default)]
    pub remark_company: String,
    #[serde(rename = "remark_mobiles", default)]
    pub remark_mobiles: Vec<String>,
    #[serde(rename = "tags", default)]
    pub tags: Vec<crate::bean::wx_cp_user_external_contact_info::Tag>,
    #[serde(rename = "add_way", default)]
    pub add_way: i32,
    #[serde(rename = "oper_userid", default)]
    pub oper_userid: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Tag {
    #[serde(rename = "group_name", default)]
    pub group_name: String,
    #[serde(rename = "tag_name", default)]
    pub tag_name: String,
    #[serde(rename = "type", default)]
    pub r#type: i32,
}

impl WxCpUserExternalContactInfo {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpUserExternalContactInfo 解析失败: {e}"))
    }
}
