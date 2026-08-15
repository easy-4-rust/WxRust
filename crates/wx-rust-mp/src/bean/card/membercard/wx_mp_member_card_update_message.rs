//! 对应 Java `bean.card.membercard.WxMpMemberCardUpdateMessage`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpMemberCardUpdateMessage {
    #[serde(rename = "code", default)]
    pub code: String,
    #[serde(rename = "card_id", default)]
    pub card_id: String,
    #[serde(rename = "background_pic_url", default)]
    pub background_pic_url: String,
    #[serde(rename = "bonus", default)]
    pub bonus: i32,
    #[serde(rename = "add_bonus", default)]
    pub add_bonus: i32,
    #[serde(rename = "record_bonus", default)]
    pub record_bonus: String,
    #[serde(rename = "balance", default)]
    pub balance: f64,
    #[serde(rename = "add_balance", default)]
    pub add_balance: f64,
    #[serde(rename = "record_balance", default)]
    pub record_balance: String,
    #[serde(rename = "custom_field_value1", default)]
    pub custom_field_value1: String,
    #[serde(rename = "custom_field_value2", default)]
    pub custom_field_value2: String,
    #[serde(rename = "custom_field_value3", default)]
    pub custom_field_value3: String,
    #[serde(rename = "notify_optional", default)]
    pub notify_optional: NotifyOptional,
}
