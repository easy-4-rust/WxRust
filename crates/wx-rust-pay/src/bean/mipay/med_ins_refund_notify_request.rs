//! 对应 Java `com.github.binarywang.wxpay.bean.mipay.MedInsRefundNotifyRequest.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MedInsRefundNotifyRequest {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sub_mchid")]
    pub sub_mchid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "med_refund_total_fee"
    )]
    pub med_refund_total_fee: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "med_refund_gov_fee"
    )]
    pub med_refund_gov_fee: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "med_refund_self_fee"
    )]
    pub med_refund_self_fee: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "med_refund_other_fee"
    )]
    pub med_refund_other_fee: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "refund_time"
    )]
    pub refund_time: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "out_refund_no"
    )]
    pub out_refund_no: Option<String>,
}
