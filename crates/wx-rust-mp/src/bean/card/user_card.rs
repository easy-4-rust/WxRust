//! 对应 Java `bean.card.UserCard`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UserCard {
    #[serde(rename = "code", default)]
    pub code: String,
    #[serde(rename = "card_id", default)]
    pub card_id: String,
}
