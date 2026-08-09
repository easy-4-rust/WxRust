//! 对应 Java `me.chanjar.weixin.cp.bean.oa.wedrive.WxCpFileListRequest.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::oa::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpFileListRequest {
    #[serde(rename = "spaceid", default)]
    pub space_id: String,
    #[serde(rename = "fatherid", default)]
    pub father_id: String,
    #[serde(rename = "sort_type", default)]
    pub sort_type: i32,
    #[serde(rename = "start", default)]
    pub start: i32,
    #[serde(rename = "limit", default)]
    pub limit: i32,
}

impl WxCpFileListRequest {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpFileListRequest 解析失败: {e}"))
    }
}

impl WxCpFileListRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpFileListRequest 序列化失败: {e}"))
    }
}
