//! 对应 Java `com.github.binarywang.wxpay.bean.request.WxPayCodepayRequest.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxPayCodepayRequest {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appid")]
    pub appid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mchid")]
    pub mchid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sp_appid")]
    pub sp_appid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sp_mchid")]
    pub sp_mchid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sub_appid")]
    pub sub_appid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sub_mchid")]
    pub sub_mchid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "description"
    )]
    pub description: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "out_trade_no"
    )]
    pub out_trade_no: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "transaction_id"
    )]
    pub transaction_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "attach")]
    pub attach: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "goods_tag")]
    pub goods_tag: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "support_fapiao"
    )]
    pub support_fapiao: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "payer")]
    pub payer: Option<Payer>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "amount")]
    pub amount: Option<Amount>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "scene_info"
    )]
    pub scene_info: Option<SceneInfo>,
    #[serde(default, rename = "promotion_detail")]
    pub promotion_details: Vec<PromotionDetail>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "settle_info"
    )]
    pub settle_info: Option<SettleInfo>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Amount {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "total")]
    pub total: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "payer_total"
    )]
    pub payer_total: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "currency")]
    pub currency: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "payer_currency"
    )]
    pub payer_currency: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Payer {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "auth_code")]
    pub auth_code: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SceneInfo {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "device_ip")]
    pub device_ip: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "device_id")]
    pub device_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "store_info"
    )]
    pub store_info: Option<StoreInfo>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StoreInfo {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "id")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "out_id")]
    pub out_id: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SettleInfo {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "profit_sharing"
    )]
    pub profit_sharing: Option<bool>,
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
    pub goods_details: Vec<GoodsDetail>,
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
