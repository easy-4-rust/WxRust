//! 对应 Java `com.github.binarywang.wxpay.bean.customs.DeclarationQueryResult.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeclarationQueryResult {
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
        rename = "verify_department"
    )]
    pub verify_department: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "verify_department_trade_id"
    )]
    pub verify_department_trade_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "offset")]
    pub offset: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "limit")]
    pub limit: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "total_count"
    )]
    pub total_count: Option<i32>,
    #[serde(default, rename = "data")]
    pub data: Vec<DeclarationData>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeclarationData {
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
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "mch_customs_no"
    )]
    pub merchant_customs_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "customs")]
    pub customs: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "duty")]
    pub duty: Option<i32>,
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
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "state")]
    pub state: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "explanation"
    )]
    pub explanation: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "modify_time"
    )]
    pub modify_time: Option<String>,
}
