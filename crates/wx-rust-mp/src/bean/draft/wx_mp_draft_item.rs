//! 对应 Java `bean.draft.WxMpDraftItem`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpDraftItem {
    #[serde(rename = "media_id", default)]
    pub media_id: String,
    #[serde(rename = "content", default)]
    pub content: WxMpDraftInfo,
    #[serde(rename = "update_time", default)]
    pub update_time: i64,
}
