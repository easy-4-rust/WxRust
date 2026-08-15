//! 对应 Java `bean.card.membercard.WxMpMemberCardActivatedMessage`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpMemberCardActivatedMessage {
    #[serde(rename = "membership_number", default)]
    pub membership_number: String,
    #[serde(rename = "code", default)]
    pub code: String,
    #[serde(rename = "card_id", default)]
    pub card_id: String,
    #[serde(rename = "background_pic_url", default)]
    pub background_pic_url: String,
    #[serde(rename = "activate_begin_time", default)]
    pub activate_begin_time: i32,
    #[serde(rename = "activate_end_time", default)]
    pub activate_end_time: i32,
    #[serde(rename = "init_bonus", default)]
    pub init_bonus: i32,
    #[serde(rename = "init_bonus_record", default)]
    pub init_bonus_record: String,
    #[serde(rename = "init_balance", default)]
    pub init_balance: f64,
    #[serde(rename = "init_custom_field_value1", default)]
    pub init_custom_field_value1: String,
    #[serde(rename = "init_custom_field_value2", default)]
    pub init_custom_field_value2: String,
    #[serde(rename = "init_custom_field_value3", default)]
    pub init_custom_field_value3: String,
}
