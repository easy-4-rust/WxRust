//! 对应 Java `com.github.binarywang.wxpay.bean.marketing.FavorStocksGetResult.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FavorStocksGetResult {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "rawJsonString"
    )]
    pub raw_json_string: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "stock_id")]
    pub stock_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "stock_creator_mchid"
    )]
    pub stock_creator_mch_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "stock_name"
    )]
    pub stock_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "status")]
    pub status: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "create_time"
    )]
    pub create_time: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "description"
    )]
    pub description: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "stock_use_rule"
    )]
    pub stock_use_rule: Option<StockUseRule>,
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
        rename = "distributed_coupons"
    )]
    pub distributed_coupons: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "no_cash")]
    pub no_cash: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "start_time"
    )]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "stop_time")]
    pub stop_time: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "cut_to_message"
    )]
    pub cut_to_message: Option<CutToMessage>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "singleitem"
    )]
    pub single_item: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "stock_type"
    )]
    pub stock_type: Option<String>,
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
pub struct StockUseRule {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "max_coupons"
    )]
    pub max_coupons: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "max_amount"
    )]
    pub max_amount: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "max_amount_by_day"
    )]
    pub max_amount_by_day: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "fixed_normal_coupon"
    )]
    pub fixed_normal_coupon: Option<FixedNormalCoupon>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "max_coupons_per_user"
    )]
    pub max_coupons_per_user: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "coupon_type"
    )]
    pub coupon_type: Option<String>,
    #[serde(default, rename = "goods_tag")]
    pub goods_tag: Vec<Option<String>>,
    #[serde(default, rename = "trade_type")]
    pub trade_type: Vec<Option<String>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "combine_use"
    )]
    pub combine_use: Option<bool>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FixedNormalCoupon {
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
