//! 对应 Java `com.github.binarywang.wxpay.bean.request.WxPayPartnerUnifiedOrderV3Request.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxPayPartnerUnifiedOrderV3Request {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sp_appid")]
    pub sp_appid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sp_mchid")]
    pub sp_mch_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sub_appid")]
    pub sub_appid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sub_mchid")]
    pub sub_mch_id: Option<String>,
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
        rename = "time_expire"
    )]
    pub time_expire: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "attach")]
    pub attach: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "notify_url"
    )]
    pub notify_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "goods_tag")]
    pub goods_tag: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "support_fapiao"
    )]
    pub support_fapiao: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "amount")]
    pub amount: Option<Amount>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "payer")]
    pub payer: Option<Payer>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "detail")]
    pub detail: Option<Discount>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "scene_info"
    )]
    pub scene_info: Option<SceneInfo>,
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
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "currency")]
    pub currency: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Payer {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sp_openid")]
    pub sp_openid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "sub_openid"
    )]
    pub sub_openid: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Discount {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "cost_price"
    )]
    pub cost_price: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "invoice_id"
    )]
    pub invoice_id: Option<String>,
    #[serde(default, rename = "goods_detail")]
    pub goods_details: Vec<GoodsDetail>,
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
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "quantity")]
    pub quantity: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "unit_price"
    )]
    pub unit_price: Option<i32>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SceneInfo {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "payer_client_ip"
    )]
    pub payer_client_ip: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "device_id")]
    pub device_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "store_info"
    )]
    pub store_info: Option<StoreInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "h5_info")]
    pub h5_info: Option<H5Info>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StoreInfo {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "id")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "name")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "area_code")]
    pub area_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "address")]
    pub address: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct H5Info {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "app_name")]
    pub app_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "app_url")]
    pub app_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bundle_id")]
    pub bundle_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "package_name"
    )]
    pub package_name: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SettleInfo {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "profit_sharing"
    )]
    pub profit_sharing: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "subsidy_amount"
    )]
    pub subsidy_amount: Option<i32>,
}
