//! 对应 Java `me.chanjar.weixin.cp.bean.external.WxCpUserExternalGroupChatInfo.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpUserExternalGroupChatInfo {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "group_chat", default)]
    pub group_chat: GroupChat,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GroupChat {
    #[serde(rename = "chat_id", default)]
    pub chat_id: String,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "owner", default)]
    pub owner: String,
    #[serde(rename = "create_time", default)]
    pub create_time: i64,
    #[serde(rename = "notice", default)]
    pub notice: String,
    #[serde(rename = "member_list", default)]
    pub member_list: Vec<crate::bean::external::wx_cp_user_external_group_chat_info::GroupMember>,
    #[serde(rename = "admin_list", default)]
    pub admin_list: Vec<crate::bean::external::wx_cp_user_external_group_chat_info::GroupAdmin>,
    #[serde(rename = "member_version", default)]
    pub member_version: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GroupMember {
    #[serde(rename = "userid", default)]
    pub user_id: String,
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "unionid", default)]
    pub union_id: String,
    #[serde(rename = "join_time", default)]
    pub join_time: i64,
    #[serde(rename = "join_scene", default)]
    pub join_scene: i32,
    #[serde(rename = "state", default)]
    pub state: String,
    #[serde(rename = "invitor", default)]
    pub invitor: crate::bean::external::wx_cp_user_external_group_chat_info::Invitor,
    #[serde(rename = "group_nickname", default)]
    pub group_nickname: String,
    #[serde(rename = "name", default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Invitor {
    #[serde(rename = "userid", default)]
    pub user_id: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GroupAdmin {
    #[serde(rename = "userid", default)]
    pub user_id: String,
}

impl WxCpUserExternalGroupChatInfo {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json)
            .map_err(|e| format!("WxCpUserExternalGroupChatInfo 解析失败: {e}"))
    }
}
