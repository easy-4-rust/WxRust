//! 对应 Java `me.chanjar.weixin.cp.bean.oa.doc.WxCpDocSheetBatchUpdateRequest.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::oa::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpDocSheetBatchUpdateRequest {
    #[serde(rename = "docid", default)]
    pub doc_id: String,
    #[serde(rename = "requests", default)]
    pub requests: Vec<Request>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Request {
    #[serde(rename = "add_sheet_request", default)]
    pub add_sheet_request: AddSheetRequest,
    #[serde(rename = "delete_sheet_request", default)]
    pub delete_sheet_request: DeleteSheetRequest,
    #[serde(rename = "update_range_request", default)]
    pub update_range_request: UpdateRangeRequest,
    #[serde(rename = "delete_dimension_request", default)]
    pub delete_dimension_request: DeleteDimensionRequest,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddSheetRequest {
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "row_count", default)]
    pub row_count: i32,
    #[serde(rename = "column_count", default)]
    pub column_count: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeleteSheetRequest {
    #[serde(rename = "sheet_id", default)]
    pub sheet_id: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateRangeRequest {
    #[serde(rename = "sheet_id", default)]
    pub sheet_id: String,
    #[serde(rename = "grid_data", default)]
    pub grid_data: crate::bean::oa::doc::wx_cp_doc_sheet_data::GridData,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeleteDimensionRequest {
    #[serde(rename = "sheet_id", default)]
    pub sheet_id: String,
    #[serde(rename = "dimension", default)]
    pub dimension: String,
    #[serde(rename = "start_index", default)]
    pub start_index: i32,
    #[serde(rename = "end_index", default)]
    pub end_index: i32,
}

impl WxCpDocSheetBatchUpdateRequest {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json)
            .map_err(|e| format!("WxCpDocSheetBatchUpdateRequest 解析失败: {e}"))
    }
}

impl WxCpDocSheetBatchUpdateRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxCpDocSheetBatchUpdateRequest 序列化失败: {e}"))
    }
}
