//! 对应 Java `bean.card.membercard.MemberCardUpdate`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
use crate::bean::card::enums::*;
use crate::bean::card::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MemberCardUpdate {
    #[serde(rename = "base_info", default)]
    pub base_info: BaseInfoUpdate,
    #[serde(rename = "background_pic_url", default)]
    pub background_pic_url: String,
    #[serde(rename = "supply_bonus", default)]
    pub supply_bonus: bool,
    #[serde(rename = "bonus_cleared", default)]
    pub bonus_cleared: String,
    #[serde(rename = "bonus_rules", default)]
    pub bonus_rules: String,
    #[serde(rename = "bonus_url", default)]
    pub bonus_url: String,
    #[serde(rename = "balance_url", default)]
    pub balance_url: String,
    #[serde(rename = "supply_balance", default)]
    pub supply_balance: bool,
    #[serde(rename = "balance_rules", default)]
    pub balance_rules: String,
    #[serde(rename = "prerogative", default)]
    pub prerogative: String,
    #[serde(rename = "auto_activate", default)]
    pub auto_activate: bool,
    #[serde(rename = "wx_activate", default)]
    pub wx_activate: bool,
    #[serde(rename = "activate_url", default)]
    pub activate_url: String,
    #[serde(rename = "custom_field1", default)]
    pub custom_field1: CustomField,
    #[serde(rename = "custom_field2", default)]
    pub custom_field2: CustomField,
    #[serde(rename = "custom_field3", default)]
    pub custom_field3: CustomField,
    #[serde(rename = "custom_cell1", default)]
    pub custom_cell1: CustomCell1,
    #[serde(rename = "custom_cell2", default)]
    pub custom_cell2: CustomCell1,
    #[serde(rename = "custom_cell3", default)]
    pub custom_cell3: CustomCell1,
    #[serde(rename = "bonus_rule", default)]
    pub bonus_rule: BonusRule,
    #[serde(rename = "discount", default)]
    pub discount: i32,
}
