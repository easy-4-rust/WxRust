//! 对应 Java `me.chanjar.weixin.cp.bean.external.WxCpGetMomentTask.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpGetMomentTask {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "next_cursor", default)]
    pub next_cursor: String,
    #[serde(rename = "task_list", default)]
    pub task_list: Vec<MomentTaskItem>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MomentTaskItem {
    #[serde(rename = "userid", default)]
    pub user_id: String,
    #[serde(rename = "publish_status", default)]
    pub publish_status: String,
}

impl WxCpGetMomentTask {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpGetMomentTask 解析失败: {e}"))
    }
}

impl WxCpGetMomentTask {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpGetMomentTask 序列化失败: {e}"))
    }
}
