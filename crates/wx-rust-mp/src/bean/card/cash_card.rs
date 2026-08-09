//! 对应 Java `bean.card.CashCard`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CashCard {
    #[serde(rename = "least_cost", default)]
    pub least_cost: i32,
    #[serde(rename = "reduce_cost", default)]
    pub reduce_cost: i32,
}
