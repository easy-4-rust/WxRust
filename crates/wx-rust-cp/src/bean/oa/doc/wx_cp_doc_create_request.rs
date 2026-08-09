//! 对应 Java `me.chanjar.weixin.cp.bean.oa.doc.WxCpDocCreateRequest.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::oa::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpDocCreateRequest {
    #[serde(rename = "spaceid", default)]
    pub space_id: String,
    #[serde(rename = "fatherid", default)]
    pub father_id: String,
    #[serde(rename = "doc_type", default)]
    pub doc_type: i32,
    #[serde(rename = "doc_name", default)]
    pub doc_name: String,
    #[serde(rename = "admin_users", default)]
    pub admin_users: Vec<String>,
}

impl WxCpDocCreateRequest {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpDocCreateRequest 解析失败: {e}"))
    }
}

impl WxCpDocCreateRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpDocCreateRequest 序列化失败: {e}"))
    }
}
