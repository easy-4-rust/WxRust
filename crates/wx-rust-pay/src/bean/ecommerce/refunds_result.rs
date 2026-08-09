//! 对应 Java `com.github.binarywang.wxpay.bean.ecommerce.RefundsResult.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RefundsResult {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "refund_id")]
    pub refund_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "out_refund_no"
    )]
    pub out_refund_no: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "create_time"
    )]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "amount")]
    pub amount: Option<Amount>,
    #[serde(default, rename = "promotion_detail")]
    pub promotion_detail: Vec<PromotionDetail>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Amount {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "refund")]
    pub refund: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "payer_refund"
    )]
    pub payer_refund: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "discount_refund"
    )]
    pub discount_refund: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "currency")]
    pub currency: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PromotionDetail {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "promotion_id"
    )]
    pub promotion_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scope")]
    pub scope: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "amount")]
    pub amount: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "refund_amount"
    )]
    pub refund_amount: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "refund_account"
    )]
    pub refund_account: Option<String>,
}
