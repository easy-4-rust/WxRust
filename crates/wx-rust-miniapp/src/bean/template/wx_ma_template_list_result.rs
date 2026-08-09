//! 对应 Java `cn.binarywang.wx.miniapp.bean.template.WxMaTemplateListResult.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaTemplateListResult {
    #[serde(rename = "list", default)]
    pub list: Vec<TemplateInfo>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TemplateInfo {
    #[serde(rename = "template_id", default)]
    pub template_id: String,
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "content", default)]
    pub content: String,
    #[serde(rename = "example", default)]
    pub example: String,
}

impl WxMaTemplateListResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxMaTemplateListResult 解析失败: {e}"))
    }
}
