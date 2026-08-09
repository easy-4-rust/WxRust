//! 对应 Java `bean.material.WxMpMaterialNewsBatchGetResult`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpMaterialNewsBatchGetResult {
    #[serde(rename = "totalCount", default)]
    pub total_count: i32,
    #[serde(rename = "itemCount", default)]
    pub item_count: i32,
    #[serde(rename = "items", default)]
    pub items: Vec<WxMaterialNewsBatchGetNewsItem>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaterialNewsBatchGetNewsItem {
    #[serde(rename = "mediaId", default)]
    pub media_id: String,
    #[serde(rename = "updateTime", default)]
    pub update_time: String,
    #[serde(rename = "content", default)]
    pub content: WxMpMaterialNews,
}

impl WxMpMaterialNewsBatchGetResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json)
            .map_err(|e| format!("WxMpMaterialNewsBatchGetResult 解析失败: {e}"))
    }
}
