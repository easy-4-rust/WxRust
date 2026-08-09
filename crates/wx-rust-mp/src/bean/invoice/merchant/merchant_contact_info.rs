//! 对应 Java `bean.invoice.merchant.MerchantContactInfo`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
use crate::bean::invoice::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MerchantContactInfo {
    #[serde(rename = "phone", default)]
    pub phone: String,
    #[serde(rename = "time_out", default)]
    pub timeout: i32,
}
