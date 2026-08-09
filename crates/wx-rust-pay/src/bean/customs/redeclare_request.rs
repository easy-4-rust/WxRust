//! 对应 Java `com.github.binarywang.wxpay.bean.customs.RedeclareRequest.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RedeclareRequest {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appid")]
    pub appid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mchid")]
    pub mchid: Option<String>,
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
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "sub_order_no"
    )]
    pub sub_order_no: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "sub_order_id"
    )]
    pub sub_order_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "customs")]
    pub customs: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "merchant_customs_no"
    )]
    pub merchant_customs_no: Option<String>,
}
