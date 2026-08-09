//! 对应 Java `bean.invoice.merchant.MerchantContactInfoWrapper`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
use crate::bean::invoice::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MerchantContactInfoWrapper {
    #[serde(rename = "contact", default)]
    pub contact: MerchantContactInfo,
}
