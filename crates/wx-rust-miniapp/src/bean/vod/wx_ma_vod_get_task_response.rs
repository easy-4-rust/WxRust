//! 对应 Java `cn.binarywang.wx.miniapp.bean.vod.WxMaVodGetTaskResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaVodGetTaskResponse {
    #[serde(rename = "errcode", default)]
    pub errcode: i32,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "task_info", default)]
    pub task_info: TaskInfo,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TaskInfo {
    #[serde(rename = "task_type", default)]
    pub task_type: i32,
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "create_time", default)]
    pub create_time: i64,
    #[serde(rename = "finish_time", default)]
    pub finish_time: i64,
    #[serde(rename = "media_id", default)]
    pub media_id: i32,
}

impl WxMaVodGetTaskResponse {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxMaVodGetTaskResponse 序列化失败: {e}"))
    }
}
