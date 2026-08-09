//! 对应 Java `com.github.binarywang.wxpay.bean.marketing.FavorCouponsUseResult.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FavorCouponsUseResult {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "stock_creator_mchid"
    )]
    pub stock_creator_mchid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "stock_id")]
    pub stock_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "coupon_id")]
    pub coupon_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "singleitem_discount_off"
    )]
    pub singleitem_discount_off: Option<SingleitemDiscountOff>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "discount_to"
    )]
    pub discount_to: Option<DiscountTo>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "coupon_name"
    )]
    pub coupon_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "status")]
    pub status: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "description"
    )]
    pub description: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "create_time"
    )]
    pub create_time: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "coupon_type"
    )]
    pub coupon_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "no_cash")]
    pub no_cash: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "available_begin_time"
    )]
    pub available_begin_time: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "available_end_time"
    )]
    pub available_end_time: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "singleitem"
    )]
    pub singleitem: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "normal_coupon_information"
    )]
    pub normal_coupon_information: Option<NormalCouponInformation>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "consume_information"
    )]
    pub consume_information: Option<ConsumeInformation>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SingleitemDiscountOff {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "single_price_max"
    )]
    pub single_price_max: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DiscountTo {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "cut_to_price"
    )]
    pub cut_to_price: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "max_price")]
    pub max_price: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NormalCouponInformation {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "coupon_amount"
    )]
    pub coupon_amount: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "transaction_minimum"
    )]
    pub transaction_minimum: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ConsumeInformation {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "consume_time"
    )]
    pub consume_time: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "consume_mchid"
    )]
    pub consume_mchid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "transaction_id"
    )]
    pub transaction_id: Option<String>,
    #[serde(default, rename = "goods_detail")]
    pub goods_detail: Vec<GoodsDetail>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GoodsDetail {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "goods_id")]
    pub goods_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "quantity")]
    pub quantity: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "price")]
    pub price: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "discount_amount"
    )]
    pub discount_amount: Option<i32>,
}
