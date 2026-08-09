//! 对应 Java `bean.freepublish.WxMpFreePublishStatus`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpFreePublishStatus {
    #[serde(rename = "publish_id", default)]
    pub publish_id: String,
    #[serde(rename = "publish_status", default)]
    pub publish_status: i32,
    #[serde(rename = "article_id", default)]
    pub article_id: String,
    #[serde(rename = "article_detail", default)]
    pub article_detail: ArticleDetail,
    #[serde(rename = "fail_idx", default)]
    pub fail_idx: Vec<i32>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ArticleDetail {
    #[serde(rename = "count", default)]
    pub count: i32,
    #[serde(rename = "item", default)]
    pub item: Vec<Item>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Item {
    #[serde(rename = "idx", default)]
    pub idx: i32,
    #[serde(rename = "article_url", default)]
    pub article_url: String,
}

impl WxMpFreePublishStatus {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxMpFreePublishStatus 解析失败: {e}"))
    }
}
