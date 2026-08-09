//! 对应 Java `me.chanjar.weixin.cp.bean.oa.doc.WxCpDocSheetProperties.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::oa::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpDocSheetProperties {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "properties", default)]
    pub properties: Vec<Properties>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Properties {
    #[serde(rename = "sheet_id", default)]
    pub sheet_id: String,
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "row_count", default)]
    pub row_count: i32,
    #[serde(rename = "column_count", default)]
    pub column_count: i32,
}

impl WxCpDocSheetProperties {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpDocSheetProperties 解析失败: {e}"))
    }
}

impl WxCpDocSheetProperties {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpDocSheetProperties 序列化失败: {e}"))
    }
}
