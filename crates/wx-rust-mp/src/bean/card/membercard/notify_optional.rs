//! 对应 Java `bean.card.membercard.NotifyOptional`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NotifyOptional {
    #[serde(rename = "is_notify_bonus", default)]
    pub is_notify_bonus: bool,
    #[serde(rename = "is_notify_balance", default)]
    pub is_notify_balance: bool,
    #[serde(rename = "is_notify_custom_field1", default)]
    pub is_notify_custom_field1: bool,
    #[serde(rename = "is_notify_custom_field2", default)]
    pub is_notify_custom_field2: bool,
    #[serde(rename = "is_notify_custom_field3", default)]
    pub is_notify_custom_field3: bool,
}
