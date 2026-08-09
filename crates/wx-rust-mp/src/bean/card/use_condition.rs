//! 对应 Java `bean.card.UseCondition`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UseCondition {
    #[serde(rename = "accept_category", default)]
    pub accept_category: String,
    #[serde(rename = "reject_category", default)]
    pub reject_category: String,
    #[serde(rename = "least_cost", default)]
    pub least_cost: i32,
    #[serde(rename = "object_use_for", default)]
    pub object_use_for: String,
    #[serde(rename = "can_use_with_other_discount", default)]
    pub can_use_with_other_discount: bool,
}
