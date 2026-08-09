//! 对应 Java `me.chanjar.weixin.cp.bean.oa.doc.WxCpDocSmartSheetResult.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::oa::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpDocSmartSheetResult {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "docid", default)]
    pub doc_id: String,
    #[serde(rename = "sheet_id", default)]
    pub sheet_id: String,
    #[serde(rename = "view_id", default)]
    pub view_id: String,
    #[serde(rename = "sheet", default)]
    pub sheet: serde_json::Value,
    #[serde(rename = "sheet_list", default)]
    pub sheet_list: serde_json::Value,
    #[serde(rename = "properties", default)]
    pub properties: serde_json::Value,
    #[serde(rename = "view", default)]
    pub view: serde_json::Value,
    #[serde(rename = "views", default)]
    pub views: serde_json::Value,
    #[serde(rename = "view_list", default)]
    pub view_list: serde_json::Value,
    #[serde(rename = "field", default)]
    pub field: serde_json::Value,
    #[serde(rename = "fields", default)]
    pub fields: serde_json::Value,
    #[serde(rename = "field_list", default)]
    pub field_list: serde_json::Value,
    #[serde(rename = "record", default)]
    pub record: serde_json::Value,
    #[serde(rename = "records", default)]
    pub records: serde_json::Value,
    #[serde(rename = "record_list", default)]
    pub record_list: serde_json::Value,
    #[serde(rename = "has_more", default)]
    pub has_more: bool,
    #[serde(rename = "next_cursor", default)]
    pub next_cursor: serde_json::Value,
}

impl WxCpDocSmartSheetResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpDocSmartSheetResult 解析失败: {e}"))
    }
}

impl WxCpDocSmartSheetResult {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpDocSmartSheetResult 序列化失败: {e}"))
    }
}
