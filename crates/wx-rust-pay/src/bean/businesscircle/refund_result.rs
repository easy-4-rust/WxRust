//! 对应 Java `com.github.binarywang.wxpay.bean.businesscircle.RefundResult.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RefundResult {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mchid")]
    pub mchid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "merchant_name"
    )]
    pub merchant_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "shop_name")]
    pub shop_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "shop_number"
    )]
    pub shop_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appid")]
    pub appid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "openid")]
    pub openid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "refund_time"
    )]
    pub refund_time: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "pay_amount"
    )]
    pub pay_amount: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "refund_amount"
    )]
    pub refund_amount: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "transaction_id"
    )]
    pub transaction_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "refund_id")]
    pub refund_id: Option<String>,
}
