//! 对应 Java `com.github.binarywang.wxpay.bean.result.WxPayRefundV3Result.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxPayRefundV3Result {
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
        rename = "transaction_id"
    )]
    pub transaction_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "out_trade_no"
    )]
    pub out_trade_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "channel")]
    pub channel: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "user_received_account"
    )]
    pub user_received_account: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "success_time"
    )]
    pub success_time: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "create_time"
    )]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "status")]
    pub status: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "funds_account"
    )]
    pub funds_account: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "amount")]
    pub amount: Option<Amount>,
    #[serde(default, rename = "promotion_detail")]
    pub promotion_detail: Vec<PromotionDetail>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Amount {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "total")]
    pub total: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "refund")]
    pub refund: Option<i32>,
    #[serde(default, rename = "from")]
    pub from: Vec<From>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "payer_total"
    )]
    pub payer_total: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "payer_refund"
    )]
    pub payer_refund: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "settlement_refund"
    )]
    pub settlement_refund: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "settlement_total"
    )]
    pub settlement_total: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "discount_refund"
    )]
    pub discount_refund: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "currency")]
    pub currency: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "refund_fee"
    )]
    pub refund_fee: Option<i32>,
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
    #[serde(default, rename = "goods_detail")]
    pub goods_detail: Vec<GoodsDetail>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct From {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "account")]
    pub account: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "amount")]
    pub amount: Option<i32>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GoodsDetail {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "merchant_goods_id"
    )]
    pub merchant_goods_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "wechatpay_goods_id"
    )]
    pub wechatpay_goods_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "goods_name"
    )]
    pub goods_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "unit_price"
    )]
    pub unit_price: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "refund_amount"
    )]
    pub refund_amount: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "refund_quantity"
    )]
    pub refund_quantity: Option<i32>,
}
