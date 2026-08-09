//! 对应 Java `me.chanjar.weixin.cp.bean.school.user.WxCpExternalContact.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::school::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpExternalContact {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "external_contact", default)]
    pub external_contact: ExternalContact,
    #[serde(rename = "follow_user", default)]
    pub followed_users: Vec<WxCpFollowUser>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpFollowUser {
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
    #[serde(rename = "remark_mobiles", default)]
    pub remark_mobiles: Vec<String>,
    #[serde(rename = "remark_corp_name", default)]
    pub remark_corp_name: String,
    #[serde(rename = "tags", default)]
    pub tags: Vec<crate::bean::school::user::wx_cp_external_contact::Tag>,
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
    #[serde(rename = "is_subscribe", default)]
    pub is_subscribe: i32,
    #[serde(rename = "subscriber_info", default)]
    pub subscriber_info: crate::bean::school::user::wx_cp_external_contact::SubscriberInfo,
    #[serde(rename = "external_profile", default)]
    pub external_profile: crate::bean::school::user::wx_cp_external_contact::ExternalProfile,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SubscriberInfo {
    #[serde(rename = "tag_id", default)]
    pub tag_id: Vec<String>,
    #[serde(rename = "remark_mobiles", default)]
    pub remark_mobiles: Vec<String>,
    #[serde(rename = "remark", default)]
    pub remark: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExternalProfile {
    #[serde(rename = "external_attr", default)]
    pub external_attrs: Vec<crate::bean::school::user::wx_cp_external_contact::ExternalAttribute>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExternalAttribute {
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "text", default)]
    pub text: crate::bean::school::user::wx_cp_external_contact::Text,
    #[serde(rename = "web", default)]
    pub web: crate::bean::school::user::wx_cp_external_contact::Web,
    #[serde(rename = "miniprogram", default)]
    pub mini_program: crate::bean::school::user::wx_cp_external_contact::MiniProgram,
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

impl WxCpExternalContact {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpExternalContact 解析失败: {e}"))
    }
}

impl WxCpExternalContact {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpExternalContact 序列化失败: {e}"))
    }
}
