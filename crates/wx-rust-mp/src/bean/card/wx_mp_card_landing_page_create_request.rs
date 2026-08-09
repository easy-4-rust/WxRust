//! 对应 Java `bean.card.WxMpCardLandingPageCreateRequest`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpCardLandingPageCreateRequest {
    #[serde(rename = "banner", default)]
    pub banner: String,
    #[serde(rename = "page_title", default)]
    pub title: String,
    #[serde(rename = "can_share", default)]
    pub can_share: bool,
    #[serde(rename = "scene", default)]
    pub scene: String,
    #[serde(rename = "card_list", default)]
    pub card_list: serde_json::Value,
}
