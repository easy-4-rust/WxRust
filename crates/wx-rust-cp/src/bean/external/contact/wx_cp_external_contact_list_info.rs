//! 对应 Java `me.chanjar.weixin.cp.bean.external.contact.WxCpExternalContactListInfo.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::external::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpExternalContactListInfo {
    #[serde(rename = "next_cursor", default)]
    pub next_cursor: String,
    #[serde(rename = "errcode", default)]
    pub errcode: String,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "info_list", default)]
    pub info_list: Vec<ExternalContactInfo>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExternalContactInfo {
    #[serde(rename = "is_customer", default)]
    pub is_customer: bool,
    #[serde(rename = "tmp_openid", default)]
    pub tmp_openid: String,
    #[serde(rename = "external_userid", default)]
    pub external_userid: String,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "follow_userid", default)]
    pub follow_userid: String,
    #[serde(rename = "chat_id", default)]
    pub chat_id: String,
    #[serde(rename = "chat_name", default)]
    pub chat_name: String,
    #[serde(rename = "add_time", default)]
    pub add_time: i64,
}

impl WxCpExternalContactListInfo {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpExternalContactListInfo 解析失败: {e}"))
    }
}
