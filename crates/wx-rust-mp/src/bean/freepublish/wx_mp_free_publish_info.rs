//! 对应 Java `bean.freepublish.WxMpFreePublishInfo`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpFreePublishInfo {
    #[serde(rename = "news_item", default)]
    pub news_item: Vec<WxMpFreePublishArticles>,
}

impl WxMpFreePublishInfo {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxMpFreePublishInfo 解析失败: {e}"))
    }
}
