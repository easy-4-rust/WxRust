//! 对应 Java `me.chanjar.weixin.cp.bean.external.WxCpUserExternalTagGroupInfo.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpUserExternalTagGroupInfo {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "tag_group", default)]
    pub tag_group: TagGroup,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TagGroup {
    #[serde(rename = "group_id", default)]
    pub group_id: String,
    #[serde(rename = "group_name", default)]
    pub group_name: String,
    #[serde(rename = "create_time", default)]
    pub create_time: i64,
    #[serde(rename = "order", default)]
    pub order: i64,
    #[serde(rename = "deleted", default)]
    pub deleted: bool,
    #[serde(rename = "tag", default)]
    pub tag: Vec<crate::bean::external::wx_cp_user_external_tag_group_info::Tag>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Tag {
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "create_time", default)]
    pub create_time: i64,
    #[serde(rename = "order", default)]
    pub order: i64,
    #[serde(rename = "deleted", default)]
    pub deleted: bool,
}

impl WxCpUserExternalTagGroupInfo {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json)
            .map_err(|e| format!("WxCpUserExternalTagGroupInfo 解析失败: {e}"))
    }
}

impl WxCpUserExternalTagGroupInfo {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxCpUserExternalTagGroupInfo 序列化失败: {e}"))
    }
}
