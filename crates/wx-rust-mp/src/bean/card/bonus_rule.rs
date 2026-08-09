//! 对应 Java `bean.card.BonusRule`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BonusRule {
    #[serde(rename = "cost_money_unit", default)]
    pub cost_money_unit: i32,
    #[serde(rename = "increase_bonus", default)]
    pub increase_bonus: i32,
    #[serde(rename = "max_increase_bonus", default)]
    pub max_increase_bonus: i32,
    #[serde(rename = "init_increase_bonus", default)]
    pub init_increase_bonus: i32,
    #[serde(rename = "cost_bonus_unit", default)]
    pub cost_bonus_unit: i32,
    #[serde(rename = "reduce_money", default)]
    pub reduce_money: i32,
    #[serde(rename = "least_money_to_use_bonus", default)]
    pub least_money_to_use_bonus: i32,
    #[serde(rename = "max_reduce_bonus", default)]
    pub max_reduce_bonus: i32,
}
