//! 对应 Java `me.chanjar.weixin.cp.bean.external.WxCpGetMomentTaskResult.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpGetMomentTaskResult {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "type", default)]
    pub r#type: String,
    #[serde(rename = "result", default)]
    pub result: TaskResult,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TaskResult {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "moment_id", default)]
    pub moment_id: String,
    #[serde(rename = "invalid_sender_list", default)]
    pub invalid_sender_list: crate::bean::external::moment::sender_list::SenderList,
    #[serde(rename = "invalid_external_contact_list", default)]
    pub invalid_external_contact_list:
        crate::bean::external::moment::external_contact_list::ExternalContactList,
}

impl WxCpGetMomentTaskResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpGetMomentTaskResult 解析失败: {e}"))
    }
}

impl WxCpGetMomentTaskResult {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpGetMomentTaskResult 序列化失败: {e}"))
    }
}
