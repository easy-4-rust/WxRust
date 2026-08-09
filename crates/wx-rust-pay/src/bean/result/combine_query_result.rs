//! 对应 Java `com.github.binarywang.wxpay.bean.result.CombineQueryResult.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CombineQueryResult {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "combine_appid"
    )]
    pub combine_appid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "combine_mchid"
    )]
    pub combine_mchid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "combine_out_trade_no"
    )]
    pub combine_out_trade_no: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "scene_info"
    )]
    pub scene_info: Option<SceneInfo>,
    #[serde(default, rename = "sub_orders")]
    pub sub_orders: Vec<SubOrders>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "combine_payer_info"
    )]
    pub combine_payer_info: Option<CombinePayerInfo>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SceneInfo {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "device_id")]
    pub device_id: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SubOrders {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mchid")]
    pub mchid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "trade_type"
    )]
    pub trade_type: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "trade_state"
    )]
    pub trade_state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bank_type")]
    pub bank_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "attach")]
    pub attach: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "success_time"
    )]
    pub success_time: Option<String>,
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
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "amount")]
    pub amount: Option<Amount>,
    #[serde(default, rename = "promotion_detail")]
    pub promotion_detail: Vec<PromotionDetail>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CombinePayerInfo {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "openid")]
    pub openid: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Amount {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "total_amount"
    )]
    pub total_amount: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "currency")]
    pub currency: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "payer_amount"
    )]
    pub payer_amount: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "payer_currency"
    )]
    pub payer_currency: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PromotionDetail {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "coupon_id")]
    pub coupon_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "name")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scope")]
    pub scope: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "amount")]
    pub amount: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "stock_id")]
    pub stock_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "wechatpay_contribute"
    )]
    pub wechatpay_contribute: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "merchant_contribute"
    )]
    pub merchant_contribute: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "other_contribute"
    )]
    pub other_contribute: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "currency")]
    pub currency: Option<String>,
    #[serde(default, rename = "goods_detail")]
    pub goods_detail: Vec<GoodsDetail>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GoodsDetail {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "goods_id")]
    pub goods_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "quantity")]
    pub quantity: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "unit_price"
    )]
    pub unit_price: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "discount_amount"
    )]
    pub discount_amount: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "goods_remark"
    )]
    pub goods_remark: Option<String>,
}
