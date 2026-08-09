//! 对应 Java `com.github.binarywang.wxpay.bean.marketing.FavorCouponsGetResult.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FavorCouponsGetResult {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "rawJsonString"
    )]
    pub raw_json_string: Option<String>,
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
        rename = "cut_to_message"
    )]
    pub cut_to_message: Option<CutToMessage>,
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
        rename = "out_request_no"
    )]
    pub out_request_no: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "normal_coupon_information"
    )]
    pub normal_coupon_information: Option<NormalCouponInformation>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CutToMessage {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "single_price_max"
    )]
    pub single_price_max: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "cut_to_price"
    )]
    pub cut_to_price: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NormalCouponInformation {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "coupon_amount"
    )]
    pub coupon_amount: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "transaction_minimum"
    )]
    pub transaction_minimum: Option<i32>,
}
