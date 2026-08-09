//! 对应 Java `bean.card.GeneralCoupon`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GeneralCoupon {
    #[serde(rename = "default_detail", default)]
    pub default_detail: String,
}
