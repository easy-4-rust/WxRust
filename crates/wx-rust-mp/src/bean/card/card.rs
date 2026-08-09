//! 对应 Java `bean.card.Card`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Card {
    #[serde(rename = "base_info", default)]
    pub base_info: BaseInfo,
    #[serde(rename = "advanced_info", default)]
    pub advanced_info: AdvancedInfo,
}
