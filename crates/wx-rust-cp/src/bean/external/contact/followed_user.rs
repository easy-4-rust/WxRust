//! 对应 Java `me.chanjar.weixin.cp.bean.external.contact.FollowedUser.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::external::*;

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
    #[serde(rename = "tag_id", default)]
    pub tag_ids: Vec<String>,
    #[serde(rename = "tags", default)]
    pub tags: Vec<Tag>,
    #[serde(rename = "remark_corp_name", default)]
    pub remark_corp_name: String,
    #[serde(rename = "add_way", default)]
    pub add_way: String,
    #[serde(rename = "oper_userid", default)]
    pub operator_user_id: String,
    #[serde(rename = "wechat_channels", default)]
    pub wechat_channels: WechatChannels,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Tag {
    #[serde(rename = "group_name", default)]
    pub group_name: String,
    #[serde(rename = "tag_name", default)]
    pub tag_name: String,
    #[serde(rename = "tag_id", default)]
    pub tag_id: String,
    #[serde(rename = "type", default)]
    pub r#type: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WechatChannels {
    #[serde(rename = "nickname", default)]
    pub nickname: String,
    #[serde(rename = "source", default)]
    pub source: i32,
}
