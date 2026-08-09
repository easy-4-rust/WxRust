//! 对应 Java `me.chanjar.weixin.cp.bean.external.contact.ExternalContact.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::external::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExternalContact {
    #[serde(rename = "external_userid", default)]
    pub external_user_id: String,
    #[serde(rename = "position", default)]
    pub position: String,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "nickname", default)]
    pub nickname: String,
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
    pub external_profile: ExternalProfile,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExternalProfile {
    #[serde(rename = "external_corp_name", default)]
    pub external_corp_name: String,
    #[serde(rename = "wechat_channels", default)]
    pub wechat_channels: crate::bean::external::contact::external_contact::WechatChannel,
    #[serde(rename = "external_attr", default)]
    pub external_attrs: Vec<crate::bean::external::contact::external_contact::ExternalAttribute>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WechatChannel {
    #[serde(rename = "nickname", default)]
    pub nickname: String,
    #[serde(rename = "status", default)]
    pub status: i32,
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
