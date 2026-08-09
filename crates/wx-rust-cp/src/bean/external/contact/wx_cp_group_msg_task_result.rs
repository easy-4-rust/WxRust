//! 对应 Java `me.chanjar.weixin.cp.bean.external.contact.WxCpGroupMsgTaskResult.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::external::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpGroupMsgTaskResult {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "task_list", default)]
    pub task_list: Vec<ExternalContactGroupMsgTaskInfo>,
    #[serde(rename = "next_cursor", default)]
    pub next_cursor: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExternalContactGroupMsgTaskInfo {
    #[serde(rename = "userid", default)]
    pub user_id: String,
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "send_time", default)]
    pub send_time: i64,
}

impl WxCpGroupMsgTaskResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpGroupMsgTaskResult 解析失败: {e}"))
    }
}
