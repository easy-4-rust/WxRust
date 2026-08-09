//! 对应 Java `com.github.binarywang.wxpay.bean.result.WxPayBillInfo.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxPayBillInfo {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tradeTime")]
    pub trade_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appId")]
    pub app_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mchId")]
    pub mch_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subMchId")]
    pub sub_mch_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "deviceInfo"
    )]
    pub device_info: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "transactionId"
    )]
    pub transaction_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "outTradeNo"
    )]
    pub out_trade_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "openId")]
    pub open_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tradeType")]
    pub trade_type: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "tradeState"
    )]
    pub trade_state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bankType")]
    pub bank_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "feeType")]
    pub fee_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "totalFee")]
    pub total_fee: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "couponFee")]
    pub coupon_fee: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "refundId")]
    pub refund_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "outRefundNo"
    )]
    pub out_refund_no: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "settlementRefundFee"
    )]
    pub settlement_refund_fee: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "couponRefundFee"
    )]
    pub coupon_refund_fee: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "refundChannel"
    )]
    pub refund_channel: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "refundState"
    )]
    pub refund_state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "body")]
    pub body: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "attach")]
    pub attach: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "poundage")]
    pub poundage: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "poundageRate"
    )]
    pub poundage_rate: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "totalAmount"
    )]
    pub total_amount: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "appliedRefundAmount"
    )]
    pub applied_refund_amount: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "feeRemark")]
    pub fee_remark: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "refundTime"
    )]
    pub refund_time: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "refundSuccessTime"
    )]
    pub refund_success_time: Option<String>,
}
