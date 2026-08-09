//! 对应 Java `bean.invoice.merchant.MerchantInvoicePlatformInfo`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
use crate::bean::invoice::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MerchantInvoicePlatformInfo {
    #[serde(rename = "mchid", default)]
    pub mchid: String,
    #[serde(rename = "s_pappid", default)]
    pub s_pappid: String,
}
