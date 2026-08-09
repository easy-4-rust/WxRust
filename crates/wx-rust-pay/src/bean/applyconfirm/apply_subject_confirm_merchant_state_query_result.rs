//! 对应 Java `com.github.binarywang.wxpay.bean.applyconfirm.ApplySubjectConfirmMerchantStateQueryResult.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApplySubjectConfirmMerchantStateQueryResult {
    #[serde(default, rename = "authorize_state")]
    pub applyment_state: String,
}
