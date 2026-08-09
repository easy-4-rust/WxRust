//! 对应 Java `bean.card.WxUserCardListResult`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxUserCardListResult {
    #[serde(rename = "card_list", default)]
    pub card_list: Vec<UserCard>,
    #[serde(rename = "has_share_card", default)]
    pub has_share_card: bool,
}
