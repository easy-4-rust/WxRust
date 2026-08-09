//! 对应 Java `bean.card.WxMpCard`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpCard {
    #[serde(rename = "cardId", default)]
    pub card_id: String,
    #[serde(rename = "beginTime", default)]
    pub begin_time: i64,
    #[serde(rename = "endTime", default)]
    pub end_time: i64,
    #[serde(rename = "userCardStatus", default)]
    pub user_card_status: String,
    #[serde(rename = "membershipNumber", default)]
    pub membership_number: String,
    #[serde(rename = "code", default)]
    pub code: String,
    #[serde(rename = "bonus", default)]
    pub bonus: i32,
}
