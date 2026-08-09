//! 对应 Java `com.github.binarywang.wxpay.bean.customs.DeclarationRequest.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeclarationRequest {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appid")]
    pub appid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mchid")]
    pub mchid: Option<String>,
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
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "customs")]
    pub customs: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "merchant_customs_no"
    )]
    pub merchant_customs_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "duty")]
    pub duty: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "sub_order_no"
    )]
    pub sub_order_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fee_type")]
    pub fee_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "order_fee")]
    pub order_fee: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "transport_fee"
    )]
    pub transport_fee: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "product_fee"
    )]
    pub product_fee: Option<i32>,
}
