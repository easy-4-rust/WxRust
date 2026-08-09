//! 对应 Java `cn.binarywang.wx.miniapp.bean.template.WxMaTemplateLibraryGetResult.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaTemplateLibraryGetResult {
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "keyword_list", default)]
    pub keyword_list: Vec<KeywordInfo>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct KeywordInfo {
    #[serde(rename = "keyword_id", default)]
    pub keyword_id: i32,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "example", default)]
    pub example: String,
}

impl WxMaTemplateLibraryGetResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json)
            .map_err(|e| format!("WxMaTemplateLibraryGetResult 解析失败: {e}"))
    }
}
