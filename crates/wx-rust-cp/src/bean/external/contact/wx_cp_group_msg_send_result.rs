//! 对应 Java `me.chanjar.weixin.cp.bean.external.contact.WxCpGroupMsgSendResult.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::external::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpGroupMsgSendResult {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "send_list", default)]
    pub send_list: Vec<ExternalContactGroupMsgSendInfo>,
    #[serde(rename = "next_cursor", default)]
    pub next_cursor: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExternalContactGroupMsgSendInfo {
    #[serde(rename = "external_userid", default)]
    pub external_user_id: String,
    #[serde(rename = "chat_id", default)]
    pub chat_id: String,
    #[serde(rename = "userid", default)]
    pub user_id: String,
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "send_time", default)]
    pub send_time: i64,
}

impl WxCpGroupMsgSendResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpGroupMsgSendResult 解析失败: {e}"))
    }
}
