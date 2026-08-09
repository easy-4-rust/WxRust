//! 对应 Java `cn.binarywang.wx.miniapp.bean.template.WxMaTemplateLibraryListResult.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaTemplateLibraryListResult {
    #[serde(rename = "total_count", default)]
    pub total_count: i32,
    #[serde(rename = "list", default)]
    pub list: Vec<TemplateItem>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TemplateItem {
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "title", default)]
    pub title: String,
}

impl WxMaTemplateLibraryListResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json)
            .map_err(|e| format!("WxMaTemplateLibraryListResult 解析失败: {e}"))
    }
}
