//! 对应 Java `me.chanjar.weixin.cp.bean.oa.doc.WxCpDocInfo.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::oa::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpDocInfo {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "doc_base_info", default)]
    pub doc_base_info: DocInfo,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DocInfo {
    #[serde(rename = "docid", default)]
    pub doc_id: String,
    #[serde(rename = "doc_name", default)]
    pub doc_name: String,
    #[serde(rename = "create_time", default)]
    pub create_time: i64,
    #[serde(rename = "modify_time", default)]
    pub modify_time: i64,
    #[serde(rename = "doc_type", default)]
    pub doc_type: i32,
}

impl WxCpDocInfo {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpDocInfo 解析失败: {e}"))
    }
}

impl WxCpDocInfo {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpDocInfo 序列化失败: {e}"))
    }
}
