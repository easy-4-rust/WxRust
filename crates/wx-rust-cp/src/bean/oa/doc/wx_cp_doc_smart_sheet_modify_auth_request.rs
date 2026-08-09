//! 对应 Java `me.chanjar.weixin.cp.bean.oa.doc.WxCpDocSmartSheetModifyAuthRequest.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::oa::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpDocSmartSheetModifyAuthRequest {
    #[serde(rename = "docid", default)]
    pub doc_id: String,
    #[serde(rename = "sheet_id", default)]
    pub sheet_id: String,
    #[serde(rename = "auth_info", default)]
    pub auth_info: serde_json::Value,
}

impl WxCpDocSmartSheetModifyAuthRequest {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json)
            .map_err(|e| format!("WxCpDocSmartSheetModifyAuthRequest 解析失败: {e}"))
    }
}

impl WxCpDocSmartSheetModifyAuthRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxCpDocSmartSheetModifyAuthRequest 序列化失败: {e}"))
    }
}
