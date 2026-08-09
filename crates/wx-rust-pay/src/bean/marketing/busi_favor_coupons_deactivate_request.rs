//! 对应 Java `com.github.binarywang.wxpay.bean.marketing.BusiFavorCouponsDeactivateRequest.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BusiFavorCouponsDeactivateRequest {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "coupon_code"
    )]
    pub coupon_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "stock_id")]
    pub stock_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "deactivate_request_no"
    )]
    pub deactivate_request_no: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "deactivate_reason"
    )]
    pub deactivate_reason: Option<String>,
}
