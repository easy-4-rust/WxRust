//! 对应 Java `com.github.binarywang.wxpay.bean.ecommerce.AccountCancelApplicationsResult.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AccountCancelApplicationsResult {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "out_apply_no"
    )]
    pub out_apply_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sub_mchid")]
    pub sub_mchid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "reject_reason"
    )]
    pub reject_reason: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "cancel_state"
    )]
    pub cancel_state: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "update_time"
    )]
    pub update_time: Option<String>,
}
