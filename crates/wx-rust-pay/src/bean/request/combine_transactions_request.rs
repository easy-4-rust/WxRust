//! 对应 Java `com.github.binarywang.wxpay.bean.request.CombineTransactionsRequest.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CombineTransactionsRequest {
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
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "time_start"
    )]
    pub time_start: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "time_expire"
    )]
    pub time_expire: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "notify_url"
    )]
    pub notify_url: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SceneInfo {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "device_id")]
    pub device_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "payer_client_ip"
    )]
    pub payer_client_ip: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "h5_info")]
    pub h5_info: Option<H5Info>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SubOrders {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mchid")]
    pub mchid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "attach")]
    pub attach: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "amount")]
    pub amount: Option<Amount>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "out_trade_no"
    )]
    pub out_trade_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "goods_tag")]
    pub goods_tag: Option<String>,
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
        rename = "settle_info"
    )]
    pub settle_info: Option<SettleInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sub_appid")]
    pub sub_appid: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CombinePayerInfo {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "openid")]
    pub openid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "sub_openid"
    )]
    pub sub_openid: Option<String>,
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
