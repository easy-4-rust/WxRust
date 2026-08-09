//! 对应 Java `com.github.binarywang.wxpay.bean.result.WxPayBillResult.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxPayBillResult {
    #[serde(default, rename = "billInfoList")]
    pub bill_info_list: Vec<WxPayBillInfo>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "totalRecord"
    )]
    pub total_record: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "totalFee")]
    pub total_fee: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "totalRefundFee"
    )]
    pub total_refund_fee: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "totalCouponFee"
    )]
    pub total_coupon_fee: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "totalPoundageFee"
    )]
    pub total_poundage_fee: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "totalAmount"
    )]
    pub total_amount: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "totalAppliedRefundFee"
    )]
    pub total_applied_refund_fee: Option<String>,
}
