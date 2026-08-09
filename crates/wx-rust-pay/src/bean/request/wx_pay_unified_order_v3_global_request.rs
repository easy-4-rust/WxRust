//! 对应 Java `com.github.binarywang.wxpay.bean.request.WxPayUnifiedOrderV3GlobalRequest.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxPayUnifiedOrderV3GlobalRequest {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appid")]
    pub appid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mchid")]
    pub mchid: Option<String>,
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
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "trade_type"
    )]
    pub trade_type: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "merchant_category_code"
    )]
    pub merchant_category_code: Option<String>,
}
