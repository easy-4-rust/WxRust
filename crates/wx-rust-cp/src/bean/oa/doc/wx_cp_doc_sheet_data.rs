//! 对应 Java `me.chanjar.weixin.cp.bean.oa.doc.WxCpDocSheetData.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::oa::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpDocSheetData {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "grid_data", default)]
    pub grid_data: GridData,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GridData {
    #[serde(rename = "start_row", default)]
    pub start_row: i32,
    #[serde(rename = "start_column", default)]
    pub start_column: i32,
    #[serde(rename = "rows", default)]
    pub rows: Vec<RowData>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RowData {
    #[serde(rename = "values", default)]
    pub values: Vec<CellData>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CellData {
    #[serde(rename = "cell_value", default)]
    pub cell_value: CellValue,
    #[serde(rename = "cell_format", default)]
    pub cell_format: CellFormat,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CellValue {
    #[serde(rename = "text", default)]
    pub text: String,
    #[serde(rename = "link", default)]
    pub link: Link,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Link {
    #[serde(rename = "url", default)]
    pub url: String,
    #[serde(rename = "text", default)]
    pub text: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CellFormat {
    #[serde(rename = "text_format", default)]
    pub text_format: TextFormat,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TextFormat {
    #[serde(rename = "font", default)]
    pub font: String,
    #[serde(rename = "font_size", default)]
    pub font_size: i32,
    #[serde(rename = "bold", default)]
    pub bold: bool,
    #[serde(rename = "italic", default)]
    pub italic: bool,
    #[serde(rename = "strikethrough", default)]
    pub strikethrough: bool,
    #[serde(rename = "underline", default)]
    pub underline: bool,
    #[serde(rename = "color", default)]
    pub color: Color,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Color {
    #[serde(rename = "red", default)]
    pub red: i32,
    #[serde(rename = "green", default)]
    pub green: i32,
    #[serde(rename = "blue", default)]
    pub blue: i32,
    #[serde(rename = "alpha", default)]
    pub alpha: i32,
}

impl WxCpDocSheetData {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpDocSheetData 解析失败: {e}"))
    }
}

impl WxCpDocSheetData {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpDocSheetData 序列化失败: {e}"))
    }
}
