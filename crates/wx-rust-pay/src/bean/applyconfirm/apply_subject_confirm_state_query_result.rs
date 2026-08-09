//! 对应 Java `com.github.binarywang.wxpay.bean.applyconfirm.ApplySubjectConfirmStateQueryResult.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApplySubjectConfirmStateQueryResult {
    #[serde(default, rename = "applyment_state")]
    pub applyment_state: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "qrcode_data"
    )]
    pub qrcode_data: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "reject_param"
    )]
    pub reject_param: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "reject_reason"
    )]
    pub reject_reason: Option<String>,
}
