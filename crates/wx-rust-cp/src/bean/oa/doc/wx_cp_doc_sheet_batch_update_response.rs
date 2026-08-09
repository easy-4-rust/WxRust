//! 对应 Java `me.chanjar.weixin.cp.bean.oa.doc.WxCpDocSheetBatchUpdateResponse.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::oa::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpDocSheetBatchUpdateResponse {
    #[serde(rename = "add_sheet_response", default)]
    pub add_sheet_response: AddSheetResponse,
    #[serde(rename = "delete_sheet_response", default)]
    pub delete_sheet_response: DeleteSheetResponse,
    #[serde(rename = "update_range_response", default)]
    pub update_range_response: UpdateRangeResponse,
    #[serde(rename = "delete_dimension_response", default)]
    pub delete_dimension_response: DeleteDimensionResponse,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddSheetResponse {
    #[serde(rename = "properties", default)]
    pub properties: crate::bean::oa::doc::wx_cp_doc_sheet_properties::Properties,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeleteSheetResponse {
    #[serde(rename = "sheet_id", default)]
    pub sheet_id: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateRangeResponse {
    #[serde(rename = "updated_cells", default)]
    pub updated_cells: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeleteDimensionResponse {
    #[serde(rename = "deleted", default)]
    pub deleted: i32,
}

impl WxCpDocSheetBatchUpdateResponse {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json)
            .map_err(|e| format!("WxCpDocSheetBatchUpdateResponse 解析失败: {e}"))
    }
}

impl WxCpDocSheetBatchUpdateResponse {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxCpDocSheetBatchUpdateResponse 序列化失败: {e}"))
    }
}
