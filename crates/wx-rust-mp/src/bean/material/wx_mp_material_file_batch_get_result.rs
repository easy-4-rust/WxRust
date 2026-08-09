//! 对应 Java `bean.material.WxMpMaterialFileBatchGetResult`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpMaterialFileBatchGetResult {
    #[serde(rename = "totalCount", default)]
    pub total_count: i32,
    #[serde(rename = "itemCount", default)]
    pub item_count: i32,
    #[serde(rename = "items", default)]
    pub items: Vec<WxMaterialFileBatchGetNewsItem>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaterialFileBatchGetNewsItem {
    #[serde(rename = "mediaId", default)]
    pub media_id: String,
    #[serde(rename = "updateTime", default)]
    pub update_time: String,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "url", default)]
    pub url: String,
}

impl WxMpMaterialFileBatchGetResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json)
            .map_err(|e| format!("WxMpMaterialFileBatchGetResult 解析失败: {e}"))
    }
}
