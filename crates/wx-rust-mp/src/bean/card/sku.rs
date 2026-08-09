//! 对应 Java `bean.card.Sku`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Sku {
    #[serde(rename = "quantity", default)]
    pub quantity: i32,
    #[serde(rename = "total_quantity", default)]
    pub total_quantity: i32,
}
