//! 对应 Java `com.github.binarywang.wxpay.bean.marketing.FavorStocksCreateRequest.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FavorStocksCreateRequest {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "stock_name"
    )]
    pub stock_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "comment")]
    pub comment: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "belong_merchant"
    )]
    pub belong_merchant: Option<String>,
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
        rename = "stock_use_rule"
    )]
    pub stock_use_rule: Option<StockUseRule>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "pattern_info"
    )]
    pub pattern_info: Option<PatternInfo>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "coupon_use_rule"
    )]
    pub coupon_use_rule: Option<CouponUseRule>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "no_cash")]
    pub no_cash: Option<bool>,
    #[serde(default, rename = "stock_type")]
    pub stock_type: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "out_request_no"
    )]
    pub out_request_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ext_info")]
    pub ext_info: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StockUseRule {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "max_coupons"
    )]
    pub max_coupons: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "max_amount"
    )]
    pub max_amount: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "max_amount_by_day"
    )]
    pub max_amount_by_day: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "max_coupons_per_user"
    )]
    pub max_coupons_per_user: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "natural_person_limit"
    )]
    pub natural_person_limit: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "prevent_api_abuse"
    )]
    pub prevent_api_abuse: Option<bool>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PatternInfo {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "description"
    )]
    pub description: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "merchant_logo"
    )]
    pub merchant_logo: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "merchant_name"
    )]
    pub merchant_name: Option<String>,
    #[serde(default, rename = "background_color")]
    pub background_color: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "coupon_image"
    )]
    pub coupon_image: Option<String>,
    #[serde(default, rename = "jump_target")]
    pub jump_target: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "mini_program_appid"
    )]
    pub mini_program_appid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "mini_program_path"
    )]
    pub mini_program_path: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CouponUseRule {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "fixed_normal_coupon"
    )]
    pub fixed_normal_coupon: Option<FixedNormalCoupon>,
    #[serde(default, rename = "goods_tag")]
    pub goods_tag: Vec<Option<String>>,
    #[serde(default, rename = "limit_pay")]
    pub limit_pay: Vec<Option<String>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "limit_card"
    )]
    pub limit_card: Option<LimitCard>,
    #[serde(default, rename = "trade_type")]
    pub trade_type: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "combine_use"
    )]
    pub combine_use: Option<bool>,
    #[serde(default, rename = "available_items")]
    pub available_items: Vec<Option<String>>,
    #[serde(default, rename = "available_merchants")]
    pub available_merchants: Vec<Option<String>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FixedNormalCoupon {
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

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LimitCard {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "name")]
    pub name: Option<String>,
    #[serde(default, rename = "bin")]
    pub bin: Vec<Option<String>>,
}
