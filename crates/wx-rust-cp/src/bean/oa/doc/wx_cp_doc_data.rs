//! 对应 Java `me.chanjar.weixin.cp.bean.oa.doc.WxCpDocData.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::oa::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpDocData {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "docid", default)]
    pub doc_id: String,
    #[serde(rename = "content", default)]
    pub content: serde_json::Value,
    #[serde(rename = "doc_content", default)]
    pub doc_content: serde_json::Value,
    #[serde(rename = "has_more", default)]
    pub has_more: bool,
    #[serde(rename = "next_cursor", default)]
    pub next_cursor: String,
}

impl WxCpDocData {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpDocData 解析失败: {e}"))
    }
}

impl WxCpDocData {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpDocData 序列化失败: {e}"))
    }
}
