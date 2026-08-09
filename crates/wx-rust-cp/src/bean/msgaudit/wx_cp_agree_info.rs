//! 对应 Java `me.chanjar.weixin.cp.bean.msgaudit.WxCpAgreeInfo.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpAgreeInfo {
    #[serde(rename = "errcode", default)]
    pub errcode: i32,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "agreeinfo", default)]
    pub agree_info: Vec<AgreeInfo>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AgreeInfo {
    #[serde(rename = "status_change_time", default)]
    pub status_change_time: i64,
    #[serde(rename = "userid", default)]
    pub userid: String,
    #[serde(rename = "exteranalopenid", default)]
    pub exteranal_open_id: String,
    #[serde(rename = "agree_status", default)]
    pub agree_status: String,
}

impl WxCpAgreeInfo {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpAgreeInfo 解析失败: {e}"))
    }
}

impl WxCpAgreeInfo {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpAgreeInfo 序列化失败: {e}"))
    }
}
